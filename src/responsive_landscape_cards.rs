//! Medium landscape layouts for card and scorecard games.

use crate::{
    cards::Card,
    fivefold::{Category, FivefoldStatus},
    freecell::FreeSource,
    solitaire::{CardSource, SolitaireStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

pub const COMPACT_CARD_INSTRUCTION_X: f32 = 250.;

pub fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn card_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(x, y, w, h)
}
pub fn card_x(column: usize) -> f32 {
    8. + column as f32 * 117.
}
pub fn draw_card(rect: Rect, card: Card, selected: bool, back_style: u8, reduced_motion: bool) {
    crate::card_render::draw_card(rect, card, selected, back_style, reduced_motion);
}
pub fn back() {
    panel(Rect::new(0., 0., 110., 44.), crate::theme::SURFACE_DARK);
    text("< CABINET", 10., 29., 12., crate::theme::BRASS);
}

pub fn draw_solitaire(state: &AppState) {
    let game = &state.games.solitaire;
    back();
    text("SOLITAIRE", 105., 20., 19., crate::theme::BRASS);
    text(
        if game.status == SolitaireStatus::Won {
            "Table cleared"
        } else {
            "Tap card > target; tap again"
        },
        COMPACT_CARD_INSTRUCTION_X,
        18.,
        12.,
        crate::theme::SECONDARY,
    );
    let stock = card_rect(10., 35., 75., 95.);
    panel(stock, crate::theme::SURFACE);
    if let Some(card) = game.stock.last() {
        draw_card(stock, *card, false, state.card_back, state.reduced_motion);
    }
    let waste = card_rect(95., 35., 75., 95.);
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
        let rect = card_rect(500. + suit as f32 * 80., 35., 72., 95.);
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
                22.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    for column in 0..7 {
        let x = card_x(column);
        text(
            &(column + 1).to_string(),
            x + 45.,
            151.,
            11.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.tableau[column].iter().enumerate() {
            draw_card(
                card_rect(x, 158. + depth as f32 * 18., 100., 68.),
                *card,
                game.selected == Some(CardSource::Tableau(column, depth)),
                state.card_back,
                state.reduced_motion,
            );
        }
        if game.tableau[column].is_empty() {
            panel(
                card_rect(x, 158., 100., 68.),
                Color::new(0.12, 0.09, 0.20, 1.),
            );
        }
    }
    panel(
        Rect::new(10., 330., 125., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 53., 359., 12., WHITE);
    text(
        &format!("Moves {}", game.moves),
        330.,
        359.,
        13.,
        crate::theme::SECONDARY,
    );
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 330., 385., 11., Color::new(0.63, 0.95, 0.72, 1.));
    }
    panel(Rect::new(310., 330., 110., 44.), crate::theme::SURFACE);
    text("HINT", 346., 359., 12., WHITE);
}
pub fn solitaire_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if card_rect(10., 35., 75., 95.).contains(p) {
        return vec![UiAction::SolitaireStock];
    }
    if card_rect(95., 35., 75., 95.).contains(p) {
        return vec![UiAction::SolitaireWaste];
    }
    for suit in 0..4 {
        if card_rect(500. + suit as f32 * 80., 35., 72., 95.).contains(p) {
            return vec![UiAction::SolitaireFoundation(suit)];
        }
    }
    if crate::ui::hit(Rect::new(310., 330., 110., 44.), p) {
        return vec![UiAction::SolitaireHint];
    }
    if crate::ui::hit(Rect::new(10., 330., 125., 44.), p) {
        return vec![UiAction::SolitaireUndo];
    }
    for column in 0..7 {
        let x = card_x(column);
        if p.x >= x && p.x <= x + 100. && p.y >= 140. {
            let depth = if state.games.solitaire.tableau[column].is_empty() {
                0
            } else {
                (((p.y - 158.) / 18.).floor().max(0.) as usize)
                    .min(state.games.solitaire.tableau[column].len() - 1)
            };
            return vec![UiAction::SolitaireTableau(column, depth)];
        }
    }
    vec![]
}

