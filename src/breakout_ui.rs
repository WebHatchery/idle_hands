//! Responsive presentation and touch routing for turn-based Breakout.

use crate::{
    accessibility,
    breakout::{BreakoutStatus, PaddleMove, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    left: Rect,
    right: Rect,
    stay: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(20., 48., 384., 288.),
            cell: 24.,
            left: Rect::new(435., 180., 80., 42.),
            stay: Rect::new(525., 180., 80., 42.),
            right: Rect::new(615., 180., 80., 42.),
            hint: Rect::new(435., 300., 90., 38.),
            undo: Rect::new(435., 250., 90., 38.),
            new_game: Rect::new(535., 250., 110., 38.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 105., 340., 255.),
            cell: 21.25,
            left: Rect::new(70., 400., 80., 42.),
            stay: Rect::new(160., 400., 80., 42.),
            right: Rect::new(250., 400., 80., 42.),
            hint: Rect::new(20., 530., 145., 42.),
            undo: Rect::new(20., 475., 145., 42.),
            new_game: Rect::new(185., 475., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(360., 100., 640., 480.),
            cell: 40.,
            left: Rect::new(1010., 220., 75., 42.),
            stay: Rect::new(1090., 220., 75., 42.),
            right: Rect::new(1170., 220., 55., 42.),
            hint: Rect::new(1010., 345., 95., 42.),
            undo: Rect::new(1010., 290., 95., 42.),
            new_game: Rect::new(1120., 290., 110., 42.),
        }
    }
}
pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, movement) in [
        (l.left, PaddleMove::Left),
        (l.stay, PaddleMove::Stay),
        (l.right, PaddleMove::Right),
    ] {
        if rect.contains(point) {
            return vec![UiAction::BreakoutStep(movement)];
        }
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::BreakoutUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::BreakoutHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::BreakoutNew];
    }
    let _ = state;
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.breakout;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let hx = if compact {
        120.
    } else if portrait {
        10.
    } else {
        360.
    };
    let hy = if compact {
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
        "BREAKOUT",
        hx,
        hy,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        &status_text(game.status, game.score),
        if compact { 435. } else { hx },
        if compact { 30. } else { hy + 25. },
        accessibility::text_size(body_size(), state.large_text),
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
            draw_rectangle_lines(
                l.board.x + column as f32 * l.cell,
                l.board.y + row as f32 * l.cell,
                l.cell,
                l.cell,
                1.,
                accessibility::grid_line(state.high_contrast),
            );
        }
    }
    for (index, brick) in game.bricks.iter().enumerate() {
        if *brick {
            let row = index as i8 / WIDTH;
            let column = index as i8 % WIDTH;
            draw_rectangle(
                l.board.x + column as f32 * l.cell + 2.,
                l.board.y + (row + 1) as f32 * l.cell + 2.,
                l.cell - 4.,
                l.cell - 4.,
                if state.high_contrast {
                    Color::new(1., 0.12 + row as f32 * 0.04, 0.18, 1.)
                } else {
                    Color::new(0.78, 0.30 + row as f32 * 0.05, 0.38, 1.)
                },
            );
        }
    }
    draw_circle(
        l.board.x + game.ball_x as f32 * l.cell + l.cell / 2.,
        l.board.y + game.ball_y as f32 * l.cell + l.cell / 2.,
        l.cell * 0.25,
        accent(),
    );
    draw_rectangle(
        l.board.x + (game.paddle - 2) as f32 * l.cell,
        l.board.y + (HEIGHT - 1) as f32 * l.cell,
        l.cell * 5.,
        l.cell * 0.55,
        if state.high_contrast {
            Color::new(0.05, 0.95, 0.30, 1.)
        } else {
            Color::new(0.35, 0.82, 0.58, 1.)
        },
    );
    text(
        &format!(
            "Score {}  •  {}",
            game.score,
            state
                .card_hint
                .as_deref()
                .unwrap_or("Tap LEFT, STAY, or RIGHT")
        ),
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
    button(l.left, "LEFT", state.large_text);
    button(l.stay, "STAY", state.large_text);
    button(l.right, "RIGHT", state.large_text);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
}
fn status_text(status: BreakoutStatus, score: u16) -> String {
    match status {
        BreakoutStatus::Playing => format!("Score {} / 64", score),
        BreakoutStatus::Won => "The wall is clear".into(),
        BreakoutStatus::Lost => "The ball fell quiet".into(),
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
        rect.x + 8.,
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
