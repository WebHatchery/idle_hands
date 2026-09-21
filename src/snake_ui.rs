//! Responsive presentation and touch routing for real-time Snake.

use crate::{
    accessibility,
    snake::{FoodKind, Snake, SnakeDirection, SnakeMode, SnakeStatus, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub cell: f32,
    pub up: Rect,
    pub left: Rect,
    pub down: Rect,
    pub right: Rect,
    pub hint: Rect,
    pub undo: Rect,
    pub pause: Rect,
    pub new_game: Rect,
    pub modes: [Rect; 3],
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(20., 48., 384., 288.),
            cell: 24.,
            up: Rect::new(500., 145., 58., 44.),
            left: Rect::new(435., 195., 58., 44.),
            down: Rect::new(500., 195., 58., 44.),
            right: Rect::new(565., 195., 58., 44.),
            hint: Rect::new(435., 300., 90., 38.),
            undo: Rect::new(435., 250., 90., 38.),
            pause: Rect::new(535., 300., 110., 38.),
            new_game: Rect::new(535., 250., 110., 38.),
            modes: [
                Rect::new(435., 60., 66., 44.),
                Rect::new(505., 60., 62., 44.),
                Rect::new(571., 60., 74., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 105., 340., 255.),
            cell: 21.25,
            up: Rect::new(145., 425., 65., 44.),
            left: Rect::new(70., 475., 65., 44.),
            down: Rect::new(145., 475., 65., 44.),
            right: Rect::new(220., 475., 65., 44.),
            hint: Rect::new(20., 590., 145., 42.),
            undo: Rect::new(20., 535., 145., 42.),
            pause: Rect::new(185., 590., 165., 42.),
            new_game: Rect::new(185., 535., 165., 42.),
            modes: [
                Rect::new(10., 370., 104., 44.),
                Rect::new(123., 370., 104., 44.),
                Rect::new(236., 370., 114., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(360., 100., 640., 480.),
            cell: 40.,
            up: Rect::new(1080., 190., 65., 42.),
            left: Rect::new(1010., 238., 65., 42.),
            down: Rect::new(1080., 238., 65., 42.),
            right: Rect::new(1150., 238., 65., 42.),
            hint: Rect::new(1010., 385., 95., 42.),
            undo: Rect::new(1010., 330., 95., 42.),
            pause: Rect::new(1120., 385., 110., 42.),
            new_game: Rect::new(1120., 330., 110., 42.),
            modes: [
                Rect::new(1010., 100., 68., 44.),
                Rect::new(1083., 100., 64., 44.),
                Rect::new(1152., 100., 78., 44.),
            ],
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in layout.modes.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::SnakeMode(SnakeMode::ALL[index])];
        }
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
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::SnakeUndo];
    }
    if crate::ui::hit(layout.pause, point) {
        return vec![UiAction::SnakePause];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::SnakeHint];
    }
    if crate::ui::hit(layout.new_game, point) {
        return vec![UiAction::SnakeNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.games.snake;
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
        &status_text(game),
        if compact { 435. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (index, rect) in layout.modes.iter().enumerate() {
        let mode = SnakeMode::ALL[index];
        mode_button(*rect, mode, game.mode == mode, state.large_text);
    }
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
    for &obstacle in &game.obstacles {
        let row = i32::from(obstacle) / WIDTH;
        let column = i32::from(obstacle) % WIDTH;
        let x = layout.board.x + column as f32 * layout.cell;
        let y = layout.board.y + row as f32 * layout.cell;
        draw_rectangle(
            x + 3.,
            y + 3.,
            layout.cell - 6.,
            layout.cell - 6.,
            crate::theme::SURFACE,
        );
        draw_line(
            x + 6.,
            y + 6.,
            x + layout.cell - 6.,
            y + layout.cell - 6.,
            2.,
            muted(),
        );
        draw_line(
            x + layout.cell - 6.,
            y + 6.,
            x + 6.,
            y + layout.cell - 6.,
            2.,
            muted(),
        );
    }
    let food_row = i32::from(game.food) / WIDTH;
    let food_column = i32::from(game.food) % WIDTH;
    let food_x = layout.board.x + food_column as f32 * layout.cell + layout.cell / 2.;
    let food_y = layout.board.y + food_row as f32 * layout.cell + layout.cell / 2.;
    if game.food_kind == FoodKind::Gold {
        draw_poly(food_x, food_y, 5, layout.cell * 0.34, -90., accent());
        draw_circle(food_x, food_y, layout.cell * 0.10, WHITE);
    } else {
        draw_circle(
            food_x,
            food_y,
            layout.cell * 0.28,
            if state.high_contrast {
                Color::new(1., 0.12, 0.18, 1.)
            } else {
                Color::new(0.98, 0.35, 0.35, 1.)
            },
        );
    }
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
                    crate::theme::BRASS
                }
            } else {
                if state.high_contrast {
                    Color::new(0.05, 0.95, 0.30, 1.)
                } else {
                    Color::new(0.38, 0.82, 0.58, 1.)
                }
            },
        );
        if part == 0 {
            draw_head_eyes(layout, row, column, game.direction, state.high_contrast);
        }
    }
    text(
        &format!(
            "Score {}  •  {}",
            game.score,
            state.card_hint.as_deref().unwrap_or(if game.paused {
                "Paused — tap RESUME"
            } else {
                "Auto-running — tap a direction to turn"
            })
        ),
        if compact {
            435.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            120.
        } else if portrait {
            650.
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
    button(layout.hint, "HINT", state.large_text);
    button(layout.undo, "UNDO", state.large_text);
    button(
        layout.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(layout.new_game, "NEW BOARD", state.large_text);
}

pub fn status_text(game: &Snake) -> String {
    match game.status {
        SnakeStatus::Playing => format!(
            "{}  •  Score {} / {}  •  Pace {}",
            game.mode.label(),
            game.score,
            game.win_score,
            game.speed_stage()
        ),
        SnakeStatus::Won => "The coil is complete".into(),
        SnakeStatus::Lost => "The coil struck an obstacle".into(),
    }
}
pub fn mode_button(rect: Rect, mode: SnakeMode, selected: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            crate::theme::MOSS_DARK
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2. } else { 1. },
        accent(),
    );
    let label = match mode {
        SnakeMode::Classic => "CLASSIC",
        SnakeMode::Wrap => "WRAP",
        SnakeMode::Garden => "GARDEN",
    };
    center_text(label, rect, accessibility::text_size(8., large_text), WHITE);
}

