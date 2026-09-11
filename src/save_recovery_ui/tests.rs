use super::*;

#[test]
fn dismiss_button_routes_in_every_layout() {
    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(vec2(855., 164.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            clicks(vec2(602., 149.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(vec2(244., 182.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
}

#[test]
fn taps_outside_dismiss_are_ignored() {
    crate::ui::with_desktop_layout(|| assert!(clicks(vec2(10., 10.)).is_none()));
}

#[test]
fn recovery_notice_and_dismiss_button_fit_each_viewport() {
    crate::ui::with_desktop_layout(assert_layout_fits);
    crate::ui::with_compact_landscape_layout(assert_layout_fits);
    crate::ui::with_portrait_layout(assert_layout_fits);
}

fn assert_layout_fits() {
    let (card, dismiss) = layout();
    let (width, height) = crate::ui::layout_size();
    let viewport = Rect::new(0., 0., width, height);
    assert!(contains(viewport, card));
    assert!(contains(card, dismiss));
    assert!(dismiss.w >= 160.);
    assert!(dismiss.h >= 48.);
}

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
