use super::*;

fn assert_button_fits(panel: Rect, button: Rect) {
    assert!(button.x >= panel.x);
    assert!(button.y >= panel.y);
    assert!(button.right() <= panel.right());
    assert!(button.bottom() <= panel.bottom());
    assert!(button.w >= 110.);
    assert!(button.h >= 44.);
}

fn assert_fits(layout: Layout, width: f32, height: f32) {
    assert!(layout.panel.x >= 0.);
    assert!(layout.panel.y >= 0.);
    assert!(layout.panel.right() <= width);
    assert!(layout.panel.bottom() <= height);
    assert_button_fits(layout.panel, layout.cancel);
    assert_button_fits(layout.panel, layout.start);
    assert!(layout.title_size > 0.);
    assert!(layout.detail_size > 0.);
}

#[test]
fn desktop_restart_modal_stays_inside_the_logical_viewport() {
    assert_fits(layout(false, false), 1280., 720.);
}

#[test]
fn compact_restart_modal_stays_inside_the_logical_viewport() {
    assert_fits(layout(false, true), 844., 390.);
}

#[test]
fn portrait_restart_modal_stays_inside_the_logical_viewport() {
    assert_fits(layout(true, false), 360., 780.);
}
