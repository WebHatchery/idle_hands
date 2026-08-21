//! Responsive touch presentation for Dots and Boxes.

use crate::{
    accessibility,
    dots_boxes::{DotsBoxes, DotsDifficulty, DotsPhase, Edge},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    difficulty: [Rect; 3],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(250., 54., 300., 300.),
            hint: Rect::new(610., 225., 110., 44.),
            undo: Rect::new(610., 115., 110., 44.),
            new_game: Rect::new(610., 170., 145., 44.),
            difficulty: [
                Rect::new(610., 54., 68., 44.),
                Rect::new(682., 54., 68., 44.),
                Rect::new(754., 54., 76., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 120., 320., 320.),
            hint: Rect::new(20., 545., 145., 44.),
            undo: Rect::new(20., 490., 145., 44.),
            new_game: Rect::new(175., 490., 165., 44.),
            difficulty: [
                Rect::new(20., 600., 90., 44.),
                Rect::new(120., 600., 90., 44.),
                Rect::new(220., 600., 100., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 105., 420., 420.),
            hint: Rect::new(810., 245., 120., 44.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
            difficulty: [
                Rect::new(810., 105., 85., 44.),
                Rect::new(900., 105., 85., 44.),
                Rect::new(990., 105., 95., 44.),
            ],
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.difficulty.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::DotsDifficulty(DotsDifficulty::ALL[index])];
        }
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::DotsUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::DotsHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::DotsNew];
    }
    edge_at(l.board, point, _state.games.dots_boxes.side())
        .map_or_else(Vec::new, |edge| vec![UiAction::DotsEdge(edge)])
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.dots_boxes;
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
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        if compact {
            "DOTS & BOXES"
        } else {
            "DOTS AND BOXES"
        },
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let score_status = if compact {
        format!(
            "Red {}  •  Blue {}  •  {} moves  •  {}",
            game.scores[0],
            game.scores[1],
            game.moves,
            game.difficulty.label()
        )
    } else if portrait {
        portrait_score_status(
            game.scores[0] as usize,
            game.scores[1] as usize,
            game.moves as usize,
        )
    } else {
        format!(
            "You {}  •  Red {}  Blue {}  •  {} moves  •  {}",
            if game.current_player == 0 {
                "draw"
            } else {
                "wait"
            },
            game.scores[0],
            game.scores[1],
            game.moves,
            game.difficulty.label()
        )
    };
    text(
        &score_status,
        if compact { 70. } else { title_x },
        if compact { 55. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(status_text(game.phase)),
        if compact {
            250.
        } else if portrait {
            20.
        } else {
            title_x
        },
        if portrait {
            465.
        } else if compact {
            375.
        } else {
            555.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
    for (index, rect) in l.difficulty.iter().enumerate() {
        mode_button(
            *rect,
            ["STD", "HARD", "EXPERT"][index],
            DotsDifficulty::ALL[index] == game.difficulty,
            state.large_text,
        );
    }
}

fn draw_board(board: Rect, game: &DotsBoxes, high_contrast: bool, large_text: bool) {
    let side = game.side();
    let step = board.w / side as f32;
    for row in 0..=side {
        for col in 0..side {
            draw_line(
                board.x + col as f32 * step,
                board.y + row as f32 * step,
                board.x + (col + 1) as f32 * step,
                board.y + row as f32 * step,
                2.,
                accessibility::grid_line(high_contrast),
            );
        }
    }
    for row in 0..side {
        for col in 0..=side {
            draw_line(
                board.x + col as f32 * step,
                board.y + row as f32 * step,
                board.x + col as f32 * step,
                board.y + (row + 1) as f32 * step,
                2.,
                accessibility::grid_line(high_contrast),
            );
        }
    }
    for row in 0..side {
        for col in 0..side {
            let owner = game.boxes[row * side + col];
            if owner != 0 {
                draw_rectangle(
                    board.x + col as f32 * step + 5.,
                    board.y + row as f32 * step + 5.,
                    step - 10.,
                    step - 10.,
                    if owner == 1 {
                        if high_contrast {
                            Color::new(0.95, 0.10, 0.18, 1.)
                        } else {
                            Color::new(0.37, 0.18, 0.22, 1.)
                        }
                    } else {
                        if high_contrast {
                            Color::new(0.08, 0.45, 1., 1.)
                        } else {
                            Color::new(0.18, 0.24, 0.40, 1.)
                        }
                    },
                );
                center_text(
                    if owner == 1 { "R" } else { "B" },
                    Rect::new(
                        board.x + col as f32 * step,
                        board.y + row as f32 * step,
                        step,
                        step,
                    ),
                    accessibility::text_size(
                        if crate::ui::is_portrait() { 18. } else { 24. },
                        large_text,
                    ),
                    WHITE,
                );
            } else if game.box_edge_count(row, col) == 3 {
                let center = vec2(
                    board.x + (col as f32 + 0.5) * step,
                    board.y + (row as f32 + 0.5) * step,
                );
                draw_circle_lines(center.x, center.y, step * 0.16, 2., accent());
                center_text(
                    "!",
                    Rect::new(
                        board.x + col as f32 * step,
                        board.y + row as f32 * step,
                        step,
                        step,
                    ),
                    accessibility::text_size(10., large_text),
                    accent(),
                );
            }
        }
    }
    for row in 0..=side {
        for col in 0..side {
            let index = row * side + col;
            if game.horizontal[index] {
                draw_line(
                    board.x + col as f32 * step,
                    board.y + row as f32 * step,
                    board.x + (col + 1) as f32 * step,
                    board.y + row as f32 * step,
                    6.,
                    edge_color(index, true, game, high_contrast),
                );
            }
        }
    }
    for row in 0..side {
        for col in 0..=side {
            let index = row * (side + 1) + col;
            if game.vertical[index] {
                draw_line(
                    board.x + col as f32 * step,
                    board.y + row as f32 * step,
                    board.x + col as f32 * step,
                    board.y + (row + 1) as f32 * step,
                    6.,
                    edge_color(index, false, game, high_contrast),
                );
            }
        }
    }
    for row in 0..=side {
        for col in 0..=side {
            draw_circle(
                board.x + col as f32 * step,
                board.y + row as f32 * step,
                7.,
                if high_contrast { WHITE } else { accent() },
            );
        }
    }
}

fn edge_at(board: Rect, point: Vec2, side: usize) -> Option<Edge> {
    let step = board.w / side as f32;
    let local = point - vec2(board.x, board.y);
    if local.x < -20. || local.y < -20. || local.x > board.w + 20. || local.y > board.h + 20. {
        return None;
    }
    let column = (local.x / step).floor() as usize;
    let row = (local.y / step).floor() as usize;
    let horizontal_row = (local.y / step).round().clamp(0., side as f32) as usize;
    let vertical_column = (local.x / step).round().clamp(0., side as f32) as usize;
    let dx = (local.x - vertical_column as f32 * step).abs();
    let dy = (local.y - horizontal_row as f32 * step).abs();
    if dy < 20. && column < side {
        Some(Edge::Horizontal(horizontal_row * side + column))
    } else if dx < 20. && row < side {
        Some(Edge::Vertical(row * (side + 1) + vertical_column))
    } else {
        None
    }
}

fn edge_color(index: usize, horizontal: bool, game: &DotsBoxes, high_contrast: bool) -> Color {
    let owner = if horizontal {
        game.edge_owner(Edge::Horizontal(index))
    } else {
        game.edge_owner(Edge::Vertical(index))
    };
    if owner == 1 {
        if high_contrast {
            Color::new(1., 0.12, 0.18, 1.)
        } else {
            Color::new(0.96, 0.36, 0.45, 1.)
        }
    } else if owner == 2 {
        if high_contrast {
            Color::new(0.10, 0.65, 1., 1.)
        } else {
            Color::new(0.40, 0.67, 1., 1.)
        }
    } else {
        WHITE
    }
}

fn status_text(phase: DotsPhase) -> &'static str {
    match phase {
        DotsPhase::Playing => "Draw a line beside an open box",
        DotsPhase::Won => "The red boxes hold the board",
        DotsPhase::Lost => "The blue boxes hold the board",
    }
}

fn portrait_score_status(red: usize, blue: usize, moves: usize) -> String {
    format!("Red {}  •  Blue {}  •  {} moves", red, blue, moves)
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

fn mode_button(rect: Rect, label: &str, selected: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            crate::theme::MOSS_DARK
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2. } else { 1. },
        accent(),
    );
    center_text(
        label,
        rect,
        accessibility::text_size(10., large_text),
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

fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
