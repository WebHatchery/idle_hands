//! Desktop cabinet rendering kept separate from the shared interaction host.

use crate::{
    cosmetics,
    data::GameData,
    state::{AppState, GameId},
};
use macroquad::prelude::*;

pub fn draw(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    draw_rectangle(24., 18., 1232., 684., crate::theme::BACKGROUND_DEEP);
    draw_rectangle_lines(24., 18., 1232., 684., 3., crate::theme::BORDER);
    text("IDLE HANDS", 46., 70., 48., accent);
    crate::cabinet_art::draw_header_motif(1160., 108., 24., accent);
    crate::cabinet_art::draw_shelves(48., 165., 1184., 510., accent);
    panel(Rect::new(48., 108., 175., 44.), crate::theme::MOSS_DARK);
    text("FAVORITES", 60., 130., 11., WHITE);
    text(
        &state
            .favorites
            .iter()
            .filter(|favorite| **favorite)
            .count()
            .to_string(),
        190.,
        130.,
        11.,
        crate::theme::BRASS,
    );
    panel(Rect::new(230., 108., 175., 44.), crate::theme::SURFACE_DARK);
    text("RECENT", 242., 130., 11., WHITE);
    text(
        &state.recent_games.len().to_string(),
        372.,
        130.,
        11.,
        crate::theme::BRASS,
    );
    for (rect, _, filter) in filter_buttons() {
        panel(
            rect,
            if state.cabinet_filter == filter {
                crate::theme::MOSS
            } else {
                crate::theme::SURFACE_DARK
            },
        );
        text(
            &crate::cabinet_status::filter_label(state, filter),
            rect.x + 9.,
            rect.y + 28.,
            10.,
            WHITE,
        );
        if state.cabinet_filter == filter {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3., WHITE);
        }
    }
    text(
        "A small collection for quiet minutes",
        48.,
        98.,
        20.,
        crate::theme::SECONDARY,
    );
    text(
        &format!(
            "{}  -  {} stamps  -  {} games",
            state.profile_name,
            state.stamps,
            GameId::ALL.len()
        ),
        720.,
        130.,
        15.,
        crate::theme::SECONDARY,
    );
    text(
        &format!(
            "{}  -  {}",
            cosmetics::cabinet_decoration_name(state.cabinet_decoration),
            cosmetics::board_theme_name(state.board_theme)
        ),
        990.,
        686.,
        15.,
        accent,
    );
    let games = visible_games(state);
    for (i, game) in games.iter().copied().enumerate() {
        let rect = cabinet_rect(i);
        let active = crate::cabinet_status::is_active(game);
        panel(rect, crate::theme::category_surface(game, active));
        let small_title = matches!(
            game,
            GameId::TinyTowerDefence
                | GameId::OneRoomRoguelike
                | GameId::DailyDungeon
                | GameId::DotsBoxes
                | GameId::Sokoban
                | GameId::Mancala
                | GameId::Hanoi
                | GameId::NumberMatch
                | GameId::FloodIt
                | GameId::ColorSort
                | GameId::Battleship
                | GameId::WordGrid
                | GameId::PipeLoop
                | GameId::MazeWalk
                | GameId::MatchThree
        );
        text(
            game.title(),
            rect.x + 18.,
            rect.y + 21.,
            if small_title { 10. } else { 18. },
            if active { crate::theme::CREAM } else { WHITE },
        );
        let status = crate::cabinet_status::status(state, game);
        text(
            status,
            rect.x + 18.,
            rect.y + 39.,
            11.,
            crate::cabinet_status::color(status),
        );
        text(
            game.subtitle(),
            rect.x + 18.,
            rect.y + 56.,
            11.,
            crate::theme::SECONDARY,
        );
        draw_circle(
            rect.right() - 34.,
            rect.y + 20.,
            12.,
            if active {
                crate::theme::BRASS
            } else {
                crate::theme::SURFACE_DARK
            },
        );
        if state.favorites.get(game.index()).copied().unwrap_or(false) {
            draw_circle(rect.right() - 34., rect.y + 49., 5., accent);
        } else {
            draw_circle_lines(rect.right() - 34., rect.y + 49., 5., 2., accent);
        }
        text(
            &crate::cabinet_status::drawer_number(game).to_string(),
            rect.right() - 37.,
            rect.y + 24.,
            11.,
            crate::theme::INK,
        );
    }
    if games.is_empty() {
        text(
            crate::cabinet_status::empty_filter_message(state.cabinet_filter),
            70.,
            210.,
            18.,
            WHITE,
        );
    }
    let selected = GameId::ALL[state.selected.min(GameId::ALL.len() - 1)];
    panel(Rect::new(720., 28., 190., 44.), crate::theme::MOSS_DARK);
    text("CONTINUE", 735., 49., 12., WHITE);
    text(selected.title(), 735., 64., 9., crate::theme::BRASS);
    for (rect, label) in [
        (Rect::new(940., 28., 90., 44.), "HELP"),
        (Rect::new(1040., 28., 90., 44.), "RECORDS"),
        (Rect::new(1140., 28., 110., 44.), "SETTINGS"),
    ] {
        panel(rect, crate::theme::SURFACE_DARK);
        text(label, rect.x + 14., rect.y + 29., 15., WHITE);
    }
    text(
        &format!(
            "Cabinet online  -  tap right FAV circles to save favorites  -  {} textures ready",
            loaded
        ),
        48.,
        686.,
        16.,
        crate::theme::SECONDARY,
    );
}

fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        48. + (index % 7) as f32 * 170.,
        150. + (index / 7) as f32 * 76.,
        160.,
        62.,
    )
}
pub fn filter_buttons_for_input() -> [(Rect, &'static str, u8); 3] {
    [
        (Rect::new(425., 104., 78., 44.), "ALL", 0),
        (Rect::new(509., 104., 78., 44.), "OPEN", 1),
        (Rect::new(593., 104., 78., 44.), "DONE", 2),
    ]
}
fn filter_buttons() -> [(Rect, &'static str, u8); 3] {
    filter_buttons_for_input()
}
fn visible_games(state: &AppState) -> Vec<GameId> {
    GameId::ALL
        .iter()
        .copied()
        .filter(|game| crate::cabinet_status::matches_filter(state, *game, state.cabinet_filter))
        .collect()
}
fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
