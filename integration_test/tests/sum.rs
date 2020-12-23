use integration_test::sum;

mod common;

use common::{setup, teardown};

#[test]
fn sum_test() {
    assert_eq!(sum(6,8), 14);
}

#[test]
fn with_fixture_test() {
    setup();
    assert_eq!(sum(7, 14), 21);
    teardown();
}
