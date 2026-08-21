use super::*;

#[test]
fn portrait_maze_scoreline_uses_compact_copy() {
    crate::ui::with_portrait_layout(|| {
        assert!(use_compact_scoreline(
            false,
            crate::ui::is_portrait(),
            crate::ui::display_width()
        ));
    });
    assert!(!use_compact_scoreline(false, false, 720.));
}
