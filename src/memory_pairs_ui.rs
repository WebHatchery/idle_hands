//! Responsive presentation and touch routing for Memory/Pairs.

use crate::{memory_pairs::MemoryStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    new_board: Rect,
    undo: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(10., 48., 300., 300.),
            cell: 75.,
            new_board: Rect::new(350., 125., 155., 48.),
            undo: Rect::new(350., 185., 155., 48.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 130., 340., 340.),
            cell: 85.,
            new_board: Rect::new(10., 525., 160., 48.),
            undo: Rect::new(180., 525., 160., 48.),
        }
    } else {
        Layout {
            board: Rect::new(380., 100., 520., 520.),
            cell: 130.,
            new_board: Rect::new(440., 650., 180., 48.),
            undo: Rect::new(650., 650., 180., 48.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.new_board.contains(point) {
        return vec![UiAction::MemoryPairsNew];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::MemoryPairsUndo];
    }
    if layout.board.contains(point) && state.memory_pairs.status != MemoryStatus::Won {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 4 && column < 4 {
            return vec![UiAction::MemoryPairsSelect(row * 4 + column)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.memory_pairs;
    let header_y = if crate::ui::is_compact_landscape() {
        35.
    } else if crate::ui::is_portrait() {
        87.
    } else {
        72.
    };
    let header_x = if crate::ui::is_compact_landscape() {
        120.
    } else {
        layout.board.x
    };
    let body_x = if crate::ui::is_compact_landscape() {
        layout.new_board.x
    } else {
        header_x
    };
    let body_y = if crate::ui::is_compact_landscape() {
        layout.new_board.y - 34.
    } else {
        header_y + 28.
    };
    text(
        "‹ CABINET",
        back_rect().x,
        back_rect().y + 20.,
        14.,
        muted(),
    );
    text("MEMORY / PAIRS", header_x, header_y, title_size(), accent());
    text(
        &status_text(game.status, game.matched_pairs),
        body_x,
        body_y,
        body_size(),
        muted(),
    );
    for index in 0..game.cards.len() {
        let row = index / 4;
        let column = index % 4;
        let rect = Rect::new(
            layout.board.x + column as f32 * layout.cell,
            layout.board.y + row as f32 * layout.cell,
            layout.cell - 4.,
            layout.cell - 4.,
        );
        let card = game.cards[index];
        let face = card.face_up || card.matched;
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if face {
                pair_color(card.pair)
            } else {
                Color::new(0.16, 0.11, 0.25, 1.)
            },
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
        if face {
            text(
                &pair_label(card.pair),
                rect.x + rect.w * 0.38,
                rect.y + rect.h * 0.62,
                (rect.w * 0.34).min(40.),
                WHITE,
            );
        } else {
            draw_circle_lines(
                rect.center().x,
                rect.center().y,
                rect.w * 0.18,
                2.,
                Color::new(0.60, 0.48, 0.78, 1.),
            );
        }
    }
    text(
        &format!("MOVES  {}  •  PAIRS  {}/8", game.moves, game.matched_pairs),
        layout.board.x,
        layout.board.bottom() + 28.,
        body_size(),
        muted(),
    );
    button(layout.new_board, "NEW BOARD");
    button(layout.undo, "UNDO");
}

fn status_text(status: MemoryStatus, pairs: u8) -> String {
    match status {
        MemoryStatus::Playing => format!("Find the quiet pairs  •  {} of 8 found", pairs),
        MemoryStatus::Won => "Every pair is resting. Start another board to play again.".into(),
    }
}

fn pair_label(pair: u8) -> String {
    char::from(b'A' + pair).to_string()
}

fn pair_color(pair: u8) -> Color {
    let palette = [
        (0.35, 0.55, 0.78),
        (0.55, 0.38, 0.72),
        (0.35, 0.68, 0.58),
        (0.78, 0.48, 0.35),
        (0.68, 0.55, 0.30),
        (0.48, 0.65, 0.35),
        (0.62, 0.42, 0.62),
        (0.36, 0.62, 0.68),
    ];
    let (r, g, b) = palette[pair as usize % palette.len()];
    Color::new(r, g, b, 1.)
}

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(10., 8., 100., 30.)
    } else if crate::ui::is_portrait() {
        Rect::new(10., 52., 110., 30.)
    } else {
        Rect::new(48., 28., 130., 36.)
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.14, 0.31, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
    text(
        label,
        rect.x + 16.,
        rect.y + rect.h * 0.64,
        body_size(),
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.76, 0.70, 0.86, 1.)
}

fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        26.
    } else {
        32.
    }
}

fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        12.
    } else {
        15.
    }
}