pub fn draw_head_eyes(
    layout: Layout,
    row: i32,
    column: i32,
    direction: SnakeDirection,
    high_contrast: bool,
) {
    let center = vec2(
        layout.board.x + (column as f32 + 0.5) * layout.cell,
        layout.board.y + (row as f32 + 0.5) * layout.cell,
    );
    let (forward, side) = match direction {
        SnakeDirection::Up => (vec2(0., -1.), vec2(1., 0.)),
        SnakeDirection::Right => (vec2(1., 0.), vec2(0., 1.)),
        SnakeDirection::Down => (vec2(0., 1.), vec2(1., 0.)),
        SnakeDirection::Left => (vec2(-1., 0.), vec2(0., 1.)),
    };
    let ink = if high_contrast {
        BLACK
    } else {
        crate::theme::BACKGROUND
    };
    for offset in [-1., 1.] {
        let eye = center + forward * layout.cell * 0.18 + side * layout.cell * 0.15 * offset;
        draw_circle(eye.x, eye.y, (layout.cell * 0.055).max(1.2), ink);
    }
}

pub fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        color,
    );
}
pub fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 10.,
        rect.y + 27.,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        29.
    } else {
        31.
    }
}
pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
pub fn accent() -> Color {
    crate::theme::BRASS
}
pub fn muted() -> Color {
    crate::theme::SECONDARY
}
pub fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
