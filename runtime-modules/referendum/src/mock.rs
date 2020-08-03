#![cfg(test)]

/////////////////// Configuration //////////////////////////////////////////////
use crate::{Error, Event, Instance, Module, ReferendumStage, Trait};

use primitives::H256;
use runtime_io;
use sr_primitives::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use srml_support::{impl_outer_event, impl_outer_origin, parameter_types};
use std::marker::PhantomData;
use system::RawOrigin;

use crate::GenesisConfig;

pub const USER_ADMIN: u64 = 1;
pub const USER_REGULAR: u64 = 2;

/////////////////// Runtime and Instances //////////////////////////////////////
// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;

// module instances

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Instance0;

parameter_types! {
    pub const MaxReferendumOptions: u64 = 10;
    pub const VoteStageDuration: u64 = 5;
}

impl<I: Instance> Trait<I> for Runtime {
    //type Event = Event<Self, I>;
    type Event = ();

    type MaxReferendumOptions = MaxReferendumOptions;
    type ReferendumOption = u64;

    type VoteStageDuration = VoteStageDuration;

    fn is_super_user(account_id: &<Self as system::Trait>::AccountId) -> bool {
        *account_id == USER_ADMIN
    }
}

/////////////////// Module implementation //////////////////////////////////////

impl_outer_origin! {
    pub enum Origin for Runtime {}
}

mod event_mod {
    pub use crate::Event;
}

// TODO: there seems to be bug in substrate - it can't handle instantiable pallet without default instance - report the bug + don't test events for now
impl_outer_event! {
    pub enum TestEvent for Runtime {
        //event_mod<T>, // not working due to bug in substrate
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const MinimumPeriod: u64 = 5;
}

// TODO: find a way to derive the trait
#[allow(non_upper_case_globals)] // `decl_storage` macro defines this weird name
impl Instance for Instance0 {
    const PREFIX: &'static str = "Instance0";

    const PREFIX_FOR_Stage: &'static str = "Instance0_stage";
    const PREFIX_FOR_ReferendumOptions: &'static str = "Instance0_referendum_options";
}

impl system::Trait for Runtime {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    //type Event = TestEvent;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
}

/////////////////// Data structures ////////////////////////////////////////////

#[allow(dead_code)]
#[derive(Clone)]
pub enum OriginType<AccountId> {
    Signed(AccountId),
    //Inherent, <== did not find how to make such an origin yet
    Root,
}

/////////////////// Utility mocks //////////////////////////////////////////////

pub fn mock_origin<T: Trait<I>, I: Instance>(origin: OriginType<T::AccountId>) -> T::Origin {
    match origin {
        OriginType::Signed(account_id) => T::Origin::from(RawOrigin::Signed(account_id)),
        _ => panic!("not implemented"),
    }
}

pub fn default_genesis_config_generic<I: Instance>() -> GenesisConfig<Runtime, I> {
    GenesisConfig::<Runtime, I> {
        stage: (ReferendumStage::default(), 0),
        //referendum_options: None,
        referendum_options: vec![],
    }
}

pub fn default_genesis_config() -> GenesisConfig<Runtime, Instance0> {
    default_genesis_config_generic::<Instance0>()
}

pub fn build_test_externalities<I: Instance>(
    config: GenesisConfig<Runtime, I>,
) -> runtime_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    config.assimilate_storage(&mut t).unwrap();

    t.into()
}

/////////////////// Mocks of Module's actions //////////////////////////////////

pub struct InstanceMocks<T: Trait<I>, I: Instance> {
    _dummy: PhantomData<(T, I)>, // 0-sized data meant only to bound generic parameters
}

impl<T: Trait<I>, I: Instance> InstanceMocks<T, I> {
    pub fn start_referendum(
        origin: OriginType<T::AccountId>,
        options: Vec<T::ReferendumOption>,
        expected_result: Result<(), Error>,
    ) -> () {
        // check method returns expected result
        assert_eq!(
            Module::<T, I>::start_referendum(mock_origin::<T, I>(origin), options,),
            expected_result,
        );
        /*
        // check event was emitted
        assert_eq!(
            System::events().last().unwrap().event,
            TestEvent::event_mod(RawEvent::ReferendumStarted(
                options
            ))
        );
        */
    }
}