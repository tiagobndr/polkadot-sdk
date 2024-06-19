// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Transaction pool view.
//!
//! The View represents the state of the transaction pool at given block. The view is created when
//! new block is notified to transaction pool. Views are removed on finalization.

use crate::{graph, graph::watcher::Watcher, log_xt_debug};
use std::sync::Arc;

use crate::graph::{ExtrinsicFor, ExtrinsicHash};
use sc_transaction_pool_api::{PoolStatus, TransactionSource};

use crate::LOG_TARGET;
use sp_blockchain::HashAndNumber;

/// Represents the state of transaction for given block.
pub(super) struct View<ChainApi: graph::ChainApi> {
	pub(super) pool: graph::Pool<ChainApi>,
	pub(super) at: HashAndNumber<ChainApi::Block>,
}

impl<ChainApi> View<ChainApi>
where
	ChainApi: graph::ChainApi,
{
	/// Creates a new empty view.
	pub(super) fn new(
		api: Arc<ChainApi>,
		at: HashAndNumber<ChainApi::Block>,
		options: graph::Options,
	) -> Self {
		Self { pool: graph::Pool::new(options, true.into(), api), at }
	}

	/// Creates a copy of the other view.
	pub(super) fn new_from_other(&self, at: &HashAndNumber<ChainApi::Block>) -> Self {
		View { at: at.clone(), pool: self.pool.deep_clone() }
	}

	/// Imports many unvalidate extrinsics into the view.
	pub(super) async fn submit_many(
		&self,
		source: TransactionSource,
		xts: impl IntoIterator<Item = ExtrinsicFor<ChainApi>>,
	) -> Vec<Result<ExtrinsicHash<ChainApi>, ChainApi::Error>> {
		let xts = xts.into_iter().collect::<Vec<_>>();
		log_xt_debug!(target: LOG_TARGET, xts.iter().map(|xt| self.pool.validated_pool().api().hash_and_length(xt).0), "[{:?}] view::submit_many at:{}", self.at.hash);
		self.pool.submit_at(&self.at, source, xts).await
	}

	/// Import a single extrinsic and starts to watch its progress in the view.
	pub(super) async fn submit_and_watch(
		&self,
		source: TransactionSource,
		xt: ExtrinsicFor<ChainApi>,
	) -> Result<Watcher<ExtrinsicHash<ChainApi>, ExtrinsicHash<ChainApi>>, ChainApi::Error> {
		log::debug!(target: LOG_TARGET, "[{:?}] view::submit_and_watch at:{}", self.pool.validated_pool().api().hash_and_length(&xt).0, self.at.hash);
		self.pool.submit_and_watch(&self.at, source, xt).await
	}

	/// Status of the pool associated withe the view.
	pub(super) fn status(&self) -> PoolStatus {
		self.pool.validated_pool().status()
	}

	/// Creates a watcher for given transaction.
	pub(super) fn create_watcher(
		&self,
		tx_hash: ExtrinsicHash<ChainApi>,
	) -> Watcher<ExtrinsicHash<ChainApi>, ExtrinsicHash<ChainApi>> {
		self.pool.validated_pool().create_watcher(tx_hash)
	}
}
