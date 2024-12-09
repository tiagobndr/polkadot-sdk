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
	subxt_client::{
		revive::{calls::types::EthTransact, events::ContractEmitted},
		system::events::ExtrinsicSuccess,
		transaction_payment::events::TransactionFeePaid,
		SrcChainConfig,
	},
	ClientError, LOG_TARGET,
};
use futures::{stream, StreamExt};
use jsonrpsee::core::async_trait;
use pallet_revive::{
	create1,
	evm::{GenericTransaction, Log, ReceiptInfo, TransactionSigned, H256, U256},
};
use sp_core::{keccak_256, H160};
use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
};
use subxt::{backend::legacy::LegacyRpcMethods, OnlineClient};
use tokio::sync::RwLock;

/// The number of recent blocks maintained by the cache.
/// For each block in the cache, we also store the EVM transaction receipts.
pub const CACHE_SIZE: usize = 256;

/// Provides information about a block,
/// This is an abstratction on top of [`SubstrateBlock`] to provide a common interface for
/// [`BlockCache`].
#[async_trait]
pub trait BlockInfo {
	/// Returns the block hash.
	fn hash(&self) -> H256;
	/// Returns the block number.
	fn number(&self) -> SubstrateBlockNumber;
	/// Extract the EVM transactions and receipts for this block
	async fn receipt_infos(&self) -> Result<Vec<(TransactionSigned, ReceiptInfo)>, ClientError>;
}

/// Extract the receipt information from an extrinsic.
async fn extract_receipt_from_extrinsic(
	block_number: U256,
	block_hash: H256,
	from: H160,
	tx_info: GenericTransaction,
	transaction_hash: H256,
	contract_address: Option<H160>,
	ext: subxt::blocks::ExtrinsicDetails<SrcChainConfig, subxt::OnlineClient<SrcChainConfig>>,
) -> Result<ReceiptInfo, ClientError> {
	let events = ext.events().await?;
	let tx_fees = events.find_first::<TransactionFeePaid>()?.ok_or(ClientError::TxFeeNotFound)?;

	let gas_price = tx_info.gas_price.unwrap_or_default();
	let gas_used = (tx_fees.tip.saturating_add(tx_fees.actual_fee))
		.checked_div(gas_price.as_u128())
		.unwrap_or_default();

	let success = events.has::<ExtrinsicSuccess>()?;
	let transaction_index = ext.index();

	// get logs from ContractEmitted event
	let logs = events
		.iter()
		.filter_map(|event_details| {
			let event_details = event_details.ok()?;
			let event = event_details.as_event::<ContractEmitted>().ok()??;

			Some(Log {
				address: event.contract,
				topics: event.topics,
				data: Some(event.data.into()),
				block_number: Some(block_number),
				transaction_hash,
				transaction_index: Some(transaction_index.into()),
				block_hash: Some(block_hash),
				log_index: Some(event_details.index().into()),
				..Default::default()
			})
		})
		.collect();

	log::debug!(target: LOG_TARGET, "Adding receipt for tx hash: {transaction_hash:?} - block: {block_number:?}");
	let receipt = ReceiptInfo::new(
		block_hash,
		block_number,
		contract_address,
		from,
		logs,
		tx_info.to,
		gas_price,
		gas_used.into(),
		success,
		transaction_hash,
		transaction_index.into(),
		tx_info.r#type.unwrap_or_default(),
	);

	Ok(receipt)
}

#[async_trait]
impl BlockInfo for SubstrateBlock {
	fn hash(&self) -> H256 {
		SubstrateBlock::hash(&self)
	}
	fn number(&self) -> u32 {
		SubstrateBlock::number(&self)
	}

	async fn receipt_infos(&self) -> Result<Vec<(TransactionSigned, ReceiptInfo)>, ClientError> {
		// Get extrinsics from the block
		let extrinsics = self.extrinsics().await?;
		let block_hash = self.hash();
		let block_number = U256::from(self.number());

		// Filter extrinsics from pallet_revive
		let extrinsics = extrinsics.iter().flat_map(|ext| {
			let call = ext.as_extrinsic::<EthTransact>().ok()??;
			let transaction_hash = H256(keccak_256(&call.payload));
			let signed_tx = TransactionSigned::decode(&call.payload).ok()?;
			let from = signed_tx.recover_eth_address().ok()?;
			let tx_info = GenericTransaction::from_signed(signed_tx.clone(), Some(from));
			let contract_address = if tx_info.to.is_none() {
				Some(create1(&from, tx_info.nonce.unwrap_or_default().try_into().ok()?))
			} else {
				None
			};

			Some((from, signed_tx, tx_info, transaction_hash, contract_address, ext))
		});

		stream::iter(extrinsics)
			.map(|(from, signed_tx, tx_info, transaction_hash, contract_address, ext)| async move {
				let receipt = extract_receipt_from_extrinsic(
					block_number,
					block_hash,
					from,
					tx_info,
					transaction_hash,
					contract_address,
					ext,
				)
				.await?;
				Ok((signed_tx, receipt))
			})
			.buffer_unordered(10)
			.collect::<Vec<Result<_, _>>>()
			.await
			.into_iter()
			.collect::<Result<Vec<_>, _>>()
	}
}

