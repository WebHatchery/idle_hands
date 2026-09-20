//! Medium landscape layouts for short touch screens.

use crate::domain::Direction;
use crate::{game_2048::Game2048Size, palette_ui, state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub const WIDTH: f32 = 844.;
pub const HEIGHT: f32 = 390.;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw_2048(state: &AppState) {
    let game = &state.games.game;
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("‹ CABINET", 12., 29., 13., crate::theme::BRASS);
    text("2048", 12., 58., 27., crate::theme::BRASS);
    text(
        &format!("Score {}  -  Best {}", game.score, game.best),
        120.,
        51.,
        12.,
        WHITE,
    );
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        let rect = Rect::new(380. + index as f32 * 100., 75., 90., 40.);
        panel(
            rect,
            if *board_size == game.board_size {
                crate::theme::LEATHER
            } else {
                crate::theme::GAME_PANEL
            },
        );
        text(board_size.label(), rect.x + 18., rect.y + 26., 10., WHITE);
    }
    let board = Rect::new(12., 65., 320., 320.);
    panel(board, crate::theme::GAME_PANEL);
    let dimension = game.board_size.dimension();
    let tile_size = if dimension == 4 { 72. } else { 54. };
    let gap = 6.;
    let grid_side = tile_size * dimension as f32 + gap * (dimension - 1) as f32;
    let origin_x = board.x + (board.w - grid_side) * 0.5;
    let origin_y = board.y + (board.h - grid_side) * 0.5;
    for index in 0..game.cells.len() {
        let rect = Rect::new(
            origin_x + (index % dimension) as f32 * (tile_size + gap),
            origin_y + (index / dimension) as f32 * (tile_size + gap),
            tile_size,
            tile_size,
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
            let size = if value < 100 {
                if dimension == 4 {
                    25.
                } else {
                    20.
                }
            } else if dimension == 4 {
                19.
            } else {
                17.
            };
            let width = crate::ui::measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + (rect.h + size * 0.36) * 0.5,
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
        panel(rect, crate::theme::SURFACE_DARK);
        text(
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            rect.x + 12.,
            rect.y + 29.,
            11.,
            crate::theme::BRASS,
        );
        let _ = direction;
    }
    panel(Rect::new(590., 145., 110., 46.), crate::theme::SURFACE_DARK);
    text("UNDO", 625., 174., 12., WHITE);
    panel(Rect::new(715., 145., 115., 46.), crate::theme::SURFACE);
    text("NEW GAME", 738., 174., 11., WHITE);
    panel(Rect::new(590., 205., 110., 46.), crate::theme::SURFACE_DARK);
    text("HINT", 625., 234., 12., WHITE);
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 380., 285., 13., crate::theme::SECONDARY);
    }
    if state.confirm_restart {
        panel(
            Rect::new(375., 215., 300., 120.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 435., 250., 18., WHITE);
        panel(Rect::new(395., 270., 115., 42.), crate::theme::MOSS_DARK);
        text("CANCEL", 425., 297., 12., WHITE);
        panel(
            Rect::new(535., 270., 115., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 572., 297., 12., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if crate::ui::hit(Rect::new(395., 270., 115., 42.), p) {
            return vec![UiAction::Cancel];
        }
        if crate::ui::hit(Rect::new(535., 270., 115., 42.), p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(590., 145., 110., 46.), p) && state.games.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(715., 145., 115., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(590., 205., 110., 46.), p) {
        return vec![UiAction::Game2048Hint];
    }
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(380. + index as f32 * 100., 75., 90., 40.), p)
            && state.games.game.board_size != *board_size
        {
            return vec![UiAction::Game2048Size(*board_size)];
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
