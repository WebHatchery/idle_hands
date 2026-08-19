//! Responsive touch presentation for Mancala.

use crate::{
    mancala::{AiLevel, Mancala, MancalaPhase, MancalaVariant, MovePreview},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    levels: [Rect; 3],
    variants: [Rect; 3],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(220., 48., 390., 245.),
            hint: Rect::new(620., 201., 100., 44.),
            undo: Rect::new(620., 149., 100., 44.),
            new_game: Rect::new(730., 149., 110., 44.),
            levels: [
                Rect::new(620., 50., 64., 40.),
                Rect::new(689., 50., 64., 40.),
                Rect::new(758., 50., 72., 40.),
            ],
            variants: [
                Rect::new(620., 97., 64., 40.),
                Rect::new(689., 97., 72., 40.),
                Rect::new(766., 97., 64., 40.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 110., 300., 230.),
            hint: Rect::new(15., 465., 145., 44.),
            undo: Rect::new(15., 520., 145., 44.),
            new_game: Rect::new(170., 520., 145., 44.),
            levels: [
                Rect::new(15., 355., 90., 42.),
                Rect::new(112., 355., 90., 42.),
                Rect::new(209., 355., 90., 42.),
            ],
            variants: [
                Rect::new(15., 410., 90., 42.),
                Rect::new(112., 410., 90., 42.),
                Rect::new(209., 410., 90., 42.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(300., 125., 500., 300.),
            hint: Rect::new(850., 245., 120., 44.),
            undo: Rect::new(850., 190., 120., 44.),
            new_game: Rect::new(990., 190., 145., 44.),
            levels: [
                Rect::new(850., 125., 90., 42.),
                Rect::new(950., 125., 90., 42.),
                Rect::new(1050., 125., 90., 42.),
            ],
            variants: [
                Rect::new(850., 310., 90., 42.),
                Rect::new(950., 310., 90., 42.),
                Rect::new(1050., 310., 90., 42.),
            ],
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for pit in 0..6 {
        if pit_rect(l.board, pit, true).contains(point) {
            return vec![UiAction::MancalaPit(pit)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::MancalaHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::MancalaUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::MancalaNew];
    }
    for (index, level) in [AiLevel::Gentle, AiLevel::Sharp, AiLevel::Expert]
        .into_iter()
        .enumerate()
    {
        if crate::ui::hit(l.levels[index], point) {
            return vec![UiAction::MancalaLevel(level)];
        }
    }
    for (index, variant) in [
        MancalaVariant::Quick,
        MancalaVariant::Classic,
        MancalaVariant::Grand,
    ]
    .into_iter()
    .enumerate()
    {
        if crate::ui::hit(l.variants[index], point) {
            return vec![UiAction::MancalaVariant(variant)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.mancala;
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
    text("‹ CABINET", 8., 30., 13., muted());
    text("MANCALA", title_x, title_y, title_size(), accent());
    let scoreline = if portrait {
        format!(
            "Y{} • C{} • M{} • Cap{} • E{}",
            game.pits[6], game.pits[13], game.moves, game.captured_stones, game.extra_turns
        )
    } else {
        format!(
            "You {}  •  Cabinet {}  •  Moves {}  •  Captured {}  •  Bonuses {}",
            game.pits[6], game.pits[13], game.moves, game.captured_stones, game.extra_turns
        )
    };
    text(
        &scoreline,
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(status_text(game.phase)),
        if compact { 220. } else { title_x },
        if portrait {
            625.
        } else if compact {
            320.
        } else {
            465.
        },
        body_size(),
        muted(),
    );
    button(l.hint, "HINT");
    button(l.undo, "UNDO");
    button(l.new_game, "NEW BOARD");
    for (rect, (label, level)) in l.levels.iter().zip([
        ("GENTLE", AiLevel::Gentle),
        ("SHARP", AiLevel::Sharp),
        ("EXPERT", AiLevel::Expert),
    ]) {
        button_selected(*rect, label, game.ai_level == level);
    }
    for (rect, (label, variant)) in l.variants.iter().zip([
        ("QUICK", MancalaVariant::Quick),
        ("CLASSIC", MancalaVariant::Classic),
        ("GRAND", MancalaVariant::Grand),
    ]) {
        button_selected(*rect, label, game.variant == variant);
    }
}

fn draw_board(board: Rect, game: &Mancala) {
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        Color::new(0.17, 0.10, 0.18, 1.),
    );
    draw_rectangle_lines(board.x, board.y, board.w, board.h, 2., accent());
    for pit in 0..6 {
        draw_pit(
            pit_rect(board, pit, false),
            game.pits[12 - pit],
            false,
            None,
        );
        draw_pit(
            pit_rect(board, pit, true),
            game.pits[pit],
            true,
            game.move_preview(pit),
        );
    }
    let store_width = if crate::ui::is_portrait() {
        36.
    } else if crate::ui::is_compact_landscape() {
        42.
    } else {
        48.
    };
    draw_store(
        Rect::new(board.x + 7., board.y + 64., store_width, 132.),
        game.pits[13],
        false,
    );
    draw_store(
        Rect::new(
            board.right() - store_width - 7.,
            board.y + 64.,
            store_width,
            132.,
        ),
        game.pits[6],
        true,
    );
    text("CABINET", board.x + 9., board.y + 214., 8., muted());
    text(
        "YOU",
        board.right() - store_width,
        board.y + 214.,
        8.,
        muted(),
    );
}

fn pit_rect(board: Rect, pit: usize, player: bool) -> Rect {
    let step = if crate::ui::is_portrait() {
        35.5
    } else if crate::ui::is_compact_landscape() {
        45.
    } else {
        60.
    };
    let width = if crate::ui::is_portrait() {
        28.
    } else if crate::ui::is_compact_landscape() {
        40.
    } else {
        50.
    };
    let x = board.x
        + if crate::ui::is_portrait() {
            47.
        } else if crate::ui::is_compact_landscape() {
            68.
        } else {
            72.
        }
        + pit as f32 * step;
    let y = if player {
        board.y + board.h - 84.
    } else {
        board.y + 24.
    };
    Rect::new(x, y, width, 60.)
}

fn draw_pit(rect: Rect, stones: u8, player: bool, preview: Option<MovePreview>) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if player {
            Color::new(0.22, 0.17, 0.31, 1.)
        } else {
            Color::new(0.13, 0.11, 0.22, 1.)
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
    center_text(
        &stones.to_string(),
        rect,
        if crate::ui::is_portrait() { 16. } else { 20. },
        WHITE,
    );
    if let Some(preview) = preview {
        let marker = if preview.extra_turn {
            Some("E")
        } else if preview.captured > 0 {
            Some("C")
        } else {
            None
        };
        if let Some(marker) = marker {
            draw_circle(rect.right() - 4., rect.y + 5., 8., accent());
            center_text(
                marker,
                Rect::new(rect.right() - 12., rect.y - 3., 16., 16.),
                9.,
                crate::theme::BACKGROUND,
            );
        }
    }
}

fn draw_store(rect: Rect, stones: u8, player: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.25, 0.16, 0.28, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        &stones.to_string(),
        rect,
        22.,
        if player { accent() } else { muted() },
    );
}

fn status_text(phase: MancalaPhase) -> &'static str {
    match phase {
        MancalaPhase::Playing => "Sow the stones from one of your six pits",
        MancalaPhase::Won => "Your store holds the majority",
        MancalaPhase::Lost => "The cabinet gathered more stones",
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}
fn button_selected(rect: Rect, label: &str, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 10., WHITE);
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
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
