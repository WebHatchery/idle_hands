//! Nonogram catalog presentation and touch input.

use crate::{
    nonogram::{NonogramMark, NonogramMode, NonogramStatus},
    state::AppState,
    ui::UiAction,
};
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

pub fn draw_nonogram(state: &AppState) {
    let game = &state.nonogram;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("NONOGRAM", 40., 105., 44., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Paint the hidden picture",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        let rect = Rect::new(830. + index as f32 * 115., 95., 105., 34.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 15., rect.y + 22., 14., WHITE);
    }
    let board = Rect::new(320., 175., 500., 500.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
    let cell = (board.w - 20.) / game.size as f32;
    for index in 0..game.marks.len() {
        let rect = Rect::new(
            board.x + 10. + (index % game.size) as f32 * cell,
            board.y + 10. + (index / game.size) as f32 * cell,
            cell - 2.,
            cell - 2.,
        );
        let selected = game.selected == Some(index);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            match game.marks[index] {
                NonogramMark::Filled => Color::new(0.80, 0.52, 0.26, 1.),
                NonogramMark::Crossed => Color::new(0.20, 0.14, 0.28, 1.),
                NonogramMark::Empty => {
                    if selected {
                        Color::new(0.30, 0.22, 0.42, 1.)
                    } else {
                        Color::new(0.15, 0.11, 0.23, 1.)
                    }
                }
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            Color::new(0.48, 0.40, 0.60, 0.8),
        );
        if game.marks[index] == NonogramMark::Crossed {
            text(
                "×",
                rect.x + cell * 0.30,
                rect.y + cell * 0.70,
                (cell * 0.55).min(24.),
                Color::new(0.65, 0.58, 0.76, 1.),
            );
        }
    }
    text(
        if game.status == NonogramStatus::Won {
            "Picture complete"
        } else {
            "Use the clues to mark each square"
        },
        850.,
        185.,
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
        if game.mode == NonogramMode::Fill {
            "FILL MODE"
        } else {
            "CROSS MODE"
        },
        850.,
        245.,
        18.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    panel(
        Rect::new(850., 275., 180., 48.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("TOGGLE MODE", 878., 306., 15., WHITE);
    panel(
        Rect::new(1050., 275., 120., 48.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 1084., 306., 16., WHITE);
    text("Rows", 850., 370., 17., Color::new(0.98, 0.83, 0.45, 1.));
    for (index, clue) in game.row_clues.iter().take(6).enumerate() {
        text(
            &clue
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            850.,
            400. + index as f32 * 26.,
            15.,
            Color::new(0.68, 0.63, 0.78, 1.),
        );
    }
}

pub fn nonogram_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        if Rect::new(830. + index as f32 * 115., 95., 105., 34.).contains(p) {
            return vec![UiAction::NonogramPreset(*preset)];
        }
    }
    if Rect::new(850., 275., 180., 48.).contains(p) {
        return vec![UiAction::NonogramMode];
    }
    if Rect::new(1050., 275., 120., 48.).contains(p) {
        return vec![UiAction::NonogramUndo];
    }
    let board = Rect::new(320., 175., 500., 500.);
    if !board.contains(p) {
        return vec![];
    }
    let cell = (board.w - 20.) / _state.nonogram.size as f32;
    let column = ((p.x - board.x - 10.) / cell) as usize;
    let row = ((p.y - board.y - 10.) / cell) as usize;
    if column < _state.nonogram.size && row < _state.nonogram.size {
        vec![UiAction::NonogramCell(row * _state.nonogram.size + column)]
    } else {
        vec![]
    }
}

pub fn drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let board = Rect::new(320., 175., 500., 500.);
    let cell = (board.w - 20.) / state.nonogram.size as f32;
    let to_cell = |point: Vec2| -> Option<(usize, usize)> {
        if !board.contains(point) {
            return None;
        }
        let x = ((point.x - board.x - 10.) / cell) as usize;
        let y = ((point.y - board.y - 10.) / cell) as usize;
        (x < state.nonogram.size && y < state.nonogram.size).then_some((x, y))
    };
    let (start, end) = match (to_cell(start), to_cell(end)) {
        (Some(start), Some(end)) => (start, end),
        _ => return Vec::new(),
    };
    crate::nonogram::stroke_indices(state.nonogram.size, start, end)
        .into_iter()
        .map(UiAction::NonogramCell)
        .collect()
}
