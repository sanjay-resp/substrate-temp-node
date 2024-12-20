#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod enums;
pub mod traits;
pub mod weightinfo;
pub use weightinfo::WeightInfo;
pub mod structs;
pub mod utils;
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::structs::*;
    use crate::utils::*;
    use crate::traits::*;
    use crate::enums::NodeOnboardingError;
    use frame_support::Parameter;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::{StorageValue, *}, traits::{Currency, ReservableCurrency,StorageVersion}, Blake2_128Concat, BoundedVec};
    use frame_system::pallet_prelude::*;
    use structs::{NetworkCapacity, TotalUps};

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type Currency: ReservableCurrency<Self::AccountId>;

        type Balance: Parameter
            + From<u128>
            + Into<u128>
            +Default
            + Copy;

        #[pallet::constant]
        type MaxSnapshot: Get<u32>;

        #[pallet::contant]
        type MaxNodes: Get<u32>;
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        NotEnoughFund,
        NodeNotFound,
        NodeRegistrationError,
        NodeAlreadyExists,
    }

    #[pallet::storage]
    #[pallet::getter(fn node_onboarded)]
    pub(super) type NodesOnboarded <T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<NodeInfo,  T::MaxNodes>,
    ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn network_storage)]
    pub (super) type NetworkStorage <T:Config> = StorageValue<Value=NetworkCapacity>;

    #[pallet::storage]
    #[pallet::getter(fn available_nodes)]
    pub (super) type AvailableNodes <T:Config> = StorageValue<Value=TotalUps>;

    #[pallet::storage]
    #[pallet::getter(fn total_nodes)]
    pub (super) type TotalNodes <T:Config> = StorageValue<Value=TotalDowns>;

    impl <T: Config> Error<T> {
        fn dispatch_error(err:NodeOnboardingError) -> DispatchResult {
            match err {
                NodeOnboardingError::AlreadyExists => Err(Error::<T>::NodeAlreadyExists.into()),
                NodeOnboardingError::InvalidFundForNode => Err(Error::<T>::NotEnoughFund.into()),
                NodeOnboardingError::NodeIdNotFound => Err(Error::<T>::NodeNotFound.into())
            }
        }
        
    }

    pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewNodeRegistered {
            node_id: [u8;32]
        },
        
        ScoreUpdated {
            time: u64
        },
        BalanceDeducted{account: T::AccountId, amount: T::Balance}
    }

    #[pallet::call]
    impl <T: Config> Pallet<T>{
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::register_node())]
        pub fn register_node_info(origin:OriginFor<T>, node_infos:NodeInfo ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            //calculate stake balance  necessary for node running
     
            //add balance check, 

        
            //valid_node_peer_id 
            //transfer to escrow service
        
            let mut user_nodes = NodesOnboarded::<T>::get(&who);
            let previous_node = user_nodes.iter().any(|nodes| nodes.node_id == node_infos.node_id);

            ensure!(!previous_node, Error::<T>::NodeAlreadyExists);
            let node_id = node_infos.node_id;
            user_nodes
            .try_push(node_infos)
            .map_err(|_| Error::<T>::NodeRegistrationError)?;

            NodesOnboarded::<T>::insert(&who, user_nodes);
            Self::deposit_event(Event::NewNodeRegistered { node_id });
            Ok(())
        }
  

    }

// impl<T:Config> NodeOnChainInfo<T::AccountId> for Pallet<T>{}
   

}