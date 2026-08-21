//! Responsive presentation and touch routing for Fling Fury.

use crate::{
    accessibility,
    fling_fury::{FlingBlock, FlingFury, FlingStatus, FlingTarget, GROUND_Y, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

mod result;
#[cfg(test)]
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
    draw_board(l, game);
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

fn draw_board(l: Layout, game: &FlingFury) {
    let board = l.board;
    draw_rectangle(
        board.x - 5.,
        board.y - 5.,
        board.w + 10.,
        board.h + 10.,
        crate::theme::INK,
    );
    for stripe in 0..7 {
        let y = board.y + stripe as f32 * board.h / 7.;
        let color = if stripe % 2 == 0 {
            Color::new(0.18, 0.34, 0.47, 1.)
        } else {
            Color::new(0.20, 0.38, 0.51, 1.)
        };
        draw_rectangle(board.x, y, board.w, board.h / 7. + 1., color);
    }
    let sun = vec2(board.x + l.cell * 4.7, board.y + l.cell * 3.1);
    draw_circle(
        sun.x,
        sun.y,
        l.cell * 1.35,
        Color::new(0.98, 0.74, 0.32, 0.92),
    );
    draw_circle(sun.x, sun.y, l.cell * 0.9, Color::new(1., 0.84, 0.43, 0.85));
    draw_cloud(board.x + l.cell * 11., board.y + l.cell * 3.1, l.cell, 0.8);
    draw_cloud(
        board.x + l.cell * 24.,
        board.y + l.cell * 2.1,
        l.cell * 0.8,
        0.55,
    );
    draw_triangle(
        vec2(board.x + l.cell * 8., board.y + l.cell * 14.),
        vec2(board.x + l.cell * 17., board.y + l.cell * 7.),
        vec2(board.x + l.cell * 25., board.y + l.cell * 14.),
        Color::new(0.20, 0.38, 0.35, 1.),
    );
    draw_triangle(
        vec2(board.x + l.cell * 17., board.y + l.cell * 14.),
        vec2(board.x + l.cell * 26., board.y + l.cell * 8.),
        vec2(board.x + l.cell * 34., board.y + l.cell * 14.),
        Color::new(0.16, 0.31, 0.30, 1.),
    );
    let ground_y = board.y + GROUND_Y * l.cell;
    draw_rectangle(
        board.x,
        ground_y,
        board.w,
        board.bottom() - ground_y,
        Color::new(0.25, 0.39, 0.25, 1.),
    );
    draw_rectangle(
        board.x,
        ground_y,
        board.w,
        l.cell * 0.22,
        Color::new(0.51, 0.65, 0.31, 1.),
    );
    draw_line(
        board.x,
        ground_y,
        board.right(),
        ground_y,
        2.,
        crate::theme::CREAM,
    );
    for index in 0..12 {
        let x = board.x + index as f32 * l.cell * 2.8 + l.cell * 0.4;
        draw_line(
            x,
            ground_y + l.cell * 0.4,
            x + l.cell * 0.3,
            ground_y + l.cell * 0.85,
            1.,
            Color::new(0.14, 0.26, 0.17, 0.7),
        );
    }

    for block in &game.blocks {
        if block.x + block.w < -1. || block.x > f32::from(WIDTH) + 1. {
            continue;
        }
        draw_block(board, l.cell, block);
    }
    for target in &game.targets {
        if target.alive {
            draw_target(board, l.cell, target);
        }
    }

    let sling_x = board.x + 5. * l.cell;
    let sling_y = board.y + 12.5 * l.cell;
    draw_line(
        sling_x - l.cell * 0.45,
        sling_y + l.cell,
        sling_x,
        sling_y - l.cell,
        3.,
        crate::theme::LEATHER,
    );
    draw_line(
        sling_x + l.cell * 0.45,
        sling_y + l.cell,
        sling_x,
        sling_y - l.cell,
        3.,
        crate::theme::LEATHER,
    );
    if let Some(shot) = game.shot {
        let center = vec2(board.x + shot.x * l.cell, board.y + shot.y * l.cell);
        draw_charge(center, l.cell, false);
        for trail in 1..=3 {
            let distance = trail as f32 * 0.55;
            draw_circle(
                board.x + (shot.x - shot.vx.signum() * distance) * l.cell,
                board.y + (shot.y - shot.vy.signum() * distance) * l.cell,
                l.cell * (0.16 - trail as f32 * 0.025),
                Color::new(1., 0.68, 0.25, 0.42),
            );
        }
    } else if game.shots_remaining > 0 && game.status == FlingStatus::Playing {
        let radians = f32::from(game.angle).to_radians();
        draw_line(
            sling_x,
            sling_y,
            sling_x + radians.cos() * l.cell * 5.,
            sling_y - radians.sin() * l.cell * 5.,
            2.,
            Color::new(1., 0.84, 0.34, 0.65),
        );
        draw_charge(vec2(sling_x, sling_y), l.cell, true);
    }
    draw_rectangle_lines(board.x, board.y, board.w, board.h, 2., crate::theme::BRASS);
}

fn draw_charge(center: Vec2, cell: f32, ready: bool) {
    let radius = cell * 0.56;
    draw_circle(
        center.x,
        center.y,
        radius * 1.75,
        Color::new(1., 0.40, 0.16, if ready { 0.22 } else { 0.30 }),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius * 1.35,
        2.5,
        Color::new(1., 0.80, 0.30, 0.95),
    );
    draw_circle(center.x, center.y, radius, Color::new(0.94, 0.26, 0.12, 1.));
    draw_circle(
        center.x,
        center.y,
        radius * 0.42,
        Color::new(1., 0.87, 0.42, 1.),
    );
    draw_line(
        center.x - radius * 0.7,
        center.y,
        center.x + radius * 0.7,
        center.y,
        1.5,
        Color::new(1., 0.96, 0.72, 0.9),
    );
    draw_line(
        center.x,
        center.y - radius * 0.7,
        center.x,
        center.y + radius * 0.7,
        1.5,
        Color::new(1., 0.96, 0.72, 0.9),
    );
    if ready {
        label(
            "SHOT",
            center.x - cell * 0.58,
            center.y + cell * 1.35,
            cell * 0.42,
            Color::new(1., 0.82, 0.36, 1.),
        );
    }
}

fn draw_block(board: Rect, cell: f32, block: &FlingBlock) {
    let center = vec2(
        board.x + (block.x + block.w * 0.5) * cell,
        board.y + (block.y + block.h * 0.5) * cell,
    );
    let width = block.w * cell;
    let height = block.h * cell;
    let corners = rotated_corners(center, width, height, block.rotation);
    let fill = if block.health >= 3 {
        Color::new(0.36, 0.48, 0.55, 1.)
    } else if block.health == 0 {
        Color::new(0.54, 0.31, 0.22, 0.83)
    } else {
        Color::new(0.74, 0.43, 0.25, 1.)
    };
    draw_triangle(corners[0], corners[1], corners[2], fill);
    draw_triangle(corners[0], corners[2], corners[3], fill);
    for index in 0..4 {
        let next = (index + 1) % 4;
        draw_line(
            corners[index].x,
            corners[index].y,
            corners[next].x,
            corners[next].y,
            2.,
            crate::theme::CREAM,
        );
    }
    let plank = corners[0].lerp(corners[3], 0.52);
    let plank_end = corners[1].lerp(corners[2], 0.52);
    draw_line(
        plank.x,
        plank.y,
        plank_end.x,
        plank_end.y,
        1.,
        Color::new(0.27, 0.16, 0.12, 0.8),
    );
    if block.health <= 1 {
        draw_line(
            center.x - width * 0.22,
            center.y - height * 0.25,
            center.x + width * 0.2,
            center.y + height * 0.24,
            1.5,
            Color::new(0.17, 0.10, 0.08, 0.9),
        );
    }
}

fn draw_target(board: Rect, cell: f32, target: &FlingTarget) {
    let center = vec2(board.x + target.x * cell, board.y + target.y * cell);
    let radius = cell * 0.64;
    let glow = if target.falling {
        Color::new(1., 0.54, 0.23, 0.45)
    } else {
        Color::new(0.31, 0.86, 0.84, 0.32)
    };
    draw_circle(center.x, center.y, radius * 1.35, glow);
    draw_circle_lines(
        center.x,
        center.y,
        radius * 1.42,
        2.5,
        if target.falling {
            Color::new(1., 0.77, 0.30, 1.)
        } else {
            Color::new(0.48, 1., 0.96, 1.)
        },
    );
    draw_circle(
        center.x,
        center.y,
        radius,
        if target.falling {
            Color::new(0.93, 0.42, 0.22, 1.)
        } else {
            Color::new(0.20, 0.73, 0.74, 1.)
        },
    );
    draw_circle(center.x, center.y, radius * 0.64, crate::theme::INK);
    let direction = vec2(target.rotation.cos(), target.rotation.sin());
    draw_line(
        center.x - direction.x * radius * 0.42,
        center.y - direction.y * radius * 0.42,
        center.x + direction.x * radius * 0.42,
        center.y + direction.y * radius * 0.42,
        2.,
        Color::new(1., 0.82, 0.36, 1.),
    );
    draw_line(
        center.x - direction.y * radius * 0.42,
        center.y + direction.x * radius * 0.42,
        center.x + direction.y * radius * 0.42,
        center.y - direction.x * radius * 0.42,
        2.,
        Color::new(1., 0.82, 0.36, 1.),
    );
    label(
        "TARGET",
        center.x - cell * 1.15,
        center.y - radius * 1.6,
        cell * 0.38,
        if target.falling {
            Color::new(1., 0.77, 0.30, 1.)
        } else {
            Color::new(0.58, 1., 0.92, 1.)
        },
    );
}

fn draw_cloud(x: f32, y: f32, size: f32, alpha: f32) {
    let color = Color::new(0.88, 0.94, 0.91, alpha);
    draw_circle(x, y, size * 0.62, color);
    draw_circle(x + size * 0.62, y - size * 0.22, size * 0.48, color);
    draw_circle(x + size * 1.12, y, size * 0.55, color);
    draw_rectangle(x, y, size * 1.12, size * 0.48, color);
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
