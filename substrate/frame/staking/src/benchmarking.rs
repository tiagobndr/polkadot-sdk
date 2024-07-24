// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Staking pallet benchmarking.

use super::*;
use crate::{migrations::v13_stake_tracker as v13, ConfigOp, Pallet as Staking};
use testing_utils::*;

use codec::Decode;
use frame_election_provider_support::{bounds::DataProviderBounds, SortedListProvider};
use frame_support::{
	assert_ok,
	migrations::SteppedMigration,
	pallet_prelude::*,
	storage::bounded_vec::BoundedVec,
	traits::{Currency, Get, Imbalance, UnfilteredDispatchable},
	weights::WeightMeter,
};
use sp_runtime::{
	traits::{Bounded, One, StaticLookup, TrailingZeroInput, Zero},
	Perbill, Percent, Saturating,
};
use sp_staking::{currency_to_vote::CurrencyToVote, SessionIndex, StakingInterface};

pub use frame_benchmarking::v1::{
	account, impl_benchmark_test_suite, whitelist_account, whitelisted_caller, BenchmarkError,
};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;
const MAX_SPANS: u32 = 100;
const MAX_SLASHES: u32 = 1000;

type MaxValidators<T> = <<T as Config>::BenchmarkingConfig as BenchmarkingConfig>::MaxValidators;
type MaxNominators<T> = <<T as Config>::BenchmarkingConfig as BenchmarkingConfig>::MaxNominators;

// Add slashing spans to a user account. Not relevant for actual use, only to benchmark
// read and write operations.
pub fn add_slashing_spans<T: Config>(who: &T::AccountId, spans: u32) {
	if spans == 0 {
		return
	}

	// For the first slashing span, we initialize
	let mut slashing_spans = crate::slashing::SlashingSpans::new(0);
	SpanSlash::<T>::insert((who, 0), crate::slashing::SpanRecord::default());

	for i in 1..spans {
		assert!(slashing_spans.end_span(i));
		SpanSlash::<T>::insert((who, i), crate::slashing::SpanRecord::default());
	}
	SlashingSpans::<T>::insert(who, slashing_spans);
}

// This function clears all existing validators and nominators from the set, and generates one new
// validator being nominated by n nominators, and returns the validator stash account and the
// nominators' stash and controller. It also starts an era and creates pending payouts.
pub fn create_validator_with_nominators<T: Config>(
	n: u32,
	upper_bound: u32,
	dead_controller: bool,
	unique_controller: bool,
	destination: RewardDestination<T::AccountId>,
) -> Result<(T::AccountId, Vec<(T::AccountId, T::AccountId)>), &'static str> {
	// Clean up any existing state.
	clear_validators_and_nominators::<T>();
	let mut points_total = 0;
	let mut points_individual = Vec::new();

	let (v_stash, v_controller) = if unique_controller {
		create_unique_stash_controller::<T>(0, 100, destination.clone(), false)?
	} else {
		create_stash_controller::<T>(0, 100, destination.clone())?
	};

	let validator_prefs =
		ValidatorPrefs { commission: Perbill::from_percent(50), ..Default::default() };
	Staking::<T>::validate(RawOrigin::Signed(v_controller).into(), validator_prefs)?;
	let stash_lookup = T::Lookup::unlookup(v_stash.clone());

	points_total += 10;
	points_individual.push((v_stash.clone(), 10));

	let original_nominator_count = Nominators::<T>::count();
	let mut nominators = Vec::new();

	// Give the validator n nominators, but keep total users in the system the same.
	for i in 0..upper_bound {
		let (n_stash, n_controller) = if !dead_controller {
			create_stash_controller::<T>(u32::MAX - i, 100, destination.clone())?
		} else {
			create_unique_stash_controller::<T>(u32::MAX - i, 100, destination.clone(), true)?
		};
		if i < n {
			Staking::<T>::nominate(
				RawOrigin::Signed(n_controller.clone()).into(),
				vec![stash_lookup.clone()],
			)?;
			nominators.push((n_stash, n_controller));
		}
	}

	ValidatorCount::<T>::put(1);

	// Start a new Era
	let new_validators = Staking::<T>::try_trigger_new_era(SessionIndex::one(), true).unwrap();

	assert_eq!(new_validators.len(), 1);
	assert_eq!(new_validators[0], v_stash, "Our validator was not selected!");
	assert_ne!(Validators::<T>::count(), 0);
	assert_eq!(Nominators::<T>::count(), original_nominator_count + nominators.len() as u32);

	// Give Era Points
	let reward = EraRewardPoints::<T::AccountId> {
		total: points_total,
		individual: points_individual.into_iter().collect(),
	};

	let current_era = CurrentEra::<T>::get().unwrap();
	ErasRewardPoints::<T>::insert(current_era, reward);

	// Create reward pool
	let total_payout = T::Currency::minimum_balance()
		.saturating_mul(upper_bound.into())
		.saturating_mul(1000u32.into());
	<ErasValidatorReward<T>>::insert(current_era, total_payout);

	Ok((v_stash, nominators))
}

struct ListScenario<T: Config> {
	/// Stash that is expected to be moved.
	origin_stash1: T::AccountId,
	/// Controller of the Stash that is expected to be moved.
	origin_controller1: T::AccountId,
	dest_weight: BalanceOf<T>,
}

