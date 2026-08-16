//! Responsive touch presentation for Maze Walk.

use crate::{
    accessibility,
    maze_walk::{MazePhase, MazeWalk, SIDE},
    state::{AppState, Direction},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    arrows: [Rect; 4],
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(250., 44., 300., 300.),
            arrows: [
                Rect::new(620., 100., 54., 44.),
                Rect::new(680., 100., 54., 44.),
                Rect::new(740., 100., 54., 44.),
                Rect::new(680., 155., 54., 44.),
            ],
            hint: Rect::new(620., 275., 105., 44.),
            undo: Rect::new(620., 220., 105., 44.),
            new_game: Rect::new(735., 220., 105., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 85., 300., 300.),
            arrows: [
                Rect::new(15., 420., 70., 44.),
                Rect::new(90., 420., 70., 44.),
                Rect::new(165., 420., 70., 44.),
                Rect::new(240., 420., 70., 44.),
            ],
            hint: Rect::new(15., 475., 145., 44.),
            undo: Rect::new(5., 530., 145., 44.),
            new_game: Rect::new(165., 530., 170., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            arrows: [
                Rect::new(810., 180., 70., 44.),
                Rect::new(890., 180., 70., 44.),
                Rect::new(970., 180., 70., 44.),
                Rect::new(890., 235., 70., 44.),
            ],
            hint: Rect::new(810., 365., 120., 44.),
            undo: Rect::new(810., 310., 120., 44.),
            new_game: Rect::new(950., 310., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.arrows.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::MazeStep(
                [
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                ][index],
            )];
        }
    }
    if l.hint.contains(point) {
        return vec![UiAction::MazeHint];
    }
    if l.undo.contains(point) {
        return vec![UiAction::MazeUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::MazeNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.maze_walk;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
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
    draw_text("‹ CABINET", 8., 30., 13., muted());
    draw_text(
        "MAZE WALK",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    draw_text(
        format!("{} moves  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast);
    for (index, rect) in l.arrows.iter().enumerate() {
        button(
            *rect,
            ["LEFT", "UP", "RIGHT", "DOWN"][index],
            state.large_text,
        );
    }
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW MAZE", state.large_text);
    let status_y = if portrait {
        610.
    } else if compact {
        365.
    } else {
        545.
    };
    draw_text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a direction to walk to the glowing exit"),
        if compact { 250. } else { title_x },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &MazeWalk, high_contrast: bool) {
    let cell = board.w / SIDE as f32;
    for index in 0..SIDE * SIDE {
        let rect = Rect::new(
            board.x + (index % SIDE) as f32 * cell,
            board.y + (index / SIDE) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if index == game.goal {
                if high_contrast {
                    Color::new(0.55, 0.55, 0.60, 1.)
                } else {
                    Color::new(0.32, 0.22, 0.35, 1.)
                }
            } else {
                accessibility::board_fill(high_contrast)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            line_color(high_contrast),
        );
        let walls = game.walls[index];
        if walls & 1 != 0 {
            draw_line(
                rect.x,
                rect.y,
                rect.right(),
                rect.y,
                3.,
                wall_color(high_contrast),
            );
        }
        if walls & 2 != 0 {
            draw_line(
                rect.right(),
                rect.y,
                rect.right(),
                rect.bottom(),
                3.,
                wall_color(high_contrast),
            );
        }
        if walls & 4 != 0 {
            draw_line(
                rect.x,
                rect.bottom(),
                rect.right(),
                rect.bottom(),
                3.,
                wall_color(high_contrast),
            );
        }
        if walls & 8 != 0 {
            draw_line(
                rect.x,
                rect.y,
                rect.x,
                rect.bottom(),
                3.,
                wall_color(high_contrast),
            );
        }
        if index == game.goal {
            draw_circle(rect.center().x, rect.center().y, cell * 0.16, accent());
        }
        if index == game.player {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.22,
                player_color(high_contrast),
            );
        }
    }
}

fn status(phase: MazePhase) -> &'static str {
    match phase {
        MazePhase::Playing => "FIND THE EXIT",
        MazePhase::Won => "EXIT FOUND",
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
        accessibility::text_size(16., large_text),
        WHITE,
    );
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.67,
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
fn player_color(high_contrast: bool) -> Color {
    if high_contrast {
        Color::new(0.15, 1., 0.85, 1.)
    } else {
        Color::new(0.35, 0.82, 0.70, 1.)
    }
}
fn wall_color(high_contrast: bool) -> Color {
    if high_contrast {
        Color::new(1., 1., 1., 1.)
    } else {
        Color::new(0.72, 0.45, 0.85, 1.)
    }
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
