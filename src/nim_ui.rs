//! Responsive touch presentation for Nim.

use crate::{
    accessibility,
    nim::{NimRule, NimStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

const PORTRAIT_HEAP_X: f32 = 10.;
const PORTRAIT_HEAP_GAP: f32 = 116.;
const PORTRAIT_HEAP_WIDTH: f32 = 108.;

fn portrait() -> bool {
    crate::ui::is_portrait()
}

fn compact() -> bool {
    crate::ui::is_compact_landscape()
}

fn heap_rect(index: usize) -> Rect {
    if portrait() {
        Rect::new(
            PORTRAIT_HEAP_X + index as f32 * PORTRAIT_HEAP_GAP,
            180.,
            PORTRAIT_HEAP_WIDTH,
            188.,
        )
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
        PORTRAIT_HEAP_GAP
    } else if compact() {
        102.
    } else {
        170.
    };
    let x = if portrait() {
        PORTRAIT_HEAP_X + amount as f32 * gap
    } else if compact() {
        15. + amount as f32 * gap
    } else {
        260. + amount as f32 * gap
    };
    Rect::new(x, y, width, 48.)
}

fn bottom_rects() -> (Rect, Rect, Rect, Rect) {
    if portrait() {
        (
            Rect::new(5., 650., 70., 42.),
            Rect::new(82., 650., 78., 42.),
            Rect::new(167., 650., 105., 42.),
            Rect::new(279., 650., 76., 42.),
        )
    } else if compact() {
        (
            Rect::new(385., 335., 105., 40.),
            Rect::new(505., 335., 105., 40.),
            Rect::new(625., 335., 105., 40.),
            Rect::new(735., 335., 100., 40.),
        )
    } else {
        (
            Rect::new(600., 625., 120., 42.),
            Rect::new(735., 625., 120., 42.),
            Rect::new(870., 625., 140., 42.),
            Rect::new(1025., 625., 130., 42.),
        )
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
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
    let (hint, undo, new_board, rule) = bottom_rects();
    if hint.contains(point) {
        return vec![UiAction::NimHint];
    }
    if undo.contains(point) {
        return vec![UiAction::NimUndo];
    }
    if new_board.contains(point) {
        return vec![UiAction::NimNew];
    }
    if rule.contains(point) {
        return vec![UiAction::NimRule(match _state.games.nim.rule {
            NimRule::Normal => NimRule::Misere,
            NimRule::Misere => NimRule::Normal,
        })];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let game = &state.games.nim;
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
        match game.rule {
            NimRule::Normal => "Take the final stone to win",
            NimRule::Misere => "Leave the final stone to win",
        },
        if compact() { 390. } else { title_x },
        if compact() { 52. } else { title_y + 25. },
        13.,
        muted(),
        state.large_text,
    );
    text(
        status(game.status, game.rule),
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
            crate::theme::BACKGROUND_DEEP,
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
        let preview = game
            .selected_heap
            .and_then(|heap| game.move_is_winning(heap, amount as u8 + 1));
        button(
            take_rect(amount),
            &format!(
                "TAKE {}{}",
                amount + 1,
                preview.map_or("", |safe| if safe { " SAFE" } else { " RISK" })
            ),
            state.large_text,
        );
    }
    let (hint, undo, new_board, rule) = bottom_rects();
    button(hint, "HINT", state.large_text);
    button(undo, "UNDO", state.large_text);
    button(new_board, "NEW BOARD", state.large_text);
    mode_button(rule, game.rule.label(), state.large_text);
    let detail = state
        .card_hint
        .as_deref()
        .unwrap_or_else(|| instruction(game.status, game.rule));
    let turn_note = if portrait() && game.last_player_take > 0 {
        format!(
            "M{} • YOU {} / CPU {} • SAFE = forced win",
            game.moves, game.last_player_take, game.last_ai_take
        )
    } else if game.last_player_take > 0 {
        format!(
            "Moves {}  •  Last YOU {} / CABINET {}  •  {}",
            game.moves, game.last_player_take, game.last_ai_take, detail
        )
    } else {
        format!("Moves {}  •  {}", game.moves, detail)
    };
    text(
        &turn_note,
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

fn status(status: NimStatus, rule: NimRule) -> &'static str {
    match (status, rule) {
        (NimStatus::Playing, _) => "YOUR TURN",
        (NimStatus::Won, NimRule::Normal) => "YOU TOOK THE FINAL STONE",
        (NimStatus::Won, NimRule::Misere) => "THE CABINET TOOK THE FINAL STONE",
        (NimStatus::Lost, NimRule::Normal) => "THE CABINET TOOK THE FINAL STONE",
        (NimStatus::Lost, NimRule::Misere) => "YOU TOOK THE FINAL STONE",
    }
}

fn instruction(status: NimStatus, rule: NimRule) -> &'static str {
    match (status, rule) {
        (NimStatus::Playing, NimRule::Normal) => "Select a heap; SAFE previews a forced win",
        (NimStatus::Playing, NimRule::Misere) => {
            "Avoid the final stone; SAFE previews a forced win"
        }
        (NimStatus::Won, _) => "The heap duel is yours",
        (NimStatus::Lost, _) => "Start a new board to try again",
    }
}

fn mode_button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.22, 0.30, 0.20, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(label, rect.x + 10., rect.y + 27., 10., WHITE, large_text);
}

fn stone_color(index: u8) -> Color {
    if index.is_multiple_of(2) {
        Color::new(0.82, 0.48, 0.30, 1.)
    } else {
        Color::new(0.42, 0.68, 0.78, 1.)
    }
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(label, rect.x + 12., rect.y + 29., 12., WHITE, large_text);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color, large_text: bool) {
    crate::ui::draw_text(
        value,
        x,
        y,
        accessibility::text_size(size, large_text),
        color,
    );
}

fn accent() -> Color {
    crate::theme::BRASS
}

fn muted() -> Color {
    crate::theme::SECONDARY
}
