//! Responsive presentation and touch routing for Peg Solitaire.

use crate::{
    accessibility,
    peg_solitaire::{Hole, PegSolitaireStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(18., 48., 280., 280.),
            cell: 40.,
            hint: Rect::new(350., 275., 290., 42.),
            undo: Rect::new(350., 220., 120., 42.),
            new_game: Rect::new(490., 220., 150., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 105., 340., 340.),
            cell: 48.5714,
            hint: Rect::new(20., 585., 330., 42.),
            undo: Rect::new(20., 530., 145., 42.),
            new_game: Rect::new(185., 530., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(360., 82., 560., 560.),
            cell: 80.,
            hint: Rect::new(950., 495., 290., 44.),
            undo: Rect::new(950., 555., 120., 44.),
            new_game: Rect::new(1090., 555., 150., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::PegSolitaireUndo];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::PegSolitaireNew];
    }
    if layout.hint.contains(point) {
        return vec![UiAction::PegSolitaireHint];
    }
    if layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 7 && column < 7 {
            return vec![UiAction::PegSolitaireTap(row * 7 + column)];
        }
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.peg_solitaire;
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
        68.
    } else {
        60.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "PEG SOLITAIRE",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let instruction = state
        .card_hint
        .as_deref()
        .unwrap_or(status_text(game.status));
    text(
        instruction,
        if compact { 350. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(game, layout, state.high_contrast);
    text(
        "TAP A PEG, THEN A TWO-STEP DESTINATION",
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
            475.
        } else {
            680.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    text(
        &format!("Moves {}  •  Leave one peg in the center", game.moves),
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
            500.
        } else {
            710.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_game, "NEW BOARD", state.large_text);
    button(layout.hint, "HINT", state.large_text);
}

fn draw_board(game: &crate::peg_solitaire::PegSolitaire, layout: Layout, high_contrast: bool) {
    for row in 0..7 {
        for column in 0..7 {
            let index = row * 7 + column;
            if !crate::peg_solitaire::valid_hole(index) {
                continue;
            }
            let x = layout.board.x + column as f32 * layout.cell;
            let y = layout.board.y + row as f32 * layout.cell;
            draw_rectangle(
                x,
                y,
                layout.cell,
                layout.cell,
                if high_contrast {
                    Color::new(0.20, 0.14, 0.32, 1.)
                } else {
                    Color::new(0.27, 0.18, 0.34, 1.)
                },
            );
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
            draw_circle(
                x + layout.cell / 2.,
                y + layout.cell / 2.,
                layout.cell * 0.32,
                match game.cells[index] {
                    Hole::Peg => {
                        if high_contrast {
                            Color::new(1., 0.85, 0.05, 1.)
                        } else {
                            Color::new(0.98, 0.75, 0.30, 1.)
                        }
                    }
                    Hole::Empty => accessibility::board_fill(high_contrast),
                },
            );
            draw_circle_lines(
                x + layout.cell / 2.,
                y + layout.cell / 2.,
                layout.cell * 0.32,
                1.,
                accessibility::grid_line(high_contrast),
            );
        }
    }
}

fn status_text(status: PegSolitaireStatus) -> &'static str {
    match status {
        PegSolitaireStatus::Playing => "Clear the quiet board",
        PegSolitaireStatus::Won => "One peg remains",
        PegSolitaireStatus::Stuck => "No jumps remain",
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
        26.
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
