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

//! Autogenerated weights for `pallet_conviction_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-wiukf8gn-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/conviction-voting/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_conviction_voting`.
pub trait WeightInfo {
	fn vote_new() -> Weight;
	fn vote_existing() -> Weight;
	fn remove_vote() -> Weight;
	fn remove_other_vote() -> Weight;
	fn delegate(r: u32, ) -> Weight;
	fn undelegate(r: u32, ) -> Weight;
	fn unlock() -> Weight;
}

/// Weights for `pallet_conviction_voting` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13141`
		//  Estimated: `219984`
		// Minimum execution time: 135_295_000 picoseconds.
		Weight::from_parts(142_897_000, 219984)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20283`
		//  Estimated: `219984`
		// Minimum execution time: 324_485_000 picoseconds.
		Weight::from_parts(337_467_000, 219984)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20035`
		//  Estimated: `219984`
		// Minimum execution time: 302_574_000 picoseconds.
		Weight::from_parts(315_016_000, 219984)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12742`
		//  Estimated: `30706`
		// Minimum execution time: 65_548_000 picoseconds.
		Weight::from_parts(71_499_000, 30706)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 1]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306 + r * (1628 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 61_383_000 picoseconds.
		Weight::from_parts(70_695_789, 109992)
			// Standard Error: 457_836
			.saturating_add(Weight::from_parts(44_163_910, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 1]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472 + r * (1377 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 33_466_000 picoseconds.
		Weight::from_parts(39_261_420, 109992)
			// Standard Error: 358_545
			.saturating_add(Weight::from_parts(43_197_579, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11800`
		//  Estimated: `30706`
		// Minimum execution time: 87_030_000 picoseconds.
		Weight::from_parts(91_851_000, 30706)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13141`
		//  Estimated: `219984`
		// Minimum execution time: 135_295_000 picoseconds.
		Weight::from_parts(142_897_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20283`
		//  Estimated: `219984`
		// Minimum execution time: 324_485_000 picoseconds.
		Weight::from_parts(337_467_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20035`
		//  Estimated: `219984`
		// Minimum execution time: 302_574_000 picoseconds.
		Weight::from_parts(315_016_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12742`
		//  Estimated: `30706`
		// Minimum execution time: 65_548_000 picoseconds.
		Weight::from_parts(71_499_000, 30706)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 1]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306 + r * (1628 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 61_383_000 picoseconds.
		Weight::from_parts(70_695_789, 109992)
			// Standard Error: 457_836
			.saturating_add(Weight::from_parts(44_163_910, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(107022), added: 109497, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 1]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472 + r * (1377 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 33_466_000 picoseconds.
		Weight::from_parts(39_261_420, 109992)
			// Standard Error: 358_545
			.saturating_add(Weight::from_parts(43_197_579, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11800`
		//  Estimated: `30706`
		// Minimum execution time: 87_030_000 picoseconds.
		Weight::from_parts(91_851_000, 30706)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
