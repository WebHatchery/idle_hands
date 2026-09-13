use super::{back_button, panel, scroll_button, text, RULES_VISIBLE_ROWS};
use crate::{
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub fn draw_rules(state: &AppState) {
    panel(
        Rect::new(8., 20., 344., 680.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("RULES", 20., 62., 29., crate::theme::BRASS);
    text(
        "Tap a drawer to open it, or INFO to inspect first.",
        20.,
        88.,
        12.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(190., 28., 155., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "SHELF {}",
            crate::rules_data::filter_label(state.rules_filter)
        ),
        214.,
        56.,
        10.,
        WHITE,
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
        let rect = Rect::new(18., 108. + index as f32 * 60., 324., 54.);
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(
            state.game_title(*game),
            rect.x + 10.,
            rect.y + 22.,
            14.,
            crate::theme::BRASS,
        );
        text(
            state.game_subtitle(*game),
            rect.x + 10.,
            rect.y + 43.,
            11.,
            crate::theme::CREAM,
        );
        text(
            crate::rules_data::action_label(*game),
            rect.right() - 92.,
            rect.y + 20.,
            9.,
            crate::theme::SECONDARY,
        );
        draw_rule_info(rect);
    }
    scroll_button(Rect::new(10., 602., 100., 44.), "PREV");
    scroll_button(Rect::new(250., 602., 100., 44.), "NEXT");
    text(
        &format!(
            "{}-{} OF {}",
            start + 1,
            (start + RULES_VISIBLE_ROWS).min(GameId::ALL.len()),
            GameId::ALL.len()
        ),
        128.,
        630.,
        11.,
        crate::theme::CREAM,
    );
    back_button(714.);
}

fn draw_filtered_rules(state: &AppState) {
    let rows = crate::rules_data::rows(state.rules_filter);
    let start = state
        .library_scroll
        .min(rows.len().saturating_sub(RULES_VISIBLE_ROWS));
    for (index, row) in rows.iter().skip(start).take(RULES_VISIBLE_ROWS).enumerate() {
        let rect = Rect::new(18., 108. + index as f32 * 60., 324., 54.);
        panel(rect, Color::new(0.13, 0.09, 0.20, 1.));
        text(
            row.title,
            rect.x + 10.,
            rect.y + 22.,
            14.,
            crate::theme::BRASS,
        );
        text(
            row.subtitle,
            rect.x + 10.,
            rect.y + 43.,
            11.,
            crate::theme::CREAM,
        );
        text(
            crate::rules_data::action_label(row.game),
            rect.right() - 92.,
            rect.y + 20.,
            9.,
            crate::theme::SECONDARY,
        );
        draw_rule_info(rect);
    }
    scroll_button(Rect::new(10., 602., 100., 44.), "PREV");
    scroll_button(Rect::new(250., 602., 100., 44.), "NEXT");
    text(
        &format!(
            "{}  ·  {}-{} OF {}",
            crate::rules_data::page_label(start, rows.len(), RULES_VISIBLE_ROWS),
            start + 1,
            (start + RULES_VISIBLE_ROWS).min(rows.len()),
            rows.len()
        ),
        128.,
        630.,
        11.,
        crate::theme::CREAM,
    );
    back_button(714.);
}

pub fn rules_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(190., 28., 155., 44.), p) {
        vec![UiAction::RulesFilter(crate::rules_data::next_filter(
            state.rules_filter,
        ))]
    } else if crate::ui::hit(Rect::new(10., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(-1)]
    } else if crate::ui::hit(Rect::new(250., 602., 100., 44.), p) {
        vec![UiAction::LibraryScroll(1)]
    } else if crate::ui::hit(Rect::new(10., 714., 150., 44.), p) {
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
            let rect = Rect::new(18., 108. + index as f32 * 60., 324., 54.);
            if crate::ui::hit(rule_info_rect(rect), point) {
                Some(UiAction::Inspect(row.game.index()))
            } else {
                rect.contains(point)
                    .then(|| UiAction::Open(row.game.index()))
            }
        })
}

pub(super) fn rule_info_rect(row: Rect) -> Rect {
    Rect::new(row.right() - 42., row.y + 4., 36., row.h - 8.)
}

fn draw_rule_info(row: Rect) {
    let info = rule_info_rect(row);
    panel(info, crate::theme::SURFACE_DARK);
    text("INFO", info.x + 5., info.y + info.h * 0.66, 8., WHITE);
}
