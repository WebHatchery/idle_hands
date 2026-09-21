//! Regression coverage for the tests module.

use idle_hands::testing::modules::number_match_ui::*;

#[test]
fn portrait_title_leaves_room_before_the_rule_card() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 19.));
}