impl<T: Config> ListScenario<T> {
	/// An expensive scenario for bags-list implementation:
	///
	/// - the node to be updated (r) is the head of a bag that has at least one other node. The bag
	///   itself will need to be read and written to update its head. The node pointed to by r.next
	///   will need to be read and written as it will need to have its prev pointer updated. Note
	///   that there are two other worst case scenarios for bag removal: 1) the node is a tail and
	///   2) the node is a middle node with prev and next; all scenarios end up with the same number
	///   of storage reads and writes.
	///
	/// - the destination bag has at least one node, which will need its next pointer updated.
	///
	/// NOTE: while this scenario specifically targets a worst case for the bags-list, it should
	/// also elicit a worst case for other known `VoterList` implementations; although
	/// this may not be true against unknown `VoterList` implementations.
	fn new(origin_weight: BalanceOf<T>, is_increase: bool) -> Result<Self, &'static str> {
		ensure!(!origin_weight.is_zero(), "origin weight must be greater than 0");

		// validator to nominate.
		let validator = create_validators_with_seed::<T>(1, 100, 42)?
			.pop()
			.expect("validator should exist");

		// burn the entire issuance.
		let i = T::Currency::burn(T::Currency::total_issuance());
		core::mem::forget(i);

		// create accounts with the origin weight
		let (origin_stash1, origin_controller1) = create_stash_controller_with_balance::<T>(
			USER_SEED + 2,
			origin_weight,
			RewardDestination::Staked,
		)?;

		Staking::<T>::nominate(
			RawOrigin::Signed(origin_controller1.clone()).into(),
			// NOTE: these *do* really need to be validators.
			vec![validator.clone()],
		)?;

		let (_origin_stash2, origin_controller2) = create_stash_controller_with_balance::<T>(
			USER_SEED + 3,
			origin_weight,
			RewardDestination::Staked,
		)?;

		Staking::<T>::nominate(
			RawOrigin::Signed(origin_controller2).into(),
			vec![validator.clone()],
		)?;

		// find a destination weight that will trigger the worst case scenario
		let dest_weight_as_vote =
			T::VoterList::score_update_worst_case(&origin_stash1, is_increase);

		let total_issuance = T::Currency::total_issuance();

		let dest_weight =
			T::CurrencyToVote::to_currency(dest_weight_as_vote as u128, total_issuance);

		// create an account with the worst case destination weight
		let (_dest_stash1, dest_controller1) = create_stash_controller_with_balance::<T>(
			USER_SEED + 1,
			dest_weight,
			RewardDestination::Staked,
		)?;
		Staking::<T>::nominate(RawOrigin::Signed(dest_controller1).into(), vec![validator])?;

		Ok(ListScenario { origin_stash1, origin_controller1, dest_weight })
	}
}

