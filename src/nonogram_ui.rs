//! Nonogram catalog presentation and touch input.

use crate::{
    accessibility,
    grid::GridLayout,
    nonogram::{NonogramMark, NonogramMode, NonogramStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const DESKTOP_BOARD: Rect = Rect {
    x: 320.,
    y: 175.,
    w: 500.,
    h: 500.,
};

fn desktop_grid(size: usize) -> GridLayout {
    GridLayout::new(Rect::new(415., 255., 395., 395.), size, size)
}

fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(s, x, y, crate::ui::readable_text_size(size), color);
}
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

pub fn draw_nonogram(state: &AppState) {
    let game = &state.nonogram;
    text("‹ CABINET", 40., 55., 20., crate::theme::BRASS);
    text("NONOGRAM", 40., 105., 44., crate::theme::BRASS);
    text(
        "Paint the hidden picture",
        44.,
        132.,
        18.,
        crate::theme::SECONDARY,
    );
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        let rect = Rect::new(830. + index as f32 * 115., 95., 105., 34.);
        panel(
            rect,
            if *preset == game.preset {
                crate::theme::LEATHER
            } else {
                crate::theme::GAME_PANEL
            },
        );
        text(preset.label(), rect.x + 15., rect.y + 22., 14., WHITE);
    }
    panel(
        DESKTOP_BOARD,
        accessibility::board_fill(state.high_contrast),
    );
    let grid = desktop_grid(game.size);
    let cell = grid.cell_width;
    for index in 0..game.marks.len() {
        let cell_rect = grid.cell_rect(index).unwrap();
        let rect = Rect::new(cell_rect.x, cell_rect.y, cell_rect.w - 2., cell_rect.h - 2.);
        let selected = game.selected == Some(index);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            match (game.marks[index], selected) {
                (NonogramMark::Empty, true) => crate::theme::WALNUT,
                (mark, _) => accessibility::nonogram_cell(mark as u8, state.high_contrast),
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.marks[index] == NonogramMark::Crossed {
            text(
                "×",
                rect.x + cell * 0.30,
                rect.y + cell * 0.70,
                accessibility::text_size((cell * 0.55).min(24.), state.large_text),
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.65, 0.58, 0.76, 1.)
                },
            );
        }
    }
    let clue_size = accessibility::text_size((cell * 0.34).min(15.), state.large_text);
    let clue_color = if state.high_contrast {
        WHITE
    } else {
        crate::theme::CREAM
    };
    for (row, clue) in game.row_clues.iter().enumerate() {
        let label = clue
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        text(
            &label,
            grid.bounds.x - 78.,
            grid.bounds.y + row as f32 * grid.cell_height + grid.cell_height * 0.68,
            clue_size,
            clue_color,
        );
    }
    let clue_line_height = clue_size + 2.;
    let column_clue_bottom = grid.bounds.y - 10.;
    for (column, clue) in game.column_clues.iter().enumerate() {
        let x = grid.bounds.x + column as f32 * grid.cell_width + grid.cell_width * 0.5
            - clue_size * 0.2;
        for (index, value) in clue.iter().enumerate() {
            let from_bottom = clue.len() - index - 1;
            text(
                &value.to_string(),
                x,
                column_clue_bottom - from_bottom as f32 * clue_line_height,
                clue_size,
                clue_color,
            );
        }
    }
    let instruction = state
        .card_hint
        .as_deref()
        .unwrap_or(if game.status == NonogramStatus::Won {
            "Picture complete"
        } else {
            "Use the clues to mark each square"
        });
    text(
        instruction,
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
    panel(Rect::new(850., 275., 180., 48.), crate::theme::SURFACE);
    text("TOGGLE MODE", 878., 306., 15., WHITE);
    panel(
        Rect::new(1050., 275., 120., 48.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 1084., 306., 16., WHITE);
    panel(Rect::new(850., 335., 320., 28.), crate::theme::SURFACE);
    text("HINT", 995., 355., 14., WHITE);
}

pub fn nonogram_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
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
    if Rect::new(850., 335., 320., 28.).contains(p) {
        return vec![UiAction::NonogramHint];
    }
    let grid = desktop_grid(state.nonogram.size);
    grid.index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::NonogramCell(index)])
}

pub fn drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let grid = desktop_grid(state.nonogram.size);
    let to_cell = |point: Vec2| -> Option<(usize, usize)> { grid.coordinate_at(point) };
    let (start, end) = match (to_cell(start), to_cell(end)) {
        (Some(start), Some(end)) => (start, end),
        _ => return Vec::new(),
    };
    crate::nonogram::stroke_indices(state.nonogram.size, start, end)
        .into_iter()
        .map(UiAction::NonogramCell)
        .collect()
}
