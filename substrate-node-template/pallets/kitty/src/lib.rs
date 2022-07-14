#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	
	pub use super::*;
	//demo here
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	// pub struct Students<T:Config> {
	// 	name: Vec<u8>,
	// 	age: u8,
	// 	gender: Gender,
	// 	account: T::AccountId,
	// }
	pub struct Kitties<T:Config> {
		dna: Vec<u8>,
		price: u32,
		gender: Gender,
		owner: T::AccountId,
	}

	pub type DnaKitty = Vec<u8>;
	pub type TotalKitty = u32;

	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default() -> Self{
			Gender::Male
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitty_total)]
	pub type KittyTotal<T> = StorageValue<_,TotalKitty, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T:Config> = StorageMap<_, Blake2_128Concat, DnaKitty, Kitties<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub(super) type KittyOwner<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<DnaKitty>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		KittyStored(Vec<u8>, u32),
	}
	#[pallet::error]
	pub enum Error<T> {
		TooYoung,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {

			let who = ensure_signed(origin)?;

			let gender = Self::gen_gender(dna.clone())?;

			let kitty = Kitties {
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};

			// calculate total kitties
			let mut current_total_kitty = <KittyTotal<T>>::get();
			current_total_kitty += 1;
			KittyTotal::<T>::put(current_total_kitty);

			// kitty info
			<Kitty<T>>::insert(dna.clone(), kitty);

			// add a kitty to owner
			let mut current_kitty_list = <KittyOwner<T>>::get(who.clone());
			current_kitty_list.push(dna.clone());
			<KittyOwner<T>>::insert(who.clone(), current_kitty_list);


			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna, price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


//helper function
impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 == 1 {
			res = Gender::Female;
		}
		Ok(res)
	}
}