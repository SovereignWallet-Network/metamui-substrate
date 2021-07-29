// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-07-21, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/staking/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
	fn bond() -> Weight;
	fn bond_extra() -> Weight;
	fn unbond() -> Weight;
	fn withdraw_unbonded_update(s: u32, ) -> Weight;
	fn withdraw_unbonded_kill(s: u32, ) -> Weight;
	fn validate() -> Weight;
	fn kick(k: u32, ) -> Weight;
	fn nominate(n: u32, ) -> Weight;
	fn chill() -> Weight;
	fn set_payee() -> Weight;
	fn set_controller() -> Weight;
	fn set_validator_count() -> Weight;
	fn force_no_eras() -> Weight;
	fn force_new_era() -> Weight;
	fn force_new_era_always() -> Weight;
	fn set_invulnerables(v: u32, ) -> Weight;
	fn force_unstake(s: u32, ) -> Weight;
	fn cancel_deferred_slash(s: u32, ) -> Weight;
	fn payout_stakers_dead_controller(n: u32, ) -> Weight;
	fn payout_stakers_alive_staked(n: u32, ) -> Weight;
	fn rebond(l: u32, ) -> Weight;
	fn set_history_depth(e: u32, ) -> Weight;
	fn reap_stash(s: u32, ) -> Weight;
	fn new_era(v: u32, n: u32, ) -> Weight;
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight;
	fn get_npos_targets(v: u32, ) -> Weight;
	fn set_staking_limits() -> Weight;
	fn chill_other() -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn bond() -> Weight {
		(73_506_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_extra() -> Weight {
		(57_019_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn unbond() -> Weight {
		(61_257_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(52_060_000 as Weight)
			// Standard Error: 0
			.saturating_add((22_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(86_303_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_425_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	fn validate() -> Weight {
		(35_189_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:0)
	fn kick(k: u32, ) -> Weight {
		(16_161_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((16_898_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	fn nominate(n: u32, ) -> Weight {
		(41_638_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((5_582_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	fn chill() -> Weight {
		(18_707_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		(12_688_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		(26_726_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		(2_309_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		(2_508_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(2_483_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		(2_487_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	fn set_invulnerables(v: u32, ) -> Weight {
		(2_465_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Payee (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	fn force_unstake(s: u32, ) -> Weight {
		(62_075_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_454_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(3_403_675_000 as Weight)
			// Standard Error: 223_000
			.saturating_add((19_948_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Payee (r:2 w:0)
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(111_168_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((48_615_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Staking Ledger (r:2 w:2)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking Payee (r:2 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(148_375_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((62_213_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn rebond(l: u32, ) -> Weight {
		(48_669_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((63_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	// Storage: Staking ErasValidatorReward (r:0 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:2)
	// Storage: Staking ErasRewardPoints (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:2)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:2)
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((34_853_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	fn reap_stash(s: u32, ) -> Weight {
		(73_475_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_390_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking CounterForNominators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking CurrentEra (r:1 w:1)
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 970_000
			.saturating_add((308_475_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 48_000
			.saturating_add((48_379_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: Staking Nominators (r:1001 w:0)
	// Storage: Staking Validators (r:501 w:0)
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 102_000
			.saturating_add((26_408_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 102_000
			.saturating_add((28_824_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_473_000
			.saturating_add((49_400_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking Validators (r:501 w:0)
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 29_000
			.saturating_add((11_333_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_limits() -> Weight {
		(6_599_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	fn chill_other() -> Weight {
		(62_743_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn bond() -> Weight {
		(73_506_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_extra() -> Weight {
		(57_019_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn unbond() -> Weight {
		(61_257_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(52_060_000 as Weight)
			// Standard Error: 0
			.saturating_add((22_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		(86_303_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_425_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	fn validate() -> Weight {
		(35_189_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:0)
	fn kick(k: u32, ) -> Weight {
		(16_161_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((16_898_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	fn nominate(n: u32, ) -> Weight {
		(41_638_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((5_582_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	fn chill() -> Weight {
		(18_707_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		(12_688_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		(26_726_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		(2_309_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		(2_508_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(2_483_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		(2_487_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	fn set_invulnerables(v: u32, ) -> Weight {
		(2_465_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Payee (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	fn force_unstake(s: u32, ) -> Weight {
		(62_075_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_454_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(3_403_675_000 as Weight)
			// Standard Error: 223_000
			.saturating_add((19_948_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Payee (r:2 w:0)
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(111_168_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((48_615_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Staking Ledger (r:2 w:2)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking Payee (r:2 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(148_375_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((62_213_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn rebond(l: u32, ) -> Weight {
		(48_669_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((63_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	// Storage: Staking ErasValidatorReward (r:0 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:2)
	// Storage: Staking ErasRewardPoints (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:2)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:2)
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((34_853_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	fn reap_stash(s: u32, ) -> Weight {
		(73_475_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_390_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking CounterForNominators (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking CurrentEra (r:1 w:1)
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 970_000
			.saturating_add((308_475_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 48_000
			.saturating_add((48_379_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: Staking Nominators (r:1001 w:0)
	// Storage: Staking Validators (r:501 w:0)
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 102_000
			.saturating_add((26_408_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 102_000
			.saturating_add((28_824_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_473_000
			.saturating_add((49_400_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking Validators (r:501 w:0)
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 29_000
			.saturating_add((11_333_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_limits() -> Weight {
		(6_599_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	fn chill_other() -> Weight {
		(62_743_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
