//! Magnified touch routes for dense Sudoku and Nonogram cells.

use crate::{
    accessibility,
    nonogram::NonogramMark,
    state::{AppState, GameId, Screen},
    ui::UiAction,
};
use macroquad::prelude::*;

pub fn entry_action(state: &AppState, point: Vec2) -> Option<UiAction> {
    let Screen::Game(game) = state.screen else {
        return None;
    };
    match game {
        GameId::Sudoku if crate::ui::hit(sudoku_entry_rect(), point) => Some(UiAction::SudokuFocus),
        GameId::Nonogram if crate::ui::hit(nonogram_entry_rect(), point) => {
            Some(UiAction::NonogramFocus)
        }
        _ => None,
    }
}

pub fn is_open(state: &AppState) -> bool {
    matches!(state.screen, Screen::Game(GameId::Sudoku)) && state.sudoku_focus_open
        || matches!(state.screen, Screen::Game(GameId::Nonogram)) && state.nonogram_focus_open
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    match state.screen {
        Screen::Game(GameId::Sudoku) if state.sudoku_focus_open => {
            sudoku_focus_clicks(state, point)
        }
        Screen::Game(GameId::Nonogram) if state.nonogram_focus_open => {
            nonogram_focus_clicks(state, point)
        }
        _ => Vec::new(),
    }
}

pub fn draw_entry_controls(state: &AppState) {
    let Screen::Game(game) = state.screen else {
        return;
    };
    match game {
        GameId::Sudoku => focus_button(
            sudoku_entry_rect(),
            "FOCUS CELLS",
            crate::theme::SURFACE,
            state,
        ),
        GameId::Nonogram => focus_button(
            nonogram_entry_rect(),
            "FOCUS CELL",
            crate::theme::SURFACE,
            state,
        ),
        _ => {}
    }
}

pub fn draw(state: &AppState) {
    match state.screen {
        Screen::Game(GameId::Sudoku) if state.sudoku_focus_open => draw_sudoku_focus(state),
        Screen::Game(GameId::Nonogram) if state.nonogram_focus_open => draw_nonogram_focus(state),
        _ => {}
    }
}

pub fn sudoku_entry_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(12., 88., 160., 60.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(590., 125., 160., 66.)
    } else {
        Rect::new(850., 105., 180., 48.)
    }
}

pub fn nonogram_entry_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(10., 80., 160., 60.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(400., 75., 160., 66.)
    } else {
        Rect::new(850., 95., 180., 48.)
    }
}

fn focus_panel() -> Rect {
    let (width, height) = crate::ui::layout_size();
    if crate::ui::is_portrait() {
        Rect::new(8., 48., width - 16., (height - 56.).min(724.))
    } else if crate::ui::is_compact_landscape() {
        Rect::new(220., 8., width - 232., height - 16.)
    } else {
        Rect::new(250., 70., width - 300., height - 90.)
    }
}

fn focus_grid_rect(panel: Rect) -> Rect {
    let cell = focus_cell_size();
    if crate::ui::is_portrait() {
        Rect::new(
            panel.x + (panel.w - cell * 3.) * 0.5,
            panel.y + 100.,
            cell * 3.,
            cell * 3.,
        )
    } else {
        Rect::new(panel.x + 28., panel.y + 92., cell * 3., cell * 3.)
    }
}

fn focus_cell_size() -> f32 {
    if crate::ui::is_portrait() {
        60.
    } else if crate::ui::is_compact_landscape() {
        64.
    } else {
        72.
    }
}

fn number_pad_rect(panel: Rect, index: usize) -> Rect {
    let gap = 8.;
    let size = if crate::ui::is_portrait() { 100. } else { 56. };
    let y = if crate::ui::is_portrait() {
        panel.y + 310. + (index / 3) as f32 * (58. + gap)
    } else {
        panel.y + 92. + (index / 3) as f32 * (size + gap)
    };
    let x = if crate::ui::is_portrait() {
        panel.x + 20. + (index % 3) as f32 * (size + gap)
    } else {
        panel.x + 270. + (index % 3) as f32 * (size + gap)
    };
    Rect::new(x, y, size, if crate::ui::is_portrait() { 58. } else { 56. })
}

fn focus_control_rect(panel: Rect, index: usize) -> Rect {
    if crate::ui::is_portrait() {
        let width = (panel.w - 52.) * 0.5;
        Rect::new(
            panel.x + 20. + (index % 2) as f32 * (width + 12.),
            panel.y + 510. + (index / 2) as f32 * 64.,
            width,
            56.,
        )
    } else {
        let width = (panel.w - 92.) * 0.25;
        Rect::new(
            panel.x + 28. + index as f32 * (width + 12.),
            panel.bottom() - 62.,
            width,
            52.,
        )
    }
}

