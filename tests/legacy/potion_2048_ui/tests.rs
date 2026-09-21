//! Regression coverage for the tests module.

use idle_hands::testing::modules::potion_2048_ui::*;

#[test]
fn portrait_potion_status_uses_a_short_header_copy() {
    let status = portrait_brew_status(0, 0, 64_096, 0);

    assert_eq!(status, "S0  •  B0  •  G64096  •  C0");
    assert!(status.chars().count() < 30);
}
