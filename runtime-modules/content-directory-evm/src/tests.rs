#![cfg(test)]

use crate::mock::*;
use crate::{AccountAddressMapping, Trait};
use sp_runtime::AccountId32;
use hex::FromHex;

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

// Test that contract can be deployed to EVM.
#[test]
fn contract_deployement() {
    let config = default_genesis_config();

    build_test_externalities(config).execute_with(|| {
        let origin = OriginType::Root;

        // TODO: load bytecode from file or compile it from source
        //let abi = [{"inputs":[],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"string","name":"message","type":"string"}],"name":"Said","type":"event"},{"constant":true,"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"}];
        let bytecode_string = "608060405234801561001057600080fd5b50336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507fc68045c3c562488255b55aa2c4c7849de001859ff0d8a36a75c2d5ed80100fb660405180806020018281038252600d8152602001807f48656c6c6f2c20776f726c64210000000000000000000000000000000000000081525060200191505060405180910390a160cf806100c76000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c80638da5cb5b14602d575b600080fd5b60336075565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff168156fea265627a7a72315820fae816ad954005c42bea7bc7cb5b19f7fd5d3a250715ca2023275c9ca7ce644064736f6c634300050f0032";
        let bytecode: Vec<u8> = Vec::from_hex(bytecode_string).expect("Invalid hex");


        Mocks::deploy_smart_contract(origin, AccountId32::from(REGULAR_ACCOUNT_1), bytecode);
    });
}
