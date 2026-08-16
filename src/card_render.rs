//! Shared card visuals and low-motion selection feedback.

use crate::{accessibility, cards::Card, cosmetics};
use macroquad::prelude::*;

pub fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8, reduced_motion: bool) {
    draw_card_accessible(
        rect,
        card,
        selected,
        back_style,
        reduced_motion,
        false,
        false,
    );
}

pub fn draw_card_accessible(
    rect: Rect,
    card: Card,
    selected: bool,
    back_style: u8,
    reduced_motion: bool,
    high_contrast: bool,
    large_text: bool,
) {
    let (back, mark) = cosmetics::card_back_colors(back_style);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if card.face_up {
            if high_contrast {
                WHITE
            } else {
                Color::new(0.94, 0.90, 0.82, 1.)
            }
        } else {
            if high_contrast {
                Color::new(0.08, 0.08, 0.12, 1.)
            } else {
                back
            }
        },
    );
    let pulse = selection_pulse(get_time() as f32, selected, reduced_motion);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        (rect.w * 0.022).max(1.) + pulse * 1.5,
        if selected {
            Color::new(0.98, 0.75 + pulse * 0.12, 0.30, 1.)
        } else {
            if high_contrast {
                WHITE
            } else {
                crate::theme::BRASS
            }
        },
    );
    if !card.face_up {
        crate::ui::draw_text(
            "✦",
            rect.x + rect.w * 0.36,
            rect.y + rect.h * 0.62,
            rect.h * 0.27,
            mark,
        );
        return;
    }
    let color = if card.red() {
        if high_contrast {
            crate::theme::LEATHER
        } else {
            Color::new(0.72, 0.16, 0.22, 1.)
        }
    } else {
        Color::new(0.03, 0.03, 0.05, 1.)
    };
    let rank_size = accessibility::text_size((rect.w * 0.27).clamp(12., 25.), large_text);
    let suit_size = accessibility::text_size((rect.w * 0.30).clamp(13., 28.), large_text);
    crate::ui::draw_text(
        rank_label(card.rank),
        rect.x + rect.w * 0.13,
        rect.y + rect.h * 0.27,
        rank_size,
        color,
    );
    crate::ui::draw_text(
        suit_label(card.suit),
        rect.x + rect.w * 0.67,
        rect.y + rect.h * 0.79,
        suit_size,
        color,
    );
}

pub fn rank_label(rank: u8) -> &'static str {
    [
        "", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ]
    .get(rank as usize)
    .copied()
    .unwrap_or("?")
}

pub fn suit_label(suit: u8) -> &'static str {
    ["♣", "♦", "♥", "♠"]
        .get(suit as usize)
        .copied()
        .unwrap_or("?")
}

pub fn selection_pulse(time: f32, selected: bool, reduced_motion: bool) -> f32 {
    if !selected || reduced_motion {
        0.
    } else {
        ((time * 4.).sin() + 1.) * 0.5
    }
}

#[cfg(test)]
mod tests;
