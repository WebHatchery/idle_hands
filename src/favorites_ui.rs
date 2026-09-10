//! Responsive quick-browse views for starred and recently opened drawers.

use crate::{
    favorites_data::{self, BrowseMode, BrowseRow},
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const DESKTOP_VISIBLE_GAMES: usize = 40;

#[cfg(test)]
mod tests;

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
            back: Rect::new(700., 330., 110., 44.),
            columns: 2,
            card_w: 360.,
            card_h: 44.,
            origin: vec2(30., 70.),
            gap_x: 380.,
            gap_y: 50.,
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(8., 38., 344., 602.),
            back: Rect::new(10., 714., 150., 44.),
            columns: 1,
            card_w: 324.,
            card_h: 54.,
            origin: vec2(18., 155.),
            gap_x: 0.,
            gap_y: 60.,
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
    let mode = BrowseMode::from_state(state);
    if crate::ui::hit(l.back, point) {
        return vec![UiAction::Cabinet];
    }
    let (favorites, recent) = browse_tab_rects();
    if crate::ui::hit(favorites, point) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(recent, point) {
        return vec![UiAction::Recent];
    }
    if mode.is_recent()
        && !state.recent_games.is_empty()
        && crate::ui::hit(quick_action_rect(), point)
    {
        return vec![UiAction::ClearRecent];
    }
    if let Some((previous, next)) = scroll_rects(state) {
        if previous.contains(point) {
            return vec![UiAction::LibraryScroll(-1)];
        }
        if next.contains(point) {
            return vec![UiAction::LibraryScroll(1)];
        }
    }
    for (slot, row) in visible_rows(state).iter().enumerate() {
        let game = row.game;
        let rect = list_card_rect(l, slot);
        if !mode.is_recent() && crate::ui::hit(favorite_remove_rect(rect), point) {
            return vec![UiAction::ToggleFavorite(game.index())];
        }
        if rect.contains(point) {
            return vec![UiAction::Open(game.index())];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    panel(l.panel, crate::theme::BACKGROUND_DEEP);
    let title_size = if crate::ui::is_portrait() { 29. } else { 38. };
    let mode = BrowseMode::from_state(state);
    let recent = mode.is_recent();
    let (favorites_tab, recent_tab) = browse_tab_rects();
    for (rect, label, active) in [
        (favorites_tab, "FAVORITES", !recent),
        (recent_tab, "RECENT", recent),
    ] {
        panel(
            rect,
            if active {
                crate::theme::MOSS_DARK
            } else {
                crate::theme::SURFACE_DARK
            },
        );
        crate::ui::draw_text(
            label,
            rect.x + if crate::ui::is_portrait() { 12. } else { 18. },
            rect.y + rect.h * 0.66,
            if crate::ui::is_portrait() { 9. } else { 11. },
            WHITE,
        );
    }
    crate::ui::draw_text(
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
        crate::theme::BRASS,
    );
    let summary = favorites_data::summary(state, mode);
    let count = summary.total;
    let subtitle_y = if crate::ui::is_compact_landscape() {
        l.panel.y + 52.
    } else if crate::ui::is_portrait() {
        l.panel.y + 96.
    } else {
        l.panel.y + 98.
    };
    crate::ui::draw_text(
        if recent {
            format!(
                "{} recently opened  ·  {} open  ·  {} done  ·  {} locked",
                count, summary.open, summary.done, summary.locked
            )
        } else {
            format!(
                "{} starred  ·  {} open  ·  {} done  ·  {} locked",
                count, summary.open, summary.done, summary.locked
            )
        },
        l.panel.x + 52.,
        subtitle_y,
        if crate::ui::is_portrait() { 11. } else { 16. },
        crate::theme::SECONDARY,
    );
    if recent && count > 0 {
        let action = quick_action_rect();
        panel(action, crate::theme::SURFACE_DARK);
        crate::ui::draw_text(
            "CLEAR RECENT",
            action.x + if crate::ui::is_portrait() { 34. } else { 42. },
            action.y + action.h * 0.64,
            if crate::ui::is_portrait() { 11. } else { 13. },
            WHITE,
        );
    }
    if count == 0 {
        crate::ui::draw_text(
            if recent {
                "Open a drawer to start a recent list."
            } else {
                "No favorites yet - use the markers on the cabinet."
            },
            l.panel.x + 52.,
            subtitle_y + 42.,
            14.,
            WHITE,
        );
    }
    for (slot, row) in visible_rows(state).iter().enumerate() {
        let game = row.game;
        let rect = list_card_rect(l, slot);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        draw_circle(
            rect.x + 10.,
            rect.y + rect.h * 0.5,
            if crate::ui::is_portrait() { 3. } else { 4. },
            crate::theme::BRASS,
        );
        crate::ui::draw_text(
            game.title(),
            rect.x + 20.,
            rect.y + rect.h * 0.62,
            if crate::ui::is_portrait() {
                14.
            } else if crate::ui::is_compact_landscape() {
                13.
            } else {
                14.
            },
            crate::theme::BRASS,
        );
        if !crate::ui::is_portrait() {
            let status = crate::cabinet_status::availability_label(state, game);
            crate::ui::draw_text(
                browse_status_label(state, game, status),
                rect.x + rect.w - 72.,
                rect.y + rect.h * 0.62,
                8.,
                crate::cabinet_status::color(status),
            );
        } else {
            let status = crate::cabinet_status::availability_label(state, game);
            crate::ui::draw_text(
                short_status(status),
                rect.right() - if recent { 62. } else { 94. },
                rect.y + 18.,
                8.,
                crate::cabinet_status::color(status),
            );
        }
        if !recent {
            let remove = favorite_remove_rect(rect);
            panel(remove, crate::theme::SURFACE_DARK);
            crate::ui::draw_text(
                "−",
                remove.x + remove.w * 0.38,
                remove.y + remove.h * 0.68,
                if crate::ui::is_portrait() { 16. } else { 13. },
                WHITE,
            );
        }
    }
    if let Some((previous, next)) = scroll_rects(state) {
        panel(previous, crate::theme::SURFACE_DARK);
        panel(next, crate::theme::SURFACE_DARK);
        crate::ui::draw_text("PREV", previous.x + 22., previous.y + 28., 11., WHITE);
        crate::ui::draw_text("NEXT", next.x + 22., next.y + 28., 11., WHITE);
    }
    panel(l.back, crate::theme::MOSS_DARK);
    crate::ui::draw_text(
        "BACK",
        l.back.x + if crate::ui::is_portrait() { 52. } else { 60. },
        l.back.y + l.back.h * 0.64,
        if crate::ui::is_portrait() { 12. } else { 18. },
        WHITE,
    );
}

fn visible_rows(state: &AppState) -> Vec<BrowseRow> {
    favorites_data::page_rows(
        state,
        BrowseMode::from_state(state),
        state.library_scroll,
        visible_capacity(),
    )
}

fn visible_capacity() -> usize {
    if crate::ui::is_portrait() {
        8
    } else if crate::ui::is_compact_landscape() {
        10
    } else {
        DESKTOP_VISIBLE_GAMES
    }
}

fn scroll_rects(state: &AppState) -> Option<(Rect, Rect)> {
    let total = favorites_data::games(state, BrowseMode::from_state(state)).len();
    if total <= visible_capacity() {
        return None;
    }
    Some(if crate::ui::is_portrait() {
        (
            Rect::new(10., 650., 100., 44.),
            Rect::new(250., 650., 100., 44.),
        )
    } else if crate::ui::is_compact_landscape() {
        (
            Rect::new(430., 330., 100., 44.),
            Rect::new(545., 330., 100., 44.),
        )
    } else {
        (
            Rect::new(700., 590., 100., 44.),
            Rect::new(815., 590., 100., 44.),
        )
    })
}

fn quick_action_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(180., 714., 170., 44.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(540., 20., 250., 36.)
    } else {
        Rect::new(700., 102., 180., 44.)
    }
}

fn browse_tab_rects() -> (Rect, Rect) {
    if crate::ui::is_portrait() {
        (
            Rect::new(10., 96., 82., 28.),
            Rect::new(100., 96., 82., 28.),
        )
    } else if crate::ui::is_compact_landscape() {
        (
            Rect::new(300., 20., 105., 36.),
            Rect::new(415., 20., 105., 36.),
        )
    } else {
        (
            Rect::new(880., 70., 100., 38.),
            Rect::new(990., 70., 100., 38.),
        )
    }
}

fn favorite_remove_rect(card: Rect) -> Rect {
    Rect::new(card.right() - 44., card.y + 4., 38., card.h - 8.)
}

fn short_status(status: &str) -> &'static str {
    match status {
        "COMPLETE" => "DONE",
        "FULL VERSION" => "FULL",
        _ => "OPEN",
    }
}

fn browse_status_label(state: &AppState, game: GameId, status: &str) -> String {
    if status == "COMPLETE" {
        if let Some(seconds) = state.records.best_time(game.index()) {
            return format!(
                "DONE {}",
                crate::state_records::format_duration(u64::from(seconds))
            );
        }
    }
    status.to_owned()
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
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
