//! Compact portrait layout for the card games.

use crate::{
    accessibility,
    cards::Card,
    freecell::FreeSource,
    reversi::{AiLevel, ReversiStatus},
    solitaire::{CardSource, SolitaireStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[path = "responsive_cards/freecell.rs"]
mod freecell;
pub use freecell::{draw_freecell, freecell_clicks};
#[path = "responsive_cards/reversi.rs"]
mod reversi;
pub use reversi::{draw_reversi, reversi_clicks};

#[cfg(test)]
#[path = "../tests/legacy/responsive_cards/tests.rs"]
mod tests;

const CARD_W: f32 = 43.;
const CARD_H: f32 = 58.;
const COL_GAP: f32 = 7.;

#[cfg(test)]
fn free_card_rect(x: f32, y: f32) -> Rect {
    freecell::free_card_rect(x, y)
}

#[cfg(test)]
fn free_card_x(slot: usize) -> f32 {
    freecell::free_card_x(slot)
}

#[cfg(test)]
fn free_foundation_x(suit: usize) -> f32 {
    freecell::free_foundation_x(suit)
}

#[cfg(test)]
fn reversi_subtitle() -> &'static str {
    reversi::reversi_subtitle()
}

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

fn card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, CARD_W, CARD_H)
}

fn card_x(column: usize) -> f32 {
    4. + column as f32 * (CARD_W + COL_GAP)
}

fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8, reduced_motion: bool) {
    crate::card_render::draw_card(rect, card, selected, back_style, reduced_motion);
}

fn back() {
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("CABINET", 8., 29., 13., crate::theme::BRASS);
}

pub fn draw_solitaire(state: &AppState) {
    let game = &state.games.solitaire;
    back();
    text("SOLITAIRE", 10., 72., 29., crate::theme::BRASS);
    text(
        if game.status == SolitaireStatus::Won {
            "Table cleared"
        } else {
            "Build four foundations"
        },
        12.,
        94.,
        13.,
        crate::theme::SECONDARY,
    );
    let top_y = 112.;
    let stock = card_rect(8., top_y);
    panel(stock, crate::theme::SURFACE);
    if let Some(card) = game.stock.last() {
        draw_card(stock, *card, false, state.card_back, state.reduced_motion);
    }
    let waste = card_rect(58., top_y);
    if let Some(card) = game.waste.last() {
        draw_card(
            waste,
            *card,
            game.selected == Some(CardSource::Waste),
            state.card_back,
            state.reduced_motion,
        );
    } else {
        panel(waste, Color::new(0.12, 0.09, 0.20, 1.));
    }
    for suit in 0..4 {
        let rect = card_rect(158. + suit as f32 * 50., top_y);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if game.foundations[suit] > 0 {
            draw_card(
                rect,
                Card {
                    rank: game.foundations[suit],
                    suit: suit as u8,
                    face_up: true,
                },
                false,
                state.card_back,
                state.reduced_motion,
            );
        } else {
            crate::card_render::draw_suit_symbol(
                rect.center(),
                20.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    text("STOCK", 10., 180., 9., Color::new(0.63, 0.58, 0.72, 1.));
    text("WASTE", 59., 180., 9., Color::new(0.63, 0.58, 0.72, 1.));
    for column in 0..7 {
        let x = card_x(column);
        text(
            &(column + 1).to_string(),
            x + 18.,
            199.,
            10.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.tableau[column].iter().enumerate() {
            let rect = card_rect(x, 205. + depth as f32 * 19.);
            draw_card(
                rect,
                *card,
                game.selected == Some(CardSource::Tableau(column, depth)),
                state.card_back,
                state.reduced_motion,
            );
        }
        if game.tableau[column].is_empty() {
            panel(card_rect(x, 205.), Color::new(0.12, 0.09, 0.20, 1.));
        }
    }
    text(
        &format!("Moves: {}", game.moves),
        10.,
        680.,
        13.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(5., 650., 105., 44.), crate::theme::SURFACE);
    text("HINT", 39., 679., 12., WHITE);
    panel(
        Rect::new(120., 650., 105., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 153., 679., 12., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap card, then destination; tap again to release."),
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn solitaire_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if card_rect(8., 112.).contains(p) {
        return vec![UiAction::SolitaireStock];
    }
    if card_rect(58., 112.).contains(p) {
        return vec![UiAction::SolitaireWaste];
    }
    for suit in 0..4 {
        if card_rect(158. + suit as f32 * 50., 112.).contains(p) {
            return vec![UiAction::SolitaireFoundation(suit)];
        }
    }
    if crate::ui::hit(Rect::new(5., 650., 105., 44.), p) {
        return vec![UiAction::SolitaireHint];
    }
    if crate::ui::hit(Rect::new(120., 650., 105., 44.), p) {
        return vec![UiAction::SolitaireUndo];
    }
    for column in 0..7 {
        let x = card_x(column);
        if p.x >= x && p.x <= x + CARD_W && p.y >= 195. {
            let depth = if state.games.solitaire.tableau[column].is_empty() {
                0
            } else {
                (((p.y - 205.) / 19.).floor().max(0.) as usize)
                    .min(state.games.solitaire.tableau[column].len() - 1)
            };
            return vec![UiAction::SolitaireTableau(column, depth)];
        }
    }
    vec![]
}
