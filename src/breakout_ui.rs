//! Responsive presentation and touch routing for real-time Breakout.

use crate::{
    accessibility,
    breakout::{BreakoutStatus, PaddleMove, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub cell: f32,
    pub left: Rect,
    pub right: Rect,
    pub stay: Rect,
    pub hint: Rect,
    pub undo: Rect,
    pub pause: Rect,
    pub new_game: Rect,
}
pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(20., 48., 384., 288.),
            cell: 24.,
            left: Rect::new(435., 180., 80., 42.),
            stay: Rect::new(525., 180., 80., 42.),
            right: Rect::new(615., 180., 80., 42.),
            hint: Rect::new(435., 300., 90., 38.),
            undo: Rect::new(435., 250., 90., 38.),
            pause: Rect::new(535., 300., 110., 38.),
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
            pause: Rect::new(185., 530., 165., 42.),
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
            pause: Rect::new(1120., 345., 110., 42.),
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
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::BreakoutPause];
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
    let game = &state.games.breakout;
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
        &status_text(game),
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
            let health = game.brick_health(index);
            let fill = if state.high_contrast {
                match health {
                    1 => Color::new(1., 0.15, 0.20, 1.),
                    2 => Color::new(1., 0.70, 0.05, 1.),
                    _ => Color::new(0.20, 0.90, 1., 1.),
                }
            } else {
                match health {
                    1 => Color::new(0.78, 0.30 + row as f32 * 0.05, 0.38, 1.),
                    2 => Color::new(0.88, 0.58, 0.24, 1.),
                    _ => Color::new(0.38, 0.70, 0.78, 1.),
                }
            };
            let brick = Rect::new(
                l.board.x + column as f32 * l.cell + 2.,
                l.board.y + (row + 1) as f32 * l.cell + 2.,
                l.cell - 4.,
                l.cell - 4.,
            );
            draw_rectangle(brick.x, brick.y, brick.w, brick.h, fill);
            if health > 1 {
                draw_rectangle_lines(brick.x, brick.y, brick.w, brick.h, 2., accent());
                for pip in 0..health.min(3) {
                    draw_circle(
                        brick.x
                            + brick.w * 0.5
                            + (f32::from(pip) - f32::from(health - 1) * 0.5) * 7.,
                        brick.y + brick.h * 0.5,
                        2.2,
                        accessibility::board_fill(state.high_contrast),
                    );
                }
            }
        }
    }
    let (ball_x, ball_y) = game.ball_position();
    draw_circle(
        l.board.x + ball_x * l.cell + l.cell / 2.,
        l.board.y + ball_y * l.cell + l.cell / 2.,
        l.cell * 0.25,
        accent(),
    );
    draw_rectangle(
        l.board.x + (game.paddle_position() - 2.) * l.cell,
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
            "Score {}  •  {} bricks  •  {}",
            game.score,
            game.remaining_bricks(),
            state.card_hint.as_deref().unwrap_or(if game.paused {
                if game.serve_ready {
                    "Ready — tap LAUNCH"
                } else {
                    "Paused — tap RESUME"
                }
            } else {
                "Auto-running — tap LEFT, STAY, or RIGHT"
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
    button(
        l.pause,
        if game.serve_ready {
            "LAUNCH"
        } else if game.paused {
            "RESUME"
        } else {
            "PAUSE"
        },
        state.large_text,
    );
    button(l.new_game, "NEW BOARD", state.large_text);
}
pub fn status_text(game: &crate::breakout::Breakout) -> String {
    match game.status {
        BreakoutStatus::Playing => format!(
            "Wall {} / {}  •  {} lives",
            game.level, game.target_level, game.lives
        ),
        BreakoutStatus::Won => "All three walls are clear".into(),
        BreakoutStatus::Lost => "No balls remain".into(),
    }
}
pub fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 8.,
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
