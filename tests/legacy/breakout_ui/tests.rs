//! Regression coverage for the tests module.

use super::*;

fn assert_layout_is_clear(width: f32, height: f32) {
    let layout = layout();
    let controls = [
        layout.left,
        layout.stay,
        layout.right,
        layout.hint,
        layout.undo,
        layout.pause,
        layout.new_game,
    ];
    assert!(controls
        .iter()
        .all(|control| !control.overlaps(&layout.board)));
    for (index, left) in controls.iter().enumerate() {
        assert!(controls[index + 1..]
            .iter()
            .all(|right| !left.overlaps(right)));
        assert!(left.x >= 0. && left.y >= 0.);
        assert!(left.right() <= width && left.bottom() <= height);
    }
}

#[test]
fn controls_stay_clear_of_the_board_in_every_layout() {
    crate::ui::with_compact_landscape_layout(|| assert_layout_is_clear(844., 390.));
    crate::ui::with_portrait_layout(|| assert_layout_is_clear(360., 780.));
    crate::ui::with_desktop_layout(|| assert_layout_is_clear(1280., 720.));
}
