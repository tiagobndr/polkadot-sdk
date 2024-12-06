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

use crate::{
	client::{SubstrateBlock, SubstrateBlockNumber},
	LOG_TARGET,
};
use pallet_revive::evm::{ReceiptInfo, TransactionSigned, H256, U256};
use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
};

/// The number of recent blocks maintained by the cache.
/// For each block in the cache, we also store the EVM transaction receipts.
pub const CACHE_SIZE: usize = 256;

/// The cache maintains a buffer of the last N blocks,
#[derive(Default)]
pub struct BlockCache<const N: usize = CACHE_SIZE> {
	/// A double-ended queue of the last N blocks.
	/// The most recent block is at the back of the queue, and the oldest block is at the front.
	pub buffer: VecDeque<Arc<SubstrateBlock>>,

	/// A map of blocks by block number.
	pub blocks_by_number: HashMap<SubstrateBlockNumber, Arc<SubstrateBlock>>,

	/// A map of blocks by block hash.
	pub blocks_by_hash: HashMap<H256, Arc<SubstrateBlock>>,

	/// A map of receipts by hash.
	pub receipts_by_hash: HashMap<H256, ReceiptInfo>,

	/// A map of Signed transaction by hash.
	pub signed_tx_by_hash: HashMap<H256, TransactionSigned>,

	/// A map of receipt hashes by block hash.
	pub tx_hashes_by_block_and_index: HashMap<H256, HashMap<U256, H256>>,
}

impl<const N: usize> BlockCache<N> {
	pub fn latest_block(&self) -> Option<&Arc<SubstrateBlock>> {
		self.buffer.back()
	}

	/// Insert an entry into the cache, and prune the oldest entry if the cache is full.
	pub fn insert(
		&mut self,
		block: SubstrateBlock,
		receipts: HashMap<H256, (TransactionSigned, ReceiptInfo)>,
	) {
		if self.buffer.len() >= N {
			if let Some(block) = self.buffer.pop_front() {
				log::trace!(target: LOG_TARGET, "Pruning block: {}", block.number());
				let hash = block.hash();
				self.blocks_by_hash.remove(&hash);
				self.blocks_by_number.remove(&block.number());
				self.signed_tx_by_hash.remove(&hash);
				if let Some(entries) = self.tx_hashes_by_block_and_index.remove(&hash) {
					for hash in entries.values() {
						self.receipts_by_hash.remove(hash);
					}
				}
			}
		}
		if !receipts.is_empty() {
			let values = receipts
				.iter()
				.map(|(hash, (_, receipt))| (receipt.transaction_index, *hash))
				.collect::<HashMap<_, _>>();

			self.tx_hashes_by_block_and_index.insert(block.hash(), values);

			self.receipts_by_hash
				.extend(receipts.iter().map(|(hash, (_, receipt))| (*hash, receipt.clone())));

			self.signed_tx_by_hash
				.extend(receipts.iter().map(|(hash, (signed_tx, _))| (*hash, signed_tx.clone())))
		}

		let block = Arc::new(block);
		self.buffer.push_back(block.clone());
		self.blocks_by_number.insert(block.number(), block.clone());
		self.blocks_by_hash.insert(block.hash(), block);
	}
}
