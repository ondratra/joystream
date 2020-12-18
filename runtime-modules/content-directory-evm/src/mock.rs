#![cfg(test)]

use frame_support::{impl_outer_event, impl_outer_origin, parameter_types};
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

impl_outer_origin! {
    pub enum Origin for Runtime {}
}

impl_outer_event! {
    pub enum TestEvent for Runtime {
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
