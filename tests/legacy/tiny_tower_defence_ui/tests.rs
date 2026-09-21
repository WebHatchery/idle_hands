//! Regression coverage for the tests module.

use idle_hands::testing::modules::tiny_tower_defence_ui::*;

#[test]
fn portrait_title_leaves_room_before_the_rule_card() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 18.));
}

#[test]
fn compact_status_stays_between_the_title_and_rule_card() {
    let title_right_budget = idle_hands::testing::ui::COMPACT_HEADER_TITLE_X + 150.;
    let rule_card_x = 494.;

    assert!(compact_status_x() > title_right_budget);
    assert!(compact_status_x() + 190. < rule_card_x);
}
