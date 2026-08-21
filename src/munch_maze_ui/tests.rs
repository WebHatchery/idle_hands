use super::*;

#[test]
fn compact_controls_stay_clear_of_the_board() {
    crate::ui::with_compact_landscape_layout(|| {
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
            .all(|control| control.x >= layout.board.right()));
        for (index, left) in controls.iter().enumerate() {
            assert!(controls[index + 1..]
                .iter()
                .all(|right| !left.overlaps(right)));
        }
    });
}
