//! Regression coverage for the tests module.

use super::*;

fn assert_controls_are_clear() {
    let layout = layout();
    let controls = [
        layout.up,
        layout.left,
        layout.down,
        layout.right,
        layout.pause,
        layout.undo,
        layout.new_game,
    ];

    assert!(controls
        .iter()
        .all(|control| !control.overlaps(&layout.board)));
    for (index, left) in controls.iter().enumerate() {
        assert!(controls[index + 1..]
            .iter()
            .all(|right| !left.overlaps(right)));
    }
}

#[test]
fn controls_stay_clear_of_the_board_in_every_layout() {
    crate::ui::with_compact_landscape_layout(assert_controls_are_clear);
    crate::ui::with_desktop_layout(assert_controls_are_clear);
    crate::ui::with_portrait_layout(assert_controls_are_clear);
}
