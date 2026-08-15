//! Responsive presentation and touch routing for Klondike Golf.

use crate::{accessibility, klondike_golf::GolfStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    col_w: f32,
    card_h: f32,
    stock: Rect,
    undo: Rect,
    new_game: Rect,
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(18., 85., 560., 190.),
            col_w: 78.,
            card_h: 28.,
            stock: Rect::new(610., 100., 75., 54.),
            undo: Rect::new(610., 180., 80., 38.),
            new_game: Rect::new(700., 180., 105., 38.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 115., 340., 270.),
            col_w: 48.,
            card_h: 30.,
            stock: Rect::new(20., 425., 100., 46.),
            undo: Rect::new(135., 425., 95., 46.),
            new_game: Rect::new(245., 425., 105., 46.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 560., 310.),
            col_w: 78.,
            card_h: 38.,
            stock: Rect::new(950., 120., 100., 58.),
            undo: Rect::new(950., 220., 100., 44.),
            new_game: Rect::new(1070., 220., 140., 44.),
        }
    }
}
pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.stock.contains(point) {
        return vec![UiAction::KlondikeGolfStock];
    }
    if l.undo.contains(point) {
        return vec![UiAction::KlondikeGolfUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::KlondikeGolfNew];
    }
    if l.board.contains(point) {
        let column = ((point.x - l.board.x) / l.col_w) as usize;
        if column < 7 {
            return vec![UiAction::KlondikeGolfColumn(column)];
        }
    }
    let _ = state;
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.klondike_golf;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let hx = if compact {
        120.
    } else if portrait {
        10.
    } else {
        350.
    };
    let hy = if compact {
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
        "KLONDIKE GOLF",
        hx,
        hy,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        status_text(game.status),
        if compact { 610. } else { hx },
        if compact { 30. } else { hy + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (column, stack) in game.tableau.iter().enumerate() {
        let x = l.board.x + column as f32 * l.col_w;
        for (depth, card) in stack.iter().enumerate() {
            let y = l.board.y + depth as f32 * l.card_h;
            draw_card(
                card.rank,
                card.suit,
                Rect::new(x + 2., y, l.col_w - 5., l.card_h - 3.),
                state.high_contrast,
                state.large_text,
            );
        }
    }
    let waste = game
        .waste
        .last()
        .map(|card| format!("{}", card.rank))
        .unwrap_or_else(|| "—".into());
    draw_rectangle(
        l.stock.x,
        l.stock.y,
        l.stock.w,
        l.stock.h,
        accessibility::board_fill(state.high_contrast),
    );
    draw_rectangle_lines(l.stock.x, l.stock.y, l.stock.w, l.stock.h, 1., accent());
    text(
        "STOCK",
        l.stock.x + 10.,
        l.stock.y + 20.,
        accessibility::text_size(10., state.large_text),
        WHITE,
    );
    text(
        &format!("WASTE {}", waste),
        l.stock.x + 10.,
        l.stock.y + 40.,
        accessibility::text_size(10., state.large_text),
        muted(),
    );
    let instruction = if portrait {
        format!(
            "Moves {}  •  Tap a top card above or below waste",
            game.moves
        )
    } else {
        format!(
            "Moves {}  •  Tap a top card one rank above or below the waste",
            game.moves
        )
    };
    text(
        &instruction,
        if compact {
            18.
        } else if portrait {
            10.
        } else {
            350.
        },
        if compact {
            315.
        } else if portrait {
            500.
        } else {
            460.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
}
fn draw_card(rank: u8, suit: u8, rect: Rect, high_contrast: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if high_contrast {
            WHITE
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.,
        accessibility::grid_line(high_contrast),
    );
    text(
        &format!("{}{}", rank, ["♠", "♥", "♦", "♣"][suit as usize]),
        rect.x + 8.,
        rect.y + rect.h * 0.68,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn status_text(status: GolfStatus) -> &'static str {
    match status {
        GolfStatus::Playing => "Clear the columns",
        GolfStatus::Won => "The columns are clear",
        GolfStatus::Stuck => "No golf move remains",
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
        rect.x + 10.,
        rect.y + 28.,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        25.
    } else {
        30.
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
