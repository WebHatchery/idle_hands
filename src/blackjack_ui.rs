//! Responsive presentation and touch routing for Blackjack.

use crate::{blackjack::BlackjackStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    hit: Rect,
    stand: Rect,
    undo: Rect,
    new_round: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            hit: Rect::new(430., 220., 110., 44.),
            stand: Rect::new(550., 220., 120., 44.),
            undo: Rect::new(430., 280., 100., 40.),
            new_round: Rect::new(540., 280., 130., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            hit: Rect::new(20., 500., 145., 44.),
            stand: Rect::new(185., 500., 165., 44.),
            undo: Rect::new(20., 560., 145., 42.),
            new_round: Rect::new(185., 560., 165., 42.),
        }
    } else {
        Layout {
            hit: Rect::new(650., 370., 140., 46.),
            stand: Rect::new(810., 370., 150., 46.),
            undo: Rect::new(650., 440., 120., 44.),
            new_round: Rect::new(790., 440., 150., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.hit.contains(point) {
        return vec![UiAction::BlackjackHit];
    }
    if l.stand.contains(point) {
        return vec![UiAction::BlackjackStand];
    }
    if l.undo.contains(point) {
        return vec![UiAction::BlackjackUndo];
    }
    if l.new_round.contains(point) {
        return vec![UiAction::BlackjackNew];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let game = &state.blackjack;
    let l = layout();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let x = if compact {
        80.
    } else if portrait {
        10.
    } else {
        400.
    };
    let y = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("BLACKJACK", x, y, title_size(), accent());
    let status_x = if compact {
        430.
    } else if portrait {
        180.
    } else {
        650.
    };
    let status_y = if compact { 30. } else { 78. };
    text(
        &status_text(game.status, game.player_total(), game.dealer_total()),
        status_x,
        status_y,
        body_size(),
        muted(),
    );
    let origin = if compact {
        vec2(125., 66.)
    } else if portrait {
        vec2(20., 92.)
    } else {
        vec2(360., 88.)
    };
    text("DEALER", origin.x, origin.y, body_size(), muted());
    draw_hand(
        &game.dealer,
        origin + vec2(0., 12.),
        !matches!(game.status, BlackjackStatus::Playing),
    );
    text(
        "PLAYER",
        origin.x,
        origin.y + if portrait { 190. } else { 158. },
        body_size(),
        muted(),
    );
    draw_hand(
        &game.player,
        origin + vec2(0., if portrait { 202. } else { 170. }),
        true,
    );
    text(
        &format!("Wins {}  •  Round {}", game.wins, game.rounds),
        origin.x,
        if portrait { 475. } else { 350. },
        body_size(),
        muted(),
    );
    button(l.hit, "HIT");
    button(l.stand, "STAND");
    button(l.undo, "UNDO");
    button(l.new_round, "NEW ROUND");
}

fn draw_hand(hand: &[crate::cards::Card], origin: Vec2, reveal: bool) {
    for (index, card) in hand.iter().enumerate() {
        let rect = Rect::new(origin.x + index as f32 * 68., origin.y, 58., 88.);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.20, 0.13, 0.30, 1.),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
        let label = if reveal || index > 0 {
            format!("{}{}", rank(card.rank), suit(card.suit))
        } else {
            "?".to_owned()
        };
        text(&label, rect.x + 10., rect.y + 52., 21., WHITE);
    }
}

fn rank(value: u8) -> &'static str {
    match value {
        1 => "A",
        11 => "J",
        12 => "Q",
        13 => "K",
        _ => "?",
    }
}
fn suit(value: u8) -> &'static str {
    ["♣", "♦", "♥", "♠"][value as usize % 4]
}
fn status_text(status: BlackjackStatus, player: u8, dealer: u8) -> String {
    match status {
        BlackjackStatus::Playing => format!("Your total {}  •  dealer shows one card", player),
        BlackjackStatus::Won => format!("You win — {} to {}", player, dealer),
        BlackjackStatus::Lost => format!("Dealer wins — {} to {}", dealer, player),
        BlackjackStatus::Push => format!("Push — both hold at {}", player),
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
    text(label, rect.x + 12., rect.y + 29., 11., WHITE);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        23.
    } else {
        29.
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
