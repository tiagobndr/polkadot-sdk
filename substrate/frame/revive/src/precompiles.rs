// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use alloy_core as alloy;

use crate::{exec::Ext, Config};
use alloy::sol_types::{SolError, SolInterface, SolValue};

pub enum AddressMatcher {
	Fixed([u8; 20]),
	Prefix([u8; 8]),
}

pub trait Precompile {
	type T: Config;
	type Interface: SolInterface;
	const MATCHER: AddressMatcher;

	fn call(
		address: &[u8; 20],
		input: &Self::Interface,
		env: &impl Ext<T = Self::T>,
	) -> Result<impl SolValue + 'static, impl SolError + 'static>;
}
