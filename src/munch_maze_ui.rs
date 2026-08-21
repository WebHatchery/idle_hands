//! Responsive presentation and touch routing for Munch Maze.

use crate::{
    accessibility,
    domain::Direction,
    munch_maze::{MunchMaze, MunchStatus, HEIGHT, WIDTH},
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
    pause: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let cell = 16.;
        Layout {
            board: Rect::new(10., 42., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            up: Rect::new(335., 52., 58., 38.),
            left: Rect::new(270., 96., 58., 38.),
            down: Rect::new(335., 96., 58., 38.),
            right: Rect::new(400., 96., 58., 38.),
            pause: Rect::new(270., 146., 92., 36.),
            undo: Rect::new(366., 146., 92., 36.),
            new_game: Rect::new(270., 190., 188., 36.),
        }
    } else if crate::ui::is_portrait() {
        let cell = 15.;
        Layout {
            board: Rect::new(10., 104., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            up: Rect::new(130., 350., 92., 40.),
            left: Rect::new(30., 395., 92., 40.),
            down: Rect::new(130., 395., 92., 40.),
            right: Rect::new(230., 395., 92., 40.),
            pause: Rect::new(20., 450., 145., 42.),
            undo: Rect::new(185., 450., 145., 42.),
            new_game: Rect::new(20., 505., 310., 42.),
        }
    } else {
        let cell = 30.;
        Layout {
            board: Rect::new(
                300.,
                100.,
                f32::from(WIDTH) * cell,
                f32::from(HEIGHT) * cell,
            ),
            cell,
            up: Rect::new(950., 125., 82., 42.),
            left: Rect::new(860., 172., 82., 42.),
            down: Rect::new(950., 172., 82., 42.),
            right: Rect::new(1040., 172., 82., 42.),
            pause: Rect::new(860., 230., 122., 42.),
            undo: Rect::new(990., 230., 122., 42.),
            new_game: Rect::new(860., 285., 252., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, direction) in [
        (l.up, Direction::Up),
        (l.left, Direction::Left),
        (l.down, Direction::Down),
        (l.right, Direction::Right),
    ] {
        if crate::ui::hit(rect, point) {
            return vec![UiAction::MunchMove(direction)];
        }
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::MunchPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::MunchUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::MunchNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.munch_maze;
    let (title_x, title_y) = if crate::ui::is_compact_landscape() {
        (80., 27.)
    } else if crate::ui::is_portrait() {
        (10., 68.)
    } else {
        (300., 60.)
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("MUNCH MAZE", title_x, title_y, title_size(), accent());
    label(
        &format!("Score {}  •  {} lives", game.score, game.lives),
        if crate::ui::is_compact_landscape() {
            270.
        } else {
            title_x
        },
        if crate::ui::is_compact_landscape() {
            27.
        } else {
            title_y + 25.
        },
        body_size(),
        muted(),
    );
    draw_rectangle(
        l.board.x,
        l.board.y,
        l.board.w,
        l.board.h,
        accessibility::board_fill(state.high_contrast),
    );
    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            let x = l.board.x + f32::from(column) * l.cell;
            let y = l.board.y + f32::from(row) * l.cell;
            if game.is_wall(column, row) {
                draw_rectangle(x, y, l.cell, l.cell, Color::new(0.13, 0.18, 0.32, 1.));
                draw_rectangle_lines(x, y, l.cell, l.cell, 1., Color::new(0.30, 0.53, 0.78, 0.8));
            } else if game.pellets[usize::from(row) * usize::from(WIDTH) + usize::from(column)] {
                draw_circle(
                    x + l.cell * 0.5,
                    y + l.cell * 0.5,
                    l.cell * 0.10,
                    crate::theme::CREAM,
                );
            }
        }
    }
    let player_x = l.board.x + (game.player % usize::from(WIDTH)) as f32 * l.cell + l.cell * 0.5;
    let player_y = l.board.y + (game.player / usize::from(WIDTH)) as f32 * l.cell + l.cell * 0.5;
    draw_circle(
        player_x,
        player_y,
        l.cell * 0.35,
        Color::new(1., 0.80, 0.18, 1.),
    );
    for ghost in &game.ghosts {
        let x = l.board.x + f32::from(ghost.x) * l.cell + l.cell * 0.5;
        let y = l.board.y + f32::from(ghost.y) * l.cell + l.cell * 0.5;
        let color = match ghost.kind % 4 {
            0 => Color::new(1., 0.28, 0.38, 1.),
            1 => Color::new(0.32, 0.84, 1., 1.),
            2 => Color::new(1., 0.45, 0.72, 1.),
            _ => Color::new(0.90, 0.55, 1., 1.),
        };
        draw_circle(x, y, l.cell * 0.34, color);
        draw_circle(x - l.cell * 0.11, y - l.cell * 0.07, l.cell * 0.05, WHITE);
        draw_circle(x + l.cell * 0.11, y - l.cell * 0.07, l.cell * 0.05, WHITE);
    }
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        if crate::ui::is_compact_landscape() {
            10.
        } else {
            title_x
        },
        if crate::ui::is_portrait() {
            342.
        } else if crate::ui::is_compact_landscape() {
            306.
        } else {
            575.
        },
        body_size(),
        muted(),
    );
    button(l.up, "UP", state.large_text);
    button(l.left, "LEFT", state.large_text);
    button(l.down, "DOWN", state.large_text);
    button(l.right, "RIGHT", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW MAZE", state.large_text);
}

fn status_text(game: &MunchMaze) -> String {
    match game.status {
        MunchStatus::Playing => "Tap a direction to clear the glowing pellets".into(),
        MunchStatus::Won => "Every pellet is yours — the maze is clear".into(),
        MunchStatus::Lost => "The patrol caught you — start a new maze".into(),
    }
}

fn button(rect: Rect, text_value: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(11., large_text);
    crate::ui::draw_text(
        text_value,
        rect.x + (rect.w - crate::ui::measure_text(text_value, None, size as u16, 1.).width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}

fn label(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        24.
    } else {
        30.
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
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 115., 42.)
}
