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

#[cfg(test)]
mod tests;

const CARD_W: f32 = 43.;
const CARD_H: f32 = 58.;
const COL_GAP: f32 = 7.;

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
    text(
        game.ruleset.label(),
        12.,
        106.,
        10.,
        Color::new(0.63, 0.95, 0.72, 1.),
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
    panel(Rect::new(235., 650., 115., 44.), crate::theme::SURFACE);
    text("NEW DEAL", 263., 679., 11., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a card, then tap its destination."),
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
    if crate::ui::hit(Rect::new(235., 650., 115., 44.), p) {
        return vec![UiAction::SolitaireNew];
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

fn free_card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 40., 54.)
}

fn free_card_x(slot: usize) -> f32 {
    4. + slot as f32 * 45.
}

fn free_foundation_x(suit: usize) -> f32 {
    185. + suit as f32 * 45.
}

pub fn draw_freecell(state: &AppState) {
    let game = &state.games.freecell;
    back();
    text("FREECELL", 10., 72., 29., crate::theme::BRASS);
    text(
        if game.status == crate::freecell::FreeCellStatus::Won {
            "All foundations complete"
        } else {
            "Every card stays in view"
        },
        12.,
        94.,
        13.,
        crate::theme::SECONDARY,
    );
    for cell in 0..4 {
        let rect = free_card_rect(free_card_x(cell), 112.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if cell >= game.variant.free_cell_limit() {
            text(
                "SEALED",
                rect.x + 5.,
                rect.y + 33.,
                8.,
                crate::theme::SECONDARY,
            );
        } else if let Some(card) = game.cells[cell] {
            draw_card(
                rect,
                card,
                game.selected == Some(FreeSource::Cell(cell)),
                state.card_back,
                state.reduced_motion,
            );
        }
    }
    for suit in 0..4 {
        let rect = free_card_rect(free_foundation_x(suit), 112.);
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
                18.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    text("CELLS", 5., 181., 9., Color::new(0.63, 0.58, 0.72, 1.));
    text(
        "FOUNDATIONS",
        185.,
        181.,
        9.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    for cascade in 0..8 {
        let x = free_card_x(cascade);
        text(
            &(cascade + 1).to_string(),
            x + 17.,
            200.,
            10.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.cascades[cascade].iter().enumerate() {
            draw_card(
                free_card_rect(x, 205. + depth as f32 * 17.),
                *card,
                game.selected == Some(FreeSource::Cascade(cascade, depth)),
                state.card_back,
                state.reduced_motion,
            );
        }
        if game.cascades[cascade].is_empty() {
            panel(free_card_rect(x, 205.), Color::new(0.12, 0.09, 0.20, 1.));
        }
    }
    text(
        &format!("Moves: {}", game.moves),
        10.,
        680.,
        13.,
        crate::theme::SECONDARY,
    );
    panel(
        Rect::new(120., 650., 105., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 153., 679., 12., WHITE);
    panel(Rect::new(235., 650., 115., 44.), crate::theme::SURFACE);
    text("NEW DEAL", 263., 679., 11., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a card, then tap a cascade or foundation."),
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    panel(Rect::new(5., 650., 105., 44.), crate::theme::SURFACE);
    text("HINT", 39., 679., 12., WHITE);
}

pub fn freecell_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(5., 650., 105., 44.), p) {
        return vec![UiAction::FreeCellHint];
    }
    if crate::ui::hit(Rect::new(120., 650., 105., 44.), p) {
        return vec![UiAction::FreeCellUndo];
    }
    if crate::ui::hit(Rect::new(235., 650., 115., 44.), p) {
        return vec![UiAction::FreeCellNew];
    }
    for cell in 0..4 {
        if cell < state.games.freecell.variant.free_cell_limit()
            && free_card_rect(free_card_x(cell), 112.).contains(p)
        {
            return vec![UiAction::FreeCellCell(cell)];
        }
    }
    for suit in 0..4 {
        if free_card_rect(free_foundation_x(suit), 112.).contains(p) {
            return vec![UiAction::FreeCellFoundation(suit)];
        }
    }
    for cascade in 0..8 {
        let x = free_card_x(cascade);
        if p.x >= x && p.x <= x + 40. && p.y >= 195. {
            let depth = if state.games.freecell.cascades[cascade].is_empty() {
                0
            } else {
                (((p.y - 205.) / 17.).floor().max(0.) as usize)
                    .min(state.games.freecell.cascades[cascade].len() - 1)
            };
            return vec![UiAction::FreeCellCascade(cascade, depth)];
        }
    }
    vec![]
}

const REVERSI_BOARD: Rect = Rect {
    x: 20.,
    y: 118.,
    w: 320.,
    h: 320.,
};

pub fn draw_reversi(state: &AppState) {
    let game = &state.games.reversi;
    back();
    text(
        "CABINET",
        8.,
        29.,
        accessibility::text_size(13., state.large_text),
        Color::new(0.78, 0.70, 0.92, 1.),
    );
    text(
        "REVERSI",
        10.,
        72.,
        accessibility::text_size(29., state.large_text),
        crate::theme::BRASS,
    );
    text(
        reversi_subtitle(),
        accessibility::text_size(12., state.large_text),
        94.,
        accessibility::text_size(12., state.large_text),
        crate::theme::SECONDARY,
    );
    text(
        &format!("DARK {}  -  LIGHT {}", game.score(1), game.score(2)),
        20.,
        110.,
        12.,
        crate::theme::BRASS,
    );
    panel(
        REVERSI_BOARD,
        if state.high_contrast {
            Color::new(0.02, 0.20, 0.16, 1.)
        } else {
            Color::new(0.10, 0.30, 0.24, 1.)
        },
    );
    let cell = REVERSI_BOARD.w / 8.;
    let legal = if game.status == ReversiStatus::Playing {
        game.legal_moves(game.turn)
    } else {
        Vec::new()
    };
    for index in 0..64 {
        let row = index / 8;
        let col = index % 8;
        let rect = Rect::new(
            REVERSI_BOARD.x + col as f32 * cell,
            REVERSI_BOARD.y + row as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.board[index] == 0 && legal.contains(&index) {
            draw_circle(
                rect.center().x,
                rect.center().y,
                5.,
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.72, 0.95, 0.72, 0.75)
                },
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    accessibility::board_fill(state.high_contrast)
                } else {
                    if state.high_contrast {
                        WHITE
                    } else {
                        Color::new(0.92, 0.85, 0.66, 1.)
                    }
                },
            );
            draw_circle_lines(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                2.,
                accessibility::grid_line(state.high_contrast),
            );
        }
    }
    text(
        match game.status {
            ReversiStatus::Playing if game.ai_level == AiLevel::TwoPlayer => {
                if game.turn == 1 {
                    "Player 1: tap a glowing square"
                } else {
                    "Player 2: tap a glowing square"
                }
            }
            ReversiStatus::Playing if game.turn == 1 => "Your turn: tap a glowing square",
            ReversiStatus::Playing => "Opponent is thinking",
            ReversiStatus::Won => match game.winner {
                Some(1) => "Dark wins the board",
                Some(2) => "Light wins the board",
                _ => "The board is tied",
            },
        },
        accessibility::text_size(12., state.large_text),
        465.,
        accessibility::text_size(12., state.large_text),
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(Rect::new(10., 485., 160., 44.), crate::theme::SURFACE_DARK);
    text(
        "PASS TURN",
        56.,
        514.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(Rect::new(185., 485., 165., 44.), crate::theme::SURFACE);
    text(
        "NEW BOARD",
        229.,
        514.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(
        Rect::new(10., 540., 105., 44.),
        if game.ai_level == AiLevel::Gentle {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    panel(
        Rect::new(127., 540., 105., 44.),
        if game.ai_level == AiLevel::Sharp {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    panel(
        Rect::new(244., 540., 106., 44.),
        if game.ai_level == AiLevel::TwoPlayer {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    text(
        "GENTLE",
        40.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "SHARP",
        160.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "2 PLAYER",
        263.,
        568.,
        accessibility::text_size(11., state.large_text),
        WHITE,
    );
    text(
        "Pass is available when no legal move remains.",
        10.,
        620.,
        accessibility::text_size(11., state.large_text),
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

fn reversi_subtitle() -> &'static str {
    if crate::ui::is_portrait() {
        "Tap a glowing square"
    } else {
        "Turn the board, one careful move at a time"
    }
}

pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(10., 485., 160., 44.), p) {
        return vec![UiAction::ReversiPass];
    }
    if crate::ui::hit(Rect::new(185., 485., 165., 44.), p) {
        return vec![UiAction::ReversiNew];
    }
    if crate::ui::hit(Rect::new(10., 540., 105., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::Gentle)];
    }
    if crate::ui::hit(Rect::new(127., 540., 105., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::Sharp)];
    }
    if crate::ui::hit(Rect::new(244., 540., 106., 44.), p) {
        return vec![UiAction::ReversiLevel(AiLevel::TwoPlayer)];
    }
    if !REVERSI_BOARD.contains(p) {
        return vec![];
    }
    let cell = REVERSI_BOARD.w / 8.;
    let col = ((p.x - REVERSI_BOARD.x) / cell) as usize;
    let row = ((p.y - REVERSI_BOARD.y) / cell) as usize;
    if row < 8 && col < 8 {
        vec![UiAction::ReversiPlace(row * 8 + col)]
    } else {
        vec![]
    }
}
