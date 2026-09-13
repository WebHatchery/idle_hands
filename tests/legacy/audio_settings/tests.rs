//! Regression coverage for the tests module.

use super::*;

#[test]
fn levels_have_stable_labels_and_volumes() {
    assert_eq!(label(true, 0), "Quiet");
    assert_eq!(label(true, 1), "Clear");
    assert_eq!(label(true, 2), "Full");
    assert_eq!(label(true, 3), "Bright");
    assert_eq!(volume(true, 0), 0.25);
    assert_eq!(volume(true, 2), 0.75);
    assert_eq!(volume(true, 3), 1.0);
    assert_eq!(meter(true, 0), "|···");
    assert_eq!(meter(true, 3), "||||");
}

#[test]
fn level_normalization_and_cycling_are_safe_for_old_saves() {
    assert_eq!(normalize(99), 3);
    assert_eq!(next_level(3), 0);
    assert_eq!(next_level(99), 0);
    assert_eq!(label(false, 3), "Off");
    assert_eq!(volume(false, 3), 0.0);
    assert_eq!(meter(false, 3), "····");
}
