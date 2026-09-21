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
        draw_diamond(rect.center(), rect.h * 0.25, mark);
        return;
    }
    let color = suit_color(card.suit, high_contrast);
    let rank_size = accessibility::text_size((rect.w * 0.27).clamp(12., 25.), large_text);
    let suit_size = accessibility::text_size((rect.w * 0.30).clamp(13., 28.), large_text);
    crate::ui::draw_text(
        rank_label(card.rank),
        rect.x + rect.w * 0.13,
        rect.y + rect.h * 0.27,
        rank_size,
        color,
    );
    draw_suit_symbol(
        vec2(rect.x + rect.w * 0.79, rect.y + rect.h * 0.69),
        suit_size,
        card.suit,
        color,
    );
}

/// Draws a filled, font-independent suit silhouette.
///
/// Suits use the shared deck order: clubs, diamonds, hearts, spades. The
/// symbol size is its approximate height, so it remains legible in the tiny
/// cards used by compact layouts as well as full-size cards.
pub fn draw_suit_symbol(center: Vec2, size: f32, suit: u8, color: Color) {
    match suit {
        0 => draw_club(center, size, color),
        1 => draw_diamond(center, size, color),
        2 => draw_heart(center, size, color),
        3 => draw_spade(center, size, color),
        _ => {}
    }
}

pub fn suit_color(suit: u8, high_contrast: bool) -> Color {
    match suit {
        1 | 2 => {
            if high_contrast {
                Color::new(0.48, 0.04, 0.08, 1.)
            } else {
                Color::new(0.72, 0.16, 0.22, 1.)
            }
        }
        _ => {
            if high_contrast {
                BLACK
            } else {
                Color::new(0.03, 0.03, 0.05, 1.)
            }
        }
    }
}

pub fn draw_club(center: Vec2, size: f32, color: Color) {
    let lobe_radius = size * 0.22;
    draw_circle(center.x, center.y - size * 0.22, lobe_radius, color);
    draw_circle(
        center.x - size * 0.23,
        center.y - size * 0.04,
        lobe_radius,
        color,
    );
    draw_circle(
        center.x + size * 0.23,
        center.y - size * 0.04,
        lobe_radius,
        color,
    );
    draw_triangle(
        vec2(center.x - size * 0.34, center.y + size * 0.01),
        vec2(center.x + size * 0.34, center.y + size * 0.01),
        vec2(center.x, center.y + size * 0.30),
        color,
    );
    draw_rectangle(
        center.x - size * 0.075,
        center.y + size * 0.12,
        size * 0.15,
        size * 0.34,
        color,
    );
}

pub fn draw_diamond(center: Vec2, size: f32, color: Color) {
    let top = vec2(center.x, center.y - size * 0.50);
    let left = vec2(center.x - size * 0.38, center.y);
    let right = vec2(center.x + size * 0.38, center.y);
    let bottom = vec2(center.x, center.y + size * 0.50);
    draw_triangle(top, left, bottom, color);
    draw_triangle(top, right, bottom, color);
}

pub fn draw_heart(center: Vec2, size: f32, color: Color) {
    let lobe_radius = size * 0.25;
    draw_circle(
        center.x - size * 0.19,
        center.y - size * 0.17,
        lobe_radius,
        color,
    );
    draw_circle(
        center.x + size * 0.19,
        center.y - size * 0.17,
        lobe_radius,
        color,
    );
    draw_triangle(
        vec2(center.x - size * 0.43, center.y - size * 0.06),
        vec2(center.x + size * 0.43, center.y - size * 0.06),
        vec2(center.x, center.y + size * 0.50),
        color,
    );
}

pub fn draw_spade(center: Vec2, size: f32, color: Color) {
    draw_triangle(
        vec2(center.x, center.y - size * 0.50),
        vec2(center.x - size * 0.43, center.y + size * 0.08),
        vec2(center.x + size * 0.43, center.y + size * 0.08),
        color,
    );
    let lobe_radius = size * 0.23;
    draw_circle(
        center.x - size * 0.19,
        center.y + size * 0.06,
        lobe_radius,
        color,
    );
    draw_circle(
        center.x + size * 0.19,
        center.y + size * 0.06,
        lobe_radius,
        color,
    );
    draw_triangle(
        vec2(center.x - size * 0.33, center.y + size * 0.11),
        vec2(center.x + size * 0.33, center.y + size * 0.11),
        vec2(center.x, center.y + size * 0.34),
        color,
    );
    draw_rectangle(
        center.x - size * 0.075,
        center.y + size * 0.20,
        size * 0.15,
        size * 0.30,
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

pub fn selection_pulse(time: f32, selected: bool, reduced_motion: bool) -> f32 {
    if !selected || reduced_motion {
        0.
    } else {
        ((time * 4.).sin() + 1.) * 0.5
    }
}
