//! Responsive session notice history and its Settings entry point.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;
use macroquad_toolkit::notifications::{LoggedNotification, NotificationType};

#[cfg(test)]
mod tests;

fn button_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(600., 268., 180., 44.)
    } else if crate::ui::is_portrait() {
        Rect::new(22., 615., 316., 38.)
    } else {
        Rect::new(990., 560., 90., 48.)
    }
}

fn log_layout() -> (Rect, Rect) {
    if crate::ui::is_compact_landscape() {
        (
            Rect::new(30., 16., 780., 358.),
            Rect::new(600., 315., 180., 44.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(12., 54., 336., 640.),
            Rect::new(190., 630., 140., 44.),
        )
    } else {
        (
            Rect::new(220., 70., 840., 600.),
            Rect::new(820., 590., 200., 48.),
        )
    }
}

pub fn button_click(state: &AppState, point: Vec2) -> Option<UiAction> {
    if state.screen == crate::state::Screen::Settings
        && !state.confirm_reset
        && crate::ui::hit(button_rect(), point)
    {
        Some(UiAction::ToggleNoticeLog)
    } else {
        None
    }
}

pub fn clicks(point: Vec2) -> Vec<UiAction> {
    let (_, close) = log_layout();
    if crate::ui::hit(close, point) {
        vec![UiAction::ToggleNoticeLog]
    } else {
        vec![]
    }
}

fn panel(rect: Rect, fill: Color, high_contrast: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        crate::accessibility::grid_line(high_contrast),
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color, large_text: bool) {
    crate::ui::draw_text(
        value,
        x,
        y,
        crate::accessibility::text_size(size, large_text),
        color,
    );
}

fn short_message(message: &str, limit: usize) -> String {
    let mut value: String = message.chars().take(limit).collect();
    if message.chars().count() > limit {
        value.push('…');
    }
    value
}

fn recent_notifications(history: &[LoggedNotification], limit: usize) -> Vec<&LoggedNotification> {
    history.iter().rev().take(limit).collect()
}

fn level_label(notification_type: NotificationType) -> &'static str {
    match notification_type {
        NotificationType::Success => "OK",
        NotificationType::Info => "INFO",
        NotificationType::Warning => "WARN",
        NotificationType::Danger => "ALERT",
    }
}

fn notice_color(high_contrast: bool, notification_type: NotificationType) -> Color {
    if high_contrast {
        WHITE
    } else {
        notification_type.color()
    }
}

pub fn draw_button(state: &AppState) {
    if state.screen != crate::state::Screen::Settings
        || state.confirm_reset
        || state.notice_log_view
    {
        return;
    }
    let rect = button_rect();
    panel(
        rect,
        if state.high_contrast {
            crate::accessibility::board_fill(true)
        } else {
            crate::theme::SURFACE
        },
        state.high_contrast,
    );
    text(
        "NOTICES",
        rect.x + rect.w * 0.5 - if crate::ui::is_portrait() { 28. } else { 31. },
        rect.y + rect.h * 0.67,
        if crate::ui::is_portrait() { 11. } else { 13. },
        WHITE,
        state.large_text,
    );
}

pub fn draw_log(state: &AppState, history: &[LoggedNotification]) {
    if !state.notice_log_view {
        return;
    }
    let (card, close) = log_layout();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    panel(
        card,
        crate::accessibility::board_fill(state.high_contrast),
        state.high_contrast,
    );
    text(
        "SESSION NOTICES",
        card.x + if portrait { 18. } else { 28. },
        card.y + if portrait { 36. } else { 44. },
        if portrait { 22. } else { 30. },
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
        state.large_text,
    );
    let row_height = if compact { 34. } else { 42. };
    let first_y = card.y + if portrait { 78. } else { 88. };
    let limit = if compact { 7 } else { 10 };
    let recent = recent_notifications(history, limit);
    if recent.is_empty() {
        text(
            "No notices have been recorded this session.",
            card.x + if portrait { 18. } else { 28. },
            first_y,
            if portrait { 12. } else { 16. },
            WHITE,
            state.large_text,
        );
    } else {
        for (row, notice) in recent.iter().enumerate() {
            let y = first_y + row as f32 * row_height;
            let color = notice_color(state.high_contrast, notice.notification_type);
            text(
                level_label(notice.notification_type),
                card.x + if portrait { 18. } else { 28. },
                y,
                if portrait { 10. } else { 12. },
                color,
                state.large_text,
            );
            text(
                &short_message(&notice.message, if portrait { 34 } else { 74 }),
                card.x + if portrait { 70. } else { 82. },
                y,
                if portrait { 10. } else { 14. },
                WHITE,
                state.large_text,
            );
        }
    }
    panel(
        close,
        if state.high_contrast {
            crate::accessibility::board_fill(true)
        } else {
            crate::theme::MOSS_DARK
        },
        state.high_contrast,
    );
    text(
        "CLOSE",
        close.x + close.w * 0.5 - if portrait { 21. } else { 25. },
        close.y + close.h * 0.67,
        if portrait { 12. } else { 14. },
        WHITE,
        state.large_text,
    );
}
