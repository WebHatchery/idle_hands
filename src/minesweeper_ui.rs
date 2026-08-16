//! Compact portrait Minesweeper presentation and shared touch mapping.

use crate::{
    accessibility,
    grid::GridLayout,
    minesweeper::{Cell, MinePreset, MineStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::drawer_surface(fill));
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw(state: &AppState) {
    let game = &state.minesweeper;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("MINESWEEPER", 40., 105., 42., crate::theme::BRASS);
    text(
        "Read the quiet field",
        44.,
        132.,
        18.,
        crate::theme::SECONDARY,
    );
    let board = Rect::new(350., 155., 450., 450.);
    panel(board, accessibility::board_fill(state.high_contrast));
    let grid = GridLayout::new(
        Rect::new(board.x + 12., board.y + 12., board.w - 24., board.h - 24.),
        game.width,
        game.height,
    );
    let cell_size = grid.cell_width;
    for index in 0..game.cells.len() {
        let cell = grid.cell_rect(index).unwrap();
        let rect = Rect::new(cell.x, cell.y, cell.w - 2., cell.h - 2.);
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
                "⚑",
                rect.x + 12.,
                rect.y + 31.,
                accessibility::text_size(25., state.large_text),
                Color::new(0.98, 0.46, 0.38, 1.),
            ),
            Cell::Mine if matches!(game.status, MineStatus::Lost) => text(
                "✹",
                rect.x + 11.,
                rect.y + 31.,
                accessibility::text_size(24., state.large_text),
                Color::new(0.98, 0.45, 0.32, 1.),
            ),
            Cell::Revealed(value) if value > 0 && value < 9 => text(
                &value.to_string(),
                rect.x + cell_size * 0.35,
                rect.y + cell_size * 0.68,
                accessibility::text_size((cell_size * 0.48).min(23.), state.large_text),
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
        &format!("Mines: {} / {}", game.flagged_count(), game.mines),
        850.,
        215.,
        22.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    text(
        &format!("Time: {:03}s", game.elapsed_whole_seconds()),
        850.,
        175.,
        22.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        let rect = Rect::new(820. + index as f32 * 110., 285., 100., 32.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 8., rect.y + 21., 11., WHITE);
    }
    let instruction = state.card_hint.as_deref().unwrap_or(match game.status {
        MineStatus::Ready => "First reveal is safe",
        MineStatus::Playing => "Find every safe square",
        MineStatus::Won => "Field cleared",
        MineStatus::Lost => "A mine was found",
    });
    text(
        instruction,
        850.,
        255.,
        18.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(850., 320., 170., 52.),
        if state.mine_flag_mode {
            Color::new(0.45, 0.20, 0.27, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    text(
        if state.mine_flag_mode {
            "FLAG MODE"
        } else {
            "REVEAL MODE"
        },
        875.,
        353.,
        16.,
        WHITE,
    );
    panel(Rect::new(850., 390., 170., 52.), crate::theme::SURFACE);
    text("RESTART", 892., 423., 16., WHITE);
    panel(Rect::new(1030., 390., 170., 52.), crate::theme::SURFACE);
    text("HINT", 1092., 423., 16., WHITE);
    text(
        "Tap a square to reveal or flag it.",
        850.,
        500.,
        16.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    text(
        "Tap a revealed number after marking its mines to chord.",
        850.,
        525.,
        15.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(850., 320., 170., 52.).contains(p) {
        return vec![UiAction::MineFlagMode];
    }
    if Rect::new(850., 390., 170., 52.).contains(p) {
        return vec![UiAction::MineRestart];
    }
    if Rect::new(1030., 390., 170., 52.).contains(p) {
        return vec![UiAction::MineHint];
    }
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        if Rect::new(820. + index as f32 * 110., 285., 100., 32.).contains(p) {
            return vec![UiAction::MinePreset(*preset)];
        }
    }
    let board = Rect::new(350., 155., 450., 450.);
    let grid = GridLayout::new(
        Rect::new(board.x + 12., board.y + 12., board.w - 24., board.h - 24.),
        state.minesweeper.width,
        state.minesweeper.height,
    );
    let Some(index) = grid.index_at(p) else {
        return vec![];
    };
    if state.mine_flag_mode {
        vec![UiAction::MineFlag(index)]
    } else if matches!(state.minesweeper.cells[index], Cell::Revealed(_)) {
        vec![UiAction::MineChord(index)]
    } else {
        vec![UiAction::MineReveal(index)]
    }
}

pub fn long_press(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let board = Rect::new(350., 155., 450., 450.);
    let grid = GridLayout::new(
        Rect::new(board.x + 12., board.y + 12., board.w - 24., board.h - 24.),
        state.minesweeper.width,
        state.minesweeper.height,
    );
    grid.index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::MineFlag(index)])
}
