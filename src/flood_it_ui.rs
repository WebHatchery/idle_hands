//! Responsive touch presentation for Flood It.

use crate::{
    flood_it::{FloodIt, FloodPhase, COLORS, SIDE},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    undo: Rect,
    new_game: Rect,
    colors: [Rect; 6],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let colors = core::array::from_fn(|i| {
            Rect::new(
                620. + (i % 3) as f32 * 48.,
                220. + (i / 3) as f32 * 48.,
                42.,
                42.,
            )
        });
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
            colors,
        }
    } else if crate::ui::is_portrait() {
        let colors = core::array::from_fn(|i| {
            Rect::new(
                25. + (i % 3) as f32 * 55.,
                520. + (i / 3) as f32 * 48.,
                48.,
                42.,
            )
        });
        Layout {
            board: Rect::new(25., 105., 330., 330.),
            undo: Rect::new(25., 460., 145., 44.),
            new_game: Rect::new(180., 460., 175., 44.),
            colors,
        }
    } else {
        let colors = core::array::from_fn(|i| {
            Rect::new(
                810. + (i % 3) as f32 * 52.,
                270. + (i / 3) as f32 * 52.,
                46.,
                46.,
            )
        });
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
            colors,
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.undo.contains(point) {
        return vec![UiAction::FloodUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::FloodNew];
    }
    for (color, rect) in l.colors.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::FloodColor(color as u8)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.flood_it;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        62.
    } else {
        58.
    };
    draw_text("‹ CABINET", 8., 30., 13., muted());
    draw_text("FLOOD IT", title_x, title_y, title_size(), accent());
    draw_text(
        format!("{} / 24 moves  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    for (color, rect) in l.colors.iter().enumerate() {
        draw_color(*rect, color as u8, game.active_color);
    }
    button(l.undo, "UNDO");
    button(l.new_game, "NEW FIELD");
    let status_y = if portrait {
        500.
    } else if compact {
        365.
    } else {
        545.
    };
    draw_text(
        "Tap a color to grow the top-left region",
        if compact { 270. } else { title_x },
        status_y,
        body_size(),
        muted(),
    );
}

fn draw_board(board: Rect, game: &FloodIt) {
    let cell = board.w / SIDE as f32;
    for index in 0..SIDE * SIDE {
        let rect = Rect::new(
            board.x + (index % SIDE) as f32 * cell,
            board.y + (index / SIDE) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, palette(game.cells[index]));
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            Color::new(0.08, 0.06, 0.14, 0.55),
        );
    }
}

fn draw_color(rect: Rect, color: u8, active: u8) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, palette(color));
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if color == active { 3. } else { 1. },
        if color == active { WHITE } else { line_color() },
    );
}

fn status(phase: FloodPhase) -> &'static str {
    match phase {
        FloodPhase::Playing => "FILL THE FIELD",
        FloodPhase::Won => "FIELD COMPLETE",
        FloodPhase::Lost => "MOVE LIMIT REACHED",
    }
}

fn palette(color: u8) -> Color {
    [
        Color::new(0.94, 0.35, 0.42, 1.),
        Color::new(0.98, 0.72, 0.28, 1.),
        Color::new(0.35, 0.82, 0.58, 1.),
        Color::new(0.32, 0.64, 0.95, 1.),
        Color::new(0.68, 0.45, 0.90, 1.),
        Color::new(0.95, 0.48, 0.72, 1.),
    ][color as usize % COLORS as usize]
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}
fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        21.
    } else {
        27.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
