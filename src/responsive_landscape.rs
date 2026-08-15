//! Medium landscape layouts for short touch screens.

use crate::{
    cosmetics,
    data::GameData,
    palette_ui,
    state::{AppState, Direction, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 844.;
pub const HEIGHT: f32 = 390.;

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
    draw_text(value, x, y, size, color);
}

fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        8. + (index % 4) as f32 * 210.,
        54. + (index / 4) as f32 * 130.,
        200.,
        112.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    text(
        "IDLE HANDS",
        12.,
        30.,
        25.,
        cosmetics::cabinet_accent(state.cabinet_decoration),
    );
    text(
        "Quiet games for a wider pause",
        190.,
        28.,
        13.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (index, game) in GameId::ALL.iter().enumerate() {
        let rect = cabinet_rect(index);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        text(
            game.title(),
            rect.x + 10.,
            rect.y + 27.,
            16.,
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        text(
            cabinet_status(state, *game),
            rect.x + 10.,
            rect.y + 51.,
            11.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        text(
            game.subtitle(),
            rect.x + 10.,
            rect.y + 78.,
            11.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            rect.right() - 20.,
            rect.y + 20.,
            12.,
            cosmetics::cabinet_accent(state.cabinet_decoration),
        );
        text(
            &(index + 1).to_string(),
            rect.right() - 24.,
            rect.y + 25.,
            11.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    text(
        &format!("{} stamps  •  {} textures", state.stamps, loaded),
        12.,
        344.,
        11.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
    for (rect, label) in [
        (Rect::new(450., 330., 112., 42.), "HELP"),
        (Rect::new(570., 330., 112., 42.), "RECORDS"),
        (Rect::new(690., 330., 140., 42.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 15., rect.y + 27., 11., WHITE);
    }
}

pub fn cabinet_clicks(p: Vec2) -> Vec<UiAction> {
    for index in 0..8 {
        if cabinet_rect(index).contains(p) {
            return vec![UiAction::Open(index)];
        }
    }
    for (rect, action) in [
        (Rect::new(450., 330., 112., 42.), UiAction::Help),
        (Rect::new(570., 330., 112., 42.), UiAction::Records),
        (Rect::new(690., 330., 140., 42.), UiAction::Settings),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    text("‹ CABINET", 12., 26., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 12., 58., 27., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  •  Best {}", game.score, game.best),
        120.,
        51.,
        12.,
        WHITE,
    );
    let board = Rect::new(12., 65., 320., 320.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
    for index in 0..16 {
        let rect = Rect::new(
            board.x + 8. + (index % 4) as f32 * 78.,
            board.y + 8. + (index / 4) as f32 * 78.,
            72.,
            72.,
        );
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette_ui::tile_color(value, state.board_theme),
        );
        if value > 0 {
            let label = value.to_string();
            let size = if value < 100 { 25. } else { 19. };
            let width = measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + 45.,
                size,
                WHITE,
            );
        }
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        );
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            rect.x + 12.,
            rect.y + 29.,
            11.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        let _ = direction;
    }
    panel(
        Rect::new(590., 145., 110., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 625., 174., 12., WHITE);
    panel(
        Rect::new(715., 145., 115., 46.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW GAME", 738., 174., 11., WHITE);
    text(
        "Swipe the board or tap a direction.",
        380.,
        285.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    if state.confirm_restart {
        panel(
            Rect::new(375., 215., 300., 120.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 435., 250., 18., WHITE);
        panel(
            Rect::new(395., 270., 115., 42.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 425., 297., 12., WHITE);
        panel(
            Rect::new(535., 270., 115., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 572., 297., 12., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(8., 5., 100., 30.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if Rect::new(395., 270., 115., 42.).contains(p) {
            return vec![UiAction::Cancel];
        }
        if Rect::new(535., 270., 115., 42.).contains(p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if Rect::new(590., 145., 110., 46.).contains(p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if Rect::new(715., 145., 115., 46.).contains(p) {
        return vec![UiAction::Restart];
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        if Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        )
        .contains(p)
        {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

fn cabinet_status(state: &AppState, game: GameId) -> &'static str {
    match game {
        GameId::Game2048 if state.records.best_2048 >= 2048 => "COMPLETE",
        GameId::Minesweeper if state.records.minesweeper.iter().any(Option::is_some) => "COMPLETE",
        GameId::Sudoku if state.records.sudoku.iter().any(Option::is_some) => "COMPLETE",
        GameId::Nonogram if state.records.nonogram.iter().any(Option::is_some) => "COMPLETE",
        GameId::Solitaire if state.records.solitaire_best_moves.is_some() => "COMPLETE",
        GameId::FreeCell if state.records.freecell_best_moves.is_some() => "COMPLETE",
        GameId::Yahtzee if state.records.fivefold_best_total > 0 => "COMPLETE",
        GameId::Reversi if state.records.reversi_best_score > 0 => "COMPLETE",
        _ => "PLAY NOW",
    }
}
