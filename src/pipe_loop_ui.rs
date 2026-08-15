//! Responsive touch presentation for Pipe Loop.

use crate::{
    pipe_loop::{PipeLoop, PipePhase, SIDE},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(260., 44., 300., 300.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(5., 105., 330., 330.),
            undo: Rect::new(5., 470., 145., 44.),
            new_game: Rect::new(165., 470., 170., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.board.contains(point) {
        let cell = l.board.w / SIDE as f32;
        let col = ((point.x - l.board.x) / cell) as usize;
        let row = ((point.y - l.board.y) / cell) as usize;
        if row < SIDE && col < SIDE {
            return vec![UiAction::PipeRotate(row * SIDE + col)];
        }
    }
    if l.undo.contains(point) {
        return vec![UiAction::PipeUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::PipeNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.pipe_loop;
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
    draw_text("PIPE LOOP", title_x, title_y, title_size(), accent());
    draw_text(
        format!("{} rotations  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    button(l.undo, "UNDO");
    button(l.new_game, "NEW LOOP");
    let status_y = if portrait {
        535.
    } else if compact {
        365.
    } else {
        545.
    };
    draw_text(
        "Tap any tile to rotate its quiet path",
        if compact { 260. } else { title_x },
        status_y,
        body_size(),
        muted(),
    );
}

fn draw_board(board: Rect, game: &PipeLoop) {
    let cell = board.w / SIDE as f32;
    for index in 0..SIDE * SIDE {
        let rect = Rect::new(
            board.x + (index % SIDE) as f32 * cell,
            board.y + (index / SIDE) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.10, 0.08, 0.17, 1.),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
        let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        draw_circle(center.x, center.y, cell * 0.10, accent());
        let mask = game.pipes[index];
        let endpoints = [
            (mask & 1 != 0, vec2(center.x, rect.y)),
            (mask & 2 != 0, vec2(rect.right(), center.y)),
            (mask & 4 != 0, vec2(center.x, rect.bottom())),
            (mask & 8 != 0, vec2(rect.x, center.y)),
        ];
        for (connected, endpoint) in endpoints {
            if connected {
                draw_line(
                    center.x,
                    center.y,
                    endpoint.x,
                    endpoint.y,
                    cell * 0.12,
                    pipe_color(),
                );
            }
        }
    }
}

fn status(phase: PipePhase) -> &'static str {
    match phase {
        PipePhase::Playing => "JOIN THE PATH",
        PipePhase::Won => "PATH COMPLETE",
    }
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
        rect.y + rect.h * 0.65,
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
fn pipe_color() -> Color {
    Color::new(0.35, 0.82, 0.70, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