pub fn draw_freecell(state: &AppState) {
    let game = &state.games.freecell;
    back();
    text("FREECELL", 105., 20., 19., crate::theme::BRASS);
    text(
        "Tap card, then pile",
        COMPACT_CARD_INSTRUCTION_X,
        18.,
        12.,
        crate::theme::SECONDARY,
    );
    for cell in 0..4 {
        let rect = card_rect(8. + cell as f32 * 84., 30., 72., 75.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if cell >= game.variant.free_cell_limit() {
            text(
                "SEALED",
                rect.x + 17.,
                rect.y + 42.,
                9.,
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
        let rect = card_rect(500. + suit as f32 * 80., 30., 72., 75.);
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
                22.,
                suit as u8,
                crate::theme::BRASS,
            );
        }
    }
    for cascade in 0..8 {
        let x = 8. + cascade as f32 * 104.;
        text(
            &(cascade + 1).to_string(),
            x + 42.,
            125.,
            11.,
            Color::new(0.63, 0.58, 0.72, 1.),
        );
        for (depth, card) in game.cascades[cascade].iter().enumerate() {
            draw_card(
                card_rect(x, 132. + depth as f32 * 15., 88., 58.),
                *card,
                game.selected == Some(FreeSource::Cascade(cascade, depth)),
                state.card_back,
                state.reduced_motion,
            );
        }
        if game.cascades[cascade].is_empty() {
            panel(
                card_rect(x, 132., 88., 58.),
                Color::new(0.12, 0.09, 0.20, 1.),
            );
        }
    }
    panel(
        Rect::new(10., 330., 125., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 53., 359., 12., WHITE);
    panel(Rect::new(150., 330., 145., 44.), crate::theme::SURFACE);
    text("NEW DEAL", 193., 359., 12., WHITE);
    text(
        &format!("Moves {}", game.moves),
        330.,
        359.,
        13.,
        crate::theme::SECONDARY,
    );
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 330., 385., 11., Color::new(0.63, 0.95, 0.72, 1.));
    }
    panel(Rect::new(310., 330., 110., 44.), crate::theme::SURFACE);
    text("HINT", 346., 359., 12., WHITE);
}

pub fn freecell_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(310., 330., 110., 44.), p) {
        return vec![UiAction::FreeCellHint];
    }
    if crate::ui::hit(Rect::new(10., 330., 125., 44.), p) {
        return vec![UiAction::FreeCellUndo];
    }
    if crate::ui::hit(Rect::new(150., 330., 145., 44.), p) {
        return vec![UiAction::FreeCellNew];
    }
    for cell in 0..4 {
        if cell < state.games.freecell.variant.free_cell_limit()
            && card_rect(8. + cell as f32 * 84., 30., 72., 75.).contains(p)
        {
            return vec![UiAction::FreeCellCell(cell)];
        }
    }
    for suit in 0..4 {
        if card_rect(500. + suit as f32 * 80., 30., 72., 75.).contains(p) {
            return vec![UiAction::FreeCellFoundation(suit)];
        }
    }
    for cascade in 0..8 {
        let x = 8. + cascade as f32 * 104.;
        if p.x >= x && p.x <= x + 88. && p.y >= 120. {
            let depth = if state.games.freecell.cascades[cascade].is_empty() {
                0
            } else {
                (((p.y - 132.) / 15.).floor().max(0.) as usize)
                    .min(state.games.freecell.cascades[cascade].len() - 1)
            };
            return vec![UiAction::FreeCellCascade(cascade, depth)];
        }
    }
    vec![]
}

