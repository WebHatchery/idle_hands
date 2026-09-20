//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_board_stays_below_the_shared_header() {
    assert!(portrait_board().y >= 120.);
}

#[test]
fn portrait_board_and_controls_fit_the_vertical_budget() {
    let board = portrait_board();
    let bottom_controls_y = 650.;
    let moves_y = 730.;

    assert!(board.bottom() < bottom_controls_y);
    assert!(moves_y < 844.);
}
