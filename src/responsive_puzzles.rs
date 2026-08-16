//! Compact portrait layouts for dense grid puzzles.

use crate::{
    accessibility,
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
    let visible = crate::nonogram::visible_size(state.nonogram.size, state.nonogram_zoomed);
    GridLayout::new(Rect::new(45., 185., 300., 300.), visible, visible)
}

fn origin(state: &AppState) -> (usize, usize) {
    crate::nonogram::focus_origin(
        state.nonogram.size,
        state.nonogram_zoomed,
        state.nonogram_focus,
    )
}

fn global_index(state: &AppState, local: usize) -> usize {
    let visible = crate::nonogram::visible_size(state.nonogram.size, state.nonogram_zoomed);
    let (origin_x, origin_y) = origin(state);
    origin_y * state.nonogram.size
        + origin_x
        + (local / visible) * state.nonogram.size
        + local % visible
}

pub fn draw_nonogram(state: &AppState) {
    let game = &state.nonogram;
    panel(
        Rect::new(0., 0., 110., 44.),
        Color::new(0.12, 0.08, 0.20, 1.),
    );
    text("‹ CABINET", 10., 29., 14., Color::new(0.78, 0.70, 0.92, 1.));
    text("NONOGRAM", 12., 78., 32., Color::new(0.98, 0.83, 0.45, 1.));
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        let rect = Rect::new(10. + index as f32 * 113., 92., 103., 44.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 12., rect.y + 29., 11., WHITE);
    }
    panel(BOARD, accessibility::board_fill(state.high_contrast));
    let layout = grid(state);
    let visible = crate::nonogram::visible_size(game.size, state.nonogram_zoomed);
    for local in 0..visible * visible {
        let index = global_index(state, local);
        let cell = layout.cell_rect(local).unwrap();
        let rect = Rect::new(cell.x, cell.y, cell.w - 1., cell.h - 1.);
        let selected = game.selected == Some(index);
        let fill = match (game.marks[index], selected) {
            (NonogramMark::Empty, true) => Color::new(0.30, 0.22, 0.42, 1.),
            (mark, _) => accessibility::nonogram_cell(mark as u8, state.high_contrast),
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
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
                rect.x + rect.w * 0.28,
                rect.y + rect.h * 0.70,
                accessibility::text_size((rect.w * 0.7).min(20.), state.large_text),
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.65, 0.58, 0.76, 1.)
                },
            );
        }
    }
    let (origin_x, origin_y) = origin(state);
    for (local, clue) in game
        .row_clues
        .iter()
        .skip(origin_y)
        .take(visible)
        .enumerate()
    {
        let label = clue
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        text(
            &label,
            7.,
            190. + local as f32 * layout.cell_height + layout.cell_height * 0.62,
            accessibility::text_size((layout.cell_height * 0.30).min(11.), state.large_text),
            if state.high_contrast {
                WHITE
            } else {
                Color::new(0.78, 0.73, 0.86, 1.)
            },
        );
    }
    for (local, clue) in game
        .column_clues
        .iter()
        .skip(origin_x)
        .take(visible)
        .enumerate()
    {
        let label = clue
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        text(
            &label,
            46. + local as f32 * layout.cell_width,
            178.,
            accessibility::text_size((layout.cell_width * 0.28).min(10.), state.large_text),
            if state.high_contrast {
                WHITE
            } else {
                Color::new(0.78, 0.73, 0.86, 1.)
            },
        );
    }
    text(
        if game.status == NonogramStatus::Won {
            "Picture complete"
        } else if state.nonogram_zoomed && game.size > visible {
            "Zoomed 9 × 9 focus"
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
    panel(
        Rect::new(12., 595., 104., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text(
        if state.nonogram_zoomed && game.size > visible {
            "FULL BOARD"
        } else {
            "ZOOM 9 × 9"
        },
        23.,
        623.,
        11.,
        WHITE,
    );
    for (rect, label) in [
        (Rect::new(128., 595., 50., 44.), "LEFT"),
        (Rect::new(184., 595., 50., 44.), "RIGHT"),
        (Rect::new(240., 595., 50., 44.), "UP"),
        (Rect::new(296., 595., 50., 44.), "DOWN"),
    ] {
        panel(rect, Color::new(0.18, 0.26, 0.34, 1.));
        text(label, rect.x + 5., 622., 9., WHITE);
    }
    text(
        &format!(
            "Moves {}  •  Best {}",
            game.moves,
            game.best_moves
                .map_or("—".into(), |value| value.to_string())
        ),
        12.,
        670.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

pub fn nonogram_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in crate::nonogram::NonogramPreset::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(10. + index as f32 * 113., 92., 103., 44.), p) {
            return vec![UiAction::NonogramPreset(*preset)];
        }
    }
    if crate::ui::hit(Rect::new(12., 545., 160., 44.), p) {
        return vec![UiAction::NonogramMode];
    }
    if crate::ui::hit(Rect::new(188., 545., 160., 44.), p) {
        return vec![UiAction::NonogramUndo];
    }
    if crate::ui::hit(Rect::new(12., 595., 104., 44.), p) {
        return vec![UiAction::NonogramZoom];
    }
    for (rect, delta) in [
        (Rect::new(128., 595., 50., 44.), (-1, 0)),
        (Rect::new(184., 595., 50., 44.), (1, 0)),
        (Rect::new(240., 595., 50., 44.), (0, -1)),
        (Rect::new(296., 595., 50., 44.), (0, 1)),
    ] {
        if rect.contains(p) {
            return vec![UiAction::NonogramPan(delta.0, delta.1)];
        }
    }
    grid(state).index_at(p).map_or_else(Vec::new, |local| {
        vec![UiAction::NonogramCell(global_index(state, local))]
    })
}

pub fn nonogram_drag_actions(state: &AppState, start: Vec2, end: Vec2) -> Vec<UiAction> {
    let layout = grid(state);
    let to_cell = |point: Vec2| layout.coordinate_at(point);
    let (start, end) = match (to_cell(start), to_cell(end)) {
        (Some(start), Some(end)) => (start, end),
        _ => return Vec::new(),
    };
    let (origin_x, origin_y) = origin(state);
    crate::nonogram::stroke_indices(
        state.nonogram.size,
        (start.0 + origin_x, start.1 + origin_y),
        (end.0 + origin_x, end.1 + origin_y),
    )
    .into_iter()
    .map(UiAction::NonogramCell)
    .collect()
}

const MINE_BOARD: Rect = Rect {
    x: 5.,
    y: 155.,
    w: 350.,
    h: 350.,
};

fn mine_grid(state: &AppState) -> GridLayout {
    GridLayout::new(
        Rect::new(MINE_BOARD.x + 5., MINE_BOARD.y + 5., 340., 340.),
        state.minesweeper.width,
        state.minesweeper.height,
    )
}

pub fn draw_minesweeper(state: &AppState) {
    let game = &state.minesweeper;
    panel(
        Rect::new(0., 0., 110., 44.),
        Color::new(0.12, 0.08, 0.20, 1.),
    );
    text("‹ CABINET", 10., 29., 14., Color::new(0.78, 0.70, 0.92, 1.));
    text(
        "MINESWEEPER",
        12.,
        72.,
        29.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (index, preset) in crate::minesweeper::MinePreset::ALL.iter().enumerate() {
        let rect = Rect::new(5. + index as f32 * 88., 88., 82., 44.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 7., rect.y + 29., 9., WHITE);
    }
    panel(MINE_BOARD, accessibility::board_fill(state.high_contrast));
    let layout = mine_grid(state);
    for index in 0..game.cells.len() {
        let cell_rect = layout.cell_rect(index).unwrap();
        let rect = Rect::new(cell_rect.x, cell_rect.y, cell_rect.w - 1., cell_rect.h - 1.);
        let cell = game.cells[index];
        let revealed = matches!(cell, crate::minesweeper::Cell::Revealed(value) if value < 9)
            || matches!(game.status, crate::minesweeper::MineStatus::Lost)
                && matches!(
                    cell,
                    crate::minesweeper::Cell::Mine
                        | crate::minesweeper::Cell::FlaggedMine
                        | crate::minesweeper::Cell::Revealed(9)
                );
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
            crate::minesweeper::Cell::Flagged | crate::minesweeper::Cell::FlaggedMine => text(
                "⚑",
                rect.x + rect.w * 0.20,
                rect.y + rect.h * 0.72,
                accessibility::text_size((rect.w * 0.7).min(20.), state.large_text),
                Color::new(0.98, 0.46, 0.38, 1.),
            ),
            crate::minesweeper::Cell::Mine
                if matches!(game.status, crate::minesweeper::MineStatus::Lost) =>
            {
                text(
                    "✹",
                    rect.x + rect.w * 0.20,
                    rect.y + rect.h * 0.72,
                    accessibility::text_size((rect.w * 0.7).min(20.), state.large_text),
                    Color::new(0.98, 0.45, 0.32, 1.),
                )
            }
            crate::minesweeper::Cell::Revealed(value) if value > 0 && value < 9 => text(
                &value.to_string(),
                rect.x + rect.w * 0.35,
                rect.y + rect.h * 0.72,
                accessibility::text_size((rect.w * 0.6).min(19.), state.large_text),
                if state.high_contrast {
                    BLACK
                } else {
                    Color::new(0.76, 0.90, 1.0, 1.)
                },
            ),
            _ => {}
        }
    }
    text(
        &format!(
            "Mines {} / {}  •  {:03}s",
            game.flagged_count(),
            game.mines,
            game.elapsed_whole_seconds()
        ),
        10.,
        535.,
        13.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    text(
        match game.status {
            crate::minesweeper::MineStatus::Ready => "First reveal is safe",
            crate::minesweeper::MineStatus::Playing => "Find every safe square",
            crate::minesweeper::MineStatus::Won => "Field cleared",
            crate::minesweeper::MineStatus::Lost => "A mine was found",
        },
        10.,
        560.,
        13.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(8., 580., 165., 44.),
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
        42.,
        608.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(187., 580., 165., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RESTART", 241., 608., 13., WHITE);
    text(
        "Tap numbers to chord; hold to flag.",
        10.,
        665.,
        12.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

pub fn minesweeper_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in crate::minesweeper::MinePreset::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(5. + index as f32 * 88., 88., 82., 44.), p) {
            return vec![UiAction::MinePreset(*preset)];
        }
    }
    if crate::ui::hit(Rect::new(8., 580., 165., 44.), p) {
        return vec![UiAction::MineFlagMode];
    }
    if crate::ui::hit(Rect::new(187., 580., 165., 44.), p) {
        return vec![UiAction::MineRestart];
    }
    let Some(index) = mine_grid(state).index_at(p) else {
        return vec![];
    };
    if state.mine_flag_mode {
        vec![UiAction::MineFlag(index)]
    } else if matches!(
        state.minesweeper.cells[index],
        crate::minesweeper::Cell::Revealed(_)
    ) {
        vec![UiAction::MineChord(index)]
    } else {
        vec![UiAction::MineReveal(index)]
    }
}

pub fn minesweeper_long_press(state: &AppState, p: Vec2) -> Vec<UiAction> {
    mine_grid(state)
        .index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::MineFlag(index)])
}
