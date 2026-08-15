//! Compact portrait layout for the card games.

use crate::{
    cards::Card,
    cosmetics,
    fivefold::{Category, FivefoldStatus},
    freecell::FreeSource,
    reversi::{AiLevel, ReversiStatus},
    solitaire::{CardSource, SolitaireStatus},
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
    text(
        game.ruleset.label(),
        12.,
        106.,
        10.,
        Color::new(0.63, 0.95, 0.72, 1.),
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
        Rect::new(5., 650., 105., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("HINT", 39., 675., 12., WHITE);
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
    if Rect::new(5., 650., 105., 38.).contains(p) {
        return vec![UiAction::SolitaireHint];
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

fn free_card_rect(x: f32, y: f32) -> Rect {
    Rect::new(x, y, 40., 54.)
}

fn free_card_x(slot: usize) -> f32 {
    4. + slot as f32 * 45.
}

pub fn draw_freecell(state: &AppState) {
    let game = &state.freecell;
    text("‹ CABINET", 8., 30., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("FREECELL", 10., 72., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        if game.status == crate::freecell::FreeCellStatus::Won {
            "All foundations complete"
        } else {
            "Every card stays in view"
        },
        12.,
        94.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    for cell in 0..4 {
        let rect = free_card_rect(free_card_x(cell), 112.);
        panel(rect, Color::new(0.12, 0.09, 0.20, 1.));
        if let Some(card) = game.cells[cell] {
            draw_card(
                rect,
                card,
                game.selected == Some(FreeSource::Cell(cell)),
                state.card_back,
            );
        }
    }
    for suit in 0..4 {
        let rect = free_card_rect(190. + suit as f32 * 45., 112.);
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
                rect.x + 15.,
                rect.y + 35.,
                18.,
                Color::new(0.46, 0.37, 0.58, 1.),
            );
        }
    }
    text("CELLS", 5., 181., 9., Color::new(0.63, 0.58, 0.72, 1.));
    text(
        "FOUNDATIONS",
        190.,
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
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a card, then tap a cascade or foundation."),
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    panel(
        Rect::new(5., 650., 105., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("HINT", 39., 675., 12., WHITE);
}

pub fn freecell_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 100., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(5., 650., 105., 38.).contains(p) {
        return vec![UiAction::FreeCellHint];
    }
    if Rect::new(120., 650., 105., 38.).contains(p) {
        return vec![UiAction::FreeCellUndo];
    }
    if Rect::new(235., 650., 115., 38.).contains(p) {
        return vec![UiAction::FreeCellNew];
    }
    for cell in 0..4 {
        if free_card_rect(free_card_x(cell), 112.).contains(p) {
            return vec![UiAction::FreeCellCell(cell)];
        }
    }
    for suit in 0..4 {
        if free_card_rect(190. + suit as f32 * 45., 112.).contains(p) {
            return vec![UiAction::FreeCellFoundation(suit)];
        }
    }
    for cascade in 0..8 {
        let x = free_card_x(cascade);
        if p.x >= x && p.x <= x + 40. && p.y >= 195. {
            let depth = if state.freecell.cascades[cascade].is_empty() {
                0
            } else {
                (((p.y - 205.) / 17.).floor().max(0.) as usize)
                    .min(state.freecell.cascades[cascade].len() - 1)
            };
            return vec![UiAction::FreeCellCascade(cascade, depth)];
        }
    }
    vec![]
}

fn dice_rect(index: usize) -> Rect {
    Rect::new(9. + index as f32 * 69., 112., 60., 60.)
}

pub fn draw_fivefold(state: &AppState) {
    let game = &state.fivefold;
    text("‹ CABINET", 8., 30., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("FIVEFOLD", 10., 72., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Five dice, thirteen calls",
        12.,
        94.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
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
        let width = measure_text(&value, None, 30, 1.).width;
        text(
            &value,
            rect.x + (rect.w - width) / 2.,
            rect.y + 39.,
            30.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        text(
            if game.held[index] { "HELD" } else { "HOLD" },
            rect.x + 15.,
            rect.y + 53.,
            8.,
            Color::new(0.68, 0.63, 0.78, 1.),
        );
    }
    panel(
        Rect::new(10., 190., 150., 42.),
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
        47.,
        217.,
        13.,
        WHITE,
    );
    text(
        &format!("Roll {}/3", game.roll_number),
        180.,
        211.,
        14.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    text(
        &format!("Total {}", game.total()),
        180.,
        232.,
        14.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(
        Rect::new(8., 255., 344., 345.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "SCORECARD",
        18.,
        280.,
        20.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (index, category) in Category::ALL.iter().enumerate() {
        let y = 302. + index as f32 * 23.;
        let rect = Rect::new(15., y - 18., 330., 21.);
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
        if game.scores[index].is_none() && game.roll_number > 0 {
            panel(
                rect,
                if game.selected_category == Some(*category) {
                    Color::new(0.35, 0.24, 0.38, 1.)
                } else {
                    Color::new(0.14, 0.10, 0.22, 1.)
                },
            );
        }
        text(
            category.label(),
            rect.x + 9.,
            y,
            11.,
            if game.scores[index].is_some() {
                Color::new(0.52, 0.48, 0.60, 1.)
            } else {
                WHITE
            },
        );
        text(&score, 314., y, 11., Color::new(0.98, 0.83, 0.45, 1.));
    }
    text(
        match game.status {
            FivefoldStatus::Ready => "Roll, hold, then choose a call",
            FivefoldStatus::Rolling => "Tap a score to record this roll",
            FivefoldStatus::Complete => "Scorecard complete",
        },
        10.,
        625.,
        11.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(10., 650., 150., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW SCORECARD", 31., 675., 11., WHITE);
}

pub fn fivefold_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 100., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(10., 190., 150., 42.).contains(p) {
        return vec![UiAction::FivefoldRoll];
    }
    if Rect::new(10., 650., 150., 38.).contains(p) {
        return vec![UiAction::FivefoldNew];
    }
    for index in 0..5 {
        if dice_rect(index).contains(p) {
            return vec![UiAction::FivefoldHold(index)];
        }
    }
    for (index, category) in Category::ALL.iter().enumerate() {
        if Rect::new(15., 284. + index as f32 * 23., 330., 21.).contains(p)
            && state.fivefold.scores[index].is_none()
        {
            return vec![UiAction::FivefoldCategory(*category)];
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
    let game = &state.reversi;
    text("‹ CABINET", 8., 30., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("REVERSI", 10., 72., 29., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Turn the board, one quiet move at a time",
        12.,
        94.,
        12.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    text(
        &format!("DARK {}  •  LIGHT {}", game.score(1), game.score(2)),
        20.,
        110.,
        12.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(REVERSI_BOARD, Color::new(0.10, 0.30, 0.24, 1.));
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
            Color::new(0.48, 0.75, 0.55, 0.7),
        );
        if game.board[index] == 0 && legal.contains(&index) {
            draw_circle(
                rect.center().x,
                rect.center().y,
                5.,
                Color::new(0.72, 0.95, 0.72, 0.75),
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    Color::new(0.08, 0.06, 0.12, 1.)
                } else {
                    Color::new(0.92, 0.85, 0.66, 1.)
                },
            );
            draw_circle_lines(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                2.,
                Color::new(0.75, 0.62, 0.35, 0.8),
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
        12.,
        465.,
        12.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(10., 485., 160., 38.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("PASS TURN", 56., 510., 12., WHITE);
    panel(
        Rect::new(185., 485., 165., 38.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW BOARD", 229., 510., 12., WHITE);
    panel(
        Rect::new(10., 540., 105., 36.),
        if game.ai_level == AiLevel::Gentle {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.18, 0.12, 0.28, 1.)
        },
    );
    panel(
        Rect::new(127., 540., 105., 36.),
        if game.ai_level == AiLevel::Sharp {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.18, 0.12, 0.28, 1.)
        },
    );
    panel(
        Rect::new(244., 540., 106., 36.),
        if game.ai_level == AiLevel::TwoPlayer {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            Color::new(0.18, 0.12, 0.28, 1.)
        },
    );
    text("GENTLE", 40., 564., 11., WHITE);
    text("SHARP", 160., 564., 11., WHITE);
    text("2 PLAYER", 263., 564., 11., WHITE);
    text(
        "Pass is available when no legal move remains.",
        10.,
        620.,
        11.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 100., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(10., 485., 160., 38.).contains(p) {
        return vec![UiAction::ReversiPass];
    }
    if Rect::new(185., 485., 165., 38.).contains(p) {
        return vec![UiAction::ReversiNew];
    }
    if Rect::new(10., 540., 105., 36.).contains(p) {
        return vec![UiAction::ReversiLevel(AiLevel::Gentle)];
    }
    if Rect::new(127., 540., 105., 36.).contains(p) {
        return vec![UiAction::ReversiLevel(AiLevel::Sharp)];
    }
    if Rect::new(244., 540., 106., 36.).contains(p) {
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
