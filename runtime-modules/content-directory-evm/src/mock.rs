#![cfg(test)]

use frame_support::{impl_outer_event, impl_outer_origin, parameter_types};
use frame_support::traits::{OnFinalize};
use frame_system::{EnsureOneOf, EnsureRoot, EnsureSigned, RawOrigin};
use sp_core::{H256, U256};
use pallet_evm::{
    Account as EVMAccount, EnsureAddressRoot, EnsureAddressNever, EnsureAddressTruncated, FeeCalculator,
    HashedAddressMapping,
};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use crate::{GenesisConfig, Module, Trait};
use std::marker::PhantomData;
use sp_io;

mod event_mod {
    pub use crate::Event;
}

impl_outer_origin! {
    pub enum Origin for Runtime {}
}

impl_outer_event! {
    pub enum TestEvent for Runtime {
        event_mod<T>,
        frame_system<T>,
        pallet_balances<T>,
        pallet_evm<T>,
    }
}


// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const MinimumPeriod: u64 = 5;

    pub const ExistentialDeposit: u32 = 0;
}

impl frame_system::Trait for Runtime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = TestEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = ();
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    //type AccountData = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

impl Trait for Runtime {
    type Event = TestEvent;
    type MembershipId = u64;

    type AddressMapping = HashedAddressMapping<Blake2Hasher>;
}

impl pallet_timestamp::Trait for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_balances::Trait for Runtime {
    type Balance = u64;
    type Event = TestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Self>;
    //type AccountStore = ();
    type WeightInfo = ();
    type MaxLocks = ();
}

//static ISTANBUL_CONFIG: Config = Config::istanbul();

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> U256 {
        // Gas price is always one token per gas.
        1.into()
    }
}

parameter_types! {
    pub const ChainId: u64 = 1;
}
/*
// This trait needs newer version of evm pallet (thus upgrade to whole branch is required)
impl pallet_evm::Trait for Runtime {
    type FeeCalculator = FixedGasPrice;
    type CallOrigin = EnsureAddressRoot<Self::AccountId>;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = pallet_balances::Module<Runtime>;

    // TODO: make events work
    type Event = TestEvent;
    //type Event: From<Event<Self>> + Into<Self::Event>;
    type Precompiles = ();
    type ChainId = ChainId;
    //fn config() -> &'static Config {  }
}
*/
// This trait needs newer version of evm pallet (thus upgrade to whole branch is required)
impl pallet_evm::Trait for Runtime {
    type FeeCalculator = FixedGasPrice;
    type CallOrigin = EnsureAddressRoot<Self::AccountId>;
    type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = pallet_balances::Module<Runtime>;

    // TODO: make events work
    type Event = TestEvent;
    //type Event: From<Event<Self>> + Into<Self::Event>;
    type Precompiles = ();
    type ChainId = ChainId;
    //fn config() -> &'static Config {  }
}

/////////////////// Data structures ////////////////////////////////////////////

#[allow(dead_code)]
#[derive(Clone)]
pub enum OriginType<AccountId> {
    Signed(AccountId),
    //Inherent, <== did not find how to make such an origin yet
    Root,
}

/////////////////// Util macros ////////////////////////////////////////////////

pub fn default_genesis_config() -> GenesisConfig<Runtime> {
    GenesisConfig::<Runtime> {
        my_storage_value: 0,
    }
}

pub fn build_test_externalities(config: GenesisConfig<Runtime>) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    config.assimilate_storage(&mut t).unwrap();

    let mut result = Into::<sp_io::TestExternalities>::into(t.clone());

    // Make sure we are not in block 1 where no events are emitted
    // see https://substrate.dev/recipes/2-appetizers/4-events.html#emitting-events
    result.execute_with(|| InstanceMockUtils::<Runtime>::increase_block_number(1));

    result
}

pub struct InstanceMockUtils<T: Trait> {
    _dummy: PhantomData<T>, // 0-sized data meant only to bound generic parameters
}

impl<T: Trait> InstanceMockUtils<T> where T::BlockNumber: From<u64> + Into<u64> {
    pub fn mock_origin(origin: OriginType<T::AccountId>) -> T::Origin {
        match origin {
            OriginType::Signed(account_id) => T::Origin::from(RawOrigin::Signed(account_id)),
            OriginType::Root => RawOrigin::Root.into(),
            //_ => panic!("not implemented"),
        }
    }

    pub fn increase_block_number(increase: u64) -> () {
        let block_number = frame_system::Module::<T>::block_number();

        for i in 0..increase {
            let tmp_index: T::BlockNumber = block_number + i.into();

            <Module<T> as OnFinalize<T::BlockNumber>>::on_finalize(tmp_index);
            frame_system::Module::<T>::set_block_number(tmp_index + 1.into());
        }
    }
}

/////////////////// Mocks of Module's actions //////////////////////////////////

pub struct InstanceMocks<T: Trait> {
    _dummy: PhantomData<T>, // 0-sized data meant only to bound generic parameters
}

impl<T: Trait> InstanceMocks<T> where T::BlockNumber: From<u64> + Into<u64> {
    pub fn test_call(
        origin: OriginType<T::AccountId>,
    ) {

        assert_eq!(
            Module::<T>::test_call(
                InstanceMockUtils::<T>::mock_origin(origin.clone())
            ).is_ok(),
            true,
        );
    }
}
