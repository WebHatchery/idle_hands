//! Compact portrait layouts for dense grid puzzles.

use crate::{
    grid::GridLayout,
    nonogram::{NonogramMark, NonogramMode, NonogramStatus},
    state::AppState,
    ui::UiAction,
};
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

const BOARD: Rect = Rect {
    x: 5.,
    y: 145.,
    w: 350.,
    h: 350.,
};

fn grid(state: &AppState) -> GridLayout {
    GridLayout::new(
        Rect::new(45., 185., 300., 300.),
        state.nonogram.size,
        state.nonogram.size,
    )
}

pub fn draw_nonogram(state: &AppState) {
    let game = &state.nonogram;
    text("‹ CABINET", 10., 30., 14., Color::new(0.78, 0.70, 0.92, 1.));
    text("NONOGRAM", 12., 78., 32., Color::new(0.98, 0.83, 0.45, 1.));
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        let rect = Rect::new(10. + index as f32 * 113., 92., 103., 30.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 12., rect.y + 20., 11., WHITE);
    }
    panel(BOARD, Color::new(0.10, 0.07, 0.16, 1.));
    let layout = grid(state);
    for index in 0..game.marks.len() {
        let cell = layout.cell_rect(index).unwrap();
        let rect = Rect::new(cell.x, cell.y, cell.w - 1., cell.h - 1.);
        let selected = game.selected == Some(index);
        let fill = match game.marks[index] {
            NonogramMark::Filled => Color::new(0.80, 0.52, 0.26, 1.),
            NonogramMark::Crossed => Color::new(0.20, 0.14, 0.28, 1.),
            NonogramMark::Empty if selected => Color::new(0.30, 0.22, 0.42, 1.),
            NonogramMark::Empty => Color::new(0.15, 0.11, 0.23, 1.),
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
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
                rect.x + rect.w * 0.28,
                rect.y + rect.h * 0.70,
                (rect.w * 0.7).min(20.),
                Color::new(0.65, 0.58, 0.76, 1.),
            );
        }
    }
    for (index, clue) in game.row_clues.iter().enumerate() {
        let label = clue
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        text(
            &label,
            7.,
            190. + index as f32 * layout.cell_height + layout.cell_height * 0.62,
            (layout.cell_height * 0.30).min(11.),
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    for (index, clue) in game.column_clues.iter().enumerate() {
        let label = clue
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        text(
            &label,
            46. + index as f32 * layout.cell_width,
            178.,
            (layout.cell_width * 0.28).min(10.),
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    text(
        if game.status == NonogramStatus::Won {
            "Picture complete"
        } else {
            "Tap or drag a row / column"
        },
        12.,
        520.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(12., 545., 160., 44.),
        if game.mode == NonogramMode::Fill {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    text(
        if game.mode == NonogramMode::Fill {
            "FILL MODE"
        } else {
            "CROSS MODE"
        },
        42.,
        573.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(188., 545., 160., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 247., 573., 13., WHITE);
    text(
        &format!(
            "Moves {}  •  Best {}",
            game.moves,
            game.best_moves
                .map_or("—".into(), |value| value.to_string())
        ),
        12.,
        625.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

pub fn nonogram_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 110., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        if Rect::new(10. + index as f32 * 113., 92., 103., 30.).contains(p) {
            return vec![UiAction::NonogramPreset(*preset)];
        }
    }
    if Rect::new(12., 545., 160., 44.).contains(p) {
        return vec![UiAction::NonogramMode];
    }
    if Rect::new(188., 545., 160., 44.).contains(p) {
        return vec![UiAction::NonogramUndo];
    }
    grid(state)
        .index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::NonogramCell(index)])
}

pub fn nonogram_drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let layout = grid(state);
    let to_cell = |point: Vec2| layout.coordinate_at(point);
    let (start, end) = match (to_cell(start), to_cell(end)) {
        (Some(start), Some(end)) => (start, end),
        _ => return Vec::new(),
    };
    crate::nonogram::stroke_indices(state.nonogram.size, start, end)
        .into_iter()
        .map(UiAction::NonogramCell)
        .collect()
}
