//! Shared restart confirmation modal layout and touch routing.

use crate::{
    state::{AppState, Screen},
    ui::UiAction,
};
use macroquad::prelude::*;

pub fn title(state: &AppState) -> String {
    if matches!(
        state.pending_restart.as_ref(),
        Some(UiAction::CycleGameVariant)
    ) {
        return "Change the rule card?".into();
    }
    match state.screen {
        Screen::Game(game) => format!("Start a new {}?", state.game_title(game)),
        _ => "Start a new game?".into(),
    }
}

pub fn detail(state: &AppState) -> &'static str {
    if matches!(
        state.pending_restart.as_ref(),
        Some(UiAction::CycleGameVariant)
    ) {
        "Current progress will be replaced by the next rule."
    } else {
        "Current progress will be replaced."
    }
}

pub fn clicks(p: Vec2) -> Vec<UiAction> {
    let layout = current_layout();
    if crate::ui::hit(layout.cancel, p) {
        vec![UiAction::Cancel]
    } else if crate::ui::hit(layout.start, p) {
        vec![UiAction::ConfirmRestart]
    } else {
        vec![]
    }
}

pub fn draw(state: &AppState) {
    let title = title(state);
    let layout = current_layout();
    let panel_rect = layout.panel;
    let cancel = layout.cancel;
    let start = layout.start;
    draw_rectangle(
        panel_rect.x,
        panel_rect.y,
        panel_rect.w,
        panel_rect.h,
        Color::new(0.16, 0.09, 0.20, 0.98),
    );
    draw_rectangle_lines(
        panel_rect.x,
        panel_rect.y,
        panel_rect.w,
        panel_rect.h,
        2.,
        if state.high_contrast {
            WHITE
        } else {
            Color::new(0.98, 0.83, 0.45, 1.)
        },
    );
    crate::ui::draw_text(
        &title,
        layout.title_position.x,
        layout.title_position.y,
        crate::accessibility::text_size(layout.title_size, state.large_text),
        WHITE,
    );
    crate::ui::draw_text(
        detail(state),
        layout.detail_position.x,
        layout.detail_position.y,
        crate::accessibility::text_size(layout.detail_size, state.large_text),
        if state.high_contrast {
            WHITE
        } else {
            Color::new(0.72, 0.68, 0.82, 1.)
        },
    );
    let button_size = crate::accessibility::text_size(14., state.large_text);
    let fills = if state.high_contrast {
        (crate::theme::SURFACE_DARK, crate::theme::MOSS_DARK)
    } else {
        (
            Color::new(0.25, 0.16, 0.32, 1.),
            Color::new(0.45, 0.22, 0.25, 1.),
        )
    };
    for (rect, label, fill) in [(cancel, "CANCEL", fills.0), (start, "START", fills.1)] {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., WHITE);
        let measured = crate::ui::measure_text(label, None, button_size.round() as u16, 1.);
        crate::ui::draw_text(
            label,
            rect.x + (rect.w - measured.width) * 0.5,
            rect.y + rect.h * 0.65,
            button_size,
            WHITE,
        );
    }
}

pub fn current_layout() -> crate::restart_modal_data::Layout {
    crate::restart_modal_data::layout(crate::ui::is_portrait(), crate::ui::is_compact_landscape())
}
