use super::*;

#[test]
fn portrait_title_leaves_room_before_the_rule_card() {
    crate::ui::with_portrait_layout(|| assert_eq!(title_size(), 18.));
}

#[test]
fn compact_status_stays_between_the_title_and_rule_card() {
    let title_right_budget = crate::ui::COMPACT_HEADER_TITLE_X + 150.;
    let rule_card_x = 494.;

    assert!(compact_status_x() > title_right_budget);
    assert!(compact_status_x() + 190. < rule_card_x);
}
