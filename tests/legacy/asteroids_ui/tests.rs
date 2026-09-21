//! Regression coverage for the tests module.

use idle_hands::testing::modules::asteroids_ui::*;

#[test]
fn compact_header_keeps_breadcrumb_title_status_and_board_separate() {
    let (title_x, title_y, status_x, status_y) = compact_header();
    let layout = compact_layout();

    assert!(title_x >= 70.);
    assert_eq!(title_x, status_x);
    assert!(status_y > title_y + 12.);
    assert!(layout.board.y > status_y + 8.);
    assert!(layout.board.y + layout.board.h < 350.);
}