const USER_SEED: u32 = 999666;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn bond() {
		let stash = create_funded_user::<T>("stash", USER_SEED, 100);
		let reward_destination = RewardDestination::Staked;
		let amount = T::Currency::minimum_balance() * 10u32.into();
		whitelist_account!(stash);

		#[extrinsic_call]
		_(RawOrigin::Signed(stash.clone()), amount, reward_destination);

		assert!(Bonded::<T>::contains_key(stash.clone()));
		assert!(Ledger::<T>::contains_key(stash));
	}

	#[benchmark]
	fn bond_extra() -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup the worst case list scenario.

		// the weight the nominator will start at.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;

		let max_additional = scenario.dest_weight - origin_weight;

		let stash = scenario.origin_stash1.clone();
		let controller = scenario.origin_controller1;
		let original_bonded: BalanceOf<T> = Ledger::<T>::get(&controller)
			.map(|l| l.active)
			.ok_or("ledger not created after")?;

		let _ = T::Currency::deposit_into_existing(&stash, max_additional).unwrap();

		whitelist_account!(stash);

		#[extrinsic_call]
		_(RawOrigin::Signed(stash), max_additional);

		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created after")?;
		let new_bonded: BalanceOf<T> = ledger.active;
		assert!(original_bonded < new_bonded);

		Ok(())
	}

	#[benchmark]
	fn unbond() -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		// setup the worst case list scenario.
		let _total_issuance = T::Currency::total_issuance();
		// the weight the nominator will start at. The value used here is expected to be
		// significantly higher than the first position in a list (e.g. the first bag threshold).
		let origin_weight = BalanceOf::<T>::try_from(952_994_955_240_703u128)
			.map_err(|_| "balance expected to be a u128")
			.unwrap();
		let scenario = ListScenario::<T>::new(origin_weight, false)?;

		let _stash = scenario.origin_stash1.clone();
		let controller = scenario.origin_controller1.clone();
		let amount = origin_weight - scenario.dest_weight;
		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created before")?;
		let original_bonded: BalanceOf<T> = ledger.active;

		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller.clone()), amount);

		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created after")?;
		let new_bonded: BalanceOf<T> = ledger.active;
		assert!(original_bonded > new_bonded);

		Ok(())
	}

	// Withdraw only updates the ledger
	#[benchmark]
	fn withdraw_unbonded_update(s: Linear<0, MAX_SPANS>) -> Result<(), BenchmarkError> {
		// Slashing Spans
		let (stash, controller) = create_stash_controller::<T>(0, 100, RewardDestination::Staked)?;
		add_slashing_spans::<T>(&stash, s);
		let amount = T::Currency::minimum_balance() * 5u32.into(); // Half of total
		Staking::<T>::unbond(RawOrigin::Signed(controller.clone()).into(), amount)?;
		CurrentEra::<T>::put(EraIndex::max_value());
		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created before")?;
		let original_total: BalanceOf<T> = ledger.total;
		whitelist_account!(controller);

		#[extrinsic_call]
		withdraw_unbonded(RawOrigin::Signed(controller.clone()), s);

		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created after")?;
		let new_total: BalanceOf<T> = ledger.total;
		assert!(original_total > new_total);

		Ok(())
	}

	// Worst case scenario, everything is removed after the bonding duration
	#[benchmark]
	fn withdraw_unbonded_kill(s: Linear<0, MAX_SPANS>) -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note that we don't care about the setup of the
		// destination position because we are doing a removal from the list but no insert.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let controller = scenario.origin_controller1.clone();
		let stash = scenario.origin_stash1;
		add_slashing_spans::<T>(&stash, s);
		assert!(T::VoterList::contains(&stash));

		let ed = T::Currency::minimum_balance();
		let mut ledger = Ledger::<T>::get(&controller).unwrap();
		ledger.active = ed - One::one();
		Ledger::<T>::insert(&controller, ledger);
		CurrentEra::<T>::put(EraIndex::max_value());

		whitelist_account!(controller);

		#[extrinsic_call]
		withdraw_unbonded(RawOrigin::Signed(controller.clone()), s);

		assert!(!Ledger::<T>::contains_key(controller));
		assert!(!T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn validate() -> Result<(), BenchmarkError> {
		let (stash, controller) = create_stash_controller::<T>(
			MaxNominationsOf::<T>::get() - 1,
			100,
			RewardDestination::Staked,
		)?;
		// because it is chilled.
		assert!(!T::VoterList::contains(&stash));

		let prefs = ValidatorPrefs::default();
		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller), prefs);

		assert!(Validators::<T>::contains_key(&stash));
		assert!(T::VoterList::contains(&stash));

		Ok(())
	}

	// scenario: we want to kick `k` nominators from nominating us (we are a validator).
	// we'll assume that `k` is under 128 for the purposes of determining the slope.
	// each nominator should have `T::MaxNominations::get()` validators nominated, and our validator
	// should be somewhere in there.
	#[benchmark]
	fn kick(k: Linear<1, 128>) -> Result<(), BenchmarkError> {
		// these are the other validators; there are `T::MaxNominations::get() - 1` of them, so
		// there are a total of `T::MaxNominations::get()` validators in the system.
		let rest_of_validators =
			create_validators_with_seed::<T>(MaxNominationsOf::<T>::get() - 1, 100, 415)?;

		// this is the validator that will be kicking.
		let (stash, controller) = create_stash_controller::<T>(
			MaxNominationsOf::<T>::get() - 1,
			100,
			RewardDestination::Staked,
		)?;
		let stash_lookup = T::Lookup::unlookup(stash.clone());

		// they start validating.
		Staking::<T>::validate(RawOrigin::Signed(controller.clone()).into(), Default::default())?;

		// we now create the nominators. there will be `k` of them; each will nominate all
		// validators. we will then kick each of the `k` nominators from the main validator.
		let mut nominator_stashes = Vec::with_capacity(k as usize);
		for i in 0..k {
			// create a nominator stash.
			let (n_stash, n_controller) = create_stash_controller::<T>(
				MaxNominationsOf::<T>::get() + i,
				100,
				RewardDestination::Staked,
			)?;

			// bake the nominations; we first clone them from the rest of the validators.
			let mut nominations = rest_of_validators.clone();
			// then insert "our" validator somewhere in there (we vary it) to avoid accidental
			// optimisations/pessimisations.
			nominations.insert(i as usize % (nominations.len() + 1), stash_lookup.clone());
			// then we nominate.
			Staking::<T>::nominate(RawOrigin::Signed(n_controller.clone()).into(), nominations)?;

			nominator_stashes.push(n_stash);
		}

		// all nominators now should be nominating our validator...
		for n in nominator_stashes.iter() {
			assert!(Nominators::<T>::get(n).unwrap().targets.contains(&stash));
		}

		// we need the unlookuped version of the nominator stash for the kick.
		let kicks = nominator_stashes
			.iter()
			.map(|n| T::Lookup::unlookup(n.clone()))
			.collect::<Vec<_>>();

		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller), kicks);

		// all nominators now should *not* be nominating our validator...
		for n in nominator_stashes.iter() {
			assert!(!Nominators::<T>::get(n).unwrap().targets.contains(&stash));
		}

		Ok(())
	}

	// Worst case scenario, T::MaxNominations::get()
	#[benchmark]
	fn nominate(n: Linear<1, { MaxNominationsOf::<T>::get() }>) -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note we don't care about the destination position,
		// because we are just doing an insert into the origin position.
		let _scenario = ListScenario::<T>::new(origin_weight, true)?;
		let (stash, controller) = create_stash_controller_with_balance::<T>(
			SEED + MaxNominationsOf::<T>::get() + 1, /* make sure the account does not conflict
			                                          * with others */
			origin_weight,
			RewardDestination::Staked,
		)
		.unwrap();

		assert!(!Nominators::<T>::contains_key(&stash));
		assert!(!T::VoterList::contains(&stash));

		let validators = create_validators::<T>(n, 100).unwrap();
		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller), validators);

		assert!(Nominators::<T>::contains_key(&stash));
		assert!(T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn chill() -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note that we don't care about the setup of the
		// destination position because we are doing a removal from the list but no insert.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let controller = scenario.origin_controller1.clone();
		let stash = scenario.origin_stash1;
		assert!(T::VoterList::contains(&stash));

		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller));

		assert!(!T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn set_payee() -> Result<(), BenchmarkError> {
		let (stash, controller) =
			create_stash_controller::<T>(USER_SEED, 100, RewardDestination::Staked)?;
		assert_eq!(Payee::<T>::get(&stash), Some(RewardDestination::Staked));
		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller.clone()), RewardDestination::Account(controller.clone()));

		assert_eq!(Payee::<T>::get(&stash), Some(RewardDestination::Account(controller)));

		Ok(())
	}

	#[benchmark]
	fn update_payee() -> Result<(), BenchmarkError> {
		let (stash, controller) =
			create_stash_controller::<T>(USER_SEED, 100, RewardDestination::Staked)?;
		Payee::<T>::insert(&stash, {
			#[allow(deprecated)]
			RewardDestination::Controller
		});
		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller.clone()), controller.clone());

		assert_eq!(Payee::<T>::get(&stash), Some(RewardDestination::Account(controller)));

		Ok(())
	}

	#[benchmark]
	fn set_controller() -> Result<(), BenchmarkError> {
		let (stash, ctlr) =
			create_unique_stash_controller::<T>(9000, 100, RewardDestination::Staked, false)?;
		// ensure `ctlr` is the currently stored controller.
		assert!(!Ledger::<T>::contains_key(&stash));
		assert!(Ledger::<T>::contains_key(&ctlr));
		assert_eq!(Bonded::<T>::get(&stash), Some(ctlr.clone()));

		whitelist_account!(stash);

		#[extrinsic_call]
		_(RawOrigin::Signed(stash.clone()));

		assert!(Ledger::<T>::contains_key(&stash));

		Ok(())
	}

	#[benchmark]
	fn set_validator_count() {
		let validator_count = MaxValidators::<T>::get();

		#[extrinsic_call]
		_(RawOrigin::Root, validator_count);

		assert_eq!(ValidatorCount::<T>::get(), validator_count);
	}

	#[benchmark]
	fn force_no_eras() {
		#[extrinsic_call]
		_(RawOrigin::Root);
		assert_eq!(ForceEra::<T>::get(), Forcing::ForceNone);
	}

	#[benchmark]
	fn force_new_era() {
		#[extrinsic_call]
		_(RawOrigin::Root);
		assert_eq!(ForceEra::<T>::get(), Forcing::ForceNew);
	}

	#[benchmark]
	fn force_new_era_always() {
		#[extrinsic_call]
		_(RawOrigin::Root);
		assert_eq!(ForceEra::<T>::get(), Forcing::ForceAlways);
	}

	// Worst case scenario, the list of invulnerables is very long.
	#[benchmark]
	fn set_invulnerables(v: Linear<0, { MaxValidators::<T>::get() }>) {
		let mut invulnerables = Vec::new();
		for i in 0..v {
			invulnerables.push(account("invulnerable", i, SEED));
		}

		#[extrinsic_call]
		_(RawOrigin::Root, invulnerables);

		assert_eq!(Invulnerables::<T>::get().len(), v as usize);
	}

	#[benchmark]
	fn deprecate_controller_batch(
		i: Linear<0, { T::MaxControllersInDeprecationBatch::get() }>,
	) -> Result<(), BenchmarkError> {
		let mut controllers: Vec<_> = vec![];
		let mut stashes: Vec<_> = vec![];
		for n in 0..i as u32 {
			let (stash, controller) =
				create_unique_stash_controller::<T>(n, 100, RewardDestination::Staked, false)?;
			controllers.push(controller);
			stashes.push(stash);
		}
		let bounded_controllers: BoundedVec<_, T::MaxControllersInDeprecationBatch> =
			BoundedVec::try_from(controllers.clone()).unwrap();

		#[extrinsic_call]
		_(RawOrigin::Root, bounded_controllers);

		for n in 0..i as u32 {
			let stash = &stashes[n as usize];
			let controller = &controllers[n as usize];
			// Ledger no longer keyed by controller.
			assert_eq!(Ledger::<T>::get(controller), None);
			// Bonded now maps to the stash.
			assert_eq!(Bonded::<T>::get(stash), Some(stash.clone()));
			// Ledger is now keyed by stash.
			assert_eq!(Ledger::<T>::get(stash).unwrap().stash, *stash);
		}

		Ok(())
	}

	#[benchmark]
	fn force_unstake(s: Linear<0, MAX_SPANS>) -> Result<(), BenchmarkError> {
		// Clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note that we don't care about the setup of the
		// destination position because we are doing a removal from the list but no insert.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let controller = scenario.origin_controller1.clone();
		let stash = scenario.origin_stash1;
		assert!(T::VoterList::contains(&stash));
		add_slashing_spans::<T>(&stash, s);

		#[extrinsic_call]
		_(RawOrigin::Root, stash.clone(), s);

		assert!(!Ledger::<T>::contains_key(&controller));
		assert!(!T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn cancel_deferred_slash(s: Linear<1, MAX_SLASHES>) {
		let mut unapplied_slashes = Vec::new();
		let era = EraIndex::one();
		let dummy = || T::AccountId::decode(&mut TrailingZeroInput::zeroes()).unwrap();
		for _ in 0..MAX_SLASHES {
			unapplied_slashes
				.push(UnappliedSlash::<T::AccountId, BalanceOf<T>>::default_from(dummy()));
		}
		UnappliedSlashes::<T>::insert(era, &unapplied_slashes);

		let slash_indices: Vec<u32> = (0..s).collect();

		#[extrinsic_call]
		_(RawOrigin::Root, era, slash_indices);

		assert_eq!(UnappliedSlashes::<T>::get(&era).len(), (MAX_SLASHES - s) as usize);
	}

	#[benchmark]
	fn payout_stakers_alive_staked(
		n: Linear<0, { T::MaxExposurePageSize::get() }>,
	) -> Result<(), BenchmarkError> {
		create_validators_with_nominators_for_era::<T>(
			1500, // 1500 validators/targets.
			1000, // 1000 nominators.
			16,   // nominations per nominator.
			true, // randomize stake.
			None,
		)?;

		// get one validator.
		let validator = Validators::<T>::iter().map(|(v, _)| v).next().unwrap();

		let current_era = CurrentEra::<T>::get().unwrap();
		// set the commission for this particular era as well.
		<ErasValidatorPrefs<T>>::insert(
			current_era,
			validator.clone(),
			<Staking<T>>::validators(&validator),
		);

		let caller = whitelisted_caller();
		let balance_before = T::Currency::free_balance(&validator);
		let mut nominator_balances_before = Vec::new();
		for (stash, _) in &nominators {
			let balance = T::Currency::free_balance(stash);
			nominator_balances_before.push(balance);
		}

		#[extrinsic_call]
		payout_stakers(RawOrigin::Signed(caller), validator.clone(), current_era);

		let balance_after = T::Currency::free_balance(&validator);
		ensure!(
			balance_before < balance_after,
			"Balance of validator stash should have increased after payout.",
		);
		for ((stash, _), balance_before) in nominators.iter().zip(nominator_balances_before.iter())
		{
			let balance_after = T::Currency::free_balance(stash);
			ensure!(
				balance_before < &balance_after,
				"Balance of nominator stash should have increased after payout.",
			);
		}

		Ok(())
	}

	#[benchmark]
	fn rebond(l: Linear<1, { T::MaxUnlockingChunks::get() }>) -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get()
			.max(T::Currency::minimum_balance())
			// we use 100 to play friendly with the list threshold values in the mock
			.max(100u32.into());

		// setup a worst case list scenario.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let dest_weight = scenario.dest_weight;

		// rebond an amount that will give the user dest_weight
		let rebond_amount = dest_weight - origin_weight;

		// spread that amount to rebond across `l` unlocking chunks,
		let value = rebond_amount / l.into();
		// if `value` is zero, we need a greater delta between dest <=> origin weight
		assert_ne!(value, Zero::zero());
		// so the sum of unlocking chunks puts voter into the dest bag.
		assert!(value * l.into() + origin_weight > origin_weight);
		assert!(value * l.into() + origin_weight <= dest_weight);
		let unlock_chunk = UnlockChunk::<BalanceOf<T>> { value, era: EraIndex::zero() };

		let _921stash = scenario.origin_stash1.clone();
		let controller = scenario.origin_controller1;
		let mut staking_ledger = Ledger::<T>::get(controller.clone()).unwrap();

		for _ in 0..l {
			staking_ledger.unlocking.try_push(unlock_chunk.clone()).unwrap()
		}
		Ledger::<T>::insert(controller.clone(), staking_ledger.clone());
		let original_bonded: BalanceOf<T> = staking_ledger.active;

		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller.clone()), rebond_amount);

		let ledger = Ledger::<T>::get(&controller).ok_or("ledger not created after")?;
		let new_bonded: BalanceOf<T> = ledger.active;
		assert!(original_bonded < new_bonded);

		Ok(())
	}

	#[benchmark]
	fn reap_stash(s: Linear<1, MAX_SPANS>) -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note that we don't care about the setup of the
		// destination position because we are doing a removal from the list but no insert.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let controller = scenario.origin_controller1.clone();
		let stash = scenario.origin_stash1;

		add_slashing_spans::<T>(&stash, s);
		let l = StakingLedger::<T>::new(stash.clone(), T::Currency::minimum_balance() - One::one());
		Ledger::<T>::insert(&controller, l);

		assert!(Bonded::<T>::contains_key(&stash));
		assert!(T::VoterList::contains(&stash));

		whitelist_account!(controller);

		#[extrinsic_call]
		_(RawOrigin::Signed(controller), stash.clone(), s);

		assert!(!Bonded::<T>::contains_key(&stash));
		assert!(!T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn new_era(v: Linear<1, 10>, n: Linear<0, 100>) -> Result<(), BenchmarkError> {
		create_validators_with_nominators_for_era::<T>(
			v,
			n,
			MaxNominationsOf::<T>::get() as usize,
			false,
			None,
		)?;
		let session_index = SessionIndex::one();

		#[block]
		{
			let validators =
				Staking::<T>::try_trigger_new_era(session_index, true).ok_or("`new_era` failed")?;
			assert!(validators.len() == v as usize);
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn payout_all(v: Linear<1, 10>, n: Linear<0, 100>) -> Result<(), BenchmarkError> {
		create_validators_with_nominators_for_era::<T>(
			v,
			n,
			MaxNominationsOf::<T>::get() as usize,
			false,
			None,
		)?;
		// Start a new Era
		let new_validators = Staking::<T>::try_trigger_new_era(SessionIndex::one(), true).unwrap();
		assert!(new_validators.len() == v as usize);

		let current_era = CurrentEra::<T>::get().unwrap();
		let mut points_total = 0;
		let mut points_individual = Vec::new();
		let mut payout_calls_arg = Vec::new();

		for validator in new_validators.iter() {
			points_total += 10;
			points_individual.push((validator.clone(), 10));
			payout_calls_arg.push((validator.clone(), current_era));
		}

		// Give Era Points
		let reward = EraRewardPoints::<T::AccountId> {
			total: points_total,
			individual: points_individual.into_iter().collect(),
		};

		ErasRewardPoints::<T>::insert(current_era, reward);

		// Create reward pool
		let total_payout = T::Currency::minimum_balance() * 1000u32.into();
		<ErasValidatorReward<T>>::insert(current_era, total_payout);

		let caller: T::AccountId = whitelisted_caller();
		let origin = RawOrigin::Signed(caller);
		let calls: Vec<_> = payout_calls_arg
			.iter()
			.map(|arg| {
				Call::<T>::payout_stakers_by_page {
					validator_stash: arg.0.clone(),
					era: arg.1,
					page: 0,
				}
				.encode()
			})
			.collect();

		#[block]
		{
			for call in calls {
				<Call<T> as Decode>::decode(&mut &*call)
					.expect("call is encoded above, encoding must be correct")
					.dispatch_bypass_filter(origin.clone().into())?;
			}
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn do_slash(l: Linear<1, { T::MaxUnlockingChunks::get() }>) -> Result<(), BenchmarkError> {
		let (stash, controller) = create_stash_controller::<T>(0, 100, RewardDestination::Staked)?;
		let mut staking_ledger = Ledger::<T>::get(controller.clone()).unwrap();
		let unlock_chunk =
			UnlockChunk::<BalanceOf<T>> { value: 1u32.into(), era: EraIndex::zero() };
		for _ in 0..l {
			staking_ledger.unlocking.try_push(unlock_chunk.clone()).unwrap();
		}
		Ledger::<T>::insert(controller, staking_ledger);
		let slash_amount = T::Currency::minimum_balance() * 10u32.into();
		let balance_before = T::Currency::free_balance(&stash);

		#[block]
		{
			crate::slashing::do_slash::<T>(
				&stash,
				slash_amount,
				&mut BalanceOf::<T>::zero(),
				&mut NegativeImbalanceOf::<T>::zero(),
				EraIndex::zero(),
			);
		}

		let balance_after = T::Currency::free_balance(&stash);
		assert!(balance_before > balance_after);

		Ok(())
	}

	#[benchmark]
	fn get_npos_voters(
		v: Linear<{ MaxValidators::<T>::get() / 2 }, { MaxValidators::<T>::get() }>,
		n: Linear<{ MaxNominators::<T>::get() / 2 }, { MaxNominators::<T>::get() }>,
	) -> Result<(), BenchmarkError> {
		let _validators = create_validators_with_nominators_for_era::<T>(
			v,
			n,
			MaxNominationsOf::<T>::get() as usize,
			false,
			None,
		)?
		.into_iter()
		.map(|v| T::Lookup::lookup(v).unwrap())
		.collect::<Vec<_>>();

		assert_eq!(Validators::<T>::count(), v);
		assert_eq!(Nominators::<T>::count(), n);

		let num_voters = (v + n) as usize;

		#[block]
		{
			// default bounds are unbounded.
			let voters = <Staking<T>>::get_npos_voters(DataProviderBounds::default());
			assert_eq!(voters.len(), num_voters);
		}

		Ok(())
	}

	#[benchmark]
	fn get_npos_targets(
		v: Linear<{ MaxValidators::<T>::get() / 2 }, { MaxValidators::<T>::get() }>,
	) -> Result<(), BenchmarkError> {
		// number of nominator intention.
		let n = MaxNominators::<T>::get();

		let _ = create_validators_with_nominators_for_era::<T>(
			v,
			n,
			MaxNominationsOf::<T>::get() as usize,
			false,
			None,
		)?;

		#[block]
		{
			// default bounds are unbounded.
			let targets = <Staking<T>>::get_npos_targets(DataProviderBounds::default());
			assert_eq!(targets.len() as u32, v);
		}

		Ok(())
	}

	#[benchmark]
	fn set_staking_configs_all_set() {
		#[extrinsic_call]
		set_staking_configs(
			RawOrigin::Root,
			ConfigOp::Set(BalanceOf::<T>::max_value()),
			ConfigOp::Set(BalanceOf::<T>::max_value()),
			ConfigOp::Set(u32::MAX),
			ConfigOp::Set(u32::MAX),
			ConfigOp::Set(Percent::max_value()),
			ConfigOp::Set(Perbill::max_value()),
			ConfigOp::Set(Percent::max_value()),
		);

		assert_eq!(MinNominatorBond::<T>::get(), BalanceOf::<T>::max_value());
		assert_eq!(MinValidatorBond::<T>::get(), BalanceOf::<T>::max_value());
		assert_eq!(MaxNominatorsCount::<T>::get(), Some(u32::MAX));
		assert_eq!(MaxValidatorsCount::<T>::get(), Some(u32::MAX));
		assert_eq!(ChillThreshold::<T>::get(), Some(Percent::from_percent(100)));
		assert_eq!(MinCommission::<T>::get(), Perbill::from_percent(100));
		assert_eq!(MaxStakedRewards::<T>::get(), Some(Percent::from_percent(100)));
	}

	#[benchmark]
	fn set_staking_configs_all_remove() {
		#[extrinsic_call]
		set_staking_configs(
			RawOrigin::Root,
			ConfigOp::Remove,
			ConfigOp::Remove,
			ConfigOp::Remove,
			ConfigOp::Remove,
			ConfigOp::Remove,
			ConfigOp::Remove,
			ConfigOp::Remove,
		);

		assert!(!MinNominatorBond::<T>::exists());
		assert!(!MinValidatorBond::<T>::exists());
		assert!(!MaxNominatorsCount::<T>::exists());
		assert!(!MaxValidatorsCount::<T>::exists());
		assert!(!ChillThreshold::<T>::exists());
		assert!(!MinCommission::<T>::exists());
		assert!(!MaxStakedRewards::<T>::exists());
	}

	#[benchmark]
	fn chill_other() -> Result<(), BenchmarkError> {
		// clean up any existing state.
		clear_validators_and_nominators::<T>();

		let origin_weight = MinNominatorBond::<T>::get().max(T::Currency::minimum_balance());

		// setup a worst case list scenario. Note that we don't care about the setup of the
		// destination position because we are doing a removal from the list but no insert.
		let scenario = ListScenario::<T>::new(origin_weight, true)?;
		let _controller = scenario.origin_controller1.clone();
		let stash = scenario.origin_stash1;
		assert!(T::VoterList::contains(&stash));

		Staking::<T>::set_staking_configs(
			RawOrigin::Root.into(),
			ConfigOp::Set(BalanceOf::<T>::max_value()),
			ConfigOp::Set(BalanceOf::<T>::max_value()),
			ConfigOp::Set(0),
			ConfigOp::Set(0),
			ConfigOp::Set(Percent::from_percent(0)),
			ConfigOp::Set(Zero::zero()),
			ConfigOp::Noop,
		)?;

		let caller = whitelisted_caller();

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), stash.clone());

		assert!(!T::VoterList::contains(&stash));

		Ok(())
	}

	#[benchmark]
	fn force_apply_min_commission() -> Result<(), BenchmarkError> {
		// Clean up any existing state
		clear_validators_and_nominators::<T>();

		// Create a validator with a commission of 50%
		let (stash, controller) = create_stash_controller::<T>(1, 1, RewardDestination::Staked)?;
		let validator_prefs =
			ValidatorPrefs { commission: Perbill::from_percent(50), ..Default::default() };
		Staking::<T>::validate(RawOrigin::Signed(controller).into(), validator_prefs)?;

		// Sanity check
		assert_eq!(
			Validators::<T>::get(&stash),
			ValidatorPrefs { commission: Perbill::from_percent(50), ..Default::default() }
		);

		// Set the min commission to 75%
		MinCommission::<T>::set(Perbill::from_percent(75));
		let caller = whitelisted_caller();

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), stash.clone());

		// The validators commission has been bumped to 75%
		assert_eq!(
			Validators::<T>::get(&stash),
			ValidatorPrefs { commission: Perbill::from_percent(75), ..Default::default() }
		);

		Ok(())
	}

	#[benchmark]
	fn set_min_commission() {
		let min_commission = Perbill::max_value();

		#[extrinsic_call]
		_(RawOrigin::Root, min_commission);

		assert_eq!(MinCommission::<T>::get(), Perbill::from_percent(100));
	}

	#[benchmark]
	fn restore_ledger() -> Result<(), BenchmarkError> {
		let (stash, controller) = create_stash_controller::<T>(0, 100, RewardDestination::Staked)?;
		// corrupt ledger.
		Ledger::<T>::remove(controller);

		#[extrinsic_call]
		_(RawOrigin::Root, stash.clone(), None, None, None);

		assert_eq!(Staking::<T>::inspect_bond_state(&stash), Ok(LedgerIntegrityState::Ok));

		Ok(())
	}

	#[benchmark]
	fn drop_dangling_nomination() -> Result<(), BenchmarkError> {
		let caller = account("caller", 0, SEED);
		whitelist_account!(caller);

		let mut targets = create_validators_with_nominators_for_era::<T>(2, 1, 2, false, None)?;
		let dangling_target = T::Lookup::lookup(targets.pop().expect("target_exists."))
			.map_err(|_| "error looking up validator account")?;
		let other_target = T::Lookup::lookup(targets.pop().expect("target_exists."))
			.map_err(|_| "error looking up validator account")?;
		let voter = nominators_of::<T>(dangling_target.clone())?.pop().expect("voter exists");

		assert_eq!(Staking::<T>::status(&dangling_target), Ok(StakerStatus::Validator));
		Pallet::<T>::setup_dangling_target(dangling_target.clone(), voter.clone());

		// now dangling_target is not validating anymore.
		assert!(Staking::<T>::status(&dangling_target).is_err());
		// but the voter is still nominating it.
		assert!(nominators_of::<T>(dangling_target.clone())?.contains(&voter));
		// and the target is still part of the TargetList (thus dangling).
		assert!(T::TargetList::contains(&dangling_target));

		assert_eq!(
			Staking::<T>::status(&voter),
			Ok(StakerStatus::Nominator(vec![other_target.clone(), dangling_target.clone()]))
		);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), voter.clone(), dangling_target.clone());

		// voter is not nominating validator anymore
		assert_eq!(Staking::<T>::status(&voter), Ok(StakerStatus::Nominator(vec![other_target])));
		// target is not in the target list anymore.
		assert!(!T::TargetList::contains(&dangling_target));

		Ok(())
	}

	/// Multi-block (partial) step benchmark for v13 stake-tracker migration.
	///
	/// The setup benchmarks the worst case scenario of a *partial-step* of the MMB migration. The
	/// partial step of this MMB migration consists of migrating one nominator, i.e. fetch all the
	/// nominated targets of one nominator and add them to the target list.
	/// The worst case scenario case should consider potential rebaggings done internally by the
	/// sorted list provider, thus we populate the target list with 1000 validators before the
	/// migration.
	///
	/// Note: a MMB migration step may include *several* partial steps, so each block during MMBs
	/// progresses as much as possible.
	#[benchmark]
	fn v13_mmb_partial_step() {
		let mut meter = WeightMeter::new();

		let n_validators = 1000;
		let n_nominators = 5;

		let _ = create_validators_with_nominators_for_era::<T>(
			n_validators,
			n_nominators,
			16,
			false,
			None,
		)
		.unwrap();

		let targets_count_before = T::TargetList::count();

		// drop targets from target list nominated by the one nominator to be migrated.
		let (_to_migrate, mut nominations) = Nominators::<T>::iter()
			.map(|(n, noms)| (n, noms.targets.into_inner()))
			.next()
			.unwrap();

		// remove duplicates.
		nominations.sort();
		nominations.dedup();

		// drop targets nominated by the first nominator.
		for t in nominations.iter() {
			assert_ok!(T::TargetList::on_remove(&t));
		}

		// confirm targets dropped.
		assert!(T::TargetList::count() < targets_count_before);

		#[block]
		{
			v13::MigrationV13::<T, crate::weights::SubstrateWeight<T>>::step(None, &mut meter)
				.unwrap();
		}

		assert_eq!(T::TargetList::count(), targets_count_before);
	}

	impl_benchmark_test_suite!(
		Staking,
		crate::mock::ExtBuilder::default().has_stakers(true),
		crate::mock::Test,
		exec_name = build_and_execute
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{Balances, ExtBuilder, RuntimeOrigin, Staking, Test};
	use frame_support::assert_ok;

	#[test]
	fn create_validators_with_nominators_for_era_works() {
		ExtBuilder::default().try_state(false).build_and_execute(|| {
			let v = 10;
			let n = 100;

			create_validators_with_nominators_for_era::<Test>(
				v,
				n,
				MaxNominationsOf::<Test>::get() as usize,
				false,
				None,
			)
			.unwrap();

			let count_validators = Validators::<Test>::iter().count();
			let count_nominators = Nominators::<Test>::iter().count();

			assert_eq!(count_validators, Validators::<Test>::count() as usize);
			assert_eq!(count_nominators, Nominators::<Test>::count() as usize);

			assert_eq!(count_validators, v as usize);
			assert_eq!(count_nominators, n as usize);
		});
	}

	#[test]
	fn create_validator_with_nominators_works() {
		ExtBuilder::default().try_state(false).build_and_execute(|| {
			let n = 10;

			let (validator_stash, nominators) = create_validator_with_nominators::<Test>(
				n,
				<<Test as Config>::MaxExposurePageSize as Get<_>>::get(),
				false,
				false,
				RewardDestination::Staked,
			)
			.unwrap();

			assert_eq!(nominators.len() as u32, n);

			let current_era = CurrentEra::<Test>::get().unwrap();

			let original_free_balance = Balances::free_balance(&validator_stash);
			assert_ok!(Staking::payout_stakers_by_page(
				RuntimeOrigin::signed(1337),
				validator_stash,
				current_era,
				0
			));
			let new_free_balance = Balances::free_balance(&validator_stash);

			assert!(original_free_balance < new_free_balance);
		});
	}

	#[test]
	fn add_slashing_spans_works() {
		ExtBuilder::default().try_state(false).build_and_execute(|| {
			let n = 10;

			let (validator_stash, _nominators) = create_validator_with_nominators::<Test>(
				n,
				<<Test as Config>::MaxExposurePageSize as Get<_>>::get(),
				false,
				false,
				RewardDestination::Staked,
			)
			.unwrap();

			// Add 20 slashing spans
			let num_of_slashing_spans = 20;
			add_slashing_spans::<Test>(&validator_stash, num_of_slashing_spans);

			let slashing_spans = SlashingSpans::<Test>::get(&validator_stash).unwrap();
			assert_eq!(slashing_spans.iter().count(), num_of_slashing_spans as usize);
			for i in 0..num_of_slashing_spans {
				assert!(SpanSlash::<Test>::contains_key((&validator_stash, i)));
			}

			// Test everything is cleaned up
			assert_ok!(Staking::kill_stash(&validator_stash, num_of_slashing_spans));
			assert!(SlashingSpans::<Test>::get(&validator_stash).is_none());
			for i in 0..num_of_slashing_spans {
				assert!(!SpanSlash::<Test>::contains_key((&validator_stash, i)));
			}
		});
	}
}
