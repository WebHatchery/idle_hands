//! Deterministic touch-first Pyramid Solitaire.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

pub const WASTE_INDEX: usize = 28;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PyramidStatus {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PyramidDraw {
    #[default]
    One,
    Three,
}

impl PyramidDraw {
    pub const fn label(self) -> &'static str {
        match self {
            Self::One => "DRAW 1",
            Self::Three => "DRAW 3",
        }
    }
    const fn count(self) -> usize {
        match self {
            Self::One => 1,
            Self::Three => 3,
        }
    }
}

type Snapshot = (
    Vec<Option<Card>>,
    Vec<Card>,
    Vec<Card>,
    Option<usize>,
    u16,
    PyramidStatus,
    u32,
    u16,
    u16,
    u8,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pyramid {
    pub pyramid: Vec<Option<Card>>,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub selected: Option<usize>,
    pub moves: u16,
    pub status: PyramidStatus,
    pub seed: u64,
    #[serde(default)]
    pub draw_rule: PyramidDraw,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub combo: u16,
    #[serde(default)]
    pub best_combo: u16,
    #[serde(default = "default_redeals")]
    pub redeals_remaining: u8,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for Pyramid {
    fn default() -> Self {
        Self::new(0x51A0_0001)
    }
}

impl Pyramid {
    pub fn new(seed: u64) -> Self {
        let (deck, rng) = shuffled_deck(seed, true);
        let mut pyramid = vec![None; 28];
        for (index, card) in deck.iter().take(28).copied().enumerate() {
            pyramid[index] = Some(card);
        }
        let mut stock = deck[28..].to_vec();
        stock.reverse();
        let mut game = Self {
            pyramid,
            stock,
            waste: Vec::new(),
            selected: None,
            moves: 0,
            status: PyramidStatus::Playing,
            seed: rng,
            draw_rule: PyramidDraw::One,
            points: 0,
            combo: 0,
            best_combo: 0,
            redeals_remaining: default_redeals(),
            history: Vec::new(),
        };
        game.resolve();
        game
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.status != PyramidStatus::Playing || !self.available(index) {
            return false;
        }
        if let Some(selected) = self.selected {
            if selected == index {
                self.selected = None;
                return true;
            }
            let Some(first) = self.card_at(selected) else {
                self.selected = None;
                return false;
            };
            let Some(second) = self.card_at(index) else {
                self.selected = None;
                return false;
            };
            if first.rank.saturating_add(second.rank) == 13 {
                self.snapshot();
                self.remove(selected);
                self.remove(index);
                self.score_clear(2);
                self.selected = None;
                self.moves = self.moves.saturating_add(1);
                self.resolve();
                return true;
            }
        }
        if self.card_at(index).is_some_and(|card| card.rank == 13) {
            self.snapshot();
            self.remove(index);
            self.score_clear(1);
            self.selected = None;
            self.moves = self.moves.saturating_add(1);
            self.resolve();
            return true;
        }
        self.selected = Some(index);
        true
    }

    pub fn draw_stock(&mut self) -> bool {
        if self.status != PyramidStatus::Playing {
            return false;
        }
        if self.stock.is_empty() {
            if self.redeals_remaining == 0 || self.waste.is_empty() {
                self.resolve();
                return false;
            }
            self.snapshot();
            self.stock = self.waste.drain(..).rev().collect();
            self.redeals_remaining = self.redeals_remaining.saturating_sub(1);
            self.selected = None;
            self.combo = 0;
            self.moves = self.moves.saturating_add(1);
            self.resolve();
            return true;
        }
        self.snapshot();
        for _ in 0..self.draw_rule.count() {
            let Some(card) = self.stock.pop() else {
                break;
            };
            self.waste.push(card);
        }
        self.selected = None;
        self.combo = 0;
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((
            pyramid,
            stock,
            waste,
            selected,
            moves,
            status,
            points,
            combo,
            best_combo,
            redeals_remaining,
        )) = self.history.pop()
        {
            self.pyramid = pyramid;
            self.stock = stock;
            self.waste = waste;
            self.selected = selected;
            self.moves = moves;
            self.status = status;
            self.points = points;
            self.combo = combo;
            self.best_combo = best_combo;
            self.redeals_remaining = redeals_remaining;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        let rule = self.draw_rule;
        *self = Self::new(seed);
        self.draw_rule = rule;
    }

    pub fn set_draw_rule(&mut self, rule: PyramidDraw, seed: u64) {
        *self = Self::new(seed);
        self.draw_rule = rule;
    }

    pub fn available(&self, index: usize) -> bool {
        if index == WASTE_INDEX {
            return !self.waste.is_empty();
        }
        index < self.pyramid.len() && self.pyramid[index].is_some() && self.exposed(index)
    }

    pub fn exposed(&self, index: usize) -> bool {
        if index >= 28 {
            return false;
        }
        let row = row_for(index);
        if row == 6 {
            return true;
        }
        let position = index - row_start(row);
        self.pyramid[row_start(row + 1) + position].is_none()
            && self.pyramid[row_start(row + 1) + position + 1].is_none()
    }

    pub fn legal_pair(&self, first: usize, second: usize) -> bool {
        first != second
            && self.available(first)
            && self.available(second)
            && self
                .card_at(first)
                .zip(self.card_at(second))
                .is_some_and(|(left, right)| left.rank.saturating_add(right.rank) == 13)
    }

    pub fn available_pair_count(&self) -> usize {
        let indices: Vec<_> = (0..=WASTE_INDEX)
            .filter(|&index| self.available(index))
            .collect();
        let kings = indices
            .iter()
            .filter(|&&index| self.card_at(index).is_some_and(|card| card.rank == 13))
            .count();
        kings
            + indices
                .iter()
                .enumerate()
                .map(|(offset, &first)| {
                    indices
                        .iter()
                        .skip(offset + 1)
                        .filter(|&&second| self.legal_pair(first, second))
                        .count()
                })
                .sum::<usize>()
    }

    fn card_at(&self, index: usize) -> Option<Card> {
        if index == WASTE_INDEX {
            self.waste.last().copied()
        } else {
            self.pyramid.get(index).copied().flatten()
        }
    }

    fn remove(&mut self, index: usize) {
        if index == WASTE_INDEX {
            self.waste.pop();
        } else if let Some(card) = self.pyramid.get_mut(index) {
            *card = None;
        }
    }

    fn resolve(&mut self) {
        if self.pyramid.iter().all(Option::is_none) {
            self.status = PyramidStatus::Won;
        } else if self.stock.is_empty()
            && (self.redeals_remaining == 0 || self.waste.is_empty())
            && !self.has_move()
        {
            self.status = PyramidStatus::Stuck;
        } else {
            self.status = PyramidStatus::Playing;
        }
    }

    fn has_move(&self) -> bool {
        if self.waste.last().is_some_and(|card| card.rank == 13) {
            return true;
        }
        let available = (0..28).filter(|&index| self.available(index));
        let mut cards = Vec::new();
        for index in available {
            let card = self.card_at(index).expect("available card");
            if card.rank == 13 {
                return true;
            }
            cards.push(card);
        }
        if let Some(waste) = self.waste.last() {
            if cards
                .iter()
                .any(|card| card.rank.saturating_add(waste.rank) == 13)
            {
                return true;
            }
        }
        cards.iter().enumerate().any(|(left, card)| {
            cards
                .iter()
                .skip(left + 1)
                .any(|other| card.rank.saturating_add(other.rank) == 13)
        })
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.pyramid.clone(),
            self.stock.clone(),
            self.waste.clone(),
            self.selected,
            self.moves,
            self.status,
            self.points,
            self.combo,
            self.best_combo,
            self.redeals_remaining,
        ));
    }

    fn score_clear(&mut self, cards: u32) {
        self.combo = self.combo.saturating_add(1);
        self.best_combo = self.best_combo.max(self.combo);
        self.points = self.points.saturating_add(
            cards
                .saturating_mul(u32::from(self.combo))
                .saturating_mul(10),
        );
    }
}

const fn default_redeals() -> u8 {
    1
}

fn row_start(row: usize) -> usize {
    row * (row + 1) / 2
}

fn row_for(index: usize) -> usize {
    (0..7).find(|&row| index < row_start(row + 1)).unwrap_or(6)
}

#[cfg(test)]
mod tests;
