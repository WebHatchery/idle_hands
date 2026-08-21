use super::*;

#[test]
fn compact_controls_stay_clear_of_the_board() {
    crate::ui::with_compact_landscape_layout(assert_controls_are_clear);
}

#[test]
fn desktop_controls_stay_clear_of_the_board() {
    crate::ui::with_desktop_layout(assert_controls_are_clear);
}

#[test]
fn portrait_controls_stay_clear_of_the_board() {
    crate::ui::with_portrait_layout(assert_controls_are_clear);
}

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