pub fn dice_rect(index: usize) -> Rect {
    Rect::new(10. + index as f32 * 100., 44., 88., 88.)
}
pub fn draw_fivefold(state: &AppState) {
    let game = &state.games.fivefold;
    back();
    text("FIVEFOLD", 105., 20., 19., crate::theme::BRASS);
    text(
        "Roll, hold, then choose a call",
        250.,
        18.,
        12.,
        crate::theme::SECONDARY,
    );
    for index in 0..5 {
        let rect = dice_rect(index);
        panel(
            rect,
            if game.held[index] {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.20, 0.14, 0.30, 1.)
            },
        );
        let value = if game.dice[index] == 0 {
            "-".to_owned()
        } else {
            game.dice[index].to_string()
        };
        text(&value, rect.x + 36., rect.y + 54., 30., crate::theme::BRASS);
        text(
            if game.held[index] { "HELD" } else { "HOLD" },
            rect.x + 30.,
            rect.y + 76.,
            9.,
            crate::theme::SECONDARY,
        );
    }
    panel(
        Rect::new(10., 170., 180., 46.),
        if game.roll_number < 3 && game.status != FivefoldStatus::Complete {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.16, 0.11, 0.24, 1.)
        },
    );
    text(
        if game.roll_number == 0 {
            "ROLL DICE"
        } else {
            "ROLL AGAIN"
        },
        64.,
        199.,
        13.,
        WHITE,
    );
    text(
        &format!("Roll {}/3   Total {}", game.roll_number, game.total()),
        220.,
        198.,
        14.,
        crate::theme::BRASS,
    );
    panel(
        Rect::new(500., 30., 330., 330.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SCORECARD", 520., 58., 20., crate::theme::BRASS);
    let page = state.fivefold_score_page.min(2);
    for (slot, (index, category)) in Category::ALL
        .iter()
        .enumerate()
        .skip(page * 5)
        .take(5)
        .enumerate()
    {
        let y = 97. + slot as f32 * 44.;
        let available = index < game.variant.category_limit();
        let score = game.scores[index].map_or_else(
            || {
                if game.roll_number > 0 {
                    game.score_for(*category).to_string()
                } else {
                    "-".to_owned()
                }
            },
            |value| value.to_string(),
        );
        if available && game.scores[index].is_none() && game.roll_number > 0 {
            panel(
                Rect::new(515., y - 31., 300., 40.),
                Color::new(0.14, 0.10, 0.22, 1.),
            );
        }
        text(
            category.label(),
            525.,
            y,
            11.,
            if available {
                WHITE
            } else {
                crate::theme::SECONDARY
            },
        );
        text(
            if available { score.as_str() } else { "LOCKED" },
            780.,
            y,
            10.,
            if available {
                crate::theme::BRASS
            } else {
                crate::theme::SECONDARY
            },
        );
    }
    page_button(Rect::new(515., 306., 92., 44.), "PREV");
    page_button(Rect::new(617., 306., 92., 44.), "NEXT");
    text(&format!("{}/3", page + 1), 748., 334., 11., WHITE);
    panel(Rect::new(10., 250., 180., 44.), crate::theme::SURFACE);
    text("NEW SCORECARD", 45., 279., 12., WHITE);
    panel(Rect::new(210., 250., 150., 44.), crate::theme::SURFACE);
    text("HINT", 265., 279., 12., WHITE);
    text(
        state.card_hint.as_deref().unwrap_or("Tap HINT for a call"),
        210.,
        320.,
        10.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
}
pub fn fivefold_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(10., 170., 180., 46.), p) {
        return vec![UiAction::FivefoldRoll];
    }
    if crate::ui::hit(Rect::new(10., 250., 180., 44.), p) {
        return vec![UiAction::FivefoldNew];
    }
    if crate::ui::hit(Rect::new(210., 250., 150., 44.), p) {
        return vec![UiAction::FivefoldHint];
    }
    for index in 0..5 {
        if dice_rect(index).contains(p) {
            return vec![UiAction::FivefoldHold(index)];
        }
    }
    if crate::ui::hit(Rect::new(515., 306., 92., 44.), p) {
        return vec![UiAction::FivefoldScorePage(-1)];
    }
    if crate::ui::hit(Rect::new(617., 306., 92., 44.), p) {
        return vec![UiAction::FivefoldScorePage(1)];
    }
    let page = state.fivefold_score_page.min(2);
    for (slot, (index, category)) in Category::ALL
        .iter()
        .enumerate()
        .skip(page * 5)
        .take(5)
        .enumerate()
    {
        if index < state.games.fivefold.variant.category_limit()
            && crate::ui::hit(Rect::new(515., 66. + slot as f32 * 44., 300., 40.), p)
            && state.games.fivefold.scores[index].is_none()
        {
            return vec![UiAction::FivefoldCategory(*category)];
        }
    }
    vec![]
}

pub fn page_button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 23., rect.y + 28., 11., WHITE);
}
