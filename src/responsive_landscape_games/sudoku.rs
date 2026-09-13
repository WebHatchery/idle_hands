use super::{back, panel, text};
use crate::{
    accessibility,
    state::AppState,
    sudoku::{SudokuDifficulty, SudokuStatus},
    ui::UiAction,
};
use macroquad::prelude::*;

const SUDOKU_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 360.,
    h: 360.,
};
pub fn draw_sudoku(state: &AppState) {
    let game = &state.games.sudoku;
    back();
    text("SUDOKU", 100., 20., 19., crate::theme::BRASS);
    panel(SUDOKU_BOARD, accessibility::board_fill(state.high_contrast));
    let cell = 40.;
    for index in 0..81 {
        let row = index / 9;
        let col = index % 9;
        let rect = Rect::new(10. + col as f32 * cell, 28. + row as f32 * cell, cell, cell);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        let value = game.values[index];
        if value > 0 {
            let color = if game.is_given(index) {
                crate::theme::CREAM
            } else {
                crate::theme::BRASS
            };
            text(
                &value.to_string(),
                rect.x + 14.,
                rect.y + 28.,
                accessibility::text_size(22., state.large_text),
                if state.high_contrast { WHITE } else { color },
            );
        }
        if game.selected == Some(index) {
            draw_rectangle_lines(
                rect.x + 2.,
                rect.y + 2.,
                rect.w - 4.,
                rect.h - 4.,
                3.,
                Color::new(0.63, 0.95, 0.72, 1.),
            );
        }
    }
    for boundary in 0..=3 {
        let offset = boundary as f32 * cell * 3.;
        draw_line(
            10. + offset,
            28.,
            10. + offset,
            388.,
            3.5,
            accessibility::grid_line(state.high_contrast),
        );
        draw_line(
            10.,
            28. + offset,
            370.,
            28. + offset,
            3.5,
            accessibility::grid_line(state.high_contrast),
        );
    }
    text(
        match game.status {
            SudokuStatus::Playing => "Select a cell, then tap a number",
            SudokuStatus::Won => "Puzzle complete",
        },
        400.,
        52.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    for (index, difficulty) in SudokuDifficulty::ALL.iter().enumerate() {
        let rect = Rect::new(400. + index as f32 * 120., 72., 112., 44.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(difficulty.label(), rect.x + 17., rect.y + 29., 10., WHITE);
    }
    for value in 1..=9 {
        let index = value - 1;
        let rect = Rect::new(
            400. + (index % 3) as f32 * 62.,
            125. + (index / 3) as f32 * 52.,
            54.,
            42.,
        );
        panel(rect, crate::theme::SURFACE);
        text(&value.to_string(), rect.x + 22., rect.y + 28., 17., WHITE);
    }
    panel(
        Rect::new(590., 280., 110., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 622., 309., 12., WHITE);
    panel(Rect::new(715., 280., 110., 44.), crate::theme::SURFACE);
    text("ERASE", 747., 309., 12., WHITE);
    panel(
        Rect::new(400., 305., 160., 44.),
        if state.sudoku_note_mode {
            Color::new(0.45, 0.20, 0.27, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    text(
        if state.sudoku_note_mode {
            "NOTES ON"
        } else {
            "NOTES OFF"
        },
        445.,
        334.,
        12.,
        WHITE,
    );
}
pub fn sudoku_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if SUDOKU_BOARD.contains(p) {
        let col = ((p.x - 10.) / 40.) as usize;
        let row = ((p.y - 28.) / 40.) as usize;
        if row < 9 && col < 9 {
            return vec![UiAction::SudokuCell(row * 9 + col)];
        }
    }
    for (index, difficulty) in SudokuDifficulty::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(400. + index as f32 * 120., 72., 112., 44.), p) {
            return vec![UiAction::SudokuDifficulty(*difficulty)];
        }
    }
    for value in 1..=9 {
        let index = value - 1;
        if Rect::new(
            400. + (index % 3) as f32 * 62.,
            125. + (index / 3) as f32 * 52.,
            54.,
            42.,
        )
        .contains(p)
        {
            return vec![UiAction::SudokuNumber(value as u8)];
        }
    }
    if crate::ui::hit(Rect::new(590., 280., 110., 44.), p) {
        return vec![UiAction::SudokuUndo];
    }
    if crate::ui::hit(Rect::new(715., 280., 110., 44.), p) {
        return vec![UiAction::SudokuErase];
    }
    if crate::ui::hit(Rect::new(400., 305., 160., 44.), p) {
        return vec![UiAction::SudokuNoteMode];
    }
    let _ = state;
    vec![]
}
