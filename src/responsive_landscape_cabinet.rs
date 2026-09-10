//! Short-landscape collection dashboard with the desktop cabinet hierarchy.

use crate::{
    cabinet_status,
    data::GameData,
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const CONTINUE: Rect = Rect::new(170., 54., 270., 82.);
const CATEGORY_RECTS: [Rect; 6] = [
    Rect::new(170., 170., 205., 82.),
    Rect::new(385., 170., 205., 82.),
    Rect::new(600., 170., 205., 82.),
    Rect::new(170., 264., 205., 82.),
    Rect::new(385., 264., 205., 82.),
    Rect::new(600., 264., 205., 82.),
];
const PAGE_SIZE: usize = 16;

pub fn draw(
    state: &AppState,
    _data: &GameData,
    loaded: usize,
    cabinet_texture: Option<&Texture2D>,
) {
    draw_rectangle(0., 0., 844., 390., crate::theme::BACKGROUND_DEEP);
    draw_rectangle(154., 5., 685., 380., crate::theme::PAPER);
    draw_rectangle_lines(154., 5., 685., 380., 2., crate::theme::BORDER);
    if let Some(texture) = cabinet_texture {
        draw_texture_ex(
            texture,
            154.,
            5.,
            Color::new(1., 1., 1., 0.16),
            DrawTextureParams {
                dest_size: Some(vec2(685., 380.)),
                ..Default::default()
            },
        );
    }
    draw_sidebar(state);
    if state.cabinet_filter == 0 {
        draw_home(state, loaded);
    } else {
        draw_library(state);
    }
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    for (index, rect) in side_rects().iter().copied().enumerate() {
        if crate::ui::hit(rect, p) {
            return match index {
                0 => vec![UiAction::CabinetFilter(0)],
                1 => vec![UiAction::CabinetFilter(9)],
                2 => vec![UiAction::Favorites],
                3 => vec![UiAction::Recent],
                _ => vec![UiAction::Records],
            };
        }
    }
    if state.cabinet_filter == 0 {
        home_clicks(p)
    } else {
        library_clicks(state, p)
    }
}

fn draw_home(state: &AppState, loaded: usize) {
    text("Good evening", 170., 34., 21., crate::theme::INK);
    if crate::game_descriptor::is_demo_build() {
        text("DEMO · 30 GAMES", 700., 34., 9., crate::theme::BRASS);
    }
    text(
        "Pick a game and unwind.",
        310.,
        33.,
        10.,
        crate::theme::SURFACE,
    );
    panel(CONTINUE, crate::theme::MOSS_DARK);
    let selected = GameId::ALL[state.selected.min(GameId::ALL.len() - 1)];
    text("CONTINUE PLAYING", 185., 73., 8., crate::theme::BRASS);
    text(selected.title(), 185., 105., 20., crate::theme::CREAM);
    text(selected.subtitle(), 185., 125., 9., crate::theme::SECONDARY);
    stat(
        Rect::new(452., 54., 104., 82.),
        "FAVORITES",
        favorite_count(state),
    );
    stat(
        Rect::new(566., 54., 104., 82.),
        "RECENT",
        state.recent_games.len(),
    );
    panel(Rect::new(680., 54., 125., 82.), crate::theme::PAPER_LIGHT);
    text(
        &crate::daily_challenge::label(
            state.games.daily_dungeon.day_key,
            state.games.daily_dungeon.challenge,
        ),
        694.,
        79.,
        9.,
        crate::theme::SURFACE_DARK,
    );
    text(
        &crate::daily_challenge::preview_action(
            state.games.daily_dungeon.phase,
            state.records.daily_score(state.games.daily_dungeon.day_key),
        ),
        694.,
        110.,
        10.,
        crate::theme::INK,
    );
    text("COLLECTION", 170., 159., 10., crate::theme::INK);
    for (index, filter) in cabinet_status::CATEGORY_FILTERS.iter().copied().enumerate() {
        category(state, CATEGORY_RECTS[index], filter);
    }
    text(
        &format!("{} stamps · {} textures", state.stamps, loaded),
        682.,
        374.,
        8.,
        crate::theme::SURFACE,
    );
}

fn draw_library(state: &AppState) {
    let (playable, full) = cabinet_status::availability_counts(state, state.cabinet_filter);
    text("< HOME", 170., 28., 9., crate::theme::SURFACE);
    text(
        cabinet_status::category_name(state.cabinet_filter),
        170.,
        54.,
        21.,
        crate::theme::INK,
    );
    if crate::game_descriptor::is_demo_build() {
        text(
            &format!("{playable} PLAYABLE · {full} FULL"),
            600.,
            54.,
            8.,
            crate::theme::BRASS,
        );
    }
    for (index, game) in page_games(state).iter().copied().enumerate() {
        let rect = game_rect(index);
        panel(rect, crate::theme::PAPER_LIGHT);
        draw_circle(
            rect.x + 18.,
            rect.y + 18.,
            11.,
            crate::theme::category_surface(game, true),
        );
        text(
            game.title(),
            rect.x + 35.,
            rect.y + 17.,
            if game.title().len() > 18 { 8. } else { 10. },
            crate::theme::INK,
        );
        text(
            if cabinet_status::is_available(game) {
                game.subtitle()
            } else {
                crate::storefront::COMPACT_LOCKED_LABEL
            },
            rect.x + 35.,
            rect.y + 31.,
            7.,
            if cabinet_status::is_available(game) {
                crate::theme::SURFACE
            } else {
                crate::theme::BRASS
            },
        );
        text(
            ">",
            rect.right() - 16.,
            rect.y + 24.,
            13.,
            crate::theme::SURFACE_DARK,
        );
    }
    button(Rect::new(580., 341., 95., 38.), "< PREV");
    button(Rect::new(690., 341., 95., 38.), "NEXT >");
}

fn draw_sidebar(state: &AppState) {
    text("IDLE", 30., 38., 24., crate::theme::CREAM);
    text("HANDS", 30., 64., 24., crate::theme::CREAM);
    text("quiet games", 31., 81., 8., crate::theme::SECONDARY);
    let labels = ["HOME", "ALL GAMES", "FAVORITES", "RECENT", "RECORDS"];
    for (index, rect) in side_rects().iter().copied().enumerate() {
        let active =
            (index == 0 && state.cabinet_filter == 0) || (index == 1 && state.cabinet_filter == 9);
        if active {
            panel(rect, crate::theme::MOSS_DARK);
        }
        text(
            labels[index],
            rect.x + 22.,
            rect.y + 25.,
            10.,
            if active {
                crate::theme::CREAM
            } else {
                crate::theme::SECONDARY
            },
        );
    }
    text(&state.profile_name, 18., 348., 10., crate::theme::CREAM);
    text(
        &format!("{} stamps", state.stamps),
        18.,
        369.,
        8.,
        crate::theme::SECONDARY,
    );
}

fn category(state: &AppState, rect: Rect, filter: u8) {
    let sample = GameId::ALL
        .iter()
        .copied()
        .find(|game| cabinet_status::category_filter(*game) == filter)
        .unwrap_or(GameId::Solitaire);
    panel(rect, crate::theme::category_surface(sample, false));
    text(
        ["C", "L", "B", "W", "A", "M"][(filter - 3) as usize],
        rect.x + 16.,
        rect.y + 35.,
        18.,
        crate::theme::BRASS,
    );
    text(
        cabinet_status::category_name(filter),
        rect.x + 49.,
        rect.y + 31.,
        16.,
        crate::theme::CREAM,
    );
    text(
        &format!("{} games  >", cabinet_status::filter_count(state, filter)),
        rect.x + 49.,
        rect.y + 55.,
        9.,
        crate::theme::SECONDARY,
    );
    crate::mascots::draw_for_filter(filter, vec2(rect.right() - 29., rect.y + 41.), 0.68);
}

fn home_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(CONTINUE, p) {
        return vec![UiAction::ContinueGame];
    }
    for (index, rect) in CATEGORY_RECTS.iter().copied().enumerate() {
        if crate::ui::hit(rect, p) {
            return vec![UiAction::CabinetFilter(3 + index as u8)];
        }
    }
    if crate::ui::hit(Rect::new(452., 54., 104., 82.), p) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(Rect::new(566., 54., 104., 82.), p) {
        return vec![UiAction::Recent];
    }
    if crate::ui::hit(Rect::new(680., 54., 125., 82.), p) {
        return vec![UiAction::Open(GameId::DailyDungeon.index())];
    }
    vec![]
}