fn draw_sudoku_focus(state: &AppState) {
    let panel = focus_panel();
    draw_focus_backdrop(panel);
    focus_heading(
        panel,
        "SUDOKU FOCUS",
        "Choose a neighboring cell at a larger size",
        state,
    );
    let selected = state.games.sudoku.selected.unwrap_or(40);
    let grid = focus_grid_rect(panel);
    for local in 0..9 {
        let row = local / 3;
        let col = local % 3;
        let source = neighbor_index(
            selected / 9,
            selected % 9,
            row as isize - 1,
            col as isize - 1,
            9,
        );
        let rect = Rect::new(
            grid.x + col as f32 * grid.w / 3.,
            grid.y + row as f32 * grid.h / 3.,
            grid.w / 3. - 2.,
            grid.h / 3. - 2.,
        );
        let conflict = state
            .games
            .sudoku
            .selected
            .map(|index| state.games.sudoku.conflicts(index).contains(&source))
            .unwrap_or(false);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if conflict {
                Color::new(0.35, 0.13, 0.20, 1.)
            } else if source == selected {
                crate::theme::WALNUT
            } else {
                crate::theme::GAME_PANEL
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.,
            accessibility::grid_line(state.high_contrast),
        );
        let value = state.games.sudoku.values[source];
        if value != 0 {
            centered_text(&value.to_string(), rect, 28., WHITE, state);
        } else if state.games.sudoku.notes[source] != 0 {
            centered_text("· · ·", rect, 13., crate::theme::SECONDARY, state);
        }
    }
    for index in 1..=9 {
        let rect = number_pad_rect(panel, index - 1);
        focus_button(rect, &index.to_string(), crate::theme::SURFACE, state);
    }
    for (index, label) in ["PENCIL", "ERASE", "UNDO", "DONE"].iter().enumerate() {
        focus_button(
            focus_control_rect(panel, index),
            label,
            if index == 3 {
                crate::theme::SURFACE_DARK
            } else {
                crate::theme::SURFACE
            },
            state,
        );
    }
}

fn draw_nonogram_focus(state: &AppState) {
    let panel = focus_panel();
    draw_focus_backdrop(panel);
    focus_heading(
        panel,
        "NONOGRAM FOCUS",
        "Mark a neighboring cell at a larger size",
        state,
    );
    let size = state.games.nonogram.size;
    let selected = state
        .games
        .nonogram
        .selected
        .unwrap_or((size / 2) * size + size / 2);
    let grid = focus_grid_rect(panel);
    for local in 0..9 {
        let row = local / 3;
        let col = local % 3;
        let source = neighbor_index(
            selected / size,
            selected % size,
            row as isize - 1,
            col as isize - 1,
            size,
        );
        let rect = Rect::new(
            grid.x + col as f32 * grid.w / 3.,
            grid.y + row as f32 * grid.h / 3.,
            grid.w / 3. - 2.,
            grid.h / 3. - 2.,
        );
        let selected_cell = source == selected;
        let fill = if selected_cell {
            crate::theme::WALNUT
        } else {
            accessibility::nonogram_cell(
                state.games.nonogram.marks[source] as u8,
                state.high_contrast,
            )
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.,
            accessibility::grid_line(state.high_contrast),
        );
        if state.games.nonogram.marks[source] == NonogramMark::Crossed {
            centered_text("×", rect, 27., crate::theme::SECONDARY, state);
        }
    }
    for (index, label) in ["LEFT", "RIGHT", "UP", "DOWN"].iter().enumerate() {
        focus_button(
            focus_pan_rect(panel, index),
            label,
            crate::theme::SURFACE,
            state,
        );
    }
    let controls = [
        if state.games.nonogram.mode == crate::nonogram::NonogramMode::Fill {
            "FILL"
        } else {
            "CROSS"
        },
        "HINT",
        "UNDO",
        "DONE",
    ];
    for (index, label) in controls.iter().enumerate() {
        focus_button(
            focus_control_rect(panel, index),
            label,
            if index == 3 {
                crate::theme::SURFACE_DARK
            } else {
                crate::theme::SURFACE
            },
            state,
        );
    }
}

