//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_difficulty_row_stays_below_the_rule_card() {
    let rule_card_right = 220.;

    for index in 0..3 {
        let difficulty = portrait_difficulty_rect(index);
        assert!(difficulty.right() <= rule_card_right);
        assert!(difficulty.y >= 100.);
    }
    assert!(portrait_board().y > portrait_difficulty_rect(0).bottom() + 8.);
}

#[test]
fn portrait_board_and_controls_fit_the_vertical_budget() {
    let board = portrait_board();
    let bottom_controls_y = 650.;
    let moves_y = 730.;

    assert!(board.bottom() < bottom_controls_y);
    assert!(moves_y < 844.);
}
