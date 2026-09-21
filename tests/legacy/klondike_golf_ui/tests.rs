//! Regression coverage for the tests module.

use idle_hands::testing::modules::klondike_golf_ui::*;

#[test]
fn portrait_title_uses_a_narrow_header_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}

#[test]
fn desktop_controls_stay_clear_of_the_tableau() {
    let l = desktop_layout();
    assert!(!l.board.overlaps(&l.hint));
    assert!(!l.board.overlaps(&l.undo));
    assert!(!l.board.overlaps(&l.new_game));
}
