//! Static collection Rules and Credits screens.

use crate::{
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

fn panel(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::BACKGROUND_DEEP,
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn back_button() {
    draw_rectangle(1030., 635., 180., 48., crate::theme::MOSS_DARK);
    crate::ui::draw_text("BACK", 1090., 666., 18., WHITE);
}
pub fn rules_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(900., 102., 210., 44.).contains(p) {
        vec![UiAction::RulesFilter(crate::rules_data::next_filter(
            state.rules_filter,
        ))]
    } else if Rect::new(1030., 635., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else if let Some(action) = rule_action(state, p) {
        vec![action]
    } else {
        vec![]
    }
}

fn rule_action(state: &AppState, point: Vec2) -> Option<UiAction> {
    let rows = crate::rules_data::rows(state.rules_filter);
    let visible = if state.rules_filter == 0 {
        rows
    } else {
        let start = state.library_scroll.min(rows.len().saturating_sub(44));
        crate::rules_data::page_rows(state.rules_filter, start, 44)
    };
    visible.into_iter().enumerate().find_map(|(index, row)| {
        let column = index / 11;
        let line = index % 11;
        let rect = Rect::new(
            152. + column as f32 * 245.,
            193. + line as f32 * 38.,
            230.,
            32.,
        );
        rect.contains(point)
            .then(|| UiAction::Open(row.game.index()))
    })
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(1030., 635., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
pub fn draw_rules(state: &AppState) {
    panel(Rect::new(120., 55., 1040., 610.));
    crate::ui::draw_text("RULES", 170., 125., 46., crate::theme::BRASS);
    crate::ui::draw_text(
        "Tap a drawer name to open its game.",
        174.,
        157.,
        19.,
        crate::theme::SECONDARY,
    );
    draw_rectangle(900., 102., 210., 44., crate::theme::SURFACE);
    draw_rectangle_lines(900., 102., 210., 44., 3., WHITE);
    crate::ui::draw_text(
        format!(
            "SHELF: {}",
            crate::rules_data::filter_label(state.rules_filter)
        ),
        925.,
        129.,
        14.,
        WHITE,
    );
    crate::ui::draw_text(
        crate::rules_data::summary_label(state.rules_filter),
        925.,
        143.,
        10.,
        crate::theme::SECONDARY,
    );
    if state.rules_filter != 0 {
        draw_filtered_rules(state);
        return;
    }
    for (index, game) in GameId::ALL.iter().enumerate() {
        let column = index / 11;
        let row = index % 11;
        let x = 160. + column as f32 * 245.;
        let y = 215. + row as f32 * 38.;
        crate::ui::draw_text(game.title(), x, y, 12., crate::theme::BRASS);
        crate::ui::draw_text(game.subtitle(), x, y + 15., 9., crate::theme::CREAM);
        crate::ui::draw_text("OPEN", x + 195., y + 15., 8., crate::theme::SECONDARY);
    }
    back_button();
}

fn draw_filtered_rules(state: &AppState) {
    let rows = crate::rules_data::rows(state.rules_filter);
    let start = state.library_scroll.min(rows.len().saturating_sub(44));
    for (index, row) in rows.iter().skip(start).take(44).enumerate() {
        let column = index / 11;
        let line = index % 11;
        let x = 160. + column as f32 * 245.;
        let y = 215. + line as f32 * 38.;
        crate::ui::draw_text(row.title, x, y, 12., crate::theme::BRASS);
        crate::ui::draw_text(row.subtitle, x, y + 15., 9., crate::theme::CREAM);
        crate::ui::draw_text("OPEN", x + 195., y + 15., 8., crate::theme::SECONDARY);
    }
    crate::ui::draw_text(
        crate::rules_data::page_label(start, rows.len(), 44),
        930.,
        620.,
        12.,
        crate::theme::SECONDARY,
    );
    back_button();
}
pub fn draw_credits() {
    panel(Rect::new(230., 95., 820., 520.));
    crate::ui::draw_text("CREDITS", 300., 175., 46., crate::theme::BRASS);
    crate::ui::draw_text("IDLE HANDS", 305., 240., 28., WHITE);
    crate::ui::draw_text(
        "A warm collection for small pauses.",
        305.,
        280.,
        20.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "Built with Rust, macroquad, and the shared macroquad-toolkit.",
        305.,
        335.,
        18.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "All games are deterministic where practical and designed for touch.",
        305.,
        375.,
        18.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "Original generated artwork created for Idle Hands; provenance ships with the game.",
        305.,
        415.,
        16.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "PRIVACY: Gameplay analytics are currently disabled.",
        305.,
        470.,
        14.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "Your game progress stays in this browser or Windows profile.",
        305.,
        495.,
        14.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "Thank you for spending a minute at the cabinet.",
        305.,
        540.,
        19.,
        crate::theme::BRASS,
    );
    back_button();
}
