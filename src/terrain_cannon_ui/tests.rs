//! Regression coverage for the tests module.

use super::*;

#[test]
fn compact_controls_stay_clear_of_the_board() {
    crate::ui::with_compact_landscape_layout(|| assert_layout_is_clear(844., 390.));
}

#[test]
fn portrait_controls_stay_clear_of_the_board() {
    crate::ui::with_portrait_layout(|| assert_layout_is_clear(360., 780.));
}

#[test]
fn desktop_controls_stay_clear_of_the_board() {
    crate::ui::with_desktop_layout(|| assert_layout_is_clear(1280., 720.));
}

fn assert_layout_is_clear(width: f32, height: f32) {
    let layout = layout();
    let controls = [
        layout.angle_down,
        layout.angle_up,
        layout.power_down,
        layout.power_up,
        layout.fire,
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
        assert!(left.x >= 0. && left.y >= 0.);
        assert!(left.right() <= width && left.bottom() <= height);
    }
}
