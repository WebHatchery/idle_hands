use super::*;

#[test]
fn portrait_pipe_scoreline_uses_compact_copy() {
    crate::ui::with_portrait_layout(|| {
        assert!(use_compact_scoreline(false, crate::ui::is_portrait(), 390.));
    });
    assert!(!use_compact_scoreline(false, false, 720.));
}
