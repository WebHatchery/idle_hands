//! Responsive touch presentation for Mancala.

use crate::{
    mancala::{Mancala, MancalaPhase},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(220., 48., 390., 245.),
            undo: Rect::new(640., 105., 100., 44.),
            new_game: Rect::new(640., 160., 135., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 125., 340., 260.),
            undo: Rect::new(10., 430., 145., 44.),
            new_game: Rect::new(165., 430., 185., 44.),
        }
    } else {
        Layout {
            board: Rect::new(300., 125., 500., 300.),
            undo: Rect::new(850., 190., 120., 44.),
            new_game: Rect::new(990., 190., 145., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    for pit in 0..6 {
        if pit_rect(l.board, pit, true).contains(point) {
            return vec![UiAction::MancalaPit(pit)];
        }
    }
    if l.undo.contains(point) {
        return vec![UiAction::MancalaUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::MancalaNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.mancala;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        62.
    } else {
        58.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("MANCALA", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Your store {}  •  Cabinet {}  •  {} moves",
            game.pits[6], game.pits[13], game.moves
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    text(
        status_text(game.phase),
        if compact { 220. } else { title_x },
        if portrait {
            410.
        } else if compact {
            320.
        } else {
            465.
        },
        body_size(),
        muted(),
    );
    button(l.undo, "UNDO");
    button(l.new_game, "NEW BOARD");
}

fn draw_board(board: Rect, game: &Mancala) {
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        Color::new(0.17, 0.10, 0.18, 1.),
    );
    draw_rectangle_lines(board.x, board.y, board.w, board.h, 2., accent());
    for pit in 0..6 {
        draw_pit(pit_rect(board, pit, false), game.pits[12 - pit], false);
        draw_pit(pit_rect(board, pit, true), game.pits[pit], true);
    }
    draw_store(
        Rect::new(board.x + 8., board.y + 64., 48., 132.),
        game.pits[13],
        false,
    );
    draw_store(
        Rect::new(board.x + board.w - 56., board.y + 64., 48., 132.),
        game.pits[6],
        true,
    );
    text(
        "CABINET",
        board.x + board.w - 53.,
        board.y + 214.,
        8.,
        muted(),
    );
    text("YOU", board.x + 17., board.y + 214., 8., muted());
}

fn pit_rect(board: Rect, pit: usize, player: bool) -> Rect {
    let step = if crate::ui::is_portrait() {
        40.
    } else if crate::ui::is_compact_landscape() {
        45.
    } else {
        60.
    };
    let width = if crate::ui::is_portrait() {
        34.
    } else if crate::ui::is_compact_landscape() {
        40.
    } else {
        50.
    };
    let x = board.x
        + if crate::ui::is_portrait() {
            58.
        } else if crate::ui::is_compact_landscape() {
            68.
        } else {
            72.
        }
        + pit as f32 * step;
    let y = if player {
        board.y + board.h - 84.
    } else {
        board.y + 24.
    };
    Rect::new(x, y, width, 60.)
}

fn draw_pit(rect: Rect, stones: u8, player: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if player {
            Color::new(0.22, 0.17, 0.31, 1.)
        } else {
            Color::new(0.13, 0.11, 0.22, 1.)
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
    center_text(
        &stones.to_string(),
        rect,
        if crate::ui::is_portrait() { 16. } else { 20. },
        WHITE,
    );
}

fn draw_store(rect: Rect, stones: u8, player: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.25, 0.16, 0.28, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        &stones.to_string(),
        rect,
        22.,
        if player { accent() } else { muted() },
    );
}

fn status_text(phase: MancalaPhase) -> &'static str {
    match phase {
        MancalaPhase::Playing => "Sow the stones from one of your six pits",
        MancalaPhase::Won => "Your store holds the quiet majority",
        MancalaPhase::Lost => "The cabinet gathered more stones",
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}

fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        23.
    } else {
        29.
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
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
