//! Responsive presentation and touch routing for Paddle Duel.

use crate::{
    accessibility,
    paddle_duel::{PaddleDuel, PaddleMove, PaddleStatus, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    up: Rect,
    down: Rect,
    pause: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let cell = 14.;
        Layout {
            board: Rect::new(10., 42., WIDTH * cell, HEIGHT * cell),
            cell,
            up: Rect::new(475., 55., 70., 42.),
            down: Rect::new(475., 102., 70., 42.),
            pause: Rect::new(475., 149., 70., 36.),
            undo: Rect::new(475., 190., 70., 36.),
            new_game: Rect::new(475., 233., 70., 36.),
        }
    } else if crate::ui::is_portrait() {
        let cell = 9.;
        Layout {
            board: Rect::new(10., 104., WIDTH * cell, HEIGHT * cell),
            cell,
            up: Rect::new(30., 290., 135., 42.),
            down: Rect::new(185., 290., 135., 42.),
            pause: Rect::new(20., 345., 145., 42.),
            undo: Rect::new(185., 345., 145., 42.),
            new_game: Rect::new(20., 400., 310., 42.),
        }
    } else {
        let cell = 24.;
        Layout {
            board: Rect::new(100., 100., WIDTH * cell, HEIGHT * cell),
            cell,
            up: Rect::new(1010., 155., 105., 60.),
            down: Rect::new(1010., 225., 105., 60.),
            pause: Rect::new(900., 330., 105., 42.),
            undo: Rect::new(1010., 330., 105., 42.),
            new_game: Rect::new(900., 385., 215., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.up, point) {
        return vec![UiAction::PaddleMove(PaddleMove::Up)];
    }
    if crate::ui::hit(l.down, point) {
        return vec![UiAction::PaddleMove(PaddleMove::Down)];
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::PaddlePause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::PaddleUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::PaddleNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.paddle_duel;
    let compact = crate::ui::is_compact_landscape();
    let title_x = if compact {
        80.
    } else if crate::ui::is_portrait() {
        10.
    } else {
        100.
    };
    let title_y = if crate::ui::is_compact_landscape() {
        27.
    } else if crate::ui::is_portrait() {
        68.
    } else {
        60.
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("PADDLE DUEL", title_x, title_y, title_size(), accent());
    let score_y = if compact { 39. } else { title_y + 25. };
    label(
        &format!("YOU {}  —  CPU {}", game.player_score, game.cpu_score),
        title_x,
        score_y,
        body_size(),
        muted(),
    );
    draw_rectangle(
        l.board.x,
        l.board.y,
        l.board.w,
        l.board.h,
        accessibility::board_fill(state.high_contrast),
    );
    for row in 0..18 {
        let y = l.board.y + (row as f32 + 0.5) * l.cell;
        draw_line(
            l.board.x + l.board.w * 0.5,
            y,
            l.board.x + l.board.w * 0.5,
            y + l.cell * 0.45,
            2.,
            Color::new(0.50, 0.48, 0.60, 0.55),
        );
    }
    draw_rectangle(
        l.board.x + l.cell,
        l.board.y + (game.paddle_y - 2.) * l.cell,
        l.cell * 0.5,
        l.cell * 4.,
        Color::new(0.36, 0.87, 0.94, 1.),
    );
    draw_rectangle(
        l.board.right() - l.cell * 1.5,
        l.board.y + (game.cpu_y - 2.) * l.cell,
        l.cell * 0.5,
        l.cell * 4.,
        Color::new(1., 0.46, 0.50, 1.),
    );
    draw_circle(
        l.board.x + game.ball_x * l.cell,
        l.board.y + game.ball_y * l.cell,
        l.cell * 0.45,
        Color::new(1., 0.80, 0.25, 1.),
    );
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        title_x,
        if crate::ui::is_portrait() {
            270.
        } else if crate::ui::is_compact_landscape() {
            330.
        } else {
            575.
        },
        body_size(),
        muted(),
    );
    button(l.up, "UP", state.large_text);
    button(l.down, "DOWN", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(
        l.new_game,
        if compact { "NEW" } else { "NEW MATCH" },
        state.large_text,
    );
}

fn status_text(game: &PaddleDuel) -> String {
    match game.status {
        PaddleStatus::Playing => "Tap UP or DOWN to move your paddle".into(),
        PaddleStatus::Won => format!("{} points — you own the table", game.win_score),
        PaddleStatus::Lost => format!("The cabinet reached {} — play again", game.win_score),
    }
}
fn button(rect: Rect, value: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(11., large_text);
    crate::ui::draw_text(
        value,
        rect.x + (rect.w - crate::ui::measure_text(value, None, size as u16, 1.).width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}
fn label(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        24.
    } else {
        30.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 115., 42.)
}
