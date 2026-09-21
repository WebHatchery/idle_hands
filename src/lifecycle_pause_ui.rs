//! Touch-first pause sheet shown after returning from a long frame gap.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub fn layout() -> (Rect, Rect) {
    if crate::ui::is_compact_landscape() {
        (
            Rect::new(150., 45., 540., 300.),
            Rect::new(300., 250., 240., 52.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(20., 250., 320., 280.),
            Rect::new(60., 410., 240., 58.),
        )
    } else {
        (
            Rect::new(300., 190., 680., 340.),
            Rect::new(500., 430., 280., 64.),
        )
    }
}

pub fn clicks(point: Vec2) -> Vec<UiAction> {
    let (_, resume) = layout();
    if crate::ui::hit(resume, point) {
        vec![UiAction::ResumeLifecycle]
    } else {
        vec![]
    }
}

pub fn panel(rect: Rect, fill: Color, high_contrast: bool) {
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

pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw(state: &AppState) {
    let (card, resume) = layout();
    let (width, height) = crate::ui::layout_size();
    draw_rectangle(0., 0., width, height, Color::new(0.03, 0.02, 0.06, 0.88));
    panel(
        card,
        crate::accessibility::board_fill(state.high_contrast),
        state.high_contrast,
    );
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let (title_x, title_y, body_x, body_y, title_size, body_size) = if compact {
        (190., 100., 190., 145., 25., 13.)
    } else if portrait {
        (48., 310., 48., 350., 25., 13.)
    } else {
        (370., 275., 370., 325., 34., 18.)
    };
    text(
        "PAUSED SAFELY",
        title_x,
        title_y,
        crate::accessibility::text_size(title_size, state.large_text),
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
    );
    text(
        "The cabinet was away for a moment.",
        body_x,
        body_y,
        crate::accessibility::text_size(body_size, state.large_text),
        WHITE,
    );
    text(
        "Your round is saved. Tap RESUME PLAY when ready.",
        body_x,
        body_y + 28.,
        crate::accessibility::text_size(body_size, state.large_text),
        WHITE,
    );
    panel(
        resume,
        if state.high_contrast {
            crate::accessibility::board_fill(true)
        } else {
            crate::theme::MOSS_DARK
        },
        state.high_contrast,
    );
    let label_size = if portrait { 14. } else { 16. };
    text(
        "RESUME PLAY",
        resume.x + resume.w * 0.5 - 52.,
        resume.y + resume.h * 0.62,
        crate::accessibility::text_size(label_size, state.large_text),
        WHITE,
    );
}
