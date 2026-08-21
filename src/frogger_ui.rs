//! Responsive presentation and touch routing for Frogger.

use crate::{
    accessibility,
    domain::Direction,
    frogger::{Car, Frogger, FroggerStatus, HEIGHT, WIDTH},
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
        compact_layout()
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 102., 330., 330.),
            cell: 27.5,
            up: Rect::new(125., 455., 80., 42.),
            left: Rect::new(38., 502., 80., 42.),
            down: Rect::new(125., 502., 80., 42.),
            right: Rect::new(212., 502., 80., 42.),
            pause: Rect::new(20., 558., 145., 42.),
            undo: Rect::new(185., 558., 145., 42.),
            new_game: Rect::new(20., 614., 310., 42.),
        }
    } else {
        Layout {
            board: Rect::new(350., 100., 432., 432.),
            cell: 36.,
            up: Rect::new(830., 150., 90., 42.),
            left: Rect::new(830., 198., 90., 42.),
            down: Rect::new(830., 246., 90., 42.),
            right: Rect::new(830., 294., 90., 42.),
            pause: Rect::new(950., 198., 100., 42.),
            undo: Rect::new(950., 246., 100., 42.),
            new_game: Rect::new(950., 294., 100., 42.),
        }
    }
}

fn compact_layout() -> Layout {
    Layout {
        board: Rect::new(12., 56., 288., 288.),
        cell: 24.,
        up: Rect::new(320., 74., 70., 42.),
        left: Rect::new(320., 122., 70., 42.),
        down: Rect::new(320., 170., 70., 42.),
        right: Rect::new(320., 218., 70., 42.),
        pause: Rect::new(400., 122., 82., 42.),
        undo: Rect::new(400., 170., 82., 42.),
        new_game: Rect::new(400., 218., 82., 42.),
    }
}

fn compact_header() -> (f32, f32, f32, f32) {
    (120., 30., 120., 46.)
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
            return vec![UiAction::FroggerMove(direction)];
        }
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::FroggerPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::FroggerUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::FroggerNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.frogger;
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let (title_x, title_y, status_x, header_status_y) = if compact {
        let (title_x, title_y, status_x, status_y) = compact_header();
        (title_x, title_y, status_x, status_y)
    } else if portrait {
        (10., 68., 10., 93.)
    } else {
        (350., 60., 350., 85.)
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("FROGGER", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Crossings {} / {}  •  Score {}  •  {} lives",
            game.crossings,
            Frogger::target_crossings(),
            game.score,
            game.lives
        ),
        status_x,
        if compact {
            header_status_y
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
            let rect = cell_rect(l, row, column);
            let fill = if row == 0 {
                Color::new(0.15, 0.36, 0.28, 1.)
            } else if row == HEIGHT - 1 {
                Color::new(0.18, 0.34, 0.22, 1.)
            } else if row.is_multiple_of(2) {
                Color::new(0.26, 0.18, 0.23, 1.)
            } else {
                Color::new(0.12, 0.12, 0.20, 1.)
            };
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.,
                Color::new(0.35, 0.30, 0.45, 0.6),
            );
        }
    }
    for car in &game.cars {
        draw_car(l, car);
    }
    let player = cell_rect(l, game.player_row, game.player_column);
    draw_poly(
        player.x + player.w * 0.5,
        player.y + player.h * 0.52,
        4,
        player.w * 0.34,
        45.,
        Color::new(0.48, 1., 0.48, 1.),
    );
    let status = status_text(game);
    text(
        state.card_hint.as_deref().unwrap_or(&status),
        if compact { 12. } else { title_x },
        if portrait {
            442.
        } else if compact {
            356.
        } else {
            560.
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
    button(l.new_game, "NEW CROSSING", state.large_text);
}

fn draw_car(layout: Layout, car: &Car) {
    for offset in 0..car.length {
        let column = (car.x + offset) % WIDTH;
        let rect = cell_rect(layout, car.row, column);
        draw_rectangle(
            rect.x + 3.,
            rect.y + 5.,
            rect.w - 6.,
            rect.h - 10.,
            Color::new(0.88, 0.30, 0.30, 1.),
        );
        draw_circle(
            rect.x + rect.w * 0.25,
            rect.y + rect.h * 0.78,
            rect.w * 0.10,
            crate::theme::SURFACE_DARK,
        );
        draw_circle(
            rect.x + rect.w * 0.75,
            rect.y + rect.h * 0.78,
            rect.w * 0.10,
            crate::theme::SURFACE_DARK,
        );
    }
}
fn cell_rect(layout: Layout, row: u8, column: u8) -> Rect {
    Rect::new(
        layout.board.x + f32::from(column) * layout.cell,
        layout.board.y + f32::from(row) * layout.cell,
        layout.cell,
        layout.cell,
    )
}
fn status_text(game: &Frogger) -> String {
    match game.status {
        FroggerStatus::Playing => "Tap the arrows to cross • traffic keeps moving".into(),
        FroggerStatus::Won => "Three safe crossings — riverbank reached".into(),
        FroggerStatus::Lost => "The traffic took all three chances".into(),
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(10., large_text);
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
        25.
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
    Rect::new(0., 0., 110., 42.)
}

#[cfg(test)]
mod tests;
