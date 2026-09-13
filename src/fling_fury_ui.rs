//! Responsive presentation and touch routing for Fling Fury.

use crate::{
    accessibility,
    fling_fury::{FlingBlock, FlingFury, FlingStatus, FlingTarget, GROUND_Y, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[path = "fling_fury_ui/board.rs"]
mod board;
mod result;
#[cfg(test)]
#[path = "../tests/legacy/fling_fury_ui/tests.rs"]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    angle_down: Rect,
    angle_up: Rect,
    power_down: Rect,
    power_up: Rect,
    fire: Rect,
    pause: Rect,
    undo: Rect,
    new_game: Rect,
    restart: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let cell = 14.;
        Layout {
            board: Rect::new(10., 42., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            angle_down: Rect::new(465., 50., 48., 38.),
            angle_up: Rect::new(518., 50., 48., 38.),
            power_down: Rect::new(465., 95., 48., 38.),
            power_up: Rect::new(518., 95., 48., 38.),
            fire: Rect::new(465., 140., 101., 42.),
            pause: Rect::new(465., 190., 101., 36.),
            undo: Rect::new(465., 234., 48., 36.),
            new_game: Rect::new(518., 234., 48., 36.),
            restart: Rect::new(465., 278., 101., 36.),
        }
    } else if crate::ui::is_portrait() {
        let cell = 9.;
        Layout {
            board: Rect::new(10., 120., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            angle_down: Rect::new(20., 300., 70., 40.),
            angle_up: Rect::new(95., 300., 70., 40.),
            power_down: Rect::new(170., 300., 70., 40.),
            power_up: Rect::new(245., 300., 85., 40.),
            fire: Rect::new(20., 350., 145., 42.),
            pause: Rect::new(185., 350., 145., 42.),
            undo: Rect::new(20., 405., 145., 42.),
            new_game: Rect::new(185., 405., 145., 42.),
            restart: Rect::new(20., 460., 310., 42.),
        }
    } else {
        let cell = 24.;
        Layout {
            board: Rect::new(
                180.,
                125.,
                f32::from(WIDTH) * cell,
                f32::from(HEIGHT) * cell,
            ),
            cell,
            angle_down: Rect::new(970., 135., 86., 42.),
            angle_up: Rect::new(1062., 135., 86., 42.),
            power_down: Rect::new(970., 185., 86., 42.),
            power_up: Rect::new(1062., 185., 86., 42.),
            fire: Rect::new(970., 235., 178., 44.),
            pause: Rect::new(970., 290., 86., 42.),
            undo: Rect::new(1062., 290., 86., 42.),
            new_game: Rect::new(970., 345., 178., 42.),
            restart: Rect::new(970., 395., 178., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, action) in [
        (l.angle_down, UiAction::FlingAngle(-5)),
        (l.angle_up, UiAction::FlingAngle(5)),
        (l.power_down, UiAction::FlingPower(-5)),
        (l.power_up, UiAction::FlingPower(5)),
    ] {
        if crate::ui::hit(rect, point) {
            return vec![action];
        }
    }
    if crate::ui::hit(l.fire, point) {
        return vec![UiAction::FlingFire];
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::FlingPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::FlingUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return if state.games.fling_fury.status == FlingStatus::Won {
            vec![UiAction::FlingNext]
        } else {
            vec![UiAction::FlingNew]
        };
    }
    if matches!(
        state.games.fling_fury.status,
        FlingStatus::Won | FlingStatus::Lost
    ) && crate::ui::hit(l.restart, point)
    {
        return vec![UiAction::FlingRestart];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.fling_fury;
    let compact = crate::ui::is_compact_landscape();
    let title_x = if compact {
        80.
    } else if crate::ui::is_portrait() {
        10.
    } else {
        180.
    };
    let title_y = if compact {
        27.
    } else if crate::ui::is_portrait() {
        68.
    } else {
        60.
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("FLING FURY", title_x, title_y, title_size(), accent());
    let score_y = if compact { 39. } else { title_y + 25. };
    let heading = if compact {
        format!(
            "L{} {} · {} pts · {} shots",
            game.level_number(),
            game.mode_label(),
            game.score,
            game.shots_remaining
        )
    } else {
        format!(
            "LEVEL {}/{} · {}",
            game.level_number(),
            FlingFury::level_count(),
            game.mode_label()
        )
    };
    label(&heading, title_x, score_y, body_size(), muted());
    board::draw_board(l, game);
    if matches!(game.status, FlingStatus::Won | FlingStatus::Lost) {
        result::draw(l.board, game);
    }
    if !compact {
        label(
            &format!(
                "Score {}  ·  Shots {}  ·  Aim {}° / {}  ·  Targets {}",
                game.score,
                game.shots_remaining,
                game.angle,
                game.power,
                game.targets.iter().filter(|target| target.alive).count()
            ),
            title_x,
            score_y + body_size() + 11.,
            body_size(),
            muted(),
        );
    }
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        title_x,
        if crate::ui::is_portrait() {
            291.
        } else if compact {
            310.
        } else {
            575.
        },
        body_size(),
        muted(),
    );
    button(
        l.angle_down,
        if compact { "A −" } else { "ANGLE −" },
        state.large_text,
    );
    button(
        l.angle_up,
        if compact { "A +" } else { "ANGLE +" },
        state.large_text,
    );
    button(
        l.power_down,
        if compact { "P −" } else { "POWER −" },
        state.large_text,
    );
    button(
        l.power_up,
        if compact { "P +" } else { "POWER +" },
        state.large_text,
    );
    button(l.fire, "FLING", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, if compact { "U" } else { "UNDO" }, state.large_text);
    button(
        l.new_game,
        if game.status == FlingStatus::Won {
            if compact {
                "NEXT"
            } else {
                "NEXT LEVEL"
            }
        } else if compact {
            "RESET"
        } else {
            "RESET RUN"
        },
        state.large_text,
    );
    if matches!(game.status, FlingStatus::Won | FlingStatus::Lost) {
        button(
            l.restart,
            if compact { "RESTART" } else { "RESTART LEVEL" },
            state.large_text,
        );
    }
}

fn rotated_corners(center: Vec2, width: f32, height: f32, rotation: f32) -> [Vec2; 4] {
    let cos = rotation.cos();
    let sin = rotation.sin();
    [
        rotate_offset(-width * 0.5, -height * 0.5, center, cos, sin),
        rotate_offset(width * 0.5, -height * 0.5, center, cos, sin),
        rotate_offset(width * 0.5, height * 0.5, center, cos, sin),
        rotate_offset(-width * 0.5, height * 0.5, center, cos, sin),
    ]
}

fn rotate_offset(x: f32, y: f32, center: Vec2, cos: f32, sin: f32) -> Vec2 {
    center + vec2(x * cos - y * sin, x * sin + y * cos)
}

fn status_text(game: &FlingFury) -> String {
    match game.status {
        FlingStatus::Playing => {
            "Orange SHOT is ready — aim at the CYAN TARGETS, then tap FLING".into()
        }
        FlingStatus::Won => "Every target fell — tap NEXT LEVEL to open the next fort".into(),
        FlingStatus::Lost => "Out of shots — tap RESTART LEVEL to try this fort again".into(),
    }
}

fn button(rect: Rect, value: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(10., large_text);
    crate::ui::draw_text(
        value,
        rect.x + (rect.w - crate::ui::measure_text(value, None, size as u16, 1.).width) * 0.5,
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
        9.
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
