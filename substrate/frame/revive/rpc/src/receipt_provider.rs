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
	client::SubstrateBlock,
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

pub mod cache;
pub use cache::CacheReceiptProvider;

#[async_trait]
pub trait ReceiptProvider {
	async fn insert(&self, block_hash: H256, receipts: Vec<(TransactionSigned, ReceiptInfo)>);
	async fn remove(&self, block_hash: H256);
	async fn receipt_by_block_hash_and_index(
		&self,
		block_hash: &H256,
		transaction_index: &U256,
	) -> Option<ReceiptInfo>;
	async fn receipts_count_per_block(&self, block_hash: &H256) -> Option<usize>;
	async fn receipt_by_hash(&self, hash: &H256) -> Option<ReceiptInfo>;
	async fn signed_tx_by_hash(&self, hash: &H256) -> Option<TransactionSigned>;
}

/// Extract the receipt information from an extrinsic.
pub async fn extract_receipt_from_extrinsic(
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

///  Extract receipts from block.
pub async fn receipt_infos(
	block: &SubstrateBlock,
) -> Result<Vec<(TransactionSigned, ReceiptInfo)>, ClientError> {
	// Get extrinsics from the block
	let extrinsics = block.extrinsics().await?;
	let block_hash = block.hash();
	let block_number = U256::from(block.number());

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
