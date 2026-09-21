//! Regression coverage for the tests module.

use idle_hands::testing::modules::pipe_loop_ui::*;

#[test]
fn portrait_pipe_scoreline_uses_compact_copy() {
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(use_compact_scoreline(
            false,
            idle_hands::testing::ui::is_portrait(),
            390.
        ));
    });
    assert!(!use_compact_scoreline(false, false, 720.));
}
