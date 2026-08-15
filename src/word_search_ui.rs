//! Responsive presentation and touch routing for Word Search.

use crate::{
    state::AppState,
    ui::UiAction,
    word_search::{WordSearchStatus, WORDS},
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    clear: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(8., 48., 300., 300.),
            cell: 30.,
            clear: Rect::new(370., 245., 120., 42.),
            new_game: Rect::new(510., 245., 140., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 145., 340., 340.),
            cell: 34.,
            clear: Rect::new(20., 660., 150., 42.),
            new_game: Rect::new(190., 660., 160., 42.),
        }
    } else {
        Layout {
            board: Rect::new(350., 105., 560., 560.),
            cell: 56.,
            clear: Rect::new(950., 575., 120., 44.),
            new_game: Rect::new(1090., 575., 150., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.clear.contains(point) {
        return vec![UiAction::WordSearchClear];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::WordSearchNew];
    }
    if layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < 10 && column < 10 {
            return vec![UiAction::WordSearchCell(row * 10 + column)];
        }
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.word_search;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        350.
    };
    let header_y = if compact {
        30.
    } else if portrait {
        76.
    } else {
        72.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("WORD SEARCH", header_x, header_y, title_size(), accent());
    text(
        match game.status {
            WordSearchStatus::Playing => "Tap two endpoints to find a word",
            WordSearchStatus::Won => "The quiet list is complete",
        },
        if compact { 350. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        body_size(),
        muted(),
    );
    for row in 0..10 {
        for column in 0..10 {
            let index = row * 10 + column;
            let rect = Rect::new(
                layout.board.x + column as f32 * layout.cell,
                layout.board.y + row as f32 * layout.cell,
                layout.cell,
                layout.cell,
            );
            let fill = if game.cell_found(index) {
                Color::new(0.25, 0.45, 0.34, 1.)
            } else if game.selected_start == Some(index) {
                Color::new(0.45, 0.30, 0.18, 1.)
            } else {
                Color::new(0.12, 0.09, 0.20, 1.)
            };
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.,
                Color::new(0.35, 0.28, 0.48, 1.),
            );
            let letter = char::from(b'A' + game.cells[index]);
            let size = if portrait {
                16.
            } else if compact {
                15.
            } else {
                25.
            };
            let letter_text = letter.to_string();
            let width = measure_text(letter_text.clone(), None, size as u16, 1.).width;
            draw_text(
                &letter_text,
                rect.x + (rect.w - width) / 2.,
                rect.y + rect.h * 0.68,
                size,
                Color::new(0.94, 0.90, 0.78, 1.),
            );
        }
    }
    draw_word_list(game, layout);
    text(
        &format!(
            "Found {} / {}  •  Moves {}",
            game.found.iter().filter(|found| **found).count(),
            WORDS.len(),
            game.moves
        ),
        if compact {
            370.
        } else if portrait {
            10.
        } else {
            950.
        },
        if compact {
            230.
        } else if portrait {
            530.
        } else {
            520.
        },
        body_size(),
        muted(),
    );
    button(layout.clear, "CLEAR");
    button(layout.new_game, "NEW BOARD");
}

fn draw_word_list(game: &crate::word_search::WordSearch, _layout: Layout) {
    if crate::ui::is_portrait() {
        text("FIND THESE", 20., 548., 13., accent());
        for (index, word) in WORDS.iter().enumerate() {
            let x = 20. + (index / 3) as f32 * 180.;
            let y = 575. + (index % 3) as f32 * 22.;
            text(
                &format!("{} {}", if game.found[index] { "✓" } else { "·" }, word),
                x,
                y,
                13.,
                if game.found[index] {
                    Color::new(0.55, 1., 0.72, 1.)
                } else {
                    muted()
                },
            );
        }
        return;
    }
    let (x, y, size) = if crate::ui::is_compact_landscape() {
        (370., 80., 13.)
    } else {
        (950., 155., 18.)
    };
    text("FIND THESE", x, y, size, accent());
    for (index, word) in WORDS.iter().enumerate() {
        text(
            &format!("{} {}", if game.found[index] { "✓" } else { "·" }, word),
            x,
            y + 28. + index as f32 * if size > 15. { 30. } else { 22. },
            size,
            if game.found[index] {
                Color::new(0.55, 1., 0.72, 1.)
            } else {
                muted()
            },
        );
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(label, rect.x + 12., rect.y + 27., 11., WHITE);
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
