//! Medium landscape layouts for the dense game boards.

use macroquad::prelude::*;

#[cfg(test)]
use crate::{state::AppState, ui::UiAction};

mod minesweeper;
mod nonogram;
mod reversi;
mod sudoku;

pub use minesweeper::{draw_minesweeper, minesweeper_clicks, minesweeper_long_press};
pub use nonogram::{draw_nonogram, nonogram_clicks, nonogram_drag_actions};
pub use reversi::{draw_reversi, reversi_clicks};
pub use sudoku::{draw_sudoku, sudoku_clicks};

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
fn back() {
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("< CABINET", 10., 29., 12., crate::theme::BRASS);
}

#[cfg(test)]
#[path = "../tests/legacy/responsive_landscape_games/tests.rs"]
mod tests;
