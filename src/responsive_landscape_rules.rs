//! Compact-landscape Rules shelf.

use crate::{
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

use crate::responsive_landscape_library::{back, panel, scroll, text};

#[cfg(test)]
mod tests;

const RULES_VISIBLE_ROWS: usize = 8;

pub fn draw_rules(state: &AppState) {
    panel(
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("RULES", 40., 48., 25., crate::theme::BRASS);
    text(
        "Tap a drawer to open its game.",
        40.,
        62.,
        9.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(650., 2., 150., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "SHELF {}",
            crate::rules_data::filter_label(state.rules_filter)
        ),
        675.,
        30.,
        9.,
        WHITE,
    );
    text(
        &crate::rules_data::summary_label(state.rules_filter),
        675.,
        42.,
        7.,
        crate::theme::SECONDARY,
    );
    if state.rules_filter != 0 {
        draw_filtered_rules(state);
        return;
    }
    let start = state
        .library_scroll
        .min(GameId::ALL.len().saturating_sub(RULES_VISIBLE_ROWS));
    for (index, game) in GameId::ALL
        .iter()
        .skip(start)
        .take(RULES_VISIBLE_ROWS)
        .enumerate()
    {
        draw_rule_card(index, *game, game.title(), game.subtitle());
    }
    draw_controls();
}

fn draw_filtered_rules(state: &AppState) {
    let rows = crate::rules_data::rows(state.rules_filter);
    let start = state
        .library_scroll
        .min(rows.len().saturating_sub(RULES_VISIBLE_ROWS));
    for (index, row) in rows.iter().skip(start).take(RULES_VISIBLE_ROWS).enumerate() {
        draw_rule_card(index, row.game, row.title, row.subtitle);
    }
    text(
        &crate::rules_data::page_label(start, rows.len(), RULES_VISIBLE_ROWS),
        650.,
        320.,
        8.,
        crate::theme::SECONDARY,
    );
    draw_controls();
}

fn draw_rule_card(index: usize, game: GameId, title: &str, subtitle: &str) {
    let rect = Rect::new(
        30. + (index % 2) as f32 * 380.,
        68. + (index / 2) as f32 * 62.,
        360.,
        56.,
    );
    panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
    text(title, rect.x + 12., rect.y + 23., 14., crate::theme::BRASS);
    text(
        subtitle,
        rect.x + 12.,
        rect.y + 44.,
        11.,
        crate::theme::CREAM,
    );
    text(
        crate::rules_data::action_label(game),
        rect.right() - 48.,
        rect.y + 20.,
        9.,
        crate::theme::SECONDARY,
    );
}

fn draw_controls() {
    scroll(Rect::new(430., 330., 100., 44.), "PREV");
    scroll(Rect::new(545., 330., 100., 44.), "NEXT");
    back(Rect::new(700., 330., 110., 44.));
}

pub fn rules_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(650., 2., 150., 44.), p) {
        vec![UiAction::RulesFilter(crate::rules_data::next_filter(
            state.rules_filter,
        ))]
    } else if crate::ui::hit(Rect::new(430., 330., 100., 44.), p) {
        vec![UiAction::LibraryScroll(-1)]
    } else if crate::ui::hit(Rect::new(545., 330., 100., 44.), p) {
        vec![UiAction::LibraryScroll(1)]
    } else if crate::ui::hit(Rect::new(700., 330., 110., 44.), p) {
        vec![UiAction::Cabinet]
    } else if let Some(action) = rule_action(state, p) {
        vec![action]
    } else {
        vec![]
    }
}

fn rule_action(state: &AppState, point: Vec2) -> Option<UiAction> {
    let rows = crate::rules_data::rows(state.rules_filter);
    let start = state.library_scroll.min(rows.len().saturating_sub(8));
    crate::rules_data::page_rows(state.rules_filter, start, 8)
        .into_iter()
        .enumerate()
        .find_map(|(index, row)| {
            let rect = Rect::new(
                30. + (index % 2) as f32 * 380.,
                68. + (index / 2) as f32 * 62.,
                360.,
                56.,
            );
            rect.contains(point)
                .then(|| UiAction::Open(row.game.index()))
        })
}
