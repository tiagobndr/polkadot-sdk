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
	subxt_client::SrcChainConfig,
	ClientError, LOG_TARGET,
};
use sp_core::H256;
use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
};
use subxt::{backend::legacy::LegacyRpcMethods, OnlineClient};
use tokio::sync::RwLock;

/// The number of recent blocks maintained by the cache.
const CACHE_SIZE: usize = 256;

/// Provides information about a block,
/// This is an abstratction on top of [`SubstrateBlock`] to provide a common interface for
/// [`BlockCache`].
trait BlockInfo {
	/// Returns the block hash.
	fn hash(&self) -> H256;
	/// Returns the block number.
	fn number(&self) -> SubstrateBlockNumber;
}

impl BlockInfo for SubstrateBlock {
	fn hash(&self) -> H256 {
		SubstrateBlock::hash(self)
	}
	fn number(&self) -> u32 {
		SubstrateBlock::number(self)
	}
}

/// The cache maintains a buffer of the last N blocks,
#[derive(frame_support::DefaultNoBound)]
struct BlockCache<const N: usize, Block> {
	/// A double-ended queue of the last N blocks.
	/// The most recent block is at the back of the queue, and the oldest block is at the front.
	buffer: VecDeque<Arc<Block>>,

	/// A map of blocks by block number.
	blocks_by_number: HashMap<SubstrateBlockNumber, Arc<Block>>,

	/// A map of blocks by block hash.
	blocks_by_hash: HashMap<H256, Arc<Block>>,
}

#[derive(frame_support::CloneNoBound)]
pub struct BlockInfoProvider {
	cache: Arc<RwLock<BlockCache<CACHE_SIZE, SubstrateBlock>>>,
	rpc: LegacyRpcMethods<SrcChainConfig>,
	api: OnlineClient<SrcChainConfig>,
}

impl BlockInfoProvider {
	/// Create a new `BlockInfoProvider` with the given rpc client.
	pub fn new(api: OnlineClient<SrcChainConfig>, rpc: LegacyRpcMethods<SrcChainConfig>) -> Self {
		Self { api, rpc, cache: Default::default() }
	}

	/// Get a read access on the shared cache.
	async fn cache(
		&self,
	) -> tokio::sync::RwLockReadGuard<'_, BlockCache<CACHE_SIZE, SubstrateBlock>> {
		self.cache.read().await
	}

	/// Cache new block and return the pruned block hash.
	pub async fn cache_block(&self, block: SubstrateBlock) -> Option<H256> {
		let mut cache = self.cache.write().await;
		cache.insert(block)
	}

	/// Latest ingested block.
	pub async fn latest_block(&self) -> Option<Arc<SubstrateBlock>> {
		let cache = self.cache().await;
		cache.buffer.back().cloned()
	}

	/// Get block by block_number.
	pub async fn block_by_number(
		&self,
		block_number: SubstrateBlockNumber,
	) -> Result<Option<Arc<SubstrateBlock>>, ClientError> {
		let cache = self.cache().await;
		if let Some(block) = cache.blocks_by_number.get(&block_number).cloned() {
			return Ok(Some(block));
		}

		let Some(hash) = self.rpc.chain_get_block_hash(Some(block_number.into())).await? else {
			return Ok(None);
		};

		self.block_by_hash(&hash).await
	}

	/// Get block by block hash.
	pub async fn block_by_hash(
		&self,
		hash: &H256,
	) -> Result<Option<Arc<SubstrateBlock>>, ClientError> {
		let cache = self.cache().await;
		if let Some(block) = cache.blocks_by_hash.get(hash).cloned() {
			return Ok(Some(block));
		}

		match self.api.blocks().at(*hash).await {
			Ok(block) => Ok(Some(Arc::new(block))),
			Err(subxt::Error::Block(subxt::error::BlockError::NotFound(_))) => Ok(None),
			Err(err) => Err(err.into()),
		}
	}
}

impl<const N: usize, B: BlockInfo> BlockCache<N, B> {
	/// Insert an entry into the cache, and prune the oldest entry if the cache is full.
	pub fn insert(&mut self, block: B) -> Option<H256> {
		let mut pruned_block_hash = None;
		if self.buffer.len() >= N {
			if let Some(block) = self.buffer.pop_front() {
				log::trace!(target: LOG_TARGET, "Pruning block: {}", block.number());
				let hash = block.hash();
				self.blocks_by_hash.remove(&hash);
				self.blocks_by_number.remove(&block.number());
				pruned_block_hash = Some(hash);
			}
		}

		let block = Arc::new(block);
		self.buffer.push_back(block.clone());
		self.blocks_by_number.insert(block.number(), block.clone());
		self.blocks_by_hash.insert(block.hash(), block);
		pruned_block_hash
	}
}

#[cfg(test)]
mod test {
	use super::*;

	struct MockBlock {
		block_number: SubstrateBlockNumber,
		block_hash: H256,
	}

	impl BlockInfo for MockBlock {
		fn hash(&self) -> H256 {
			self.block_hash
		}

		fn number(&self) -> u32 {
			self.block_number
		}
	}

	#[test]
	fn cache_insert_works() {
		let mut cache = BlockCache::<2, MockBlock>::default();

		let pruned = cache.insert(MockBlock { block_number: 1, block_hash: H256::from([1; 32]) });
		assert_eq!(pruned, None);

		let pruned = cache.insert(MockBlock { block_number: 2, block_hash: H256::from([2; 32]) });
		assert_eq!(pruned, None);

		let pruned = cache.insert(MockBlock { block_number: 3, block_hash: H256::from([3; 32]) });
		assert_eq!(pruned, Some(H256::from([1; 32])));

		assert_eq!(cache.buffer.len(), 2);
		assert_eq!(cache.blocks_by_number.len(), 2);
		assert_eq!(cache.blocks_by_hash.len(), 2);
	}
}
