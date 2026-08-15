//! Deterministic touch-first TriPeaks Solitaire.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriPeaksStatus {
    Playing,
    Won,
    Stuck,
}

type Snapshot = (Vec<Option<Card>>, Vec<Card>, Vec<Card>, u16, TriPeaksStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriPeaks {
    pub tableau: Vec<Option<Card>>,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub moves: u16,
    pub status: TriPeaksStatus,
    pub seed: u64,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for TriPeaks {
    fn default() -> Self {
        Self::new(0x7A1F_0001)
    }
}

impl TriPeaks {
    pub fn new(seed: u64) -> Self {
        let (deck, rng) = shuffled_deck(seed, true);
        let tableau = deck.iter().take(28).copied().map(Some).collect();
        let mut stock = deck[28..].to_vec();
        stock.reverse();
        let waste = stock.pop().into_iter().collect();
        let mut game = Self {
            tableau,
            stock,
            waste,
            moves: 0,
            status: TriPeaksStatus::Playing,
            seed: rng,
            history: Vec::new(),
        };
        game.resolve();
        game
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.status != TriPeaksStatus::Playing || !self.can_play(index) {
            return false;
        }
        self.snapshot();
        let card = self.tableau[index].take().expect("playable card");
        self.waste.push(card);
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn draw_stock(&mut self) -> bool {
        if self.status != TriPeaksStatus::Playing || self.stock.is_empty() {
            self.resolve();
            return false;
        }
        self.snapshot();
        self.waste
            .push(self.stock.pop().expect("stock checked above"));
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

    pub fn exposed(&self, index: usize) -> bool {
        index < self.tableau.len()
            && self.tableau[index].is_some()
            && children(index)
                .iter()
                .all(|&child| self.tableau[child].is_none())
    }

    pub fn can_play(&self, index: usize) -> bool {
        self.exposed(index)
            && self
                .waste
                .last()
                .is_some_and(|waste| self.tableau[index].unwrap().rank.abs_diff(waste.rank) == 1)
    }

    fn resolve(&mut self) {
        if self.tableau.iter().all(Option::is_none) {
            self.status = TriPeaksStatus::Won;
        } else if self.stock.is_empty() && !(0..28).any(|index| self.can_play(index)) {
            self.status = TriPeaksStatus::Stuck;
        } else {
            self.status = TriPeaksStatus::Playing;
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

fn children(index: usize) -> &'static [usize] {
    match index {
        0 => &[3, 4],
        1 => &[5, 6],
        2 => &[7, 8],
        3 => &[9, 10],
        4 => &[10, 11],
        5 => &[12, 13],
        6 => &[13, 14],
        7 => &[15, 16],
        8 => &[16, 17],
        9 => &[18, 19],
        10 => &[19, 20],
        11 => &[20, 21],
        12 => &[21, 22],
        13 => &[22, 23],
        14 => &[23, 24],
        15 => &[24, 25],
        16 => &[25, 26],
        17 => &[26, 27],
        _ => &[],
    }
}

#[cfg(test)]
mod tests;
