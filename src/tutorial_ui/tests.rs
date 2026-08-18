use super::*;

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}

#[test]
fn desktop_continue_button_stays_inside_the_how_to_play_panel() {
    assert!(contains(OVERLAY_RECT, CONTINUE_RECT));
}
