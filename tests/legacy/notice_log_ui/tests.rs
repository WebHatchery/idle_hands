//! Regression coverage for the tests module.

use idle_hands::testing::modules::notice_log_ui::*;
use macroquad_toolkit::notifications::NotificationType;

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
    idle_hands::testing::ui::with_desktop_layout(assert_layout_fits);
    idle_hands::testing::ui::with_compact_landscape_layout(assert_layout_fits);
    idle_hands::testing::ui::with_portrait_layout(assert_layout_fits);
}

fn assert_layout_fits() {
    let (card, close) = log_layout();
    let button = button_rect();
    let (width, height) = idle_hands::testing::ui::layout_size();
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
