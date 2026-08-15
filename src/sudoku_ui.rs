//! Sudoku board presentation and touch input.

use crate::{state::AppState, sudoku::SudokuStatus, ui::UiAction};
use macroquad::prelude::*;

fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
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

pub fn draw_sudoku(state: &AppState) {
    let game = &state.sudoku;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("SUDOKU", 40., 105., 48., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Fill every row, column, and box",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    let board = Rect::new(300., 150., 504., 504.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
    for index in 0..81 {
        let rect = Rect::new(
            board.x + 4. + (index % 9) as f32 * 54.,
            board.y + 4. + (index / 9) as f32 * 54.,
            52.,
            52.,
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
            if index % 3 == 0 || index / 9 % 3 == 0 {
                2.
            } else {
                1.
            },
            Color::new(0.48, 0.40, 0.60, 0.8),
        );
        let value = game.values[index];
        if value != 0 {
            text(
                &value.to_string(),
                rect.x + 19.,
                rect.y + 36.,
                28.,
                if game.is_given(index) {
                    WHITE
                } else {
                    Color::new(0.98, 0.72, 0.38, 1.)
                },
            );
        } else if game.notes[index] != 0 {
            text(
                "· · ·",
                rect.x + 10.,
                rect.y + 30.,
                14.,
                Color::new(0.63, 0.58, 0.72, 1.),
            );
        }
    }
    text(
        if game.status == SudokuStatus::Won {
            "Puzzle complete"
        } else {
            "Select a cell, then tap a number"
        },
        850.,
        190.,
        18.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    text(
        &format!(
            "Moves: {}  Best: {}",
            game.moves,
            game.best_moves
                .map_or("—".to_owned(), |best| best.to_string())
        ),
        850.,
        215.,
        17.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    text(
        if state.sudoku_note_mode {
            "PENCIL MARKS"
        } else {
            "ENTER NUMBERS"
        },
        850.,
        245.,
        18.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        let rect = Rect::new(850. + col as f32 * 78., 260. + row as f32 * 62., 66., 50.);
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(&number.to_string(), rect.x + 25., rect.y + 34., 24., WHITE);
    }
    panel(
        Rect::new(850., 465., 210., 48.),
        if state.sudoku_note_mode {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    text("TOGGLE PENCIL", 880., 496., 16., WHITE);
    panel(
        Rect::new(1080., 465., 110., 48.),
        Color::new(0.28, 0.16, 0.30, 1.),
    );
    text("ERASE", 1108., 496., 16., WHITE);
    panel(
        Rect::new(850., 530., 140., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 900., 558., 16., WHITE);
    text(
        "Given clues are white. Your entries are gold.",
        850.,
        570.,
        15.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        let rect = Rect::new(850. + index as f32 * 110., 105., 100., 32.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(difficulty.label(), rect.x + 14., rect.y + 21., 12., WHITE);
    }
}

pub fn sudoku_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    let board = Rect::new(300., 150., 504., 504.);
    if board.contains(p) {
        let column = ((p.x - board.x - 4.) / 54.) as usize;
        let row = ((p.y - board.y - 4.) / 54.) as usize;
        if column < 9 && row < 9 {
            return vec![UiAction::SudokuCell(row * 9 + column)];
        }
    }
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        if Rect::new(850. + col as f32 * 78., 260. + row as f32 * 62., 66., 50.).contains(p) {
            return vec![UiAction::SudokuNumber(number as u8)];
        }
    }
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        if Rect::new(850. + index as f32 * 110., 105., 100., 32.).contains(p) {
            return vec![UiAction::SudokuDifficulty(*difficulty)];
        }
    }
    if Rect::new(850., 465., 210., 48.).contains(p) {
        return vec![UiAction::SudokuNoteMode];
    }
    if Rect::new(1080., 465., 110., 48.).contains(p) {
        return vec![UiAction::SudokuErase];
    }
    if Rect::new(850., 530., 140., 44.).contains(p) {
        return vec![UiAction::SudokuUndo];
    }
    vec![]
}
