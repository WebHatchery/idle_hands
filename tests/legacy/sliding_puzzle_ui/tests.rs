//! Regression coverage for the tests module.

use idle_hands::testing::modules::sliding_puzzle_ui::*;

#[test]
fn portrait_sliding_puzzle_title_uses_a_narrow_header_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
