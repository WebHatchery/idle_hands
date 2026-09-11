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

#[test]
fn recent_notifications_are_newest_first_and_bounded() {
    let history = [
        LoggedNotification {
            message: "old".into(),
            notification_type: NotificationType::Info,
        },
        LoggedNotification {
            message: "middle".into(),
            notification_type: NotificationType::Warning,
        },
        LoggedNotification {
            message: "new".into(),
            notification_type: NotificationType::Success,
        },
    ];
    let recent = recent_notifications(&history, 2);
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].message, "new");
    assert_eq!(recent[1].message, "middle");
}

#[test]
fn high_contrast_notice_rows_stop_relying_on_severity_color() {
    assert_eq!(notice_color(true, NotificationType::Warning), WHITE);
    let ordinary = notice_color(false, NotificationType::Warning);
    assert!(ordinary.r < 1.0 || ordinary.g < 1.0 || ordinary.b < 1.0);
}

#[test]
fn notice_log_controls_stay_inside_each_logical_viewport() {
    crate::ui::with_desktop_layout(assert_layout_fits);
    crate::ui::with_compact_landscape_layout(assert_layout_fits);
    crate::ui::with_portrait_layout(assert_layout_fits);
}

fn assert_layout_fits() {
    let (card, close) = log_layout();
    let button = button_rect();
    let (width, height) = crate::ui::layout_size();
    let viewport = Rect::new(0., 0., width, height);
    assert!(contains(viewport, card));
    assert!(contains(card, close));
    assert!(contains(viewport, button));
    assert!(close.w >= 140.);
    assert!(close.h >= 44.);
}

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
