// TODO: module documentation
// TODO: adjust all extrinsic weights

/////////////////// Configuration //////////////////////////////////////////////
#![cfg_attr(not(feature = "std"), no_std)]

// used dependencies
use codec::{Codec, Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, error::BadOrigin, Parameter,
};
use frame_support::traits::Currency;
use frame_system::ensure_signed;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::{Hash, MaybeSerialize, Member, SaturatedConversion, Saturating, UniqueSaturatedInto};
use sp_core::{U256, H256, H160, Hasher};
use pallet_evm::{AddressMapping, EnsureAddressSame, EnsureAddressOrigin};


mod mock;
mod tests;

/////////////////// Data Structures ////////////////////////////////////////////

#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize)]
/// Account definition used for genesis block construction.
pub struct GenesisAccount {
    /// Account nonce.
    pub nonce: U256,
    /// Account balance.
    pub balance: U256,
    /// Full account storage.
    pub storage: std::collections::BTreeMap<H256, H256>,
    /// Account code.
    pub code: Vec<u8>,
}

/////////////////// Trait, Storage, Errors, and Events /////////////////////////

/// The main content directory evm trait.
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Representation for content directory evm membership.
    type MembershipId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + From<u64>
        + Into<u64>;

    type AccountAddressMapping: AccountAddressMapping<Self::AccountId, H160>;

    type Currency: Currency<Self::AccountId>;

    type Evm: pallet_evm::Trait;
}

pub trait AccountAddressMapping<AccountId, Address> {
    // TODO: this function might need a rework as it is not sure if accounts and addresses have 1:1 relation
    fn into_account_id(address: &Address) -> AccountId;

    fn into_address(account_id: &AccountId) -> Address;
}

decl_storage! {
    trait Store for Module<T: Trait> as ContentDirectoryEvm {
        /// Dummy storage value
        pub MyStorageValue get(fn my_storage_value) config(): T::MembershipId;

        AccountCodes get(fn account_codes): map hasher(blake2_128_concat) H160 => Vec<u8>;
        AccountStorages get(fn account_storages):
            double_map hasher(blake2_128_concat) H160, hasher(blake2_128_concat) H256 => H256;
    }

    add_extra_genesis {
        config(accounts): std::collections::BTreeMap<H160, GenesisAccount>;
        build(|config: &GenesisConfig::<T>| {
            for (address, account) in &config.accounts {
                let account_id = <T as Trait>::AccountAddressMapping::into_account_id(address);

                // ASSUME: in one single EVM transaction, the nonce will not increase more than
                // `u128::max_value()`.
                for _ in 0..account.nonce.low_u128() {
                    frame_system::Module::<T>::inc_account_nonce(&account_id);
                }

                <T as Trait>::Currency::deposit_creating(
                    &account_id,
                    account.balance.low_u128().unique_saturated_into(),
                );

                AccountCodes::insert(address, &account.code);

                for (index, value) in &account.storage {
                    AccountStorages::insert(address, index, value);
                }
            }
        });
    }
}

decl_event! {
    pub enum Event<T> where
        MembershipId = <T as Trait>::MembershipId,
    {
        /// Dummy event
        MyDummyEvent(MembershipId),

        /// Dummy event
        MyDummyEvent2(),
    }
}

decl_error! {
    /// ContentDirectoryEvm errors
    pub enum Error for Module<T: Trait> {
        /// Dummy error
        MyDummyError,

        BadOrigin,
    }
}

impl<T: Trait> From<BadOrigin> for Error<T> {
    fn from(_error: BadOrigin) -> Self {
        Error::<T>::BadOrigin
    }
}

/////////////////// Module definition and implementation ///////////////////////

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Setup events
        fn deposit_event() = default;

        /// Testing extrinsic - test each evm feature piece by piece before creating more sophisticated tests
        #[weight = 10_000_000]
        pub fn test_call(
            origin,
        ) -> Result<(), Error<T>> {
            /*
            <Module::<T> as pallet_evm::Trait>::execute_call(
                source: H160,
                target: H160,
                input: Vec<u8>,
                value: U256,
                gas_limit: u32,
                gas_price: U256,
                nonce: Option<U256>,
                apply_state: bool
            );
            */
            let account_id = ensure_signed(origin)?;

            //let _tmp = <EnsureAddressSame as EnsureAddressOrigin<T::Origin>>::try_address_origin(H160::zero(), origin);

            //let address_from = T::AddressMapping::into_account_id(account_id.into());
            let address_from = T::AccountAddressMapping::into_address(&account_id);

            // topup account
            <T as Trait>::Currency::deposit_creating(
                &account_id,
                10000000.into(),
            );

            let result = pallet_evm::Module::<T::Evm>::execute_call(
                //H160::zero(),
                address_from,
                H160::zero(),
                vec![],
                10000.into(),
                4000000,
                1.into(),
                Some(1.into()),
                true,
            );
            println!("{:?}", result);

            // emit event
            Self::deposit_event(RawEvent::MyDummyEvent2());

            Ok(())
        }
    }
}

/*
struct EvmWrapper<T: Trait> {
    _dummy: PhantomData<T>, // 0-sized data meant only to bound generic parameters
}

impl<T: Trait> EvmWrapper<T> {
    // Wrapper
    fn call(
        source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u32,
        gas_price: U256,
        nonce: Option<U256>,
        apply_state: bool
    ) {

        source
        target
        input
        value
        gas_limit
        gas_price
        nonce
        apply_state
    }
}
*/
