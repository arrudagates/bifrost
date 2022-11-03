// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
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

//! Autogenerated weights for `bifrost_fee_share`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `yml`, CPU: `AMD Ryzen 9 3950X 16-Core Processor`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/debug/bifrost
// benchmark
// pallet
// --pallet=bifrost_fee_share
// --extrinsic=*
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./bifrost_fee_share.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for the pallet.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn create_distribution() -> Weight;
	fn edit_distribution() -> Weight;
	fn set_era_length() -> Weight;
	fn execute_distribute() -> Weight;
	fn delete_distribution() -> Weight;
}

/// Weight functions for `bifrost_fee_share`.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BifrostWeight<T> {
	// Storage: FeeShare AutoEra (r:1 w:0)
	fn on_initialize() -> Weight {
		(27_563_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: FeeShare DistributionNextId (r:1 w:1)
	// Storage: FeeShare DistributionInfos (r:0 w:1)
	fn create_distribution() -> Weight {
		(216_370_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: FeeShare DistributionInfos (r:1 w:1)
	fn edit_distribution() -> Weight {
		(224_055_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: FeeShare AutoEra (r:0 w:1)
	fn set_era_length() -> Weight {
		(157_288_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: FeeShare DistributionInfos (r:1 w:0)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:0)
	fn execute_distribute() -> Weight {
		(308_936_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
	// Storage: FeeShare DistributionInfos (r:1 w:1)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:0)
	fn delete_distribution() -> Weight {
		(318_504_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}