/// Get the receipt infos from the extrinsics in a block.

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

#[derive(frame_support::CloneNoBound)]
pub struct BlockInfoProvider<Block = SubstrateBlock> {
	cache: Arc<RwLock<BlockCache<CACHE_SIZE, Block>>>,
	rpc: LegacyRpcMethods<SrcChainConfig>,
	api: OnlineClient<SrcChainConfig>,
}

impl<B: BlockInfo> BlockInfoProvider<B> {
	/// Create a new `BlockInfoProvider` with the given rpc client.
	pub fn new(api: OnlineClient<SrcChainConfig>, rpc: LegacyRpcMethods<SrcChainConfig>) -> Self {
		Self { api, rpc, cache: Default::default() }
	}
}

impl BlockInfoProvider<SubstrateBlock> {
	/// Get a read access on the shared cache.
	async fn cache(
		&self,
	) -> tokio::sync::RwLockReadGuard<'_, BlockCache<CACHE_SIZE, SubstrateBlock>> {
		self.cache.read().await
	}

	/// Ingest a new block.
	pub async fn ingest(&self, block: SubstrateBlock) {
		let receipts = block
			.receipt_infos()
			.await
			.inspect_err(|err| {
				log::error!(target: LOG_TARGET, "Failed to get receipts for block: {}: {err:?}", block.number());
			})
			.unwrap_or_default();
		let mut cache = self.cache.write().await;
		cache.insert(block, receipts);
	}

	/// Latest ingested block.
	pub async fn latest_block(&self) -> Option<Arc<SubstrateBlock>> {
		let cache = self.cache().await;
		cache.buffer.back().cloned()
	}

	/// Find receipt by block hash and index
	pub async fn receipt_by_block_hash_and_index(
		&self,
		block_hash: &H256,
		transaction_index: &U256,
	) -> Option<ReceiptInfo> {
		let cache = self.cache().await;
		let receipt_hash =
			cache.tx_hashes_by_block_and_index.get(block_hash)?.get(transaction_index)?;
		let receipt = cache.receipts_by_hash.get(receipt_hash)?;
		// TODO if not found query db to find tx_hash for block and index, then use rpc to get block
		// and receipt
		Some(receipt.clone())
	}

	/// Get receipt count by block
	pub async fn receipts_count_per_block(&self, block_hash: &H256) -> Option<usize> {
		let cache = self.cache().await;
		cache.tx_hashes_by_block_and_index.get(block_hash).map(|v| v.len())
		// TODO if not found query db to find receipts for block_hash
		// mean we need a second table ...
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
		return self.block_by_hash(&hash).await;
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

	/// Get receipts by transaction hash
	pub async fn receipt_by_hash(&self, hash: &H256) -> Option<ReceiptInfo> {
		let cache = self.cache().await;
		cache.receipts_by_hash.get(hash).cloned()
		// TODO query DB then get block and get receipt for tx
	}

	/// Get signed transaction by transaction hash.
	pub async fn signed_tx_by_hash(&self, hash: &H256) -> Option<TransactionSigned> {
		let cache = self.cache().await;
		cache.signed_tx_by_hash.get(hash).cloned()
		// TODO query DB then get block and get receipt for tx
	}
}

impl<const N: usize, B: BlockInfo> BlockCache<N, B> {
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

	#[async_trait]
	impl BlockInfo for MockBlock {
		fn hash(&self) -> H256 {
			self.block_hash
		}

		fn number(&self) -> u32 {
			self.block_number
		}

		async fn receipt_infos(
			&self,
		) -> Result<Vec<(TransactionSigned, ReceiptInfo)>, ClientError> {
			Ok(vec![(
				TransactionSigned::default(),
				ReceiptInfo { transaction_hash: self.hash(), ..Default::default() },
			)])
		}
	}

	#[tokio::test]
	async fn cache_insert_and_prune_works() {
		let mut cache = BlockCache::<2, MockBlock>::default();

		for i in 1..=3 {
			let block = MockBlock { block_number: i.into(), block_hash: H256::from([i; 32]) };
			let receipts = block.receipt_infos().await.unwrap();
			cache.insert(block, receipts)
		}

		assert_eq!(cache.buffer.len(), 2);
		assert_eq!(cache.blocks_by_number.len(), 2);
		assert_eq!(cache.blocks_by_hash.len(), 2);
		assert_eq!(cache.receipts_by_hash.len(), 2);
		assert_eq!(cache.signed_tx_by_hash.len(), 2);
		assert_eq!(cache.tx_hashes_by_block_and_index.len(), 2);
	}
}
