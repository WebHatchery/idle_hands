//! Compact portrait Sudoku rendering and touch routing.

use crate::{state::AppState, ui::UiAction};
use macroquad::prelude::*;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw(state: &AppState) {
    let game = &state.games.sudoku;
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("‹ CABINET", 10., 29., 14., crate::theme::BRASS);
    text("SUDOKU", 12., 78., 34., crate::theme::BRASS);
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        let rect = Rect::new(148. + index as f32 * 68., 48., 62., 44.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                crate::theme::LEATHER
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
                crate::theme::WALNUT
            } else {
                Color::new(0.15, 0.11, 0.23, 1.)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
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
    for boundary in 0..=3 {
        let offset = boundary as f32 * cell * 3.;
        draw_line(
            board.x + 4. + offset,
            board.y + 4.,
            board.x + 4. + offset,
            board.y + 4. + cell * 9.,
            3.5,
            crate::accessibility::grid_line(state.high_contrast),
        );
        draw_line(
            board.x + 4.,
            board.y + 4. + offset,
            board.x + 4. + cell * 9.,
            board.y + 4. + offset,
            3.5,
            crate::accessibility::grid_line(state.high_contrast),
        );
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
        crate::theme::SECONDARY,
    );
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        let rect = Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.);
        panel(rect, crate::theme::SURFACE);
        text(&number.to_string(), rect.x + 46., rect.y + 29., 20., WHITE);
    }
    panel(
        Rect::new(12., 650., 104., 44.),
        Color::new(0.45, 0.25, 0.42, 1.),
    );
    text("PENCIL", 35., 679., 13., WHITE);
    panel(Rect::new(128., 650., 104., 44.), crate::theme::SURFACE);
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
        crate::theme::SECONDARY,
    );
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
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
        if crate::ui::hit(Rect::new(148. + index as f32 * 68., 48., 62., 44.), p) {
            return vec![UiAction::SudokuDifficulty(*difficulty)];
        }
    }
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        if crate::ui::hit(
            Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.),
            p,
        ) {
            return vec![UiAction::SudokuNumber(number as u8)];
        }
    }
    if crate::ui::hit(Rect::new(12., 650., 104., 44.), p) {
        return vec![UiAction::SudokuNoteMode];
    }
    if crate::ui::hit(Rect::new(128., 650., 104., 44.), p) {
        return vec![UiAction::SudokuErase];
    }
    if crate::ui::hit(Rect::new(244., 650., 104., 44.), p) {
        return vec![UiAction::SudokuUndo];
    }
    let _ = state;
    vec![]
}
