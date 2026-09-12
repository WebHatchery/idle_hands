//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_controls_leave_a_right_edge_margin() {
    crate::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(layout.hint.right() <= crate::responsive_ui::WIDTH - 8.);
        assert!(layout.difficulty.right() <= crate::responsive_ui::WIDTH - 8.);
    });
}
