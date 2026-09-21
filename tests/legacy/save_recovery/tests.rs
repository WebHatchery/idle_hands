//! Regression coverage for the tests module.

use idle_hands::testing::modules::save_recovery::*;

#[test]
fn recovery_summary_distinguishes_preserved_and_unpreserved_saves() {
    let mut one = SaveRecoveryNotice::new();
    one.record(true);
    assert_eq!(one.summary(), "One save was moved aside safely.");

    let mut mixed = SaveRecoveryNotice::new();
    mixed.record(true);
    mixed.record(false);
    assert_eq!(
        mixed.summary(),
        "2 saves need attention before they can load."
    );
}
