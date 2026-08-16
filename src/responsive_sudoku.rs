//! Compact portrait Sudoku rendering and touch routing.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

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

pub fn draw(state: &AppState) {
    let game = &state.sudoku;
    panel(
        Rect::new(0., 0., 110., 44.),
        Color::new(0.12, 0.08, 0.20, 1.),
    );
    text("‹ CABINET", 10., 29., 14., Color::new(0.78, 0.70, 0.92, 1.));
    text("SUDOKU", 12., 78., 34., Color::new(0.98, 0.83, 0.45, 1.));
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        let rect = Rect::new(148. + index as f32 * 68., 48., 62., 44.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(difficulty.label(), rect.x + 8., rect.y + 28., 10., WHITE);
    }
    let board = Rect::new(10., 100., 340., 340.);
    panel(board, crate::accessibility::board_fill(state.high_contrast));
    let cell = 36.8;
    for index in 0..81 {
        let row = index / 9;
        let col = index % 9;
        let rect = Rect::new(
            board.x + 4. + col as f32 * cell,
            board.y + 4. + row as f32 * cell,
            cell - 1.,
            cell - 1.,
        );
        let selected = game.selected == Some(index);
        let conflict = game
            .selected
            .map(|selected| game.conflicts(selected).contains(&index))
            .unwrap_or(false);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if conflict {
                Color::new(0.35, 0.13, 0.20, 1.)
            } else if selected {
                Color::new(0.30, 0.22, 0.42, 1.)
            } else {
                Color::new(0.15, 0.11, 0.23, 1.)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if col % 3 == 0 || row % 3 == 0 { 2. } else { 1. },
            crate::accessibility::grid_line(state.high_contrast),
        );
        if game.values[index] != 0 {
            text(
                &game.values[index].to_string(),
                rect.x + 12.,
                rect.y + 26.,
                crate::accessibility::text_size(21., state.large_text),
                if game.is_given(index) {
                    WHITE
                } else {
                    Color::new(0.98, 0.72, 0.38, 1.)
                },
            );
        }
    }
    text(
        if state.sudoku_note_mode {
            "PENCIL"
        } else {
            "NUMBER PAD"
        },
        12.,
        465.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        let rect = Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.);
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(&number.to_string(), rect.x + 46., rect.y + 29., 20., WHITE);
    }
    panel(
        Rect::new(12., 650., 104., 44.),
        Color::new(0.45, 0.25, 0.42, 1.),
    );
    text("PENCIL", 35., 679., 13., WHITE);
    panel(
        Rect::new(128., 650., 104., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("ERASE", 158., 679., 13., WHITE);
    panel(
        Rect::new(244., 650., 104., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 276., 679., 13., WHITE);
    text(
        &format!(
            "Moves {}  •  Best {}",
            game.moves,
            game.best_moves.map_or("—".into(), |v| v.to_string())
        ),
        12.,
        730.,
        12.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 110., 44.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    let board = Rect::new(10., 100., 340., 340.);
    if board.contains(p) {
        let col = ((p.x - board.x - 4.) / 36.8).floor() as usize;
        let row = ((p.y - board.y - 4.) / 36.8).floor() as usize;
        if col < 9 && row < 9 {
            return vec![UiAction::SudokuCell(row * 9 + col)];
        }
    }
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        if Rect::new(148. + index as f32 * 68., 48., 62., 44.).contains(p) {
            return vec![UiAction::SudokuDifficulty(*difficulty)];
        }
    }
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        if Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.).contains(p) {
            return vec![UiAction::SudokuNumber(number as u8)];
        }
    }
    if Rect::new(12., 650., 104., 44.).contains(p) {
        return vec![UiAction::SudokuNoteMode];
    }
    if Rect::new(128., 650., 104., 44.).contains(p) {
        return vec![UiAction::SudokuErase];
    }
    if Rect::new(244., 650., 104., 44.).contains(p) {
        return vec![UiAction::SudokuUndo];
    }
    let _ = state;
    vec![]
}
