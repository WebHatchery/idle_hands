//! Touch-first Klondike presentation.

use crate::{
    cards::Card,
    solitaire::{CardSource, Solitaire},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const TOP_ROW_Y: f32 = 150.;
const TABLEAU_LABEL_Y: f32 = 305.;
const TABLEAU_TOP: f32 = 315.;
const TABLEAU_BOTTOM: f32 = 614.;
const MAX_TABLEAU_GAP: f32 = 30.;
const MIN_TABLEAU_GAP: f32 = 12.;

fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(s, x, y, crate::ui::readable_text_size(size), color);
}
fn card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 92., 116.)
}
fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8, reduced_motion: bool) {
    crate::card_render::draw_card(rect, card, selected, back_style, reduced_motion);
}

fn tableau_gap(game: &Solitaire) -> f32 {
    let deepest = game
        .tableau
        .iter()
        .map(|column| column.len().saturating_sub(1))
        .max()
        .unwrap_or(0);
    if deepest == 0 {
        return MAX_TABLEAU_GAP;
    }
    ((TABLEAU_BOTTOM - TABLEAU_TOP - 116.) / deepest as f32).clamp(MIN_TABLEAU_GAP, MAX_TABLEAU_GAP)
}

fn tableau_depth_at(game: &Solitaire, column: usize, y: f32, gap: f32) -> usize {
    if game.tableau[column].is_empty() || y < TABLEAU_TOP {
        return 0;
    }
    (((y - TABLEAU_TOP) / gap).floor() as usize).min(game.tableau[column].len() - 1)
}

pub(crate) fn tableau_card_at(game: &Solitaire, p: Vec2) -> Option<(usize, usize)> {
    if !(TABLEAU_TOP..=TABLEAU_BOTTOM).contains(&p.y) {
        return None;
    }
    let column = (0..7).find(|column| {
        let x = 35. + *column as f32 * 120.;
        p.x >= x && p.x <= x + 92.
    })?;
    if game.tableau[column].is_empty() {
        return None;
    }
    let gap = tableau_gap(game);
    let last_depth = game.tableau[column].len() - 1;
    let last_bottom = TABLEAU_TOP + last_depth as f32 * gap + 116.;
    if p.y > last_bottom {
        return None;
    }
    Some((column, tableau_depth_at(game, column, p.y, gap)))
}

pub fn draw_solitaire(state: &AppState) {
    let game = &state.solitaire;
    let tableau_gap = tableau_gap(game);
    text("‹ CABINET", 40., 43., 20., crate::theme::BRASS);
    text("SOLITAIRE", 40., 93., 44., crate::theme::BRASS);
    text(
        if game.status == crate::solitaire::SolitaireStatus::Won {
            "Table cleared"
        } else {
            "Build the four foundations"
        },
        44.,
        120.,
        18.,
        crate::theme::SECONDARY,
    );
    text(
        game.ruleset.label(),
        44.,
        141.,
        14.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(card_rect(60., TOP_ROW_Y), crate::theme::SURFACE);
    if let Some(card) = game.stock.last() {
        draw_card(
            card_rect(60., TOP_ROW_Y),
            *card,
            false,
            state.card_back,
            state.reduced_motion,
        );
    }
    text("STOCK", 68., TOP_ROW_Y + 134., 13., crate::theme::SECONDARY);
    if let Some(card) = game.waste.last() {
        draw_card(
            card_rect(170., TOP_ROW_Y),
            *card,
            game.selected == Some(CardSource::Waste),
            state.card_back,
            state.reduced_motion,
        );
    } else {
        panel(card_rect(170., TOP_ROW_Y), crate::theme::GAME_PANEL);
    }
    text(
        "WASTE",
        180.,
        TOP_ROW_Y + 134.,
        13.,
        crate::theme::SECONDARY,
    );
    for suit in 0..4 {
        let rect = card_rect(690. + suit as f32 * 105., TOP_ROW_Y);
        panel(rect, crate::theme::GAME_PANEL);
        if game.foundations[suit] > 0 {
            let card = Card {
                rank: game.foundations[suit],
                suit: suit as u8,
                face_up: true,
            };
            draw_card(rect, card, false, state.card_back, state.reduced_motion);
        } else {
            crate::card_render::draw_suit_symbol(
                vec2(rect.center().x, rect.center().y),
                32.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    for column in 0..7 {
        let x = 35. + column as f32 * 120.;
        text(
            &(column + 1).to_string(),
            x + 38.,
            TABLEAU_LABEL_Y,
            15.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.tableau[column].iter().enumerate() {
            let rect = card_rect(x, TABLEAU_TOP + depth as f32 * tableau_gap);
            let selected = game.selected == Some(CardSource::Tableau(column, depth));
            draw_card(rect, *card, selected, state.card_back, state.reduced_motion);
        }
        if game.tableau[column].is_empty() {
            panel(card_rect(x, TABLEAU_TOP), crate::theme::GAME_PANEL);
        }
    }
    if let Some(crate::solitaire::CardSource::Tableau(column, depth)) = state.solitaire_peek {
        if let Some(card) = game.tableau[column].get(depth) {
            let x = 35. + column as f32 * 120.;
            let rect = card_rect(x, TABLEAU_TOP + depth as f32 * tableau_gap);
            draw_card(rect, *card, true, state.card_back, state.reduced_motion);
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
    text(
        "Hover or press a stack to inspect a card.",
        850.,
        567.,
        14.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 690., 590., 14., Color::new(0.63, 0.95, 0.72, 1.));
    }
    panel(Rect::new(690., 620., 140., 44.), crate::theme::SURFACE);
    text("HINT", 737., 648., 16., WHITE);
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

pub fn solitaire_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let tableau_gap = tableau_gap(&state.solitaire);
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if card_rect(60., TOP_ROW_Y).contains(p) {
        return vec![UiAction::SolitaireStock];
    }
    if card_rect(170., TOP_ROW_Y).contains(p) {
        return vec![UiAction::SolitaireWaste];
    }
    for suit in 0..4 {
        if card_rect(690. + suit as f32 * 105., TOP_ROW_Y).contains(p) {
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
        if p.x >= x && p.x <= x + 92. && p.y >= TABLEAU_LABEL_Y {
            let depth = tableau_depth_at(&state.solitaire, column, p.y, tableau_gap);
            return vec![UiAction::SolitaireTableau(column, depth)];
        }
    }
    vec![]
}

#[cfg(test)]
mod tests;
