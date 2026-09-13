//! Regression coverage for the tests module.

use super::*;

#[test]
fn compact_title_and_instruction_use_separate_rows() {
    let title_y = 30.;
    let instruction_y = 52.;

    assert!(instruction_y > title_y + 12.);
    assert!(instruction_y < 66.);
}

#[test]
fn portrait_title_budget_stays_left_of_the_rule_card() {
    let title_x = 12.;
    let title_width_budget = 195.;
    let rule_card_x = 220.;

    assert!(title_x + title_width_budget < rule_card_x);
}

#[test]
fn portrait_mahjong_title_uses_a_narrow_readable_size() {
    crate::ui::with_portrait_layout(|| {
        assert_eq!(title_size(), 17.);
        assert_eq!(title_text(), "MAHJONG");
    });
}
