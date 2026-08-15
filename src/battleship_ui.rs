//! Responsive touch presentation for Battleship.

use crate::{
    accessibility,
    battleship::{Battleship, BattleshipPhase, Shot, SIDE},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(5., 105., 330., 330.),
            undo: Rect::new(5., 470., 145., 44.),
            new_game: Rect::new(165., 470., 170., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.board.contains(point) {
        let cell = l.board.w / SIDE as f32;
        let col = ((point.x - l.board.x) / cell) as usize;
        let row = ((point.y - l.board.y) / cell) as usize;
        if row < SIDE && col < SIDE {
            return vec![UiAction::BattleshipFire(row * SIDE + col)];
        }
    }
    if l.undo.contains(point) {
        return vec![UiAction::BattleshipUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::BattleshipNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.battleship;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        62.
    } else {
        58.
    };
    draw_text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    draw_text(
        "BATTLESHIP",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    draw_text(
        format!("{} / 5 hits  •  {}", game.hits(), status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW FLEET", state.large_text);
    let status_y = if portrait {
        535.
    } else if compact {
        365.
    } else {
        545.
    };
    draw_text(
        "Tap unknown waters to search for the fleet",
        if compact { 270. } else { title_x },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &Battleship, high_contrast: bool, large_text: bool) {
    let cell = board.w / SIDE as f32;
    for index in 0..SIDE * SIDE {
        let rect = Rect::new(
            board.x + (index % SIDE) as f32 * cell,
            board.y + (index / SIDE) as f32 * cell,
            cell,
            cell,
        );
        let shot = game.shots[index];
        let fill = match shot {
            Shot::Unknown => {
                if high_contrast {
                    Color::new(0.05, 0.24, 0.42, 1.)
                } else {
                    Color::new(0.11, 0.18, 0.30, 1.)
                }
            }
            Shot::Miss => accessibility::board_fill(high_contrast),
            Shot::Hit => {
                if high_contrast {
                    Color::new(1., 0.15, 0.20, 1.)
                } else {
                    Color::new(0.75, 0.25, 0.30, 1.)
                }
            }
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            if shot == Shot::Hit {
                accent()
            } else {
                line_color(high_contrast)
            },
        );
        match shot {
            Shot::Miss => center_text(
                "·",
                rect,
                accessibility::text_size(30., large_text),
                muted(),
            ),
            Shot::Hit => center_text("×", rect, accessibility::text_size(24., large_text), WHITE),
            Shot::Unknown => {}
        }
    }
}

fn status(phase: BattleshipPhase) -> &'static str {
    match phase {
        BattleshipPhase::Playing => "SEARCH THE WATERS",
        BattleshipPhase::Won => "FLEET FOUND",
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
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}
fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        21.
    } else {
        27.
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
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
