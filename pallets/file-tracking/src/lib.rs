#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, Blake2_128Concat, traits::StorageVersion,BoundedVec};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]    
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, PartialEq, Eq)]
    #[scale_info(skip_type_params(T))]
    pub struct FileOwned<T: Config> {
        pub file_id: [u8; 32],
        pub owner: T::AccountId,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxFiles: Get<u32>;

	}

    #[pallet::storage]
    #[pallet::getter(fn files_owned)]
    pub(super) type FilesOwned<T: Config> = StorageMap<
        _,                             
        Blake2_128Concat,              
        T::AccountId,                  
		BoundedVec<[u8; 32], T::MaxFiles>, 
		ValueQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        SameFileHash,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FileUploadEvent {
            file_id: [u8; 32],         // The unique file ID.
            who: T::AccountId,         // The account that uploaded the file.
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::store_file_info())] // Set an appropriate weight.
        pub fn store_file_info(origin: OriginFor<T>, cid: [u8; 32]) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Retrieve the list of currently owned files.
            let mut currently_owned = FilesOwned::<T>::get(&who);

            // Ensure the file ID is not already owned by the account.
            ensure!(!currently_owned.contains(&cid), Error::<T>::SameFileHash);

            // Add the new file ID to the list and store it.
			currently_owned
            .try_push(cid)
            .map_err(|_| Error::<T>::NoneValue)?; 
            FilesOwned::<T>::insert(&who, currently_owned);


            // Emit an event.
            Self::deposit_event(Event::FileUploadEvent { file_id: cid, who });

            Ok(())
        }
    }
}
