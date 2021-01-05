#![cfg(test)]

use crate::mock::*;

type Mocks = InstanceMocks<Runtime>;
type MockUtils = InstanceMockUtils<Runtime>;

#[test]
fn evm_first_test() {
    let config = default_genesis_config();

    build_test_externalities(config).execute_with(|| {
        let origin = OriginType::Signed(REGULAR_ACCOUNT_1);

        Mocks::test_call(origin);
        // todo
    });
}
