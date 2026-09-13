//! Touchable recovery notice for saves preserved after a load failure.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
#[path = "../tests/legacy/save_recovery_ui/tests.rs"]
mod tests;

fn layout() -> (Rect, Rect) {
    if crate::ui::is_compact_landscape() {
        (
            Rect::new(135., 54., 570., 140.),
            Rect::new(520., 125., 165., 48.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(16., 60., 328., 160.),
            Rect::new(164., 158., 160., 48.),
        )
    } else {
        (
            Rect::new(320., 68., 640., 140.),
            Rect::new(770., 140., 170., 48.),
        )
    }
}

pub fn clicks(point: Vec2) -> Option<UiAction> {
    let (_, dismiss) = layout();
    crate::ui::hit(dismiss, point).then_some(UiAction::DismissSaveRecovery)
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

pub fn draw(state: &AppState) {
    let Some(notice) = state.save_recovery.as_ref() else {
        return;
    };
    let (card, dismiss) = layout();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    panel(
        card,
        crate::accessibility::board_fill(state.high_contrast),
        state.high_contrast,
    );
    let (title_x, title_y, body_x, body_y) = if compact {
        (155., 84., 155., 111.)
    } else if portrait {
        (30., 88., 30., 116.)
    } else {
        (340., 100., 340., 128.)
    };
    text(
        "SAVE RECOVERY",
        title_x,
        title_y,
        if portrait { 18. } else { 22. },
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
        state.large_text,
    );
    text(
        &notice.summary(),
        body_x,
        body_y,
        if portrait { 11. } else { 14. },
        WHITE,
        state.large_text,
    );
    text(
        "Your cabinet is ready. Tap DISMISS when understood.",
        body_x,
        body_y + if portrait { 24. } else { 28. },
        if portrait { 10. } else { 13. },
        WHITE,
        state.large_text,
    );
    panel(
        dismiss,
        if state.high_contrast {
            crate::accessibility::board_fill(true)
        } else {
            crate::theme::MOSS_DARK
        },
        state.high_contrast,
    );
    text(
        "DISMISS",
        dismiss.x + dismiss.w * 0.5 - 32.,
        dismiss.y + dismiss.h * 0.63,
        if portrait { 13. } else { 15. },
        WHITE,
        state.large_text,
    );
}
