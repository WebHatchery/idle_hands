//! Responsive quick-browse views for starred and recently opened drawers.

use crate::{
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    panel: Rect,
    back: Rect,
    columns: usize,
    card_w: f32,
    card_h: f32,
    origin: Vec2,
    gap_x: f32,
    gap_y: f32,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            panel: Rect::new(20., 12., 804., 365.),
            back: Rect::new(700., 330., 110., 38.),
            columns: 5,
            card_w: 145.,
            card_h: 24.,
            origin: vec2(35., 70.),
            gap_x: 155.,
            gap_y: 31.,
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(8., 38., 344., 602.),
            back: Rect::new(10., 650., 150., 38.),
            columns: 3,
            card_w: 106.,
            card_h: 30.,
            origin: vec2(14., 130.),
            gap_x: 112.,
            gap_y: 32.,
        }
    } else {
        Layout {
            panel: Rect::new(120., 55., 1040., 610.),
            back: Rect::new(930., 590., 180., 48.),
            columns: 5,
            card_w: 185.,
            card_h: 42.,
            origin: vec2(160., 190.),
            gap_x: 205.,
            gap_y: 47.,
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if l.back.contains(point) {
        return vec![UiAction::Cabinet];
    }
    for (slot, &game) in browse_games(state).iter().enumerate() {
        let rect = list_card_rect(l, slot);
        if rect.contains(point) {
            return vec![UiAction::Open(game.index())];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    panel(l.panel, Color::new(0.08, 0.06, 0.14, 1.));
    let title_size = if crate::ui::is_portrait() { 29. } else { 38. };
    let recent = state.recent_view;
    draw_text(
        if recent {
            "RECENT DRAWERS"
        } else {
            "FAVORITE DRAWERS"
        },
        l.panel.x + 50.,
        l.panel.y
            + if crate::ui::is_compact_landscape() {
                35.
            } else if crate::ui::is_portrait() {
                42.
            } else {
                70.
            },
        title_size,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    let games = browse_games(state);
    let count = games.len();
    let subtitle_y = if crate::ui::is_compact_landscape() {
        l.panel.y + 52.
    } else if crate::ui::is_portrait() {
        l.panel.y + 66.
    } else {
        l.panel.y + 98.
    };
    draw_text(
        if recent {
            format!(
                "{} recently opened games  •  tap a drawer to open it",
                count
            )
        } else {
            format!("{} starred games  •  tap a drawer to open it", count)
        },
        l.panel.x + 52.,
        subtitle_y,
        if crate::ui::is_portrait() { 11. } else { 16. },
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    if count == 0 {
        draw_text(
            if recent {
                "Open a drawer to start a recent list."
            } else {
                "No favorites yet — use the markers on the cabinet."
            },
            l.panel.x + 52.,
            subtitle_y + 42.,
            14.,
            WHITE,
        );
    }
    for (slot, &game) in games.iter().enumerate() {
        let rect = list_card_rect(l, slot);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        draw_circle(
            rect.x + 10.,
            rect.y + rect.h * 0.5,
            if crate::ui::is_portrait() { 3. } else { 4. },
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        draw_text(
            game.title(),
            rect.x + 20.,
            rect.y + rect.h * 0.62,
            if crate::ui::is_portrait() {
                8.
            } else if crate::ui::is_compact_landscape() {
                10.
            } else {
                14.
            },
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        if !crate::ui::is_portrait() {
            draw_text(
                crate::cabinet_status::status(state, game),
                rect.x + rect.w - 72.,
                rect.y + rect.h * 0.62,
                8.,
                crate::cabinet_status::color(crate::cabinet_status::status(state, game)),
            );
        }
    }
    panel(l.back, Color::new(0.25, 0.16, 0.32, 1.));
    draw_text(
        "BACK",
        l.back.x + if crate::ui::is_portrait() { 52. } else { 60. },
        l.back.y + l.back.h * 0.64,
        if crate::ui::is_portrait() { 12. } else { 18. },
        WHITE,
    );
}

fn browse_games(state: &AppState) -> Vec<GameId> {
    if state.recent_view {
        state.recent_games.clone()
    } else {
        GameId::ALL
            .iter()
            .copied()
            .filter(|game| state.favorites.get(game.index()).copied().unwrap_or(false))
            .collect()
    }
}

fn list_card_rect(layout: Layout, slot: usize) -> Rect {
    Rect::new(
        layout.origin.x + (slot % layout.columns) as f32 * layout.gap_x,
        layout.origin.y + (slot / layout.columns) as f32 * layout.gap_y,
        layout.card_w,
        layout.card_h,
    )
}

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
