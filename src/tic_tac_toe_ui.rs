//! Responsive presentation and touch routing for Tic-Tac-Toe.

use crate::{
    accessibility,
    state::AppState,
    tic_tac_toe::{Mark, TicTacToeStatus},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    new_board: Rect,
    undo: Rect,
    hint: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(24., 48., 300., 300.),
            cell: 100.,
            new_board: Rect::new(370., 125., 150., 48.),
            undo: Rect::new(370., 185., 150., 48.),
            hint: Rect::new(530., 185., 150., 48.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 145., 320., 320.),
            cell: 106.6667,
            new_board: Rect::new(20., 510., 155., 48.),
            undo: Rect::new(185., 510., 155., 48.),
            hint: Rect::new(20., 570., 155., 48.),
        }
    } else {
        Layout {
            board: Rect::new(390., 130., 500., 500.),
            cell: 166.6667,
            new_board: Rect::new(440., 650., 180., 48.),
            undo: Rect::new(650., 650., 180., 48.),
            hint: Rect::new(860., 650., 180., 48.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.new_board, point) {
        return vec![UiAction::TicTacToeNew];
    }
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::TicTacToeUndo];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::TicTacToeHint];
    }
    if layout.board.contains(point) && state.tic_tac_toe.status == TicTacToeStatus::Playing {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 3 && column < 3 {
            return vec![UiAction::TicTacToePress(row * 3 + column)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.tic_tac_toe;
    let header_y = if crate::ui::is_compact_landscape() {
        35.
    } else if crate::ui::is_portrait() {
        105.
    } else {
        72.
    };
    let header_x = if crate::ui::is_compact_landscape() {
        132.
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
        accessibility::text_size(14., state.large_text),
        muted(),
    );
    text(
        "TIC-TAC-TOE",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        status_text(game.status),
        body_x,
        body_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for index in 0..9 {
        let row = index / 3;
        let column = index % 3;
        let rect = Rect::new(
            layout.board.x + column as f32 * layout.cell,
            layout.board.y + row as f32 * layout.cell,
            layout.cell - 4.,
            layout.cell - 4.,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            accessibility::board_fill(state.high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.,
            accessibility::grid_line(state.high_contrast),
        );
        match game.cells[index] {
            Mark::X => draw_x(rect, state.high_contrast),
            Mark::O => draw_circle_lines(
                rect.center().x,
                rect.center().y,
                rect.w * 0.25,
                5.,
                if state.high_contrast {
                    Color::new(0.05, 1., 0.90, 1.)
                } else {
                    Color::new(0.45, 0.82, 0.80, 1.)
                },
            ),
            Mark::Empty => {}
        }
    }
    text(
        &format!("MOVES  {}", game.moves),
        layout.board.x,
        layout.board.bottom() + 28.,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.new_board, "NEW BOARD", state.large_text);
    button(layout.undo, "UNDO", state.large_text);
    button(layout.hint, "HINT", state.large_text);
    if let Some(hint) = state.card_hint.as_deref() {
        let (x, y) = if crate::ui::is_compact_landscape() {
            (530., 250.)
        } else if crate::ui::is_portrait() {
            (20., 640.)
        } else {
            (40., 625.)
        };
        text(
            hint,
            x,
            y,
            accessibility::text_size(12., state.large_text),
            accent(),
        );
    }
}

fn draw_x(rect: Rect, high_contrast: bool) {
    let inset = rect.w * 0.25;
    let color = if high_contrast {
        Color::new(1., 0.15, 0.20, 1.)
    } else {
        Color::new(0.98, 0.55, 0.42, 1.)
    };
    draw_line(
        rect.x + inset,
        rect.y + inset,
        rect.right() - inset,
        rect.bottom() - inset,
        6.,
        color,
    );
    draw_line(
        rect.right() - inset,
        rect.y + inset,
        rect.x + inset,
        rect.bottom() - inset,
        6.,
        color,
    );
}

fn status_text(status: TicTacToeStatus) -> &'static str {
    match status {
        TicTacToeStatus::Playing => "Your turn • the cabinet answers after each move",
        TicTacToeStatus::Won(Mark::X) => "You made three in a row.",
        TicTacToeStatus::Won(Mark::O) => "The cabinet made three in a row.",
        TicTacToeStatus::Won(Mark::Empty) => "The board is complete.",
        TicTacToeStatus::Draw => "A quiet draw. Start another board to play again.",
    }
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

fn button(rect: Rect, label: &str, large_text: bool) {
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
        accessibility::text_size(body_size(), large_text),
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.76, 0.70, 0.86, 1.)
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        26.
    } else {
        32.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        12.
    } else {
        15.
    }
}
