

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;


pub trait WeightInfo {
	fn register_node() -> Weight;
    // fn score_update() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

	fn register_node() -> Weight {

        Weight::from_parts(10_000_000, 0) // Base execution weight (adjust after benchmarks).
            .saturating_add(RocksDbWeight::get().reads(1_u64))  // Read from storage.
            .saturating_add(RocksDbWeight::get().writes(1_u64)) // Write to storage.
	}


}


impl WeightInfo for () {
	fn register_node() -> Weight {
        Weight::from_parts(10_000_000, 0) // Base execution weight (adjust after benchmarks).
            .saturating_add(RocksDbWeight::get().reads(1_u64))  // Read from storage.
            .saturating_add(RocksDbWeight::get().writes(1_u64)) // Write to storage.
	}
}
