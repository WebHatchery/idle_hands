//! Regression coverage for the tests module.

use super::*;

#[test]
fn responsive_continue_buttons_stay_inside_their_panels() {
    assert!(contains(PORTRAIT_PANEL, continue_rect(false)));
    assert!(contains(LANDSCAPE_PANEL, continue_rect(true)));
}

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
