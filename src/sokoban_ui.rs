//! Responsive touch presentation for Sokoban.

use crate::domain::Direction;
use crate::{
    accessibility,
    sokoban::{Sokoban, SokobanPhase, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    directions: [Rect; 4],
    hint: Rect,
    undo: Rect,
    restart: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            directions: [
                Rect::new(18., 82., 62., 44.),
                Rect::new(18., 127., 62., 44.),
                Rect::new(18., 172., 62., 44.),
                Rect::new(18., 217., 62., 44.),
            ],
            hint: Rect::new(610., 220., 110., 44.),
            undo: Rect::new(610., 110., 110., 44.),
            restart: Rect::new(610., 275., 145., 44.),
            new_game: Rect::new(610., 165., 145., 44.),
        }
    } else if crate::ui::is_portrait() && screen_height() < 700. {
        Layout {
            board: Rect::new(32., 96., 256., 256.),
            directions: [
                Rect::new(15., 360., 62., 44.),
                Rect::new(91., 360., 62., 44.),
                Rect::new(167., 360., 62., 44.),
                Rect::new(243., 360., 62., 44.),
            ],
            hint: Rect::new(15., 468., 140., 44.),
            undo: Rect::new(15., 414., 140., 44.),
            restart: Rect::new(165., 468., 140., 44.),
            new_game: Rect::new(165., 414., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 105., 300., 300.),
            directions: [
                Rect::new(15., 420., 60., 44.),
                Rect::new(90., 420., 60., 44.),
                Rect::new(165., 420., 60., 44.),
                Rect::new(240., 420., 60., 44.),
            ],
            hint: Rect::new(15., 580., 145., 44.),
            undo: Rect::new(15., 525., 145., 44.),
            restart: Rect::new(170., 580., 145., 44.),
            new_game: Rect::new(170., 525., 145., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            directions: [
                Rect::new(810., 130., 62., 44.),
                Rect::new(882., 130., 62., 44.),
                Rect::new(954., 130., 62., 44.),
                Rect::new(1026., 130., 62., 44.),
            ],
            hint: Rect::new(810., 270., 120., 44.),
            undo: Rect::new(810., 205., 120., 44.),
            restart: Rect::new(950., 270., 140., 44.),
            new_game: Rect::new(950., 205., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.directions.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::SokobanMove(
                [
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ][index],
            )];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::SokobanHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::SokobanUndo];
    }
    if crate::ui::hit(l.restart, point) {
        return vec![UiAction::SokobanRestart];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::SokobanNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.sokoban;
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
        55.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "SOKOBAN",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        &format!(
            "Room {}/{}  •  Moves {}/{}  •  Pushes {}  •  {}",
            game.level + 1,
            crate::sokoban::LEVEL_COUNT,
            game.moves,
            game.par_moves(),
            game.pushes,
            if game.won() {
                game.clear_rank()
            } else if game.phase == SokobanPhase::Stuck {
                "CORNERED"
            } else {
                "PUSH TO MARKS"
            }
        ),
        if compact { 420. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(status_text(game.phase)),
        if compact { 270. } else { title_x },
        if portrait {
            if screen_height() < 700. {
                535.
            } else {
                480.
            }
        } else if compact {
            365.
        } else {
            545.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(l.hint, "HINT", state.large_text);
    for (rect, label) in l.directions.iter().zip(["UP", "LEFT", "DOWN", "RIGHT"]) {
        button(*rect, label, state.large_text);
    }
    button(l.undo, "UNDO", state.large_text);
    button(l.restart, "RESTART", state.large_text);
    button(l.new_game, "NEXT ROOM", state.large_text);
}

fn draw_board(board: Rect, game: &Sokoban, high_contrast: bool, large_text: bool) {
    let cell = board.w / WIDTH as f32;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let index = row * WIDTH + col;
            let rect = Rect::new(
                board.x + col as f32 * cell,
                board.y + row as f32 * cell,
                cell,
                cell,
            );
            let tile = game.tiles[index];
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                tile_color(tile, high_contrast),
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.,
                line_color(high_contrast),
            );
            if tile == 2 || tile == 4 {
                draw_circle_lines(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.5,
                    rect.w * 0.22,
                    2.,
                    if high_contrast { WHITE } else { accent() },
                );
            }
            if tile == 3 || tile == 4 {
                let inset = rect.w * 0.19;
                draw_rectangle(
                    rect.x + inset,
                    rect.y + inset,
                    rect.w - inset * 2.,
                    rect.h - inset * 2.,
                    crate_color(tile, high_contrast),
                );
                if game.is_deadlocked_crate(index) {
                    let warning = if high_contrast { YELLOW } else { RED };
                    draw_line(
                        rect.x + inset,
                        rect.y + inset,
                        rect.right() - inset,
                        rect.bottom() - inset,
                        3.,
                        warning,
                    );
                    draw_line(
                        rect.right() - inset,
                        rect.y + inset,
                        rect.x + inset,
                        rect.bottom() - inset,
                        3.,
                        warning,
                    );
                }
            }
            if index == game.player {
                center_text("@", rect, cell_size(large_text), WHITE);
            }
        }
    }
}

fn tile_color(tile: u8, high_contrast: bool) -> Color {
    if tile == 0 {
        accessibility::board_fill(high_contrast)
    } else {
        if high_contrast {
            Color::new(0.18, 0.16, 0.24, 1.)
        } else {
            Color::new(0.18, 0.13, 0.27, 1.)
        }
    }
}

fn crate_color(tile: u8, high_contrast: bool) -> Color {
    if tile == 4 {
        if high_contrast {
            Color::new(0.05, 1., 0.30, 1.)
        } else {
            Color::new(0.42, 0.82, 0.58, 1.)
        }
    } else {
        if high_contrast {
            Color::new(1., 0.15, 0.20, 1.)
        } else {
            Color::new(0.92, 0.46, 0.46, 1.)
        }
    }
}

fn status_text(phase: SokobanPhase) -> &'static str {
    match phase {
        SokobanPhase::Playing => "Push each crate onto a marked square",
        SokobanPhase::Won => "Room clear • Tap NEXT ROOM",
        SokobanPhase::Stuck => "A crate is cornered • Tap UNDO or RESTART",
    }
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

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        23.
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

fn cell_size(large_text: bool) -> f32 {
    if crate::ui::is_portrait() {
        accessibility::text_size(17., large_text).min(21.)
    } else {
        accessibility::text_size(22., large_text).min(27.)
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
