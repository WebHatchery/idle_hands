//! Responsive touch presentation for Nim.

use crate::{accessibility, nim::NimStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

fn portrait() -> bool {
    crate::ui::is_portrait()
}

fn compact() -> bool {
    crate::ui::is_compact_landscape()
}

fn heap_rect(index: usize) -> Rect {
    if portrait() {
        Rect::new(12. + index as f32 * 122., 180., 108., 188.)
    } else if compact() {
        Rect::new(15. + index as f32 * 102., 108., 92., 170.)
    } else {
        Rect::new(250. + index as f32 * 245., 210., 190., 210.)
    }
}

fn take_rect(amount: usize) -> Rect {
    let y = if portrait() {
        430.
    } else if compact() {
        300.
    } else {
        465.
    };
    let width = if portrait() {
        108.
    } else if compact() {
        92.
    } else {
        150.
    };
    let gap = if portrait() {
        122.
    } else if compact() {
        102.
    } else {
        170.
    };
    let x = if portrait() {
        12. + amount as f32 * gap
    } else if compact() {
        15. + amount as f32 * gap
    } else {
        260. + amount as f32 * gap
    };
    Rect::new(x, y, width, 48.)
}

fn bottom_rects() -> (Rect, Rect, Rect) {
    if portrait() {
        (
            Rect::new(5., 650., 80., 42.),
            Rect::new(95., 650., 105., 42.),
            Rect::new(210., 650., 125., 42.),
        )
    } else if compact() {
        (
            Rect::new(385., 335., 105., 40.),
            Rect::new(505., 335., 105., 40.),
            Rect::new(625., 335., 105., 40.),
        )
    } else {
        (
            Rect::new(740., 625., 120., 42.),
            Rect::new(880., 625., 120., 42.),
            Rect::new(1020., 625., 140., 42.),
        )
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    for index in 0..3 {
        if heap_rect(index).contains(point) {
            return vec![UiAction::NimSelect(index)];
        }
    }
    for amount in 0..3 {
        if take_rect(amount).contains(point) {
            return vec![UiAction::NimTake(amount as u8 + 1)];
        }
    }
    let (hint, undo, new_board) = bottom_rects();
    if hint.contains(point) {
        return vec![UiAction::NimHint];
    }
    if undo.contains(point) {
        return vec![UiAction::NimUndo];
    }
    if new_board.contains(point) {
        return vec![UiAction::NimNew];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let game = &state.nim;
    let (title_x, title_y) = if portrait() {
        (10., 72.)
    } else if compact() {
        (390., 30.)
    } else {
        (30., 58.)
    };
    text("‹ CABINET", 8., 30., 13., muted(), state.large_text);
    text("NIM", title_x, title_y, 27., accent(), state.large_text);
    text(
        "Take the final stone from the quiet heaps",
        if compact() { 390. } else { title_x },
        if compact() { 52. } else { title_y + 25. },
        13.,
        muted(),
        state.large_text,
    );
    text(
        status(game.status),
        if portrait() { 12. } else { 30. },
        if portrait() {
            145.
        } else if compact() {
            82.
        } else {
            170.
        },
        16.,
        accent(),
        state.large_text,
    );
    for (index, rect) in (0..3).map(|index| (index, heap_rect(index))) {
        let selected = game.selected_heap == Some(index);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.08, 0.06, 0.14, 1.),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if selected { 3. } else { 1. },
            if selected { accent() } else { muted() },
        );
        text(
            &format!("HEAP {}", index + 1),
            rect.x + 12.,
            rect.y + 28.,
            12.,
            WHITE,
            state.large_text,
        );
        for stone in 0..game.heaps[index] {
            let x = rect.x + 18. + (stone as f32 % 4.) * (rect.w - 36.) / 3.;
            let y = rect.y + 58. + (stone as f32 / 4.).floor() * 25.;
            draw_circle(x, y, 8., stone_color(stone));
        }
        text(
            &game.heaps[index].to_string(),
            rect.x + rect.w - 28.,
            rect.y + rect.h - 18.,
            18.,
            accent(),
            state.large_text,
        );
    }
    for amount in 0..3 {
        button(
            take_rect(amount),
            &format!("TAKE {}", amount + 1),
            state.large_text,
        );
    }
    let (hint, undo, new_board) = bottom_rects();
    button(hint, "HINT", state.large_text);
    button(undo, "UNDO", state.large_text);
    button(new_board, "NEW BOARD", state.large_text);
    let detail = state
        .card_hint
        .as_deref()
        .unwrap_or_else(|| instruction(game.status));
    text(
        &format!("Moves {}  •  {}", game.moves, detail),
        if portrait() || compact() { 12. } else { 30. },
        if portrait() {
            625.
        } else if compact() {
            98.
        } else {
            590.
        },
        12.,
        muted(),
        state.large_text,
    );
}

fn status(status: NimStatus) -> &'static str {
    match status {
        NimStatus::Playing => "YOUR TURN",
        NimStatus::Won => "HEAPS CLEAR",
        NimStatus::Lost => "THE CABINET TOOK THE LAST STONE",
    }
}

fn instruction(status: NimStatus) -> &'static str {
    match status {
        NimStatus::Playing => "Select a heap, then take 1–3",
        NimStatus::Won => "You took the final stone",
        NimStatus::Lost => "Start a new board to try again",
    }
}

fn stone_color(index: u8) -> Color {
    if index.is_multiple_of(2) {
        Color::new(0.82, 0.48, 0.30, 1.)
    } else {
        Color::new(0.42, 0.68, 0.78, 1.)
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
    text(label, rect.x + 12., rect.y + 29., 12., WHITE, large_text);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color, large_text: bool) {
    draw_text(
        value,
        x,
        y,
        accessibility::text_size(size, large_text),
        color,
    );
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
