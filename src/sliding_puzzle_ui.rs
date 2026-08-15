//! Responsive presentation and touch routing for Sliding Puzzle.

use crate::{sliding_puzzle::SlidingStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    new_board: Rect,
    undo: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(10., 48., 300., 300.),
            cell: 75.,
            new_board: Rect::new(350., 125., 155., 48.),
            undo: Rect::new(350., 185., 155., 48.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 130., 340., 340.),
            cell: 85.,
            new_board: Rect::new(10., 525., 160., 48.),
            undo: Rect::new(180., 525., 160., 48.),
        }
    } else {
        Layout {
            board: Rect::new(360., 120., 520., 520.),
            cell: 130.,
            new_board: Rect::new(440., 650., 180., 48.),
            undo: Rect::new(650., 650., 180., 48.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.new_board.contains(point) {
        return vec![UiAction::SlidingPuzzleNew];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::SlidingPuzzleUndo];
    }
    if layout.board.contains(point) && state.sliding_puzzle.status != SlidingStatus::Won {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 4 && column < 4 {
            return vec![UiAction::SlidingPuzzleMove(row * 4 + column)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.sliding_puzzle;
    let header_y = if crate::ui::is_compact_landscape() {
        35.
    } else if crate::ui::is_portrait() {
        87.
    } else {
        72.
    };
    let header_x = if crate::ui::is_compact_landscape() {
        120.
    } else if crate::ui::is_portrait() {
        20.
    } else {
        layout.board.x
    };
    let body_x = if crate::ui::is_compact_landscape() {
        layout.new_board.x
    } else {
        header_x
    };
    let body_y = if crate::ui::is_compact_landscape() {
        layout.new_board.y - 34.
    } else {
        header_y + 28.
    };
    text(
        "‹ CABINET",
        back_rect().x,
        back_rect().y + 20.,
        14.,
        muted(),
    );
    text("SLIDING PUZZLE", header_x, header_y, title_size(), accent());
    text(
        status_text(game.status),
        body_x,
        body_y,
        body_size(),
        muted(),
    );
    for index in 0..game.cells.len() {
        let row = index / 4;
        let column = index % 4;
        let rect = Rect::new(
            layout.board.x + column as f32 * layout.cell,
            layout.board.y + row as f32 * layout.cell,
            layout.cell - 4.,
            layout.cell - 4.,
        );
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if value == 0 {
                Color::new(0.08, 0.06, 0.15, 1.)
            } else {
                tile_color(value)
            },
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
        if value != 0 {
            text(
                &value.to_string(),
                rect.x + rect.w * 0.40,
                rect.y + rect.h * 0.61,
                (rect.w * 0.28).min(34.),
                WHITE,
            );
        }
    }
    text(
        &format!("MOVES  {}", game.moves),
        layout.board.x,
        layout.board.bottom() + 28.,
        body_size(),
        muted(),
    );
    button(layout.new_board, "NEW BOARD");
    button(layout.undo, "UNDO");
}

fn status_text(status: SlidingStatus) -> &'static str {
    match status {
        SlidingStatus::Playing => "Tap a tile beside the empty space.",
        SlidingStatus::Won => "The tiles are in order. Start another board to play again.",
    }
}

fn tile_color(value: u8) -> Color {
    let shade = 0.28 + (value % 5) as f32 * 0.06;
    Color::new(
        0.28 + shade * 0.25,
        0.18 + shade * 0.22,
        0.42 + shade * 0.35,
        1.,
    )
}

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(10., 8., 100., 30.)
    } else if crate::ui::is_portrait() {
        Rect::new(10., 52., 110., 30.)
    } else {
        Rect::new(48., 28., 130., 36.)
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.14, 0.31, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
    text(
        label,
        rect.x + 16.,
        rect.y + rect.h * 0.64,
        body_size(),
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.76, 0.70, 0.86, 1.)
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
        12.
    } else {
        15.
    }
}
