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
