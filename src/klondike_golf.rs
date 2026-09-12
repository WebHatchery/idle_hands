//! Deterministic touch-first Klondike Golf.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GolfStatus {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GolfRule {
    #[default]
    Classic,
    Wrap,
    Relaxed,
}

impl GolfRule {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Wrap, Self::Relaxed];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · ADJACENT RANKS",
            Self::Wrap => "WRAP · ACE TOUCHES KING",
            Self::Relaxed => "RELAXED · MATCHING RANKS TOO",
        }
    }
}

type Snapshot = (Vec<Vec<Card>>, Vec<Card>, Vec<Card>, u16, GolfStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlondikeGolf {
    pub tableau: Vec<Vec<Card>>,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub moves: u16,
    pub status: GolfStatus,
    pub seed: u64,
    #[serde(default)]
    pub rule: GolfRule,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for KlondikeGolf {
    fn default() -> Self {
        Self::new(0x601F_0001)
    }
}

impl KlondikeGolf {
    pub fn new(seed: u64) -> Self {
        Self::new_with_rule(seed, GolfRule::default())
    }

    pub fn new_with_rule(seed: u64, rule: GolfRule) -> Self {
        let (deck, rng) = shuffled_deck(seed, false);
        let mut tableau = vec![Vec::new(); 7];
        let mut cursor = 0;
        for column in &mut tableau {
            for _ in 0..5 {
                let mut card = deck[cursor];
                card.face_up = true;
                column.push(card);
                cursor += 1;
            }
        }
        let mut stock = deck[cursor..].to_vec();
        stock.reverse();
        let waste = stock.pop().into_iter().collect();
        Self {
            tableau,
            stock,
            waste,
            moves: 0,
            status: GolfStatus::Playing,
            seed: rng,
            rule,
            history: Vec::new(),
        }
    }

    pub fn tap_column(&mut self, column: usize) -> bool {
        if self.status != GolfStatus::Playing {
            return false;
        }
        let Some(card) = self
            .tableau
            .get(column)
            .and_then(|stack| stack.last())
            .copied()
        else {
            return false;
        };
        let Some(waste) = self.waste.last().copied() else {
            return false;
        };
        if !self.rank_is_playable(card.rank, waste.rank) {
            return false;
        }
        self.snapshot();
        self.tableau[column].pop();
        self.waste.push(card);
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn draw_stock(&mut self) -> bool {
        if self.status != GolfStatus::Playing || self.stock.is_empty() {
            self.resolve();
            return false;
        }
        self.snapshot();
        // The guard above establishes that the stock contains a card.
        self.waste.push(self.stock.pop().unwrap());
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((tableau, stock, waste, moves, status)) = self.history.pop() {
            self.tableau = tableau;
            self.stock = stock;
            self.waste = waste;
            self.moves = moves;
            self.status = status;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn resolve(&mut self) {
        if self.tableau.iter().all(Vec::is_empty) {
            self.status = GolfStatus::Won;
        } else if self.stock.is_empty()
            && !self
                .tableau
                .iter()
                .enumerate()
                .any(|(column, _)| self.can_play(column))
        {
            self.status = GolfStatus::Stuck;
        }
    }

    fn can_play(&self, column: usize) -> bool {
        let Some(card) = self.tableau.get(column).and_then(|stack| stack.last()) else {
            return false;
        };
        self.waste
            .last()
            .is_some_and(|waste| self.rank_is_playable(card.rank, waste.rank))
    }

    fn rank_is_playable(&self, card: u8, waste: u8) -> bool {
        match self.rule {
            GolfRule::Classic => card.abs_diff(waste) == 1,
            GolfRule::Wrap => {
                card.abs_diff(waste) == 1
                    || (card == 1 && waste == 13)
                    || (card == 13 && waste == 1)
            }
            GolfRule::Relaxed => card.abs_diff(waste) <= 1,
        }
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.tableau.clone(),
            self.stock.clone(),
            self.waste.clone(),
            self.moves,
            self.status,
        ));
    }
}

#[cfg(test)]
mod tests;
