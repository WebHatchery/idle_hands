//! Regression coverage for the tests module.

use idle_hands::testing::modules::maze_walk_ui::*;

#[test]
fn portrait_maze_scoreline_uses_compact_copy() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(use_compact_scoreline(
            false,
            idle_hands::testing::ui::is_portrait(),
            idle_hands::testing::ui::display_width()
        ));
    });
    assert!(!use_compact_scoreline(false, false, 720.));
}
