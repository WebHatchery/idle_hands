use super::*;

#[test]
fn resume_button_routes_in_every_layout() {
    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(vec2(640., 462.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            clicks(vec2(420., 276.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(vec2(180., 439.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
    });
}

#[test]
fn taps_outside_resume_button_are_blocked() {
    crate::ui::with_desktop_layout(|| assert!(clicks(vec2(10., 10.)).is_empty()));
}