fn sudoku_focus_clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let panel = focus_panel();
    if crate::ui::hit(focus_control_rect(panel, 3), point) {
        return vec![UiAction::SudokuFocus];
    }
    let selected = state.games.sudoku.selected.unwrap_or(40);
    let grid = focus_grid_rect(panel);
    if let Some(local) = focus_cell_at(grid, point) {
        let row = local / 3;
        let col = local % 3;
        return vec![UiAction::SudokuCell(neighbor_index(
            selected / 9,
            selected % 9,
            row as isize - 1,
            col as isize - 1,
            9,
        ))];
    }
    for index in 0..9 {
        if crate::ui::hit(number_pad_rect(panel, index), point) {
            return vec![UiAction::SudokuNumber((index + 1) as u8)];
        }
    }
    match focus_control_index(panel, point) {
        Some(0) => vec![UiAction::SudokuNoteMode],
        Some(1) => vec![UiAction::SudokuErase],
        Some(2) => vec![UiAction::SudokuUndo],
        _ => vec![],
    }
}

fn nonogram_focus_clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let panel = focus_panel();
    if crate::ui::hit(focus_control_rect(panel, 3), point) {
        return vec![UiAction::NonogramFocus];
    }
    let size = state.games.nonogram.size;
    let selected = state
        .games
        .nonogram
        .selected
        .unwrap_or((size / 2) * size + size / 2);
    for (index, delta) in [(0, (0, -1)), (1, (0, 1)), (2, (-1, 0)), (3, (1, 0))] {
        if crate::ui::hit(focus_pan_rect(panel, index), point) {
            return vec![UiAction::NonogramFocusMove(delta.0, delta.1)];
        }
    }
    if let Some(local) = focus_cell_at(focus_grid_rect(panel), point) {
        return vec![UiAction::NonogramCell(neighbor_index(
            selected / size,
            selected % size,
            (local / 3) as isize - 1,
            (local % 3) as isize - 1,
            size,
        ))];
    }
    match focus_control_index(panel, point) {
        Some(0) => vec![UiAction::NonogramMode],
        Some(1) => vec![UiAction::NonogramHint],
        Some(2) => vec![UiAction::NonogramUndo],
        _ => vec![],
    }
}

fn focus_pan_rect(panel: Rect, index: usize) -> Rect {
    let grid = focus_grid_rect(panel);
    let gap = 8.;
    let (x, y, width) = if crate::ui::is_portrait() {
        (panel.x + 20., panel.y + 350., panel.w - 40.)
    } else {
        (grid.x, grid.bottom() + 12., grid.w)
    };
    let button_width = (width - gap * 3.) / 4.;
    Rect::new(
        x + index as f32 * (button_width + gap),
        y,
        button_width,
        if crate::ui::is_portrait() { 42. } else { 44. },
    )
}

fn focus_cell_at(rect: Rect, point: Vec2) -> Option<usize> {
    if !rect.contains(point) {
        return None;
    }
    let cell = rect.w / 3.;
    let col = ((point.x - rect.x) / cell).floor() as usize;
    let row = ((point.y - rect.y) / cell).floor() as usize;
    (row < 3 && col < 3).then_some(row * 3 + col)
}

fn focus_control_index(panel: Rect, point: Vec2) -> Option<usize> {
    (0..4).find(|index| crate::ui::hit(focus_control_rect(panel, *index), point))
}

fn neighbor_index(
    row: usize,
    col: usize,
    delta_row: isize,
    delta_col: isize,
    size: usize,
) -> usize {
    let row = (row as isize + delta_row).clamp(0, size as isize - 1) as usize;
    let col = (col as isize + delta_col).clamp(0, size as isize - 1) as usize;
    row * size + col
}

fn draw_focus_backdrop(panel: Rect) {
    let (width, height) = crate::ui::layout_size();
    draw_rectangle(0., 0., width, height, Color::new(0.02, 0.01, 0.04, 0.78));
    crate::ui::draw_rounded_panel(panel, 10., crate::theme::SURFACE_DARK, crate::theme::BRASS);
}

fn focus_heading(panel: Rect, title: &str, detail: &str, state: &AppState) {
    crate::ui::draw_text(
        title,
        panel.x + 20.,
        panel.y + 38.,
        accessibility::text_size(20., state.large_text),
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
    );
    crate::ui::draw_text(
        detail,
        panel.x + 20.,
        panel.y + 65.,
        accessibility::text_size(13., state.large_text),
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::SECONDARY
        },
    );
}

fn focus_button(rect: Rect, label: &str, fill: Color, state: &AppState) {
    crate::ui::draw_rounded_panel(
        rect,
        6.,
        fill,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
    let size = accessibility::text_size(13., state.large_text);
    let width = crate::ui::measure_text(label, None, size.round() as u16, 1.).width;
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}

fn centered_text(value: &str, rect: Rect, size: f32, color: Color, state: &AppState) {
    let size = accessibility::text_size(size, state.large_text);
    let width = crate::ui::measure_text(value, None, size.round() as u16, 1.).width;
    crate::ui::draw_text(
        value,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        if state.high_contrast { WHITE } else { color },
    );
}
