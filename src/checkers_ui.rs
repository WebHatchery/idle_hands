//! Responsive presentation and touch routing for Checkers.

use crate::{
    accessibility,
    checkers::{CheckersStatus, Piece, Side},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(18., 48., 280., 280.),
            cell: 35.,
            undo: Rect::new(350., 220., 120., 42.),
            new_game: Rect::new(490., 220., 150., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 115., 340., 340.),
            cell: 42.5,
            undo: Rect::new(20., 535., 145., 42.),
            new_game: Rect::new(185., 535., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(360., 95., 560., 560.),
            cell: 70.,
            undo: Rect::new(950., 560., 120., 44.),
            new_game: Rect::new(1090., 560., 150., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::CheckersUndo];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::CheckersNew];
    }
    if layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 8 && column < 8 {
            return vec![UiAction::CheckersTap(row * 8 + column)];
        }
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.checkers;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        360.
    };
    let header_y = if compact {
        30.
    } else if portrait {
        72.
    } else {
        68.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "CHECKERS",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        status_text(game.status, game.turn),
        if compact { 350. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(game, layout, state.high_contrast);
    text(
        "TAP A PIECE, THEN A DESTINATION",
        if compact {
            350.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            150.
        } else if portrait {
            490.
        } else {
            690.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    text(
        &format!("Moves {}  •  Red is you  •  Yellow answers", game.moves),
        if compact {
            350.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            175.
        } else if portrait {
            515.
        } else {
            720.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_game, "NEW BOARD", state.large_text);
}

fn draw_board(game: &crate::checkers::Checkers, layout: Layout, high_contrast: bool) {
    draw_rectangle(
        layout.board.x,
        layout.board.y,
        layout.board.w,
        layout.board.h,
        if high_contrast {
            Color::new(0.04, 0.10, 0.24, 1.)
        } else {
            Color::new(0.12, 0.16, 0.30, 1.)
        },
    );
    for row in 0..8 {
        for column in 0..8 {
            let x = layout.board.x + column as f32 * layout.cell;
            let y = layout.board.y + row as f32 * layout.cell;
            let dark = (row + column) % 2 == 1;
            draw_rectangle(
                x,
                y,
                layout.cell,
                layout.cell,
                if dark {
                    if high_contrast {
                        Color::new(0.20, 0.12, 0.32, 1.)
                    } else {
                        Color::new(0.27, 0.18, 0.34, 1.)
                    }
                } else {
                    if high_contrast {
                        Color::new(0.72, 0.72, 0.72, 1.)
                    } else {
                        Color::new(0.72, 0.60, 0.45, 1.)
                    }
                },
            );
            let index = row * 8 + column;
            if game.selected == Some(index) {
                draw_rectangle_lines(
                    x + 2.,
                    y + 2.,
                    layout.cell - 4.,
                    layout.cell - 4.,
                    3.,
                    accent(),
                );
            }
            draw_piece(
                game.cells[index],
                x + layout.cell / 2.,
                y + layout.cell / 2.,
                layout.cell * 0.34,
                high_contrast,
            );
        }
    }
}

fn draw_piece(piece: Piece, x: f32, y: f32, radius: f32, high_contrast: bool) {
    if piece == Piece::Empty {
        return;
    }
    let color = match piece {
        Piece::RedMan | Piece::RedKing => {
            if high_contrast {
                Color::new(1., 0.12, 0.18, 1.)
            } else {
                Color::new(0.90, 0.30, 0.35, 1.)
            }
        }
        Piece::YellowMan | Piece::YellowKing => {
            if high_contrast {
                Color::new(1., 0.85, 0.05, 1.)
            } else {
                Color::new(0.98, 0.75, 0.30, 1.)
            }
        }
        Piece::Empty => BLACK,
    };
    draw_circle(x, y, radius, color);
    draw_circle_lines(x, y, radius, 2., Color::new(0.08, 0.05, 0.12, 1.));
    if matches!(piece, Piece::RedKing | Piece::YellowKing) {
        draw_text(
            "K",
            x - radius * 0.3,
            y + radius * 0.3,
            radius,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
}

fn status_text(status: CheckersStatus, turn: Side) -> &'static str {
    match status {
        CheckersStatus::Playing if turn == Side::Red => "Your turn",
        CheckersStatus::Playing => "Yellow is thinking",
        CheckersStatus::Won(Side::Red) => "Red takes the board",
        CheckersStatus::Won(Side::Yellow) => "Yellow takes the board",
        CheckersStatus::Draw => "The board rests",
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
    text(
        label,
        rect.x + 12.,
        rect.y + 28.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        29.
    } else {
        31.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
