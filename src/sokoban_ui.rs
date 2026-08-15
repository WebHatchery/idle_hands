//! Responsive touch presentation for Sokoban.

use crate::{
    sokoban::{Sokoban, SokobanPhase, HEIGHT, WIDTH},
    state::{AppState, Direction},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    directions: [Rect; 4],
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            directions: [
                Rect::new(18., 82., 62., 44.),
                Rect::new(18., 127., 62., 44.),
                Rect::new(18., 172., 62., 44.),
                Rect::new(18., 217., 62., 44.),
            ],
            undo: Rect::new(610., 110., 110., 44.),
            new_game: Rect::new(610., 165., 145., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(30., 105., 330., 330.),
            directions: [
                Rect::new(30., 450., 68., 44.),
                Rect::new(114., 450., 68., 44.),
                Rect::new(198., 450., 68., 44.),
                Rect::new(282., 450., 68., 44.),
            ],
            undo: Rect::new(30., 510., 145., 44.),
            new_game: Rect::new(185., 510., 175., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            directions: [
                Rect::new(810., 130., 62., 44.),
                Rect::new(882., 130., 62., 44.),
                Rect::new(954., 130., 62., 44.),
                Rect::new(1026., 130., 62., 44.),
            ],
            undo: Rect::new(810., 205., 120., 44.),
            new_game: Rect::new(950., 205., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.directions.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::SokobanMove(
                [
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ][index],
            )];
        }
    }
    if l.undo.contains(point) {
        return vec![UiAction::SokobanUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::SokobanNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.sokoban;
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
        55.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("SOKOBAN", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Crates {}  •  {} moves  •  {}",
            game.crates,
            game.moves,
            if game.won() {
                "ROOM CLEAR"
            } else {
                "PUSH TO MARKS"
            }
        ),
        if compact { 420. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    text(
        status_text(game.phase),
        if compact { 270. } else { title_x },
        if portrait {
            475.
        } else if compact {
            365.
        } else {
            545.
        },
        body_size(),
        muted(),
    );
    for (rect, label) in l.directions.iter().zip(["UP", "LEFT", "DOWN", "RIGHT"]) {
        button(*rect, label);
    }
    button(l.undo, "UNDO");
    button(l.new_game, "NEW ROOM");
}

fn draw_board(board: Rect, game: &Sokoban) {
    let cell = board.w / WIDTH as f32;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let index = row * WIDTH + col;
            let rect = Rect::new(
                board.x + col as f32 * cell,
                board.y + row as f32 * cell,
                cell,
                cell,
            );
            let tile = game.tiles[index];
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, tile_color(tile));
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
            if tile == 2 || tile == 4 {
                center_text("○", rect, cell_size(), accent());
            }
            if tile == 3 || tile == 4 {
                center_text("■", rect, cell_size(), crate_color(tile));
            }
            if index == game.player {
                center_text("@", rect, cell_size(), WHITE);
            }
        }
    }
}

fn tile_color(tile: u8) -> Color {
    if tile == 0 {
        Color::new(0.10, 0.07, 0.16, 1.)
    } else {
        Color::new(0.18, 0.13, 0.27, 1.)
    }
}

fn crate_color(tile: u8) -> Color {
    if tile == 4 {
        Color::new(0.42, 0.82, 0.58, 1.)
    } else {
        Color::new(0.92, 0.46, 0.46, 1.)
    }
}

fn status_text(phase: SokobanPhase) -> &'static str {
    match phase {
        SokobanPhase::Playing => "Push each crate onto a marked square",
        SokobanPhase::Won => "The quiet room is clear",
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
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        23.
    } else {
        29.
    }
}

fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}

fn cell_size() -> f32 {
    if crate::ui::is_portrait() {
        17.
    } else {
        22.
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
