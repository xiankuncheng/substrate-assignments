#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"kty!");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct KittiesAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for KittiesAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for KittiesAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}
#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::traits::{Randomness, ReservableCurrency};
	use frame_support::{log, pallet_prelude::*, traits::Currency};
	use frame_system::offchain::SendSignedTransaction;
	use frame_system::offchain::{AppCrypto, CreateSignedTransaction, Signer};
	use frame_system::pallet_prelude::*;
	use sp_io::offchain_index;
	use sp_runtime::offchain::storage::StorageValueRef;
	use sp_runtime::traits::Zero;
	use sp_runtime::traits::{AtLeast32Bit, Bounded, CheckedAdd};

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::type_value]
	pub fn GetDefaultValue<T: Config>() -> T::KittyIndex {
		0_u8.into()
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty {
		pub dna: [u8; 16],
		pub asset: u32,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type KittyIndex: AtLeast32Bit + Copy + Parameter + Default + Bounded + MaxEncodedLen;

		#[pallet::constant]
		type MaxKittyIndex: Get<u32>;

		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> =
		StorageValue<_, T::KittyIndex, ValueQuery, GetDefaultValue<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn all_kitties)]
	pub type AllKitties<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::KittyIndex, T::MaxKittyIndex>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex, Kitty),
		KittyBreeded(T::AccountId, T::KittyIndex, Kitty, T::KittyIndex, T::KittyIndex),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		KittyIdOverflow,
		SameKittyId,
		NotOwner,
		NotEnoughBalance,
		OwnTooManyKitties,
	}

	const ONCHAIN_TX_KEY: &[u8] = b"kitty_pallet::indexing01";
	#[derive(Debug, Encode, Decode, Default)]
	struct IndexingData<T: Config>(T::KittyIndex);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let key = Self::derived_key(block_number);
			let storage_ref = StorageValueRef::persistent(&key);

			if let Ok(Some(data)) = storage_ref.get::<IndexingData<T>>() {
				// Sleep 8000ms to simulate heavy calculation for kitty asset index.
				let timeout = sp_io::offchain::timestamp()
					.add(sp_runtime::offchain::Duration::from_millis(8000));
				sp_io::offchain::sleep_until(timeout);

				let kitty_id = data.0.into();

				if block_number % 2u32.into() != Zero::zero() {
					_ = Self::send_signed_tx(kitty_id, 1);
				} else {
					_ = Self::send_signed_tx(kitty_id, 2);
				}
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_price = T::KittyPrice::get();
			ensure!(T::Currency::can_reserve(&who, kitty_price), Error::<T>::NotEnoughBalance);

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let dna = Self::random_value(&who);
			let kitty = Kitty { dna, asset: 0 };

			T::Currency::reserve(&who, kitty_price)?;
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			let next_kitty_id = kitty_id
				.checked_add(&(T::KittyIndex::from(1_u8)))
				.ok_or(Error::<T>::KittyIdOverflow)
				.unwrap();
			NextKittyId::<T>::set(next_kitty_id);

			AllKitties::<T>::try_mutate(&who, |ref mut kitties| {
				kitties.try_push(kitty_id).map_err(|_| Error::<T>::OwnTooManyKitties)?;
				Ok::<(), DispatchError>(())
			})?;

			Self::save_kitty_to_indexing(kitty_id);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_price = T::KittyPrice::get();
			ensure!(T::Currency::can_reserve(&who, kitty_price), Error::<T>::NotEnoughBalance);

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let selector = Self::random_value(&who);

			let mut data = [0u8; 16];
			for i in 0..kitty_1.dna.len() {
				data[i] = (kitty_1.dna[i] & selector[i]) | (kitty_2.dna[i] & !selector[i]);
			}
			let new_kitty = Kitty { dna: data, asset: 0 };

			T::Currency::reserve(&who, kitty_price)?;
			Kitties::<T>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			let next_kitty_id = kitty_id
				.checked_add(&(T::KittyIndex::from(1_u8)))
				.ok_or(Error::<T>::KittyIdOverflow)
				.unwrap();
			NextKittyId::<T>::set(next_kitty_id);

			AllKitties::<T>::try_mutate(&who, |ref mut kitties| {
				kitties.try_push(kitty_id).map_err(|_| Error::<T>::OwnTooManyKitties)?;
				Ok::<(), DispatchError>(())
			})?;

			Self::save_kitty_to_indexing(kitty_id);

			Self::deposit_event(Event::KittyBreeded(
				who, kitty_id, new_kitty, kitty_id_1, kitty_id_2,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_price = T::KittyPrice::get();
			ensure!(
				T::Currency::can_reserve(&new_owner, kitty_price),
				Error::<T>::NotEnoughBalance
			);

			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			T::Currency::unreserve(&who, kitty_price);
			T::Currency::reserve(&new_owner, kitty_price)?;
			KittyOwner::<T>::insert(kitty_id, &new_owner);

			AllKitties::<T>::try_mutate(&who, |ref mut kitties| {
				let index = kitties.iter().position(|&r| r == kitty_id).unwrap();
				kitties.remove(index);
				Ok::<(), DispatchError>(())
			})?;
			AllKitties::<T>::try_mutate(&new_owner, |ref mut kitties| {
				kitties.try_push(kitty_id).map_err(|_| Error::<T>::OwnTooManyKitties)?;
				Ok::<(), DispatchError>(())
			})?;

			Self::deposit_event(Event::KittyTransferred(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn update_kitty(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			asset: u32,
		) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			let kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			let new_kitty = Kitty { dna: kitty.dna, asset };

			Kitties::<T>::insert(kitty_id, &new_kitty);

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(sp_io::hashing::blake2_128)
		}

		fn get_next_id() -> Result<T::KittyIndex, ()> {
			let kitty_id = Self::next_kitty_id();
			match kitty_id {
				_ if T::KittyIndex::max_value() <= kitty_id => Err(()),
				val => Ok(val),
			}
		}

		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

		fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY
					.clone()
					.into_iter()
					.chain(b"/".into_iter())
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			})
		}

		fn save_kitty_to_indexing(kitty_id: T::KittyIndex) {
			let key = Self::derived_key(frame_system::Module::<T>::block_number());
			let data: IndexingData<T> = IndexingData(kitty_id);
			offchain_index::set(&key, &data.encode());
		}

		fn send_signed_tx(kitty_id: T::KittyIndex, payload: u32) -> Result<(), &'static str> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				);
			}

			let results = signer.send_signed_transaction(|_account| Call::update_kitty {
				kitty_id,
				asset: payload,
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted data:{:?}", acc.id, (kitty_id, payload)),
					Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
				}
			}

			Ok(())
		}
	}
}
