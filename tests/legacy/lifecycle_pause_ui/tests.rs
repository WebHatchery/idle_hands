//! Regression coverage for the tests module.

use super::*;

#[test]
fn taps_outside_resume_button_are_blocked() {
    crate::ui::with_desktop_layout(|| assert!(clicks(vec2(10., 10.)).is_empty()));
}

#[test]
fn pause_sheet_fits_inside_each_logical_viewport() {
    crate::ui::with_desktop_layout(assert_layout_fits);
    crate::ui::with_compact_landscape_layout(assert_layout_fits);
    crate::ui::with_portrait_layout(assert_layout_fits);
}

fn assert_layout_fits() {
    let (card, resume) = layout();
    let (width, height) = crate::ui::layout_size();
    let viewport = Rect::new(0., 0., width, height);
    assert!(contains(viewport, card));
    assert!(contains(card, resume));
    assert!(resume.w >= 240.);
    assert!(resume.h >= 52.);
}

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
