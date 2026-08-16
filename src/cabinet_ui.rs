//! Desktop cabinet rendering kept separate from the shared interaction host.

use crate::{
    cosmetics,
    data::GameData,
    state::{AppState, GameId},
};
use macroquad::prelude::*;

pub fn draw(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 46., 70., 48., accent);
    crate::cabinet_art::draw_header_motif(1160., 108., 24., accent);
    crate::cabinet_art::draw_shelves(48., 165., 1184., 510., accent);
    panel(
        Rect::new(48., 108., 175., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
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
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(
        Rect::new(230., 108., 175., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RECENT", 242., 130., 11., WHITE);
    text(
        &state.recent_games.len().to_string(),
        372.,
        130.,
        11.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (rect, _, filter) in filter_buttons() {
        panel(
            rect,
            if state.cabinet_filter == filter {
                Color::new(0.35, 0.22, 0.42, 1.)
            } else {
                Color::new(0.20, 0.13, 0.30, 1.)
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
        Color::new(0.72, 0.68, 0.82, 1.),
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
        Color::new(0.60, 0.56, 0.72, 1.),
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
        panel(
            rect,
            if active {
                Color::new(0.17, 0.12, 0.27, 1.)
            } else {
                Color::new(0.09, 0.075, 0.15, 1.)
            },
        );
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
            if active {
                Color::new(0.98, 0.82, 0.42, 1.)
            } else {
                WHITE
            },
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
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            rect.right() - 34.,
            rect.y + 20.,
            12.,
            if active {
                Color::new(0.85, 0.55, 0.28, 1.)
            } else {
                Color::new(0.22, 0.18, 0.31, 1.)
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
            Color::new(0.08, 0.05, 0.12, 1.),
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
    panel(
        Rect::new(720., 28., 190., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("CONTINUE", 735., 49., 12., WHITE);
    text(
        selected.title(),
        735.,
        64.,
        9.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (rect, label) in [
        (Rect::new(940., 28., 90., 44.), "HELP"),
        (Rect::new(1040., 28., 90., 44.), "RECORDS"),
        (Rect::new(1140., 28., 110., 44.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
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
        Color::new(0.52, 0.48, 0.64, 1.),
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
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
