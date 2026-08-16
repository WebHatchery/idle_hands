//! Touch-first Klondike presentation.

use crate::{cards::Card, solitaire::CardSource, state::AppState, ui::UiAction};
use macroquad::prelude::*;

fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(s, x, y, crate::ui::readable_text_size(size), color);
}
fn card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 92., 116.)
}
fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8, reduced_motion: bool) {
    crate::card_render::draw_card(rect, card, selected, back_style, reduced_motion);
}

pub fn draw_solitaire(state: &AppState) {
    let game = &state.solitaire;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("SOLITAIRE", 40., 105., 44., crate::theme::BRASS);
    text(
        if game.status == crate::solitaire::SolitaireStatus::Won {
            "Table cleared"
        } else {
            "Build the four foundations"
        },
        44.,
        132.,
        18.,
        crate::theme::SECONDARY,
    );
    text(
        game.ruleset.label(),
        44.,
        153.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(card_rect(60., 80.), crate::theme::SURFACE);
    if let Some(card) = game.stock.last() {
        draw_card(
            card_rect(60., 80.),
            *card,
            false,
            state.card_back,
            state.reduced_motion,
        );
    }
    text("STOCK", 68., 214., 13., Color::new(0.63, 0.58, 0.72, 1.));
    if let Some(card) = game.waste.last() {
        draw_card(
            card_rect(170., 80.),
            *card,
            game.selected == Some(CardSource::Waste),
            state.card_back,
            state.reduced_motion,
        );
    } else {
        panel(card_rect(170., 80.), Color::new(0.12, 0.09, 0.20, 1.));
    }
    text("WASTE", 180., 214., 13., Color::new(0.63, 0.58, 0.72, 1.));
    for suit in 0..4 {
        let rect = card_rect(690. + suit as f32 * 105., 80.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if game.foundations[suit] > 0 {
            let card = Card {
                rank: game.foundations[suit],
                suit: suit as u8,
                face_up: true,
            };
            draw_card(rect, card, false, state.card_back, state.reduced_motion);
        } else {
            text(
                ["♣", "♦", "♥", "♠"][suit],
                rect.x + 30.,
                rect.y + 70.,
                32.,
                Color::new(0.46, 0.37, 0.58, 1.),
            );
        }
    }
    for column in 0..7 {
        let x = 35. + column as f32 * 120.;
        text(
            &(column + 1).to_string(),
            x + 38.,
            240.,
            15.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.tableau[column].iter().enumerate() {
            let rect = card_rect(x, 250. + depth as f32 * 30.);
            let selected = game.selected == Some(CardSource::Tableau(column, depth));
            draw_card(rect, *card, selected, state.card_back, state.reduced_motion);
        }
        if game.tableau[column].is_empty() {
            panel(card_rect(x, 250.), Color::new(0.12, 0.09, 0.20, 1.));
        }
    }
    text(
        &format!("Moves: {}", game.moves),
        45.,
        685.,
        17.,
        crate::theme::SECONDARY,
    );
    panel(
        Rect::new(850., 620., 140., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 894., 648., 16., WHITE);
    panel(Rect::new(1010., 620., 160., 44.), crate::theme::SURFACE);
    text("NEW DEAL", 1042., 648., 16., WHITE);
    text(
        "Tap a card, then tap its destination.",
        850.,
        545.,
        16.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 690., 590., 14., Color::new(0.63, 0.95, 0.72, 1.));
    }
    panel(Rect::new(690., 620., 140., 44.), crate::theme::SURFACE);
    text("HINT", 737., 648., 16., WHITE);
}

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}

pub fn solitaire_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if card_rect(60., 80.).contains(p) {
        return vec![UiAction::SolitaireStock];
    }
    if card_rect(170., 80.).contains(p) {
        return vec![UiAction::SolitaireWaste];
    }
    for suit in 0..4 {
        if card_rect(690. + suit as f32 * 105., 80.).contains(p) {
            return vec![UiAction::SolitaireFoundation(suit)];
        }
    }
    if Rect::new(690., 620., 140., 44.).contains(p) {
        return vec![UiAction::SolitaireHint];
    }
    if Rect::new(850., 620., 140., 44.).contains(p) {
        return vec![UiAction::SolitaireUndo];
    }
    if Rect::new(1010., 620., 160., 44.).contains(p) {
        return vec![UiAction::SolitaireNew];
    }
    for column in 0..7 {
        let x = 35. + column as f32 * 120.;
        if p.x >= x && p.x <= x + 92. && p.y >= 240. {
            let depth = if state.solitaire.tableau[column].is_empty() {
                0
            } else {
                (((p.y - 250.) / 30.).floor() as usize)
                    .min(state.solitaire.tableau[column].len() - 1)
            };
            return vec![UiAction::SolitaireTableau(column, depth)];
        }
    }
    vec![]
}
