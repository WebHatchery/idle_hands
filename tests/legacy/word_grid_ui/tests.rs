//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_scoreline_sits_below_the_rule_card_and_above_the_board() {
    let (scoreline_x, scoreline_y) = portrait_scoreline_position();
    let rule_card_bottom = 90.;
    let board_y = 105.;

    assert!(scoreline_x >= 10.);
    assert!(scoreline_y > rule_card_bottom + 8.);
    assert!(scoreline_y < board_y);
}

#[test]
fn compact_scoreline_stays_before_the_rule_card() {
    let (scoreline_x, scoreline_y) = compact_scoreline_position();
    let rule_card_x = 494.;
    assert!(scoreline_x >= 250.);
    assert_eq!(scoreline_y, 28.);
    assert!(scoreline_x + 190. < rule_card_x);
}

#[test]
fn portrait_mode_button_stays_inside_the_logical_viewport() {
    crate::ui::with_portrait_layout(|| {
        let mode = layout().mode;
        assert!(mode.x >= 0.);
        assert!(mode.right() <= crate::responsive_ui::WIDTH);
    });
}
