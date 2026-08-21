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
    sonar: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            sonar: Rect::new(735., 220., 90., 44.),
            hint: Rect::new(620., 220., 105., 44.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 95., 300., 300.),
            sonar: Rect::new(170., 440., 145., 44.),
            hint: Rect::new(15., 440., 145., 44.),
            undo: Rect::new(15., 495., 145., 44.),
            new_game: Rect::new(170., 495., 145., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            sonar: Rect::new(950., 245., 140., 44.),
            hint: Rect::new(810., 245., 120., 44.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
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
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::BattleshipHint];
    }
    if crate::ui::hit(l.sonar, point) {
        return vec![UiAction::BattleshipSonar];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::BattleshipUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::BattleshipNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.battleship;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        crate::ui::COMPACT_HEADER_TITLE_X
    } else if portrait {
        25.
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
    crate::ui::draw_text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    crate::ui::draw_text(
        "BATTLESHIP",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let scoreline = if compact || portrait {
        format!(
            "{}/{} sunk • H{}/{}",
            game.sunk_ships(),
            game.ship_count(),
            game.hits(),
            game.ship_cells()
        )
    } else if screen_width() < 360. {
        format!(
            "{}/{} sunk • P{} • C{}",
            game.sunk_ships(),
            game.ship_count(),
            game.score,
            game.streak
        )
    } else {
        format!(
            "Ships {}/{}  •  Hits {}/{}  •  Points {}  •  Chain {}",
            game.sunk_ships(),
            game.ship_count(),
            game.hits(),
            game.ship_cells(),
            game.score,
            game.streak
        )
    };
    crate::ui::draw_text(
        scoreline,
        if compact {
            crate::ui::COMPACT_HEADER_STATUS_X
        } else {
            title_x
        },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    button(l.hint, "HINT", state.large_text);
    mode_button(
        l.sonar,
        &format!("SONAR ×{}", game.sonar_charges),
        game.sonar_armed,
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW FLEET", state.large_text);
    let status_y = if portrait {
        420.
    } else if compact {
        365.
    } else {
        545.
    };
    crate::ui::draw_text(
        state.card_hint.as_deref().unwrap_or(status_text(game)),
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
        let sunk = shot == Shot::Hit && game.ship_sunk(game.ships[index]);
        let scanned = game.is_scanned(index);
        let contact = scanned && game.ships[index] != 0 && shot == Shot::Unknown;
        let fill = match shot {
            Shot::Unknown => {
                if contact {
                    Color::new(0.38, 0.28, 0.10, 1.)
                } else if scanned {
                    Color::new(0.08, 0.30, 0.38, 1.)
                } else if high_contrast {
                    Color::new(0.05, 0.24, 0.42, 1.)
                } else {
                    Color::new(0.11, 0.18, 0.30, 1.)
                }
            }
            Shot::Miss => accessibility::board_fill(high_contrast),
            Shot::Hit if sunk => Color::new(0.54, 0.42, 0.18, 1.),
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
            Shot::Hit => center_text(
                if sunk { "S" } else { "×" },
                rect,
                accessibility::text_size(24., large_text),
                WHITE,
            ),
            Shot::Unknown if contact => center_text(
                "!",
                rect,
                accessibility::text_size(20., large_text),
                accent(),
            ),
            Shot::Unknown if scanned => center_text(
                "~",
                rect,
                accessibility::text_size(18., large_text),
                muted(),
            ),
            Shot::Unknown => {}
        }
    }
}

fn status_text(game: &Battleship) -> &'static str {
    match game.phase {
        BattleshipPhase::Playing if game.sonar_armed => "Tap the center of a 3 × 3 SONAR sweep",
        BattleshipPhase::Playing if game.contact_count() > 0 => {
            "SONAR contact ! marks a ship segment — tap it to fire"
        }
        BattleshipPhase::Playing => "Tap unknown waters to fire, or arm SONAR",
        BattleshipPhase::Won => "Every vessel has been sunk",
    }
}

fn mode_button(rect: Rect, label: &str, active: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active {
            Color::new(0.22, 0.36, 0.22, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
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
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
