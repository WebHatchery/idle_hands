//! Portrait collection dashboard matching the category-first mobile cabinet.

use crate::{
    cabinet_status,
    data::GameData,
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const CONTINUE: Rect = Rect::new(10., 58., 340., 104.);
const CATEGORY_RECTS: [Rect; 6] = [
    Rect::new(10., 252., 340., 62.),
    Rect::new(10., 320., 340., 62.),
    Rect::new(10., 388., 340., 62.),
    Rect::new(10., 456., 340., 62.),
    Rect::new(10., 524., 340., 62.),
    Rect::new(10., 592., 340., 62.),
];
const PAGE_SIZE: usize = 12;

pub fn draw(
    state: &AppState,
    _data: &GameData,
    loaded: usize,
    cabinet_texture: Option<&Texture2D>,
) {
    draw_rectangle(3., 3., 354., 774., crate::theme::BACKGROUND_DEEP);
    draw_rectangle_lines(3., 3., 354., 774., 2., crate::theme::BORDER);
    if let Some(texture) = cabinet_texture {
        draw_texture_ex(
            texture,
            3.,
            3.,
            Color::new(1., 1., 1., 0.16),
            DrawTextureParams {
                dest_size: Some(vec2(354., 774.)),
                ..Default::default()
            },
        );
    }
    if state.cabinet_filter == 0 {
        draw_home(state, loaded);
    } else {
        draw_library(state);
    }
    draw_bottom_nav(state);
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    for (index, rect) in bottom_rects().iter().copied().enumerate() {
        if crate::ui::hit(rect, p) {
            return match index {
                0 => vec![UiAction::CabinetFilter(0)],
                1 => vec![UiAction::CabinetFilter(9)],
                2 => vec![UiAction::Favorites],
                _ => vec![UiAction::Records],
            };
        }
    }
    if crate::ui::hit(Rect::new(306., 10., 42., 40.), p) {
        return vec![UiAction::Settings];
    }
    if state.cabinet_filter == 0 {
        home_clicks(state, p)
    } else {
        library_clicks(state, p)
    }
}

fn draw_home(state: &AppState, loaded: usize) {
    text("IDLE HANDS", 77., 35., 23., crate::theme::CREAM);
    if crate::game_descriptor::is_demo_build() {
        text("DEMO · 30 GAMES", 236., 35., 8., crate::theme::BRASS);
    }
    text(
        "quiet games for idle hands",
        96.,
        51.,
        8.,
        crate::theme::SECONDARY,
    );
    button(
        Rect::new(306., 10., 42., 40.),
        "SET",
        crate::theme::SURFACE_DARK,
    );
    panel(CONTINUE, crate::theme::MOSS_DARK);
    text("CONTINUE PLAYING", 24., 77., 9., crate::theme::BRASS);
    let selected = GameId::ALL[state.selected.min(GameId::ALL.len() - 1)];
    draw_circle(55., 117., 27., crate::theme::PAPER_LIGHT);
    text(
        &selected.title()[..selected.title().len().min(1)],
        48.,
        124.,
        19.,
        crate::theme::INK,
    );
    text(selected.title(), 94., 112., 20., crate::theme::CREAM);
    text(selected.subtitle(), 94., 133., 10., crate::theme::SECONDARY);
    panel(Rect::new(230., 116., 104., 34.), crate::theme::MOSS);
    text("CONTINUE", 250., 138., 10., crate::theme::CREAM);
    stat(
        Rect::new(10., 172., 105., 68.),
        "FAVORITES",
        favorite_count(state),
    );
    stat(
        Rect::new(122., 172., 105., 68.),
        "RECENT",
        state.recent_games.len(),
    );
    panel(Rect::new(234., 172., 116., 68.), crate::theme::SURFACE_DARK);
    text(
        &crate::daily_challenge::label(
            state.games.daily_dungeon.day_key,
            state.games.daily_dungeon.challenge,
        ),
        246.,
        194.,
        9.,
        crate::theme::BRASS,
    );
    text(
        if state.games.daily_dungeon.won() {
            "REPLAY  >"
        } else {
            "CHALLENGE  >"
        },
        246.,
        220.,
        10.,
        crate::theme::CREAM,
    );
    for (index, filter) in cabinet_status::CATEGORY_FILTERS.iter().copied().enumerate() {
        category_row(state, CATEGORY_RECTS[index], filter);
    }
    text(
        &format!("{} stamps  ·  {} textures", state.stamps, loaded),
        12.,
        680.,
        9.,
        crate::theme::SECONDARY,
    );
}

fn draw_library(state: &AppState) {
    let (playable, full) = cabinet_status::availability_counts(state, state.cabinet_filter);
    text("<  HOME", 12., 31., 11., crate::theme::BRASS);
    text(
        cabinet_status::category_name(state.cabinet_filter),
        12.,
        67.,
        27.,
        crate::theme::CREAM,
    );
    text(
        &if crate::game_descriptor::is_demo_build() {
            format!("{playable} playable · {full} full")
        } else {
            format!("{} quiet games", visible_games(state).len())
        },
        14.,
        86.,
        10.,
        crate::theme::SECONDARY,
    );
    button(
        Rect::new(306., 10., 42., 40.),
        "SET",
        crate::theme::SURFACE_DARK,
    );
    let games = page_games(state);
    for (index, game) in games.iter().copied().enumerate() {
        let rect = game_rect(index);
        panel(rect, crate::theme::category_surface(game, true));
        text(
            game.title(),
            rect.x + 9.,
            rect.y + 22.,
            if game.title().len() > 17 { 9. } else { 12. },
            crate::theme::CREAM,
        );
        text(
            if cabinet_status::is_available(game) {
                game.subtitle()
            } else {
                crate::storefront::COMPACT_LOCKED_LABEL
            },
            rect.x + 9.,
            rect.y + 40.,
            8.,
            if cabinet_status::is_available(game) {
                crate::theme::SECONDARY
            } else {
                crate::theme::BRASS
            },
        );
        text(
            "PLAY  >",
            rect.x + 9.,
            rect.y + 60.,
            9.,
            crate::theme::BRASS,
        );
        if cabinet_status::is_available(game) {
            let fav = state.favorites.get(game.index()).copied().unwrap_or(false);
            text(
                if fav { "*" } else { "+" },
                rect.right() - 24.,
                rect.y + 29.,
                18.,
                crate::theme::BRASS,
            );
        }
    }
    let total = visible_games(state).len();
    let start = state.cabinet_scroll.min(total.saturating_sub(1));
    button(
        Rect::new(10., 632., 100., 46.),
        "< PREV",
        crate::theme::SURFACE_DARK,
    );
    button(
        Rect::new(250., 632., 100., 46.),
        "NEXT >",
        crate::theme::SURFACE_DARK,
    );
    text(
        &format!(
            "{}-{} OF {}",
            start + usize::from(total > 0),
            (start + PAGE_SIZE).min(total),
            total
        ),
        139.,
        660.,
        9.,
        crate::theme::SECONDARY,
    );
}

fn draw_bottom_nav(state: &AppState) {
    draw_line(3., 704., 357., 704., 2., crate::theme::BORDER);
    let labels = ["HOME", "ALL GAMES", "FAVORITES", "RECORDS"];
    for (index, rect) in bottom_rects().iter().copied().enumerate() {
        let active =
            (index == 0 && state.cabinet_filter == 0) || (index == 1 && state.cabinet_filter == 9);
        text(
            labels[index],
            rect.x + 10.,
            rect.y + 37.,
            if index == 1 { 8. } else { 9. },
            if active {
                crate::theme::BRASS
            } else {
                crate::theme::SECONDARY
            },
        );
        text(
            ["H", "#", "*", "!"][index],
            rect.x + 31.,
            rect.y + 18.,
            12.,
            if active {
                crate::theme::BRASS
            } else {
                crate::theme::SECONDARY
            },
        );
    }
}

fn category_row(state: &AppState, rect: Rect, filter: u8) {
    let sample = GameId::ALL
        .iter()
        .copied()
        .find(|game| cabinet_status::category_filter(*game) == filter)
        .unwrap_or(GameId::Solitaire);
    panel(rect, crate::theme::category_surface(sample, false));
    draw_circle(rect.x + 31., rect.y + 31., 20., crate::theme::PAPER_LIGHT);
    text(
        ["C", "L", "B", "W", "A", "M"][(filter - 3) as usize],
        rect.x + 24.,
        rect.y + 38.,
        16.,
        crate::theme::INK,
    );
    text(
        cabinet_status::category_name(filter),
        rect.x + 64.,
        rect.y + 27.,
        18.,
        crate::theme::CREAM,
    );
    text(
        &format!("{} games", cabinet_status::filter_count(state, filter)),
        rect.x + 64.,
        rect.y + 46.,
        10.,
        crate::theme::SECONDARY,
    );
    crate::mascots::draw_for_filter(filter, vec2(rect.x + 31., rect.y + 31.), 0.62);
    text(
        ">",
        rect.right() - 24.,
        rect.y + 39.,
        20.,
        crate::theme::CREAM,
    );
}

fn home_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(CONTINUE, p) {
        return vec![UiAction::ContinueGame];
    }
    if crate::ui::hit(Rect::new(10., 172., 105., 68.), p) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(Rect::new(122., 172., 105., 68.), p) {
        return vec![UiAction::Recent];
    }
    if crate::ui::hit(Rect::new(234., 172., 116., 68.), p) {
        return vec![UiAction::Open(GameId::DailyDungeon.index())];
    }
    for (index, rect) in CATEGORY_RECTS.iter().copied().enumerate() {
        if crate::ui::hit(rect, p) {
            return vec![UiAction::CabinetFilter(3 + index as u8)];
        }
    }
    let _ = state;
    vec![]
}

