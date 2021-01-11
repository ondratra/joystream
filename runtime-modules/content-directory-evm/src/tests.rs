#![cfg(test)]

use crate::mock::*;
use crate::{AccountAddressMapping, Trait};
use sp_runtime::AccountId32;

type Mocks = InstanceMocks<Runtime>;
type MockUtils = InstanceMockUtils<Runtime>;

// test that environment bare setup is ok
#[test]
fn evm_first_test() {
    let config = default_genesis_config();

    build_test_externalities(config).execute_with(|| {
        let origin = OriginType::Signed(AccountId32::from(REGULAR_ACCOUNT_1));

        Mocks::test_call(origin, AccountId32::from(REGULAR_ACCOUNT_2));
        // todo
    });
}

// Test that EVM address can be converted into Substrate account and vice versa
#[test]
fn address_account_conversions() {
    let config = default_genesis_config();

    // TODO: handle rest of the 32 bytes (last 12 bytes - 20 are handled now)
    build_test_externalities(config).execute_with(|| {
        let account_id = AccountId32::from(REGULAR_ACCOUNT_1);

        let address = <Runtime as Trait>::AccountAddressMapping::into_address(&account_id);

        let account_id_derived =
            <Runtime as Trait>::AccountAddressMapping::into_account_id(&address);

        assert!(MockUtils::accounts_compare_20_bytes(
            &account_id,
            &account_id_derived
        ));

        // last 12 bytes will was lost during the conversion so the addresses are not completely equal
        assert_ne!(account_id_derived, account_id);
    });
}

#[ignore]
#[test]
fn currency_conversions() {}
