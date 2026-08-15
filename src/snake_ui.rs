//! Responsive presentation and touch routing for turn-based Snake.

use crate::{
    accessibility,
    snake::{SnakeDirection, SnakeStatus, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    up: Rect,
    left: Rect,
    down: Rect,
    right: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(20., 48., 384., 288.),
            cell: 24.,
            up: Rect::new(500., 130., 58., 42.),
            left: Rect::new(435., 178., 58., 42.),
            down: Rect::new(500., 178., 58., 42.),
            right: Rect::new(565., 178., 58., 42.),
            undo: Rect::new(435., 250., 90., 38.),
            new_game: Rect::new(535., 250., 110., 38.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 105., 340., 255.),
            cell: 21.25,
            up: Rect::new(145., 400., 65., 40.),
            left: Rect::new(70., 445., 65., 40.),
            down: Rect::new(145., 445., 65., 40.),
            right: Rect::new(220., 445., 65., 40.),
            undo: Rect::new(20., 535., 145., 42.),
            new_game: Rect::new(185., 535., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(360., 82., 640., 480.),
            cell: 40.,
            up: Rect::new(1080., 190., 65., 42.),
            left: Rect::new(1010., 238., 65., 42.),
            down: Rect::new(1080., 238., 65., 42.),
            right: Rect::new(1150., 238., 65., 42.),
            undo: Rect::new(1010., 330., 95., 42.),
            new_game: Rect::new(1120., 330., 110., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, direction) in [
        (layout.up, SnakeDirection::Up),
        (layout.left, SnakeDirection::Left),
        (layout.down, SnakeDirection::Down),
        (layout.right, SnakeDirection::Right),
    ] {
        if rect.contains(point) {
            return vec![UiAction::SnakeStep(direction)];
        }
    }
    if layout.undo.contains(point) {
        return vec![UiAction::SnakeUndo];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::SnakeNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.snake;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        360.
    };
    let header_y = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "SNAKE",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        &status_text(game.status, game.score),
        if compact { 435. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_rectangle(
        layout.board.x,
        layout.board.y,
        layout.board.w,
        layout.board.h,
        accessibility::board_fill(state.high_contrast),
    );
    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            draw_rectangle_lines(
                layout.board.x + column as f32 * layout.cell,
                layout.board.y + row as f32 * layout.cell,
                layout.cell,
                layout.cell,
                1.,
                accessibility::grid_line(state.high_contrast),
            );
        }
    }
    let food_row = i32::from(game.food) / WIDTH;
    let food_column = i32::from(game.food) % WIDTH;
    draw_circle(
        layout.board.x + food_column as f32 * layout.cell + layout.cell / 2.,
        layout.board.y + food_row as f32 * layout.cell + layout.cell / 2.,
        layout.cell * 0.28,
        if state.high_contrast {
            Color::new(1., 0.12, 0.18, 1.)
        } else {
            Color::new(0.98, 0.35, 0.35, 1.)
        },
    );
    for (part, &cell) in game.body.iter().enumerate() {
        let row = i32::from(cell) / WIDTH;
        let column = i32::from(cell) % WIDTH;
        draw_rectangle(
            layout.board.x + column as f32 * layout.cell + 2.,
            layout.board.y + row as f32 * layout.cell + 2.,
            layout.cell - 4.,
            layout.cell - 4.,
            if part == 0 {
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.98, 0.83, 0.45, 1.)
                }
            } else {
                if state.high_contrast {
                    Color::new(0.05, 0.95, 0.30, 1.)
                } else {
                    Color::new(0.38, 0.82, 0.58, 1.)
                }
            },
        );
    }
    text(
        &format!("Score {}  •  Tap a direction to move", game.score),
        if compact {
            435.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            90.
        } else if portrait {
            385.
        } else {
            600.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (rect, label) in [
        (layout.up, "UP"),
        (layout.left, "LEFT"),
        (layout.down, "DOWN"),
        (layout.right, "RIGHT"),
    ] {
        button(rect, label, state.large_text);
    }
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_game, "NEW BOARD", state.large_text);
}

fn status_text(status: SnakeStatus, score: u16) -> String {
    match status {
        SnakeStatus::Playing => format!("Score {} / 20", score),
        SnakeStatus::Won => "The coil is complete".into(),
        SnakeStatus::Lost => "The coil touched quiet space".into(),
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 10.,
        rect.y + 27.,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        29.
    } else {
        31.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
