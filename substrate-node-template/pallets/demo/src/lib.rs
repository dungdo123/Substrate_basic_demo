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
	pub struct Students<T:Config> {
		name: Vec<u8>,
		age: u8,
		gender: Gender,
		account: T::AccountId,
	}

	pub type Id = u32;

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
	#[pallet::getter(fn student_id)]

	pub type StudentId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn student)]
	pub(super) type Student<T:Config> = StorageMap<_, Blake2_128Concat, Id, Students<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		StudentStored(Vec<u8>, u8),
	}
	#[pallet::error]
	pub enum Error<T> {
		TooYoung,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_student(origin: OriginFor<T>, name: Vec<u8>, age: u8) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(age > 20, Error::<T>::TooYoung);

			let gender = Self::gen_gender(name.clone())?;

			let student = Students {
				name: name.clone(),
				age: age,
				gender: gender,
				account: who,
			};

			let mut current_id = <StudentId<T>>::get();

			<Student<T>>::insert(current_id, student);
			current_id += 1;
			StudentId::<T>::put(current_id);

			// Emit an event.
			Self::deposit_event(Event::StudentStored(name, age));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


//helper function
impl<T> Pallet<T> {
	fn gen_gender(name: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if name.len() % 2 == 0 {
			res = Gender::Female;
		}
		Ok(res)
	}
}