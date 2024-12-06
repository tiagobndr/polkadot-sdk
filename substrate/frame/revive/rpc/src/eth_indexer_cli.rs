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
//! The Ethereum JSON-RPC server.
use crate::client::Client;
use anyhow::bail;
use clap::Parser;
use futures::{pin_mut, FutureExt};
use sc_cli::{SharedParams, Signals};
use sc_service::TaskManager;

// Parsed command instructions from the command line
#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct CliCommand {
	/// The node url to connect to
	#[clap(long, default_value = "ws://127.0.0.1:9944")]
	pub node_rpc_url: String,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub shared_params: SharedParams,
}

/// Initialize the logger
#[cfg(not(test))]
fn init_logger(params: &SharedParams) -> anyhow::Result<()> {
	let mut logger = sc_cli::LoggerBuilder::new(params.log_filters().join(","));
	logger
		.with_log_reloading(params.enable_log_reloading)
		.with_detailed_output(params.detailed_log_output);

	if let Some(tracing_targets) = &params.tracing_targets {
		let tracing_receiver = params.tracing_receiver.into();
		logger.with_profiling(tracing_receiver, tracing_targets);
	}

	if params.disable_log_color {
		logger.with_colors(false);
	}

	logger.init()?;
	Ok(())
}

pub async fn run_indexer(node_rpc_url: &str, task_manager: TaskManager) -> anyhow::Result<()> {
	let essential_spawn_handle = task_manager.spawn_essential_handle();
	let client = Client::from_url(&node_rpc_url).await?;
	client.subscribe_blocks(&essential_spawn_handle);
	Ok(())
}

/// Start the indexer using the given command line arguments.
pub fn run(cmd: CliCommand) -> anyhow::Result<()> {
	let CliCommand { node_rpc_url, shared_params } = cmd;

	#[cfg(not(test))]
	init_logger(&shared_params)?;

	let tokio_runtime = sc_cli::build_runtime()?;
	let tokio_handle = tokio_runtime.handle();
	let task_manager = TaskManager::new(tokio_handle.clone(), None)?;

	let signals = tokio_runtime.block_on(async { Signals::capture() })?;
	let fut = run_indexer(&node_rpc_url, task_manager).fuse();
	pin_mut!(fut);

	match tokio_handle.block_on(signals.try_until_signal(fut)) {
		Ok(_) => Ok(()),
		Err(_) => bail!("Client connection interrupted"),
	}
}
