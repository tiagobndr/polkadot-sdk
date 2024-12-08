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
use tokio::sync::RwLock;

/// The number of recent blocks maintained by the cache.
/// For each block in the cache, we also store the EVM transaction receipts.
pub const CACHE_SIZE: usize = 256;

/// Abstraction over `SubstrateBlock` use to test `BlockCache` with `MockBlock`
pub trait Block {
	fn hash(&self) -> H256;
	fn number(&self) -> SubstrateBlockNumber;
}

impl Block for SubstrateBlock {
	fn hash(&self) -> H256 {
		SubstrateBlock::hash(&self)
	}
	fn number(&self) -> u32 {
		SubstrateBlock::number(&self)
	}
}

/// The cache maintains a buffer of the last N blocks,
#[derive(frame_support::DefaultNoBound)]
pub struct BlockCache<const N: usize, Block> {
	/// A double-ended queue of the last N blocks.
	/// The most recent block is at the back of the queue, and the oldest block is at the front.
	buffer: VecDeque<Arc<Block>>,

	/// A map of blocks by block number.
	blocks_by_number: HashMap<SubstrateBlockNumber, Arc<Block>>,

	/// A map of blocks by block hash.
	blocks_by_hash: HashMap<H256, Arc<Block>>,

	/// A map of receipts by hash.
	receipts_by_hash: HashMap<H256, ReceiptInfo>,

	/// A map of Signed transaction by hash.
	signed_tx_by_hash: HashMap<H256, TransactionSigned>,

	/// A map of receipt hashes by block hash.
	tx_hashes_by_block_and_index: HashMap<H256, HashMap<U256, H256>>,
}

#[derive(frame_support::CloneNoBound, frame_support::DefaultNoBound)]
pub struct BlockCacheProvider<Block = SubstrateBlock> {
	cache: Arc<RwLock<BlockCache<CACHE_SIZE, Block>>>,
}

impl<B: Block> BlockCacheProvider<B> {
	async fn cache(&self) -> tokio::sync::RwLockReadGuard<'_, BlockCache<CACHE_SIZE, B>> {
		self.cache.read().await
	}

	pub async fn ingest(&self, block: B, receipts: Vec<(TransactionSigned, ReceiptInfo)>) {
		let mut cache = self.cache.write().await;
		cache.insert(block, receipts);
	}

	pub async fn latest_block(&self) -> Option<Arc<B>> {
		let cache = self.cache().await;
		cache.buffer.back().cloned()
	}

	pub async fn receipt_by_block_hash_and_index(
		&self,
		block_hash: &H256,
		transaction_index: &U256,
	) -> Option<ReceiptInfo> {
		let cache = self.cache().await;
		let receipt_hash =
			cache.tx_hashes_by_block_and_index.get(block_hash)?.get(transaction_index)?;
		let receipt = cache.receipts_by_hash.get(receipt_hash)?;
		Some(receipt.clone())
	}

	pub async fn receipts_count_per_block(&self, block_hash: &H256) -> Option<usize> {
		let cache = self.cache().await;
		cache.tx_hashes_by_block_and_index.get(block_hash).map(|v| v.len())
	}

	pub async fn block_by_number(&self, number: SubstrateBlockNumber) -> Option<Arc<B>> {
		let cache = self.cache().await;
		cache.blocks_by_number.get(&number).cloned()
	}

	pub async fn block_by_hash(&self, hash: &H256) -> Option<Arc<B>> {
		let cache = self.cache().await;
		cache.blocks_by_hash.get(hash).cloned()
	}

	pub async fn receipt_by_hash(&self, hash: &H256) -> Option<ReceiptInfo> {
		let cache = self.cache().await;
		cache.receipts_by_hash.get(hash).cloned()
	}

	pub async fn signed_tx_by_hash(&self, hash: &H256) -> Option<TransactionSigned> {
		let cache = self.cache().await;
		cache.signed_tx_by_hash.get(hash).cloned()
	}
}

impl<const N: usize, B: Block> BlockCache<N, B> {
	/// Insert an entry into the cache, and prune the oldest entry if the cache is full.
	pub fn insert(&mut self, block: B, receipts: Vec<(TransactionSigned, ReceiptInfo)>) {
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
				.map(|(_, receipt)| (receipt.transaction_index, receipt.transaction_hash))
				.collect::<HashMap<_, _>>();

			self.tx_hashes_by_block_and_index.insert(block.hash(), values);

			self.receipts_by_hash.extend(
				receipts.iter().map(|(_, receipt)| (receipt.transaction_hash, receipt.clone())),
			);

			self.signed_tx_by_hash.extend(
				receipts
					.iter()
					.map(|(signed_tx, receipt)| (receipt.transaction_hash, signed_tx.clone())),
			)
		}

		let block = Arc::new(block);
		self.buffer.push_back(block.clone());
		self.blocks_by_number.insert(block.number(), block.clone());
		self.blocks_by_hash.insert(block.hash(), block);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	struct MockBlock {
		block_number: SubstrateBlockNumber,
		block_hash: H256,
	}

	impl Block for MockBlock {
		fn hash(&self) -> H256 {
			self.block_hash
		}
		fn number(&self) -> u32 {
			self.block_number
		}
	}

	#[test]
	fn cache_insert_and_prune_works() {
		let mut cache = BlockCache::<2, MockBlock>::default();

		let mut insert = |block_number: u8| {
			let block_hash = H256::from([block_number; 32]);
			cache.insert(
				MockBlock { block_number: block_number.into(), block_hash },
				vec![(
					TransactionSigned::default(),
					ReceiptInfo { transaction_hash: block_hash, ..Default::default() },
				)],
			);
		};

		insert(1);
		insert(2);
		insert(3);

		assert_eq!(cache.buffer.len(), 2);
		assert_eq!(cache.blocks_by_number.len(), 2);
		assert_eq!(cache.blocks_by_hash.len(), 2);
		assert_eq!(cache.receipts_by_hash.len(), 2);
		assert_eq!(cache.signed_tx_by_hash.len(), 2);
		assert_eq!(cache.tx_hashes_by_block_and_index.len(), 2);
	}
}
