//! Regression coverage for the tests module.

use idle_hands::testing::modules::mastermind_ui::*;

#[test]
fn portrait_mastermind_title_uses_a_narrow_readable_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
