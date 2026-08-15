//! Compact portrait cabinet and 2048 layouts.

use crate::{
    cosmetics,
    data::GameData,
    palette_ui,
    state::{AppState, Direction, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 360.;
pub const HEIGHT: f32 = 780.;

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

pub fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        8. + (index % 2) as f32 * 172.,
        115. + (index / 2) as f32 * 128.,
        164.,
        110.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    text(
        "IDLE HANDS",
        18.,
        55.,
        30.,
        cosmetics::cabinet_accent(state.cabinet_decoration),
    );
    text(
        "Quiet games for a small screen",
        18.,
        78.,
        14.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (index, game) in GameId::ALL.iter().enumerate() {
        let rect = cabinet_rect(index);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        text(
            game.title(),
            rect.x + 10.,
            rect.y + 28.,
            17.,
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        text(
            cabinet_status(state, *game),
            rect.x + 10.,
            rect.y + 51.,
            12.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        text(
            game.subtitle(),
            rect.x + 10.,
            rect.y + 76.,
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
    for (rect, label) in [
        (Rect::new(8., 665., 108., 40.), "HELP"),
        (Rect::new(126., 665., 108., 40.), "RECORDS"),
        (Rect::new(244., 665., 108., 40.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 12., rect.y + 26., 11., WHITE);
    }
    text(
        &format!("{} stamps  •  {} textures", state.stamps, loaded),
        18.,
        640.,
        12.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
}

pub fn cabinet_clicks(p: Vec2) -> Vec<UiAction> {
    for index in 0..8 {
        if cabinet_rect(index).contains(p) {
            return vec![UiAction::Open(index)];
        }
    }
    for (rect, action) in [
        (Rect::new(8., 665., 108., 40.), UiAction::Help),
        (Rect::new(126., 665., 108., 40.), UiAction::Records),
        (Rect::new(244., 665., 108., 40.), UiAction::Settings),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    text("‹ CABINET", 16., 35., 15., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 16., 82., 38., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  •  Best {}", game.score, game.best),
        18.,
        108.,
        14.,
        WHITE,
    );
    let board = Rect::new(20., 130., 320., 320.);
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
        let rect = Rect::new(20. + index as f32 * 82., 475., 74., 46.);
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            ["↑", "←", "↓", "→"][index],
            rect.x + 27.,
            rect.y + 32.,
            24.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        let _ = direction;
    }
    panel(
        Rect::new(20., 545., 150., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 70., 575., 15., WHITE);
    panel(
        Rect::new(190., 545., 150., 46.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW GAME", 220., 575., 14., WHITE);
    text(
        "Swipe the board or tap an arrow.",
        42.,
        635.,
        14.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    if state.confirm_restart {
        panel(
            Rect::new(25., 265., 310., 145.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 58., 305., 20., WHITE);
        panel(
            Rect::new(45., 335., 120., 42.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 76., 362., 14., WHITE);
        panel(
            Rect::new(195., 335., 120., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 235., 362., 14., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 10., 110., 38.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if Rect::new(45., 335., 120., 42.).contains(p) {
            return vec![UiAction::Cancel];
        }
        if Rect::new(195., 335., 120., 42.).contains(p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if Rect::new(20., 545., 150., 46.).contains(p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if Rect::new(190., 545., 150., 46.).contains(p) {
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
        if Rect::new(20. + index as f32 * 82., 475., 74., 46.).contains(p) {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

pub fn draw_settings(state: &AppState) {
    panel(
        Rect::new(8., 30., 344., 700.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("SETTINGS", 22., 82., 30., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Profile: {}", state.profile_name),
        22.,
        120.,
        15.,
        WHITE,
    );
    for (y, label) in [
        (
            165.,
            format!("Card back: {}", cosmetics::card_back_name(state.card_back)),
        ),
        (
            215.,
            format!(
                "Board theme: {}",
                cosmetics::board_theme_name(state.board_theme)
            ),
        ),
        (
            265.,
            format!("Sound set: {}", cosmetics::sound_set_name(state.sound_set)),
        ),
        (
            315.,
            format!(
                "Decoration: {}",
                cosmetics::cabinet_decoration_name(state.cabinet_decoration)
            ),
        ),
    ] {
        panel(
            Rect::new(22., y - 28., 316., 42.),
            Color::new(0.16, 0.11, 0.24, 1.),
        );
        text(&label, 34., y, 14., WHITE);
    }
    panel(
        Rect::new(22., 360., 150., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("Sound: {}", if state.sound { "On" } else { "Off" }),
        35.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(186., 360., 152., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Motion: {}",
            if state.reduced_motion {
                "Reduced"
            } else {
                "Full"
            }
        ),
        198.,
        387.,
        13.,
        WHITE,
    );
    text(
        "Tap a row to cycle unlocked cosmetics.",
        22.,
        445.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    for (rect, label) in [
        (Rect::new(22., 665., 100., 42.), "BACK"),
        (Rect::new(130., 665., 100., 42.), "SAVE"),
        (Rect::new(238., 665., 100., 42.), "RESET"),
    ] {
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(label, rect.x + 25., rect.y + 27., 13., WHITE);
    }
}

pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        return vec![];
    }
    if Rect::new(22., 665., 100., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(130., 665., 100., 42.).contains(p) {
        return vec![UiAction::Save];
    }
    if Rect::new(238., 665., 100., 42.).contains(p) {
        return vec![UiAction::ResetData];
    }
    for (rect, action) in [
        (Rect::new(22., 137., 316., 42.), UiAction::CycleCardBack),
        (Rect::new(22., 187., 316., 42.), UiAction::CycleBoardTheme),
        (Rect::new(22., 237., 316., 42.), UiAction::CycleSoundSet),
        (
            Rect::new(22., 287., 316., 42.),
            UiAction::CycleCabinetDecoration,
        ),
        (Rect::new(22., 360., 150., 42.), UiAction::ToggleSound),
        (Rect::new(186., 360., 152., 42.), UiAction::ToggleMotion),
    ] {
        if rect.contains(p) {
            return vec![action];
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
