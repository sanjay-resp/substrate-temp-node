#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
// use frame_support::BoundedVec;
pub use weights::*;
// extern crate sp_api;

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
    
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, PartialEq, Eq,Debug)]
    #[scale_info(skip_type_params(T))]
    pub struct FileMetadata {
    pub name: [u8;32],     
    pub size: u64,         
    pub cid: [u8;32],       
    pub hash: [u8; 32],     
    pub timestamp: u64,     
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

    #[pallet::storage]
    #[pallet::getter(fn sharding_info)]
    pub(super) type ShardingApproachStorage<T: Config> = StorageMap<
    _,
    Blake2_128Concat,  
    T::AccountId,
    BoundedVec<FileMetadata, T::MaxFiles>, 
    ValueQuery
    >;

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        SameFileHash,
        NoFileData,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FileUploadEvent {
            file_id: [u8; 32],         // The unique file ID.
            who: T::AccountId,         // The account that uploaded the file.
        },

        FileRemoval {
            who: T::AccountId,
        }
    
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
   
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::store_file_info())]
        pub fn store_file_with_metadata(origin: OriginFor<T>, uploaded_data: FileMetadata)-> DispatchResult {
            let who = ensure_signed(origin)?;
            let mut currently_owned: BoundedVec<FileMetadata, <T as Config>::MaxFiles> = ShardingApproachStorage::<T>::get(&who);
            let file_exists = currently_owned.iter().any(|filemetadata| filemetadata.cid == uploaded_data.cid);
            ensure!(!file_exists, Error::<T>::SameFileHash);
            let file_info= uploaded_data.cid;
            currently_owned
            .try_push(uploaded_data)
            .map_err(|_| Error::<T>::NoneValue)?;

            ShardingApproachStorage::<T>::insert(&who, currently_owned);

            Self::deposit_event(Event::FileUploadEvent {file_id: file_info, who });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::remove_file())]
        pub fn remove_file_index(origin: OriginFor<T>, cid: [u8; 32]) -> DispatchResult{
            let who = ensure_signed(origin)?;
            let mut currently_owned: BoundedVec<[u8; 32], <T as Config>::MaxFiles>  = FilesOwned::<T>::get(&who);

            ensure!(currently_owned.contains(&cid), Error::<T>::NoFileData);

                // Remove the `cid` from the `BoundedVec`
            currently_owned.retain(|x| x != &cid);

            FilesOwned::<T>::insert(&who, currently_owned);
            Self::deposit_event(Event::FileRemoval {who});
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::remove_file())]
        pub fn remove_file_metadata(origin: OriginFor<T>, cid: [u8;32 ]) -> DispatchResult{
            let who = ensure_signed(origin)?;
            let mut currently_owned: BoundedVec<FileMetadata, <T as Config>::MaxFiles> = ShardingApproachStorage::<T>::get(&who);
            let file_exists = currently_owned.iter().any(|filemetadata| filemetadata.cid == cid);
            ensure!(file_exists, Error::<T>::NoFileData);

            currently_owned.retain(|x| x.cid != cid );
            ShardingApproachStorage::<T>::insert(&who, currently_owned);
            Self::deposit_event(Event::FileRemoval {who});
            Ok(())
        }

    }

    // impl<T: Config> Pallet<T> {
    //     pub fn get_files_owned(address: T::AccountId) -> Option<BoundedVec<[u8; 32], T::MaxFiles>> {
    //         // Retrieve and return the list of owned files.
    //         let currently_owned = FilesOwned::<T>::get(&address);
    //         Some(currently_owned)
    //     }
    // }
}

// sp_api::decl_runtime_apis! {
//     pub trait FileTrackingApi< AccountId, MaxFiles> where
//     AccountId: Codec
//     {
//         fn get_files_owned(account: AccountId) -> Option<BoundedVec<[u8; 32], MaxFiles>>;
//     }
// }
