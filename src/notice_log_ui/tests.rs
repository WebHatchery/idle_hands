use super::*;

#[test]
fn settings_notice_button_routes_in_every_layout() {
    let state = crate::state::AppState {
        screen: crate::state::Screen::Settings,
        ..Default::default()
    };
    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            button_click(&state, vec2(1035., 584.)),
            Some(UiAction::ToggleNoticeLog)
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            button_click(&state, vec2(690., 290.)),
            Some(UiAction::ToggleNoticeLog)
        ));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            button_click(&state, vec2(180., 634.)),
            Some(UiAction::ToggleNoticeLog)
        ));
    });
}

#[test]
fn log_close_button_routes_in_every_layout() {
    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(vec2(920., 614.)).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            clicks(vec2(690., 337.)).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(vec2(260., 652.)).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
}

#[test]
fn long_notice_messages_are_shortened_for_the_log() {
    assert_eq!(short_message("abcdef", 4), "abcd…");
    assert_eq!(short_message("short", 10), "short");
}
