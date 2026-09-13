use super::{back, panel, text};
use crate::{
    accessibility,
    grid::GridLayout,
    minesweeper::{Cell, MinePreset, MineStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const MINE_BOARD: Rect = Rect {
    x: 10.,
    y: 28.,
    w: 350.,
    h: 350.,
};
fn mine_grid(state: &AppState) -> GridLayout {
    GridLayout::new(
        Rect::new(15., 33., 340., 340.),
        state.games.minesweeper.width,
        state.games.minesweeper.height,
    )
}

pub fn draw_minesweeper(state: &AppState) {
    let game = &state.games.minesweeper;
    back();
    text("MINESWEEPER", 100., 20., 19., crate::theme::BRASS);
    panel(MINE_BOARD, accessibility::board_fill(state.high_contrast));
    let layout = mine_grid(state);
    for index in 0..game.cells.len() {
        let Some(cell_rect) = layout.cell_rect(index) else {
            continue;
        };
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
        let rect = Rect::new(400. + index as f32 * 105., 105., 98., 44.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 8., rect.y + 29., 10., WHITE);
    }
    panel(
        Rect::new(400., 155., 170., 44.),
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
        445.,
        183.,
        13.,
        WHITE,
    );
    panel(Rect::new(590., 155., 170., 44.), crate::theme::SURFACE);
    text("RESTART", 645., 183., 13., WHITE);
    text(
        "Tap reveal; hold to flag.",
        400.,
        240.,
        13.,
        crate::theme::SECONDARY,
    );
}
pub fn minesweeper_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(400. + index as f32 * 105., 105., 98., 44.), p) {
            return vec![UiAction::MinePreset(*preset)];
        }
    }
    if crate::ui::hit(Rect::new(400., 155., 170., 44.), p) {
        return vec![UiAction::MineFlagMode];
    }
    if crate::ui::hit(Rect::new(590., 155., 170., 44.), p) {
        return vec![UiAction::MineRestart];
    }
    if let Some(index) = mine_grid(state).index_at(p) {
        if state.mine_flag_mode {
            vec![UiAction::MineFlag(index)]
        } else if matches!(state.games.minesweeper.cells[index], Cell::Revealed(_)) {
            vec![UiAction::MineChord(index)]
        } else {
            vec![UiAction::MineReveal(index)]
        }
    } else {
        vec![]
    }
}

pub fn minesweeper_long_press(state: &AppState, p: Vec2) -> Vec<UiAction> {
    mine_grid(state)
        .index_at(p)
        .map_or_else(Vec::new, |index| vec![UiAction::MineFlag(index)])
}
