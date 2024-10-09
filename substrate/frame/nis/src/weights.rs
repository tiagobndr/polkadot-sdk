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

//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-10-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a4847292742a`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/release/wbuild/kitchensink-runtime/kitchensink_runtime.wasm
// --pallet=pallet_nis
// --header=/__w/polkadot-sdk/polkadot-sdk/substrate/HEADER-APACHE2
// --output=/__w/polkadot-sdk/polkadot-sdk/substrate/frame/nis/src/weights.rs
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --template=substrate/.maintain/frame-weight-template.hbs
// --no-storage-info
// --no-min-squares
// --no-median-slopes
// --genesis-builder-policy=none
// --exclude-pallets=pallet_xcm,pallet_xcm_benchmarks::fungible,pallet_xcm_benchmarks::generic,pallet_nomination_pools,pallet_remark,pallet_transaction_storage

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_nis`.
pub trait WeightInfo {
	fn place_bid(l: u32, ) -> Weight;
	fn place_bid_max() -> Weight;
	fn retract_bid(l: u32, ) -> Weight;
	fn fund_deficit() -> Weight;
	fn communify() -> Weight;
	fn privatize() -> Weight;
	fn thaw_private() -> Weight;
	fn thaw_communal() -> Weight;
	fn process_queues() -> Weight;
	fn process_queue() -> Weight;
	fn process_bid() -> Weight;
}

/// Weights for `pallet_nis` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6115 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 63_898_000 picoseconds.
		Weight::from_parts(76_285_070, 51487)
			// Standard Error: 487
			.saturating_add(Weight::from_parts(63_291, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54117`
		//  Estimated: `51487`
		// Minimum execution time: 155_828_000 picoseconds.
		Weight::from_parts(162_482_000, 51487)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6115 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 66_706_000 picoseconds.
		Weight::from_parts(70_002_853, 51487)
			// Standard Error: 340
			.saturating_add(Weight::from_parts(48_269, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:0)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `3593`
		// Minimum execution time: 45_306_000 picoseconds.
		Weight::from_parts(46_157_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533`
		//  Estimated: `3802`
		// Minimum execution time: 103_649_000 picoseconds.
		Weight::from_parts(104_811_000, 3802)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `3802`
		// Minimum execution time: 133_544_000 picoseconds.
		Weight::from_parts(135_914_000, 3802)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3802`
		// Minimum execution time: 71_703_000 picoseconds.
		Weight::from_parts(73_840_000, 3802)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `642`
		//  Estimated: `3675`
		// Minimum execution time: 131_242_000 picoseconds.
		Weight::from_parts(134_492_000, 3675)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6563`
		//  Estimated: `7487`
		// Minimum execution time: 29_321_000 picoseconds.
		Weight::from_parts(30_781_000, 7487)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `51487`
		// Minimum execution time: 4_353_000 picoseconds.
		Weight::from_parts(4_527_000, 51487)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:0 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_025_000 picoseconds.
		Weight::from_parts(7_215_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6115 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 63_898_000 picoseconds.
		Weight::from_parts(76_285_070, 51487)
			// Standard Error: 487
			.saturating_add(Weight::from_parts(63_291, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54117`
		//  Estimated: `51487`
		// Minimum execution time: 155_828_000 picoseconds.
		Weight::from_parts(162_482_000, 51487)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6115 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 66_706_000 picoseconds.
		Weight::from_parts(70_002_853, 51487)
			// Standard Error: 340
			.saturating_add(Weight::from_parts(48_269, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:0)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `3593`
		// Minimum execution time: 45_306_000 picoseconds.
		Weight::from_parts(46_157_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533`
		//  Estimated: `3802`
		// Minimum execution time: 103_649_000 picoseconds.
		Weight::from_parts(104_811_000, 3802)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `3802`
		// Minimum execution time: 133_544_000 picoseconds.
		Weight::from_parts(135_914_000, 3802)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(337), added: 2812, mode: `MaxEncodedLen`)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3802`
		// Minimum execution time: 71_703_000 picoseconds.
		Weight::from_parts(73_840_000, 3802)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `642`
		//  Estimated: `3675`
		// Minimum execution time: 131_242_000 picoseconds.
		Weight::from_parts(134_492_000, 3675)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6563`
		//  Estimated: `7487`
		// Minimum execution time: 29_321_000 picoseconds.
		Weight::from_parts(30_781_000, 7487)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `51487`
		// Minimum execution time: 4_353_000 picoseconds.
		Weight::from_parts(4_527_000, 51487)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Nis::Receipts` (r:0 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_025_000 picoseconds.
		Weight::from_parts(7_215_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
