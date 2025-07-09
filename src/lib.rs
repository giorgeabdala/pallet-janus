mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub  struct Pallet<T>(_);

    #[pallet::config]
    pub  trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub  enum Event<T: Config> {
        SomethingStored {
            something: u32,
            who: T::AccountId
        }
    }

    #[pallet::error]
    pub  enum Error<T> {
        NoneValue,
        StorageOverflow
    }

    #[pallet::storage]
    pub  type Store<T> = StorageValue<_, u32>;

    #[pallet::call]
    impl <T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub  fn store_janus(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Store::<T>::put(something);
            Self::deposit_event(Event::SomethingStored { something, who });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub  fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            match Store::<T>::get() {
                None => Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    Store::<T>::put(new);
                    Ok(())
                }
            }

        }

    }

}
