//! Seeded Klondike card state with selection-based touch moves.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolitaireStatus {
    Playing,
    Won,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SolitaireRuleset {
    #[default]
    DrawOneUnlimited,
    DrawThreeUnlimited,
}
impl SolitaireRuleset {
    pub const ALL: [Self; 2] = [Self::DrawOneUnlimited, Self::DrawThreeUnlimited];

    pub fn draw_count(self) -> usize {
        match self {
            Self::DrawOneUnlimited => 1,
            Self::DrawThreeUnlimited => 3,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::DrawOneUnlimited => "DRAW 1 · UNLIMITED REDEALS",
            Self::DrawThreeUnlimited => "DRAW 3 · UNLIMITED REDEALS",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardSource {
    Tableau(usize, usize),
    Waste,
}

type SolitaireSnapshot = (Vec<Vec<Card>>, Vec<Card>, Vec<Card>, [u8; 4], u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solitaire {
    pub tableau: Vec<Vec<Card>>,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: [u8; 4],
    pub selected: Option<CardSource>,
    pub moves: u32,
    pub status: SolitaireStatus,
    pub seed: u64,
    #[serde(default)]
    pub ruleset: SolitaireRuleset,
    #[serde(skip)]
    history: Vec<SolitaireSnapshot>,
}

impl Default for Solitaire {
    fn default() -> Self {
        Self::new(0x51_01_17)
    }
}
impl Solitaire {
    pub fn new(seed: u64) -> Self {
        Self::with_ruleset(seed, SolitaireRuleset::default())
    }
    pub fn with_ruleset(seed: u64, ruleset: SolitaireRuleset) -> Self {
        let (deck, rng) = shuffled_deck(seed, false);
        let mut tableau = vec![Vec::new(); 7];
        let mut cursor = 0;
        for (column, stack) in tableau.iter_mut().enumerate().take(7) {
            for depth in 0..=column {
                let mut card = deck[cursor];
                card.face_up = depth == column;
                stack.push(card);
                cursor += 1;
            }
        }
        let mut stock = deck[cursor..].to_vec();
        stock.reverse();
        Self {
            tableau,
            stock,
            waste: Vec::new(),
            foundations: [0; 4],
            selected: None,
            moves: 0,
            status: SolitaireStatus::Playing,
            seed: rng,
            ruleset,
            history: Vec::new(),
        }
    }
    pub fn draw_stock(&mut self) {
        self.selected = None;
        self.snapshot();
        if !self.stock.is_empty() {
            for _ in 0..self.ruleset.draw_count() {
                let Some(mut card) = self.stock.pop() else {
                    break;
                };
                card.face_up = true;
                self.waste.push(card);
            }
        } else if !self.waste.is_empty() {
            self.stock = self.waste.drain(..).rev().collect();
        }
        self.moves += 1;
    }
    pub fn select_tableau(&mut self, column: usize, depth: usize) -> bool {
        if self
            .tableau
            .get(column)
            .and_then(|cards| cards.get(depth))
            .is_some_and(|card| card.face_up)
        {
            self.selected = Some(CardSource::Tableau(column, depth));
            true
        } else {
            false
        }
    }
    pub fn select_waste(&mut self) -> bool {
        if self.waste.last().is_some() {
            self.selected = Some(CardSource::Waste);
            true
        } else {
            false
        }
    }
    pub fn tap_waste(&mut self) -> bool {
        if self.waste.last().is_some() {
            if self.selected == Some(CardSource::Waste) {
                self.selected = None;
            } else {
                self.select_waste();
            }
            true
        } else {
            false
        }
    }
    pub fn tap_tableau(&mut self, column: usize, depth: usize) -> bool {
        if self.selected == Some(CardSource::Tableau(column, depth)) {
            self.selected = None;
            return true;
        }
        if self.selected.is_some() && self.move_to_tableau(column) {
            return true;
        }
        self.select_tableau(column, depth)
    }
    pub fn move_to_tableau(&mut self, destination: usize) -> bool {
        let Some(source) = self.selected.take() else {
            return false;
        };
        let card = match source {
            CardSource::Waste => self.waste.last().copied(),
            CardSource::Tableau(column, depth) => self
                .tableau
                .get(column)
                .and_then(|cards| cards.get(depth))
                .copied(),
        };
        let Some(card) = card else {
            self.selected = Some(source);
            return false;
        };
        if !self.can_place(destination, card) {
            self.selected = Some(source);
            return false;
        }
        self.snapshot();
        let moving = match source {
            CardSource::Waste => self.waste.pop().map(|card| vec![card]),
            CardSource::Tableau(column, depth) => self
                .tableau
                .get_mut(column)
                .and_then(|cards| (depth <= cards.len()).then(|| cards.split_off(depth))),
        };
        let Some(moving) = moving else {
            self.selected = Some(source);
            return false;
        };
        self.tableau[destination].extend(moving);
        self.flip_top();
        self.moves += 1;
        true
    }
    pub fn move_to_foundation(&mut self, suit: usize) -> bool {
        if suit >= self.foundations.len() {
            return false;
        }
        let Some(source) = self.selected.take() else {
            return false;
        };
        let card = match source {
            CardSource::Waste => self.waste.last().copied(),
            CardSource::Tableau(column, depth) if self.tableau[column].len() == depth + 1 => {
                self.tableau[column].last().copied()
            }
            _ => None,
        };
        let Some(card) = card else {
            self.selected = Some(source);
            return false;
        };
        if card.suit as usize != suit || card.rank != self.foundations[suit] + 1 {
            self.selected = Some(source);
            return false;
        }
        self.snapshot();
        match source {
            CardSource::Waste => {
                self.waste.pop();
            }
            CardSource::Tableau(column, _) => {
                self.tableau[column].pop();
                self.flip_top();
            }
        }
        self.foundations[suit] += 1;
        self.moves += 1;
        if self.foundations == [13; 4] {
            self.status = SolitaireStatus::Won;
        }
        true
    }
    pub fn undo(&mut self) -> bool {
        if let Some((tableau, stock, waste, foundations, moves)) = self.history.pop() {
            self.tableau = tableau;
            self.stock = stock;
            self.waste = waste;
            self.foundations = foundations;
            self.moves = moves;
            self.status = SolitaireStatus::Playing;
            self.selected = None;
            true
        } else {
            false
        }
    }
    fn can_place(&self, destination: usize, card: Card) -> bool {
        let Some(column) = self.tableau.get(destination) else {
            return false;
        };
        match column.last() {
            None => card.rank == 13,
            Some(top) => top.face_up && top.rank == card.rank + 1 && top.red() != card.red(),
        }
    }
    fn flip_top(&mut self) {
        for column in &mut self.tableau {
            if let Some(card) = column.last_mut() {
                card.face_up = true;
            }
        }
    }
    fn snapshot(&mut self) {
        self.history.push((
            self.tableau.clone(),
            self.stock.clone(),
            self.waste.clone(),
            self.foundations,
            self.moves,
        ));
    }
}
#[cfg(test)]
mod tests;
