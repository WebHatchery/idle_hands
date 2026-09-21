//! Regression coverage for the tests module.

use idle_hands::testing::modules::save_recovery_ui::*;

#[test]
fn dismiss_button_routes_in_every_layout() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(vec2(855., 164.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
    idle_hands::testing::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            clicks(vec2(602., 149.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(vec2(244., 182.)),
            Some(UiAction::DismissSaveRecovery)
        ));
    });
}

#[test]
fn taps_outside_dismiss_are_ignored() {
    idle_hands::testing::ui::with_desktop_layout(|| assert!(clicks(vec2(10., 10.)).is_none()));
}

#[test]
fn recovery_notice_and_dismiss_button_fit_each_viewport() {
    idle_hands::testing::ui::with_desktop_layout(assert_layout_fits);
    idle_hands::testing::ui::with_compact_landscape_layout(assert_layout_fits);
    idle_hands::testing::ui::with_portrait_layout(assert_layout_fits);
}

fn assert_layout_fits() {
    let (card, dismiss) = layout();
    let (width, height) = idle_hands::testing::ui::layout_size();
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