fn library_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(160., 4., 110., 58.), p) {
        return vec![UiAction::CabinetFilter(0)];
    }
    for (index, game) in page_games(state).iter().copied().enumerate() {
        if crate::ui::hit(game_rect(index), p) {
            return vec![UiAction::Open(game.index())];
        }
    }
    if crate::ui::hit(Rect::new(580., 341., 95., 38.), p) {
        return vec![UiAction::CabinetScroll(-(PAGE_SIZE as i8))];
    }
    if crate::ui::hit(Rect::new(690., 341., 95., 38.), p) {
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
fn side_rects() -> [Rect; 5] {
    std::array::from_fn(|index| Rect::new(10., 100. + index as f32 * 48., 134., 40.))
}
fn game_rect(index: usize) -> Rect {
    Rect::new(
        170. + (index % 4) as f32 * 160.,
        70. + (index / 4) as f32 * 63.,
        150.,
        52.,
    )
}
fn favorite_count(state: &AppState) -> usize {
    state.favorites.iter().filter(|value| **value).count()
}
fn stat(rect: Rect, label: &str, count: usize) {
    panel(rect, crate::theme::PAPER_LIGHT);
    text(
        label,
        rect.x + 10.,
        rect.y + 23.,
        8.,
        crate::theme::SURFACE_DARK,
    );
    text(
        &count.to_string(),
        rect.x + 42.,
        rect.y + 59.,
        20.,
        crate::theme::INK,
    );
}
fn button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 17., rect.y + 24., 9., crate::theme::CREAM);
}
fn panel(rect: Rect, fill: Color) {
    crate::ui::draw_rounded_panel(rect, 8., fill, crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, size, color);
}
