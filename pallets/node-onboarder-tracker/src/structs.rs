
use crate::Config;
use frame_support::{ pallet_prelude::*, BoundedVec};



#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, PartialEq, Debug,Eq)]
#[scale_info(skip_type_params(T))]
pub struct NodeInfo{
    pub node_id: [u8;32],
    pub registered_time: u64,
    pub status: bool,
    pub capacity: u64
}

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct DailyScore<T: Config> {
    pub node_id: [u8;32],
    pub registered_time_with_status: BoundedVec<[u8;32],T::MaxSnapshot>,
}

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
pub struct ActiveScore<T: Config> {
    pub node_id: [u8;32],
    pub registered_time_with_status: BoundedVec<[u8;32], T::MaxSnapshot>,
}


pub type StoragePriceQuote = u64;
pub type NetworkCapacity = u64;
pub type TotalUps = u64;
pub type TotalDowns = u64;
