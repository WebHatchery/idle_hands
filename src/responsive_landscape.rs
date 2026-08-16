//! Medium landscape layouts for short touch screens.

use crate::{
    palette_ui,
    state::{AppState, Direction},
    ui::UiAction,
};
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
    let game = &state.game;
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("‹ CABINET", 12., 29., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 12., 58., 27., crate::theme::BRASS);
    text(
        &format!("Score {}  -  Best {}", game.score, game.best),
        120.,
        51.,
        12.,
        WHITE,
    );
    let board = Rect::new(12., 65., 320., 320.);
    panel(board, crate::theme::GAME_PANEL);
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
            let width = crate::ui::measure_text(&label, None, size as u16, 1.).width;
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
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Swipe the board or tap a direction."),
        380.,
        285.,
        13.,
        crate::theme::SECONDARY,
    );
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
    if crate::ui::hit(Rect::new(590., 145., 110., 46.), p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(715., 145., 115., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(590., 205., 110., 46.), p) {
        return vec![UiAction::Game2048Hint];
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
