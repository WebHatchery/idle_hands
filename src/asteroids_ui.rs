//! Responsive presentation and touch routing for Asteroids.

use crate::{
    accessibility,
    asteroids::{Asteroid, Asteroids, AsteroidsStatus, ShipDirection, HEIGHT, WIDTH},
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
        Layout {
            board: Rect::new(10., 42., 400., 280.),
            cell: 20.,
            left: Rect::new(430., 70., 62., 42.),
            right: Rect::new(500., 70., 62., 42.),
            fire: Rect::new(430., 120., 132., 42.),
            pause: Rect::new(430., 172., 62., 38.),
            undo: Rect::new(500., 172., 62., 38.),
            new_game: Rect::new(430., 218., 132., 38.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 102., 340., 238.),
            cell: 17.,
            left: Rect::new(20., 380., 100., 44.),
            right: Rect::new(130., 380., 100., 44.),
            fire: Rect::new(240., 380., 110., 44.),
            pause: Rect::new(20., 435., 145., 42.),
            undo: Rect::new(185., 435., 145., 42.),
            new_game: Rect::new(20., 490., 310., 42.),
        }
    } else {
        Layout {
            board: Rect::new(330., 100., 600., 420.),
            cell: 30.,
            left: Rect::new(970., 155., 82., 44.),
            right: Rect::new(1060., 155., 82., 44.),
            fire: Rect::new(1150., 155., 102., 44.),
            pause: Rect::new(970., 214., 82., 42.),
            undo: Rect::new(1060., 214., 82., 42.),
            new_game: Rect::new(1150., 214., 102., 42.),
        }
    }
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
            return vec![UiAction::AsteroidsStep(direction)];
        }
    }
    if crate::ui::hit(l.fire, point) {
        return vec![UiAction::AsteroidsFire];
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::AsteroidsPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::AsteroidsUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::AsteroidsNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.asteroids;
    let (title_x, title_y) = if crate::ui::is_compact_landscape() {
        (10., 26.)
    } else if crate::ui::is_portrait() {
        (10., 68.)
    } else {
        (330., 60.)
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("ASTEROIDS", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Score {} / {}  •  {} lives",
            game.score,
            Asteroids::target_score(),
            game.lives
        ),
        if crate::ui::is_compact_landscape() {
            210.
        } else {
            title_x
        },
        if crate::ui::is_compact_landscape() {
            26.
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
                Color::new(0.25, 0.22, 0.40, 0.45),
            );
        }
    }
    for asteroid in &game.asteroids {
        draw_asteroid(l, asteroid);
    }
    if let Some(shot) = game.shot {
        draw_rectangle(
            cell_x(l, shot.x) + l.cell * 0.43,
            cell_y(l, shot.y),
            l.cell * 0.14,
            l.cell * 0.65,
            Color::new(1., 0.78, 0.28, 1.),
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
        Color::new(0.42, 0.90, 1., 1.),
    );
    let status = status_text(game);
    text(
        state.card_hint.as_deref().unwrap_or(&status),
        if crate::ui::is_compact_landscape() {
            10.
        } else {
            title_x
        },
        if crate::ui::is_portrait() {
            360.
        } else if crate::ui::is_compact_landscape() {
            342.
        } else {
            550.
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
    button(l.new_game, "NEW FIELD", state.large_text);
}

fn draw_asteroid(layout: Layout, asteroid: &Asteroid) {
    let x = cell_x(layout, asteroid.x) + layout.cell * 0.5;
    let y = cell_y(layout, asteroid.y) + layout.cell * 0.5;
    let radius = layout.cell * (0.25 + f32::from(asteroid.size) * 0.08);
    draw_circle_lines(x, y, radius, 2., Color::new(0.72, 0.56, 0.48, 1.));
    draw_line(
        x - radius * 0.7,
        y - radius * 0.2,
        x + radius * 0.7,
        y + radius * 0.2,
        1.,
        Color::new(0.48, 0.38, 0.42, 1.),
    );
}

fn status_text(game: &Asteroids) -> String {
    match game.status {
        AsteroidsStatus::Playing => "Tap LEFT or RIGHT to dodge • tap FIRE to split rocks".into(),
        AsteroidsStatus::Won => "The asteroid belt is clear".into(),
        AsteroidsStatus::Lost => "Your ship has run out of lives".into(),
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
