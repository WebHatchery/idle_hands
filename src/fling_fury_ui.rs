//! Responsive presentation and touch routing for Fling Fury.

use crate::{
    accessibility,
    fling_fury::{FlingFury, FlingStatus, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

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
        }
    } else if crate::ui::is_portrait() {
        let cell = 9.;
        Layout {
            board: Rect::new(10., 104., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            angle_down: Rect::new(20., 285., 70., 40.),
            angle_up: Rect::new(95., 285., 70., 40.),
            power_down: Rect::new(170., 285., 70., 40.),
            power_up: Rect::new(245., 285., 85., 40.),
            fire: Rect::new(20., 335., 145., 42.),
            pause: Rect::new(185., 335., 145., 42.),
            undo: Rect::new(20., 390., 145., 42.),
            new_game: Rect::new(185., 390., 145., 42.),
        }
    } else {
        let cell = 24.;
        Layout {
            board: Rect::new(
                180.,
                100.,
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
        return vec![UiAction::FlingNew];
    }
    let _ = state;
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
    let title_y = if crate::ui::is_compact_landscape() {
        27.
    } else if crate::ui::is_portrait() {
        68.
    } else {
        60.
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("FLING FURY", title_x, title_y, title_size(), accent());
    let score_y = if compact { 39. } else { title_y + 25. };
    label(
        &format!(
            "Score {}  •  Birds {}  •  Aim {}° / {}",
            game.score, game.birds, game.angle, game.power
        ),
        title_x,
        score_y,
        body_size(),
        muted(),
    );
    draw_rectangle(
        l.board.x,
        l.board.y,
        l.board.w,
        l.board.h,
        Color::new(0.08, 0.13, 0.24, 1.),
    );
    draw_circle(
        l.board.x + 5. * l.cell,
        l.board.y + 4. * l.cell,
        l.cell * 1.6,
        Color::new(0.95, 0.74, 0.26, 0.8),
    );
    draw_rectangle(
        l.board.x,
        l.board.y + 14. * l.cell,
        l.board.w,
        4. * l.cell,
        Color::new(0.25, 0.42, 0.26, 1.),
    );
    for block in &game.blocks {
        if block.health == 0 {
            continue;
        }
        let rect = Rect::new(
            l.board.x + f32::from(block.x) * l.cell,
            l.board.y + f32::from(block.y) * l.cell,
            f32::from(block.w) * l.cell,
            f32::from(block.h) * l.cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if block.health > 2 {
                Color::new(0.55, 0.62, 0.70, 1.)
            } else {
                Color::new(0.76, 0.42, 0.25, 1.)
            },
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::CREAM);
    }
    for target in &game.targets {
        if !target.alive {
            continue;
        }
        let x = l.board.x + f32::from(target.x) * l.cell;
        let y = l.board.y + f32::from(target.y) * l.cell;
        draw_circle(x, y, l.cell * 0.56, Color::new(0.38, 0.92, 0.50, 1.));
        draw_circle(x, y, l.cell * 0.23, Color::new(0.12, 0.22, 0.17, 1.));
    }
    let sling_x = l.board.x + 5. * l.cell;
    let sling_y = l.board.y + 13. * l.cell;
    draw_line(
        sling_x - l.cell * 0.4,
        sling_y + l.cell,
        sling_x,
        sling_y - l.cell,
        3.,
        Color::new(0.58, 0.30, 0.18, 1.),
    );
    draw_line(
        sling_x + l.cell * 0.4,
        sling_y + l.cell,
        sling_x,
        sling_y - l.cell,
        3.,
        Color::new(0.58, 0.30, 0.18, 1.),
    );
    let radians = f32::from(game.angle).to_radians();
    draw_line(
        sling_x,
        sling_y,
        sling_x + radians.cos() * l.cell * 4.,
        sling_y - radians.sin() * l.cell * 4.,
        2.,
        Color::new(1., 0.80, 0.24, 0.7),
    );
    if let Some(shot) = game.shot {
        draw_circle(
            l.board.x + shot.x * l.cell,
            l.board.y + shot.y * l.cell,
            l.cell * 0.45,
            Color::new(0.95, 0.25, 0.25, 1.),
        );
    } else if game.birds > 0 {
        draw_circle(
            sling_x,
            sling_y,
            l.cell * 0.45,
            Color::new(0.95, 0.25, 0.25, 1.),
        );
    }
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        title_x,
        if crate::ui::is_portrait() {
            275.
        } else if crate::ui::is_compact_landscape() {
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
        if compact { "NEW" } else { "NEW FORT" },
        state.large_text,
    );
}

fn status_text(game: &FlingFury) -> String {
    match game.status {
        FlingStatus::Playing => "Tune ANGLE and POWER, then tap FLING at the targets".into(),
        FlingStatus::Won => "The fort is down — physics did the talking".into(),
        FlingStatus::Lost => "Out of birds — build a new fort and try again".into(),
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
