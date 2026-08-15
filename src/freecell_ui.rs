//! Touch-first FreeCell presentation.

use crate::{cards::Card, freecell::FreeSource, state::AppState, ui::UiAction};
use macroquad::prelude::*;

fn card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 92., 116.)
}
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
fn draw_card(rect: Rect, card: Card, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.94, 0.90, 0.82, 1.),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 4. } else { 2. },
        if selected {
            Color::new(0.98, 0.75, 0.30, 1.)
        } else {
            Color::new(0.55, 0.45, 0.68, 1.)
        },
    );
    let ranks = [
        "", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    draw_text(
        ranks[card.rank as usize],
        rect.x + 12.,
        rect.y + 31.,
        25.,
        if card.red() {
            Color::new(0.72, 0.16, 0.22, 1.)
        } else {
            Color::new(0.10, 0.08, 0.16, 1.)
        },
    );
    draw_text(
        ["♣", "♦", "♥", "♠"][card.suit as usize],
        rect.x + 62.,
        rect.y + 92.,
        28.,
        if card.red() {
            Color::new(0.72, 0.16, 0.22, 1.)
        } else {
            Color::new(0.10, 0.08, 0.16, 1.)
        },
    );
}

pub fn draw_freecell(state: &AppState) {
    let game = &state.freecell;
    draw_text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    draw_text("FREECELL", 40., 105., 44., Color::new(0.98, 0.83, 0.45, 1.));
    draw_text(
        if game.status == crate::freecell::FreeCellStatus::Won {
            "All foundations complete"
        } else {
            "Every card stays in view"
        },
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    for cell in 0..4 {
        let rect = card_rect(45. + cell as f32 * 105., 165.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if let Some(card) = game.cells[cell] {
            draw_card(rect, card, game.selected == Some(FreeSource::Cell(cell)));
        }
        draw_text(
            format!("CELL {}", cell + 1),
            rect.x + 8.,
            300.,
            12.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
    }
    for suit in 0..4 {
        let rect = card_rect(690. + suit as f32 * 105., 165.);
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
            );
        } else {
            draw_text(
                ["♣", "♦", "♥", "♠"][suit],
                rect.x + 30.,
                rect.y + 70.,
                32.,
                Color::new(0.46, 0.37, 0.58, 1.),
            );
        }
        draw_text(
            "FOUND",
            rect.x + 12.,
            300.,
            12.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
    }
    for cascade in 0..8 {
        let x = 28. + cascade as f32 * 122.;
        draw_text(
            (cascade + 1).to_string(),
            x + 38.,
            340.,
            15.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.cascades[cascade].iter().enumerate() {
            draw_card(
                card_rect(x, 350. + depth as f32 * 28.),
                *card,
                game.selected == Some(FreeSource::Cascade(cascade, depth)),
            );
        }
        if game.cascades[cascade].is_empty() {
            panel(card_rect(x, 350.), Color::new(0.12, 0.09, 0.20, 1.));
        }
    }
    draw_text(
        format!("Moves: {}", game.moves),
        45.,
        685.,
        17.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(850., 620., 140., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    draw_text("UNDO", 894., 648., 16., WHITE);
    panel(
        Rect::new(1010., 620., 160., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_text("NEW DEAL", 1042., 648., 16., WHITE);
    draw_text(
        "Tap a card, then tap a cascade or foundation.",
        850.,
        545.,
        16.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    if let Some(hint) = state.card_hint.as_deref() {
        draw_text(hint, 690., 590., 14., Color::new(0.63, 0.95, 0.72, 1.));
    }
    panel(
        Rect::new(690., 620., 140., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_text("HINT", 737., 648., 16., WHITE);
}

pub fn freecell_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(690., 620., 140., 44.).contains(p) {
        return vec![UiAction::FreeCellHint];
    }
    if Rect::new(850., 620., 140., 44.).contains(p) {
        return vec![UiAction::FreeCellUndo];
    }
    if Rect::new(1010., 620., 160., 44.).contains(p) {
        return vec![UiAction::FreeCellNew];
    }
    for cell in 0..4 {
        if card_rect(45. + cell as f32 * 105., 165.).contains(p) {
            return vec![UiAction::FreeCellCell(cell)];
        }
    }
    for suit in 0..4 {
        if card_rect(690. + suit as f32 * 105., 165.).contains(p) {
            return vec![UiAction::FreeCellFoundation(suit)];
        }
    }
    for cascade in 0..8 {
        let x = 28. + cascade as f32 * 122.;
        if p.x >= x && p.x <= x + 92. && p.y >= 330. {
            let depth = if state.freecell.cascades[cascade].is_empty() {
                0
            } else {
                (((p.y - 350.) / 28.).floor().max(0.) as usize)
                    .min(state.freecell.cascades[cascade].len() - 1)
            };
            return vec![UiAction::FreeCellCascade(cascade, depth)];
        }
    }
    vec![]
}
