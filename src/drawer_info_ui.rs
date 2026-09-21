//! Responsive pre-launch details for a cabinet drawer.

use crate::{
    state::{AppState, GameId, Screen},
    ui::UiAction,
};
use macroquad::prelude::*;

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let Screen::DrawerInfo(game) = state.screen else {
        return Vec::new();
    };
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(open_rect(), point) {
        return vec![UiAction::Open(game.index())];
    }
    if crate::cabinet_status::is_available(game) && crate::ui::hit(favorite_rect(), point) {
        return vec![UiAction::ToggleFavorite(game.index())];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let Screen::DrawerInfo(game) = state.screen else {
        return;
    };
    if crate::ui::is_compact_landscape() {
        draw_compact(state, game);
    } else if crate::ui::is_portrait() {
        draw_portrait(state, game);
    } else {
        draw_desktop(state, game);
    }
}

pub fn draw_desktop(state: &AppState, game: GameId) {
    panel(state, Rect::new(160., 35., 960., 650.));
    text(state, "DRAWER INFO", 220., 94., 34., accent(state));
    draw_game_card(state, game, Rect::new(220., 145., 260., 300.), 30.);
    draw_details(state, game, 540., 160., 19.);
    draw_actions(state);
}

pub fn draw_compact(state: &AppState, game: GameId) {
    panel(state, Rect::new(20., 12., 804., 365.));
    text(state, "DRAWER INFO", 40., 48., 25., accent(state));
    draw_game_card(state, game, Rect::new(40., 85., 205., 175.), 22.);
    draw_details(state, game, 275., 100., 12.);
    draw_actions(state);
}

pub fn draw_portrait(state: &AppState, game: GameId) {
    panel(state, Rect::new(8., 16., 344., 688.));
    text(state, "DRAWER INFO", 20., 60., 27., accent(state));
    draw_game_card(state, game, Rect::new(20., 82., 320., 120.), 24.);
    draw_details(state, game, 20., 235., 12.);
    draw_actions(state);
}

pub fn draw_game_card(state: &AppState, game: GameId, rect: Rect, initial_size: f32) {
    panel_fill(state, rect, crate::theme::category_surface(game, true));
    draw_circle(
        rect.x + 48.,
        rect.y + rect.h * 0.5,
        rect.h.min(92.) * 0.28,
        crate::theme::PAPER_LIGHT,
    );
    text(
        state,
        &state.game_title(game)[..state.game_title(game).len().min(1)],
        rect.x + 37.,
        rect.y + rect.h * 0.5 + initial_size * 0.35,
        initial_size,
        crate::theme::INK,
    );
    text(
        state,
        crate::stats_data::short_title_from(
            state,
            game,
            if crate::ui::is_portrait() { 22 } else { 18 },
        ),
        rect.x + 86.,
        rect.y + rect.h * 0.5 + 6.,
        if crate::ui::is_portrait() { 18. } else { 22. },
        crate::theme::CREAM,
    );
}

pub fn draw_details(state: &AppState, game: GameId, x: f32, y: f32, size: f32) {
    let availability = crate::storefront::availability(game);
    let status = if availability.is_playable() {
        crate::cabinet_status::status(state, game)
    } else {
        availability.cabinet_label(crate::ui::is_portrait())
    };
    text(
        state,
        state.game_subtitle(game),
        x,
        y,
        size,
        crate::theme::CREAM,
    );
    text(
        state,
        format!(
            "CATEGORY  {}",
            game.descriptor().category.label().to_uppercase()
        ),
        x,
        y + size * 1.8,
        size * 0.72,
        secondary(state),
    );
    text(
        state,
        format!("STATUS  {status}"),
        x,
        y + size * 3.0,
        size * 0.72,
        accent(state),
    );
    text(
        state,
        if game.descriptor().has_variants {
            "RULE CARD  AVAILABLE"
        } else {
            "RULE CARD  CLASSIC"
        },
        x,
        y + size * 4.2,
        size * 0.72,
        secondary(state),
    );
    text(
        state,
        if crate::cabinet_status::is_available(game) {
            if state.favorites.get(game.index()).copied().unwrap_or(false) {
                "FAVORITED"
            } else {
                "NOT FAVORITED"
            }
        } else {
            "FAVORITES AVAILABLE WHEN PLAYABLE"
        },
        x,
        y + size * 5.4,
        size * 0.72,
        secondary(state),
    );
}

pub fn draw_actions(state: &AppState) {
    let game = match state.screen {
        Screen::DrawerInfo(game) => game,
        _ => return,
    };
    button(
        state,
        open_rect(),
        crate::storefront::action_label(game),
        crate::theme::MOSS_DARK,
    );
    if crate::cabinet_status::is_available(game) {
        let label = if state.favorites.get(game.index()).copied().unwrap_or(false) {
            "UNFAVORITE"
        } else {
            "FAVORITE"
        };
        button(state, favorite_rect(), label, crate::theme::SURFACE_DARK);
    }
    button(state, back_rect(), "BACK", crate::theme::SURFACE_DARK);
}

pub fn button(state: &AppState, rect: Rect, label: &str, fill: Color) {
    panel_fill(state, rect, fill);
    let width = crate::ui::measure_text(label, None, 12, 1.).width;
    text(
        state,
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        12.,
        WHITE,
    );
}

pub fn panel(state: &AppState, rect: Rect) {
    panel_fill(state, rect, crate::theme::BACKGROUND_DEEP);
}

pub fn panel_fill(state: &AppState, rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
}

pub fn text(state: &AppState, value: impl AsRef<str>, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(
        value,
        x,
        y,
        crate::accessibility::text_size(size, state.large_text),
        color,
    );
}

pub fn open_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(10., 602., 100., 44.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(40., 315., 120., 44.)
    } else {
        Rect::new(540., 500., 170., 50.)
    }
}

pub fn favorite_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(122., 602., 108., 44.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(175., 315., 135., 44.)
    } else {
        Rect::new(730., 500., 170., 50.)
    }
}

pub fn back_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(242., 602., 100., 44.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(690., 315., 110., 44.)
    } else {
        Rect::new(930., 500., 170., 50.)
    }
}

pub fn accent(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::BRASS
    }
}

pub fn secondary(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::SECONDARY
    }
}
