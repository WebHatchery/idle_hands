//! Responsive presentation and touch routing for Space Invaders.

use crate::{
    accessibility,
    space_invaders::{Invader, ShipDirection, SpaceInvaders, SpaceInvadersStatus, HEIGHT, WIDTH},
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
    fire: Rect,
    pause: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        compact_layout()
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 104., 340., 204.),
            cell: 17.,
            left: Rect::new(20., 350., 100., 44.),
            right: Rect::new(130., 350., 100., 44.),
            fire: Rect::new(240., 350., 110., 44.),
            pause: Rect::new(20., 405., 145., 42.),
            undo: Rect::new(185., 405., 145., 42.),
            new_game: Rect::new(20., 460., 310., 42.),
        }
    } else {
        Layout {
            board: Rect::new(330., 100., 720., 432.),
            cell: 36.,
            left: Rect::new(1080., 155., 82., 44.),
            right: Rect::new(1170., 155., 82., 44.),
            fire: Rect::new(1080., 207., 172., 44.),
            pause: Rect::new(1080., 270., 82., 42.),
            undo: Rect::new(1170., 270., 82., 42.),
            new_game: Rect::new(1080., 324., 172., 42.),
        }
    }
}

fn compact_layout() -> Layout {
    Layout {
        board: Rect::new(12., 54., 440., 264.),
        cell: 22.,
        left: Rect::new(465., 80., 82., 44.),
        right: Rect::new(465., 132., 82., 44.),
        fire: Rect::new(465., 184., 82., 44.),
        pause: Rect::new(465., 236., 82., 36.),
        undo: Rect::new(465., 278., 82., 36.),
        new_game: Rect::new(465., 318., 82., 36.),
    }
}

fn compact_header() -> (f32, f32, f32, f32) {
    (120., 30., 120., 45.)
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, direction) in [
        (l.left, ShipDirection::Left),
        (l.right, ShipDirection::Right),
    ] {
        if crate::ui::hit(rect, point) {
            return vec![UiAction::SpaceInvadersStep(direction)];
        }
    }
    if crate::ui::hit(l.fire, point) {
        return vec![UiAction::SpaceInvadersFire];
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::SpaceInvadersPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::SpaceInvadersUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::SpaceInvadersNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.space_invaders;
    let (title_x, title_y) = if crate::ui::is_compact_landscape() {
        let (title_x, title_y, _, _) = compact_header();
        (title_x, title_y)
    } else if crate::ui::is_portrait() {
        (10., 68.)
    } else {
        (330., 60.)
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("SPACE INVADERS", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Wave {} / {}  •  Score {}  •  {} lives",
            game.wave, game.target_wave, game.score, game.lives
        ),
        if crate::ui::is_compact_landscape() {
            compact_header().2
        } else {
            title_x
        },
        if crate::ui::is_compact_landscape() {
            compact_header().3
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
            draw_rectangle_lines(
                l.board.x + f32::from(column) * l.cell,
                l.board.y + f32::from(row) * l.cell,
                l.cell,
                l.cell,
                1.,
                Color::new(0.25, 0.22, 0.40, 0.55),
            );
        }
    }
    for invader in &game.invaders {
        if invader.alive {
            draw_invader(l, invader, state.high_contrast);
        }
    }
    if let Some(shot) = game.player_shot {
        draw_rectangle(
            cell_x(l, shot.x) + l.cell * 0.45,
            cell_y(l, shot.y),
            l.cell * 0.10,
            l.cell * 0.75,
            Color::new(0.45, 1., 0.75, 1.),
        );
    }
    if let Some(shot) = game.enemy_shot {
        draw_rectangle(
            cell_x(l, shot.x) + l.cell * 0.42,
            cell_y(l, shot.y),
            l.cell * 0.16,
            l.cell * 0.75,
            Color::new(1., 0.40, 0.35, 1.),
        );
    }
    let ship_x = cell_x(l, game.ship_x) + l.cell * 0.5;
    let ship_y = cell_y(l, HEIGHT - 1) + l.cell * 0.5;
    draw_poly(
        ship_x,
        ship_y,
        3,
        l.cell * 0.48,
        -90.,
        if state.high_contrast {
            Color::new(0.35, 1., 0.55, 1.)
        } else {
            Color::new(0.35, 0.82, 0.72, 1.)
        },
    );
    let status = status_text(game);
    text(
        state.card_hint.as_deref().unwrap_or(&status),
        if crate::ui::is_compact_landscape() {
            18.
        } else {
            title_x
        },
        if crate::ui::is_portrait() {
            328.
        } else if crate::ui::is_compact_landscape() {
            378.
        } else {
            565.
        },
        body_size(),
        muted(),
    );
    button(l.left, "LEFT", state.large_text);
    button(l.right, "RIGHT", state.large_text);
    button(l.fire, "FIRE", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW WAVE", state.large_text);
}

fn draw_invader(layout: Layout, invader: &Invader, high_contrast: bool) {
    let x = cell_x(layout, invader.x);
    let y = cell_y(layout, invader.y);
    let color = match invader.kind {
        0 => Color::new(0.95, 0.38, 0.55, 1.),
        1 => Color::new(0.96, 0.72, 0.25, 1.),
        _ => Color::new(0.44, 0.76, 1., 1.),
    };
    let color = if high_contrast {
        Color::new(
            (color.r + 0.15).min(1.),
            (color.g + 0.15).min(1.),
            (color.b + 0.15).min(1.),
            1.,
        )
    } else {
        color
    };
    draw_rectangle(
        x + layout.cell * 0.18,
        y + layout.cell * 0.26,
        layout.cell * 0.64,
        layout.cell * 0.45,
        color,
    );
    draw_line(
        x + layout.cell * 0.30,
        y + layout.cell * 0.72,
        x + layout.cell * 0.18,
        y + layout.cell * 0.90,
        2.,
        color,
    );
    draw_line(
        x + layout.cell * 0.70,
        y + layout.cell * 0.72,
        x + layout.cell * 0.82,
        y + layout.cell * 0.90,
        2.,
        color,
    );
    draw_circle(
        x + layout.cell * 0.38,
        y + layout.cell * 0.44,
        layout.cell * 0.06,
        crate::theme::SURFACE_DARK,
    );
    draw_circle(
        x + layout.cell * 0.62,
        y + layout.cell * 0.44,
        layout.cell * 0.06,
        crate::theme::SURFACE_DARK,
    );
}

fn status_text(game: &SpaceInvaders) -> String {
    match game.status {
        SpaceInvadersStatus::Playing => "Tap LEFT or RIGHT to line up • tap FIRE to launch".into(),
        SpaceInvadersStatus::Won => "The sky is clear — new wave ready".into(),
        SpaceInvadersStatus::Lost => "The fleet reached your ship".into(),
    }
}

fn cell_x(layout: Layout, column: u8) -> f32 {
    layout.board.x + f32::from(column) * layout.cell
}
fn cell_y(layout: Layout, row: u8) -> f32 {
    layout.board.y + f32::from(row) * layout.cell
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(11., large_text);
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - crate::ui::measure_text(label, None, size as u16, 1.).width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        22.
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
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 115., 42.)
}

#[cfg(test)]
mod tests;