fn library_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(4., 4., 120., 84.), p) {
        return vec![UiAction::CabinetFilter(0)];
    }
    for (index, game) in page_games(state).iter().copied().enumerate() {
        let rect = game_rect(index);
        if cabinet_status::is_available(game)
            && crate::ui::hit(Rect::new(rect.right() - 44., rect.y, 44., rect.h), p)
        {
            return vec![UiAction::ToggleFavorite(game.index())];
        }
        if crate::ui::hit(rect, p) {
            return vec![UiAction::Open(game.index())];
        }
    }
    if crate::ui::hit(Rect::new(10., 632., 100., 46.), p) {
        return vec![UiAction::CabinetScroll(-(PAGE_SIZE as i8))];
    }
    if crate::ui::hit(Rect::new(250., 632., 100., 46.), p) {
        return vec![UiAction::CabinetScroll(PAGE_SIZE as i8)];
    }
    vec![]
}

fn visible_games(state: &AppState) -> Vec<GameId> {
    GameId::ALL
        .iter()
        .copied()
        .filter(|game| cabinet_status::matches_filter(state, *game, state.cabinet_filter))
        .collect()
}
fn page_games(state: &AppState) -> Vec<GameId> {
    let games = visible_games(state);
    let start = state.cabinet_scroll.min(games.len().saturating_sub(1));
    games.into_iter().skip(start).take(PAGE_SIZE).collect()
}
fn game_rect(index: usize) -> Rect {
    Rect::new(
        8. + (index % 2) as f32 * 174.,
        98. + (index / 2) as f32 * 86.,
        168.,
        78.,
    )
}
fn bottom_rects() -> [Rect; 4] {
    std::array::from_fn(|index| Rect::new(4. + index as f32 * 88., 708., 88., 66.))
}
fn favorite_count(state: &AppState) -> usize {
    state.favorites.iter().filter(|value| **value).count()
}
fn stat(rect: Rect, label: &str, count: usize) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 11., rect.y + 22., 9., crate::theme::BRASS);
    text(
        &count.to_string(),
        rect.x + 45.,
        rect.y + 50.,
        20.,
        crate::theme::CREAM,
    );
}
fn button(rect: Rect, label: &str, fill: Color) {
    panel(rect, fill);
    text(label, rect.x + 13., rect.y + 29., 9., crate::theme::CREAM);
}
fn panel(rect: Rect, fill: Color) {
    crate::ui::draw_rounded_panel(rect, 8., fill, crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, size, color);
}
