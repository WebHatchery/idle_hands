//! Regression coverage for the tests module.

use idle_hands::testing::modules::memory_pairs_ui::*;

#[test]
fn portrait_memory_pairs_title_uses_a_narrow_header_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
