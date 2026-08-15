//! Compact portrait layout for the card games.

use crate::{
    cosmetics,
    solitaire::{Card, CardSource, SolitaireStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const CARD_W: f32 = 43.;
const CARD_H: f32 = 58.;
const COL_GAP: f32 = 7.;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, CARD_W, CARD_H)
}

fn card_x(column: usize) -> f32 {
    4. + column as f32 * (CARD_W + COL_GAP)
}

fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8) {
    let (back, mark) = cosmetics::card_back_colors(back_style);
    let face = if card.face_up {
        Color::new(0.94, 0.90, 0.82, 1.)
    } else {
        back
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, face);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 3. } else { 1. },
        if selected {
            Color::new(0.98, 0.75, 0.30, 1.)
        } else {
            Color::new(0.55, 0.45, 0.68, 1.)
        },
    );
    if !card.face_up {
        text("*", rect.x + 17., rect.y + 38., 22., mark);
        return;
    }
    let rank = [
        "", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ][card.rank as usize];
    let suit = ["C", "D", "H", "S"][card.suit as usize];
    let color = if card.red() {
        Color::new(0.72, 0.16, 0.22, 1.)
    } else {
        Color::new(0.10, 0.08, 0.16, 1.)
    };
    text(rank, rect.x + 4., rect.y + 17., 13., color);
    text(suit, rect.x + 15., rect.y + 43., 19., color);
}

pub fn draw_solitaire(state: &AppState) {
    let game = &state.solitaire;
    text("‹ CABINET", 8., 30., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("SOLITAIRE", 10., 72., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        if game.status == SolitaireStatus::Won {
            "Table cleared"
        } else {
            "Build four foundations"
        },
        12.,
        94.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    let top_y = 112.;
    let stock = card_rect(8., top_y);
    panel(stock, Color::new(0.20, 0.13, 0.30, 1.));
    if let Some(card) = game.stock.last() {
        draw_card(stock, *card, false, state.card_back);
    }
    let waste = card_rect(58., top_y);
    if let Some(card) = game.waste.last() {
        draw_card(
            waste,
            *card,
            game.selected == Some(CardSource::Waste),
            state.card_back,
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
            );
        } else {
            text(
                ["C", "D", "H", "S"][suit],
                rect.x + 13.,
                rect.y + 38.,
                20.,
                Color::new(0.46, 0.37, 0.58, 1.),
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
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(120., 650., 105., 38.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 153., 675., 12., WHITE);
    panel(
        Rect::new(235., 650., 115., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW DEAL", 263., 675., 11., WHITE);
    text(
        "Tap a card, then tap its destination.",
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn solitaire_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 100., 42.).contains(p) {
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
    if Rect::new(120., 650., 105., 38.).contains(p) {
        return vec![UiAction::SolitaireUndo];
    }
    if Rect::new(235., 650., 115., 38.).contains(p) {
        return vec![UiAction::SolitaireNew];
    }
    for column in 0..7 {
        let x = card_x(column);
        if p.x >= x && p.x <= x + CARD_W && p.y >= 195. {
            let depth = if state.solitaire.tableau[column].is_empty() {
                0
            } else {
                (((p.y - 205.) / 19.).floor().max(0.) as usize)
                    .min(state.solitaire.tableau[column].len() - 1)
            };
            return vec![UiAction::SolitaireTableau(column, depth)];
        }
    }
    vec![]
}
