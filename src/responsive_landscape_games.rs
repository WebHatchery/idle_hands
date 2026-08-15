//! Medium landscape layouts for the dense game boards.

use crate::{
    accessibility,
    grid::GridLayout,
    minesweeper::{Cell, MinePreset, MineStatus},
    nonogram::{NonogramMark, NonogramMode, NonogramPreset, NonogramStatus},
    reversi::{AiLevel, ReversiStatus},
    state::AppState,
    sudoku::{SudokuDifficulty, SudokuStatus},
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
fn back() {
    text("< CABINET", 10., 18., 12., Color::new(0.78, 0.70, 0.92, 1.));
}

const MINE_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 350.,
    h: 350.,
};
fn mine_grid(state: &AppState) -> GridLayout {
    GridLayout::new(
        Rect::new(15., 33., 340., 340.),
        state.minesweeper.width,
        state.minesweeper.height,
    )
}

pub fn draw_minesweeper(state: &AppState) {
    let game = &state.minesweeper;
    back();
    text(
        "MINESWEEPER",
        100.,
        20.,
        19.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(MINE_BOARD, accessibility::board_fill(state.high_contrast));
    let layout = mine_grid(state);
    for index in 0..game.cells.len() {
        let cell_rect = layout.cell_rect(index).unwrap();
        let rect = Rect::new(cell_rect.x, cell_rect.y, cell_rect.w - 1., cell_rect.h - 1.);
        let cell = game.cells[index];
        let revealed = matches!(cell, Cell::Revealed(value) if value < 9)
            || matches!(game.status, MineStatus::Lost)
                && matches!(cell, Cell::Mine | Cell::FlaggedMine | Cell::Revealed(9));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            accessibility::mine_cell(revealed, state.high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        match cell {
            Cell::Flagged | Cell::FlaggedMine => text(
                "F",
                rect.x + rect.w * 0.35,
                rect.y + rect.h * 0.72,
                (rect.w * 0.55).min(18.),
                Color::new(0.98, 0.46, 0.38, 1.),
            ),
            Cell::Mine if matches!(game.status, MineStatus::Lost) => text(
                "*",
                rect.x + rect.w * 0.30,
                rect.y + rect.h * 0.72,
                (rect.w * 0.55).min(18.),
                Color::new(0.98, 0.45, 0.32, 1.),
            ),
            Cell::Revealed(value) if value > 0 && value < 9 => text(
                &value.to_string(),
                rect.x + rect.w * 0.35,
                rect.y + rect.h * 0.72,
                (rect.w * 0.6).min(18.),
                Color::new(0.76, 0.90, 1., 1.),
            ),
            _ => {}
        }
    }
    text(
        &format!(
            "Mines {} / {}   {:03}s",
            game.flagged_count(),
            game.mines,
            game.elapsed_whole_seconds()
        ),
        400.,
        55.,
        15.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    text(
        match game.status {
            MineStatus::Ready => "First reveal is safe",
            MineStatus::Playing => "Find every safe square",
            MineStatus::Won => "Field cleared",
            MineStatus::Lost => "A mine was found",
        },
        400.,
        82.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        let rect = Rect::new(400. + index as f32 * 105., 105., 98., 32.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 8., rect.y + 21., 10., WHITE);
    }
    panel(
        Rect::new(400., 155., 170., 44.),
        if state.mine_flag_mode {
            Color::new(0.45, 0.20, 0.27, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    text(
        if state.mine_flag_mode {
            "FLAG MODE"
        } else {
            "REVEAL MODE"
        },
        445.,
        183.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(590., 155., 170., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RESTART", 645., 183., 13., WHITE);
    text(
        "Tap reveal; hold to flag.",
        400.,
        240.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}
pub fn minesweeper_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 90., 28.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        if Rect::new(400. + index as f32 * 105., 105., 98., 32.).contains(p) {
            return vec![UiAction::MinePreset(*preset)];
        }
    }
    if Rect::new(400., 155., 170., 44.).contains(p) {
        return vec![UiAction::MineFlagMode];
    }
    if Rect::new(590., 155., 170., 44.).contains(p) {
        return vec![UiAction::MineRestart];
    }
    if let Some(index) = mine_grid(state).index_at(p) {
        if state.mine_flag_mode {
            vec![UiAction::MineFlag(index)]
        } else if matches!(state.minesweeper.cells[index], Cell::Revealed(_)) {
            vec![UiAction::MineChord(index)]
        } else {
            vec![UiAction::MineReveal(index)]
        }
    } else {
        vec![]
    }
}

const SUDOKU_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 360.,
    h: 360.,
};
pub fn draw_sudoku(state: &AppState) {
    let game = &state.sudoku;
    back();
    text("SUDOKU", 100., 20., 19., Color::new(0.98, 0.83, 0.45, 1.));
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
            if row % 3 == 0 || col % 3 == 0 { 2. } else { 1. },
            accessibility::grid_line(state.high_contrast),
        );
        let value = game.values[index];
        if value > 0 {
            let color = if game.is_given(index) {
                Color::new(0.78, 0.73, 0.86, 1.)
            } else {
                Color::new(0.98, 0.83, 0.45, 1.)
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
        let rect = Rect::new(400. + index as f32 * 120., 72., 112., 32.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(difficulty.label(), rect.x + 17., rect.y + 21., 10., WHITE);
    }
    for value in 1..=9 {
        let index = value - 1;
        let rect = Rect::new(
            400. + (index % 3) as f32 * 62.,
            125. + (index / 3) as f32 * 52.,
            54.,
            42.,
        );
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(&value.to_string(), rect.x + 22., rect.y + 28., 17., WHITE);
    }
    panel(
        Rect::new(590., 280., 110., 42.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 622., 307., 12., WHITE);
    panel(
        Rect::new(715., 280., 110., 42.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("ERASE", 747., 307., 12., WHITE);
    panel(
        Rect::new(400., 305., 160., 42.),
        if state.sudoku_note_mode {
            Color::new(0.45, 0.20, 0.27, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    text(
        if state.sudoku_note_mode {
            "NOTES ON"
        } else {
            "NOTES OFF"
        },
        445.,
        332.,
        12.,
        WHITE,
    );
}
pub fn sudoku_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 90., 28.).contains(p) {
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
        if Rect::new(400. + index as f32 * 120., 72., 112., 32.).contains(p) {
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
    if Rect::new(590., 280., 110., 42.).contains(p) {
        return vec![UiAction::SudokuUndo];
    }
    if Rect::new(715., 280., 110., 42.).contains(p) {
        return vec![UiAction::SudokuErase];
    }
    if Rect::new(400., 305., 160., 42.).contains(p) {
        return vec![UiAction::SudokuNoteMode];
    }
    let _ = state;
    vec![]
}

const NONO_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 360.,
    h: 360.,
};
pub fn draw_nonogram(state: &AppState) {
    let game = &state.nonogram;
    back();
    text("NONOGRAM", 100., 20., 19., Color::new(0.98, 0.83, 0.45, 1.));
    panel(NONO_BOARD, accessibility::board_fill(state.high_contrast));
    let layout = nonogram_grid(state);
    let visible = crate::nonogram::visible_size(game.size, state.nonogram_zoomed);
    let (_, origin_y) = nonogram_origin(state);
    for local in 0..visible * visible {
        let index = nonogram_global_index(state, local);
        let cell = layout.cell_rect(local).unwrap();
        let fill = accessibility::nonogram_cell(game.marks[index] as u8, state.high_contrast);
        draw_rectangle(cell.x, cell.y, cell.w - 1., cell.h - 1., fill);
        draw_rectangle_lines(
            cell.x,
            cell.y,
            cell.w - 1.,
            cell.h - 1.,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.marks[index] == NonogramMark::Crossed {
            text(
                "x",
                cell.x + cell.w * 0.35,
                cell.y + cell.h * 0.68,
                (cell.w * 0.55).min(16.),
                Color::new(0.65, 0.58, 0.76, 1.),
            );
        }
    }
    for (local, clue) in game
        .row_clues
        .iter()
        .skip(origin_y)
        .take(visible)
        .enumerate()
    {
        text(
            &clue
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            15.,
            105. + local as f32 * layout.cell_height,
            9.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    text(
        if game.status == NonogramStatus::Won {
            "Picture complete"
        } else if state.nonogram_zoomed && game.size > visible {
            "Zoomed 9 × 9 focus"
        } else {
            "Fill or cross from the clues"
        },
        400.,
        50.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    for (index, preset) in NonogramPreset::ALL.iter().enumerate() {
        let rect = Rect::new(400. + index as f32 * 120., 75., 112., 32.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 14., rect.y + 21., 10., WHITE);
    }
    panel(
        Rect::new(400., 135., 160., 44.),
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
        445.,
        163.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(590., 135., 160., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 650., 163., 12., WHITE);
    panel(
        Rect::new(400., 190., 160., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text(
        if state.nonogram_zoomed && game.size > visible {
            "FULL BOARD"
        } else {
            "ZOOM 9 × 9"
        },
        445.,
        218.,
        11.,
        WHITE,
    );
    for (rect, label) in [
        (Rect::new(590., 190., 50., 44.), "LEFT"),
        (Rect::new(646., 190., 50., 44.), "RIGHT"),
        (Rect::new(702., 190., 50., 44.), "UP"),
        (Rect::new(758., 190., 50., 44.), "DOWN"),
    ] {
        panel(rect, Color::new(0.18, 0.26, 0.34, 1.));
        text(label, rect.x + 5., 217., 9., WHITE);
    }
    text(
        &format!("Moves {}", game.moves),
        400.,
        260.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

fn nonogram_grid(state: &AppState) -> GridLayout {
    let visible = crate::nonogram::visible_size(state.nonogram.size, state.nonogram_zoomed);
    GridLayout::new(Rect::new(70., 88., 280., 280.), visible, visible)
}

fn nonogram_origin(state: &AppState) -> (usize, usize) {
    crate::nonogram::focus_origin(
        state.nonogram.size,
        state.nonogram_zoomed,
        state.nonogram_focus,
    )
}

fn nonogram_global_index(state: &AppState, local: usize) -> usize {
    let visible = crate::nonogram::visible_size(state.nonogram.size, state.nonogram_zoomed);
    let (origin_x, origin_y) = nonogram_origin(state);
    origin_y * state.nonogram.size
        + origin_x
        + (local / visible) * state.nonogram.size
        + local % visible
}

pub fn nonogram_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 90., 28.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in NonogramPreset::ALL.iter().enumerate() {
        if Rect::new(400. + index as f32 * 120., 75., 112., 32.).contains(p) {
            return vec![UiAction::NonogramPreset(*preset)];
        }
    }
    if Rect::new(400., 135., 160., 44.).contains(p) {
        return vec![UiAction::NonogramMode];
    }
    if Rect::new(590., 135., 160., 44.).contains(p) {
        return vec![UiAction::NonogramUndo];
    }
    if Rect::new(400., 190., 160., 44.).contains(p) {
        return vec![UiAction::NonogramZoom];
    }
    for (rect, delta) in [
        (Rect::new(590., 190., 50., 44.), (-1, 0)),
        (Rect::new(646., 190., 50., 44.), (1, 0)),
        (Rect::new(702., 190., 50., 44.), (0, -1)),
        (Rect::new(758., 190., 50., 44.), (0, 1)),
    ] {
        if rect.contains(p) {
            return vec![UiAction::NonogramPan(delta.0, delta.1)];
        }
    }
    if let Some(local) = nonogram_grid(state).index_at(p) {
        return vec![UiAction::NonogramCell(nonogram_global_index(state, local))];
    }
    vec![]
}

pub fn nonogram_drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let layout = nonogram_grid(state);
    let (Some(start), Some(end)) = (layout.coordinate_at(start), layout.coordinate_at(end)) else {
        return vec![];
    };
    let (origin_x, origin_y) = nonogram_origin(state);
    crate::nonogram::stroke_indices(
        state.nonogram.size,
        (start.0 + origin_x, start.1 + origin_y),
        (end.0 + origin_x, end.1 + origin_y),
    )
    .into_iter()
    .map(UiAction::NonogramCell)
    .collect()
}

pub fn minesweeper_long_press(state: &AppState, p: Vec2) -> Vec<UiAction> {
    mine_grid(state)
        .index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::MineFlag(index)])
}

const REV_BOARD: Rect = Rect {
    x: 10.,
    y: 20.,
    w: 350.,
    h: 350.,
};
pub fn draw_reversi(state: &AppState) {
    let game = &state.reversi;
    back();
    text(
        "REVERSI",
        100.,
        20.,
        accessibility::text_size(19., state.large_text),
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(
        REV_BOARD,
        if state.high_contrast {
            Color::new(0.02, 0.20, 0.16, 1.)
        } else {
            Color::new(0.10, 0.30, 0.24, 1.)
        },
    );
    let cell = REV_BOARD.w / 8.;
    let legal = if game.status == ReversiStatus::Playing {
        game.legal_moves(game.turn)
    } else {
        Vec::new()
    };
    for index in 0..64 {
        let row = index / 8;
        let col = index % 8;
        let rect = Rect::new(
            REV_BOARD.x + col as f32 * cell,
            REV_BOARD.y + row as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.board[index] == 0 && legal.contains(&index) {
            draw_circle(
                rect.center().x,
                rect.center().y,
                5.,
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.72, 0.95, 0.72, 0.75)
                },
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    accessibility::board_fill(state.high_contrast)
                } else {
                    if state.high_contrast {
                        WHITE
                    } else {
                        Color::new(0.92, 0.85, 0.66, 1.)
                    }
                },
            );
        }
    }
    text(
        &format!("DARK {}  •  LIGHT {}", game.score(1), game.score(2)),
        400.,
        55.,
        accessibility::text_size(15., state.large_text),
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        match game.status {
            ReversiStatus::Playing if game.turn == 1 => "Your turn: glowing square",
            ReversiStatus::Playing => "Opponent is thinking",
            ReversiStatus::Won => "Board complete",
        },
        400.,
        82.,
        accessibility::text_size(14., state.large_text),
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(400., 120., 160., 44.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text(
        "PASS TURN",
        450.,
        148.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(
        Rect::new(590., 120., 160., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text(
        "NEW BOARD",
        635.,
        148.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    for (index, label) in [
        ("GENTLE", AiLevel::Gentle),
        ("SHARP", AiLevel::Sharp),
        ("2 PLAYER", AiLevel::TwoPlayer),
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(400. + index as f32 * 120., 190., 112., 38.);
        panel(
            rect,
            if game.ai_level == label.1 {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.18, 0.12, 0.28, 1.)
            },
        );
        text(
            label.0,
            rect.x + 25.,
            rect.y + 25.,
            accessibility::text_size(10., state.large_text),
            WHITE,
        );
    }
}
pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 90., 28.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(400., 120., 160., 44.).contains(p) {
        return vec![UiAction::ReversiPass];
    }
    if Rect::new(590., 120., 160., 44.).contains(p) {
        return vec![UiAction::ReversiNew];
    }
    for (index, level) in [AiLevel::Gentle, AiLevel::Sharp, AiLevel::TwoPlayer]
        .iter()
        .enumerate()
    {
        if Rect::new(400. + index as f32 * 120., 190., 112., 38.).contains(p) {
            return vec![UiAction::ReversiLevel(*level)];
        }
    }
    if !REV_BOARD.contains(p) {
        return vec![];
    }
    let cell = REV_BOARD.w / 8.;
    let col = ((p.x - REV_BOARD.x) / cell) as usize;
    let row = ((p.y - REV_BOARD.y) / cell) as usize;
    if row < 8 && col < 8 {
        vec![UiAction::ReversiPlace(row * 8 + col)]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests;
