//! Deterministic four-suit Spider Solitaire.

use crate::cards::{shuffle_cards, shuffled_deck, Card};
use serde::{Deserialize, Serialize};

const COLUMNS: usize = 10;
const RUNS: usize = 8;
const RUN: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiderSolitaireStatus {
    Playing,
    Won,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpiderRule {
    #[default]
    FourSuit,
    TwoSuit,
    OneSuit,
}

impl SpiderRule {
    pub const ALL: [Self; 3] = [Self::FourSuit, Self::TwoSuit, Self::OneSuit];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FourSuit => "4 SUIT · EXPERT",
            Self::TwoSuit => "2 SUIT · TACTICAL",
            Self::OneSuit => "1 SUIT · STUDY",
        }
    }

    const fn suit_count(self) -> u8 {
        match self {
            Self::FourSuit => 4,
            Self::TwoSuit => 2,
            Self::OneSuit => 1,
        }
    }
}

type Snapshot = (Vec<Vec<Card>>, Vec<Card>, u8, u32, SpiderSolitaireStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderSolitaire {
    pub tableau: Vec<Vec<Card>>,
    pub stock: Vec<Card>,
    pub selected: Option<(usize, usize)>,
    pub completed: u8,
    pub moves: u32,
    pub status: SpiderSolitaireStatus,
    pub seed: u64,
    #[serde(default)]
    pub rule: SpiderRule,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for SpiderSolitaire {
    fn default() -> Self {
        Self::new(0x5A1D_0001)
    }
}

impl SpiderSolitaire {
    pub fn new(seed: u64) -> Self {
        Self::new_with_rule(seed, SpiderRule::default())
    }

    pub fn new_with_rule(seed: u64, rule: SpiderRule) -> Self {
        let (first_deck, first_seed) = shuffled_deck(seed, false);
        let (second_deck, second_seed) = shuffled_deck(first_seed, false);
        let mut deck = first_deck;
        deck.extend(second_deck);
        let (mut deck, shuffled_seed) = shuffle_cards(deck, second_seed);
        for card in &mut deck {
            card.face_up = false;
            card.suit %= rule.suit_count();
        }
        let mut tableau = vec![Vec::new(); COLUMNS];
        let mut cursor = 0;
        for (column, stack) in tableau.iter_mut().enumerate() {
            let count = if column < 4 { 6 } else { 5 };
            for depth in 0..count {
                let mut card = deck[cursor];
                card.face_up = depth + 1 == count;
                stack.push(card);
                cursor += 1;
            }
        }
        let mut stock = deck[cursor..].to_vec();
        stock.reverse();
        Self {
            tableau,
            stock,
            selected: None,
            completed: 0,
            moves: 0,
            status: SpiderSolitaireStatus::Playing,
            seed: shuffled_seed,
            rule,
            history: Vec::new(),
        }
    }

    pub fn select_column(&mut self, column: usize, depth: usize) -> bool {
        let Some(stack) = self.tableau.get(column) else {
            return false;
        };
        if depth >= stack.len() || !is_run(&stack[depth..]) {
            return false;
        }
        self.selected = Some((column, depth));
        true
    }

    pub fn tap_column(&mut self, column: usize, depth: usize) -> bool {
        if self.selected == Some((column, depth)) {
            self.selected = None;
            return true;
        }
        if self.selected.is_some() && self.move_selected(column) {
            return true;
        }
        self.select_column(column, depth)
    }

    pub fn move_selected(&mut self, destination: usize) -> bool {
        let Some((source, depth)) = self.selected.take() else {
            return false;
        };
        if source == destination || !self.can_place(destination, depth, source) {
            self.selected = Some((source, depth));
            return false;
        }
        self.snapshot();
        let moving = self.tableau[source].split_off(depth);
        self.tableau[destination].extend(moving);
        flip_top(&mut self.tableau[source]);
        self.moves = self.moves.saturating_add(1);
        self.remove_completed_run(destination);
        true
    }

    pub fn deal_stock(&mut self) -> bool {
        if self.stock.is_empty()
            || self.status != SpiderSolitaireStatus::Playing
            || self.tableau.iter().any(Vec::is_empty)
        {
            return false;
        }
        self.selected = None;
        self.snapshot();
        for stack in &mut self.tableau {
            let Some(mut card) = self.stock.pop() else {
                break;
            };
            card.face_up = true;
            stack.push(card);
        }
        self.moves = self.moves.saturating_add(1);
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((tableau, stock, completed, moves, status)) = self.history.pop() {
            self.tableau = tableau;
            self.stock = stock;
            self.completed = completed;
            self.moves = moves;
            self.status = status;
            self.selected = None;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn can_place(&self, destination: usize, depth: usize, source: usize) -> bool {
        let Some(card) = self.tableau[source].get(depth) else {
            return false;
        };
        let Some(top) = self.tableau.get(destination).and_then(|stack| stack.last()) else {
            return true;
        };
        top.face_up && top.rank == card.rank + 1
    }

    fn remove_completed_run(&mut self, destination: usize) {
        let stack = &mut self.tableau[destination];
        if stack.len() < RUN {
            return;
        }
        let start = stack.len() - RUN;
        if is_complete_run(&stack[start..]) {
            stack.truncate(start);
            self.completed = self.completed.saturating_add(1);
            if self.completed == RUNS as u8 {
                self.status = SpiderSolitaireStatus::Won;
            }
        }
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.tableau.clone(),
            self.stock.clone(),
            self.completed,
            self.moves,
            self.status,
        ));
    }
}

pub fn is_run(cards: &[Card]) -> bool {
    !cards.is_empty()
        && cards.windows(2).all(|pair| {
            pair[0].face_up
                && pair[1].face_up
                && pair[0].suit == pair[1].suit
                && pair[0].rank == pair[1].rank + 1
        })
}

pub fn is_complete_run(cards: &[Card]) -> bool {
    cards.len() == RUN
        && is_run(cards)
        && cards.first().is_some_and(|card| card.rank == 13)
        && cards.last().is_some_and(|card| card.rank == 1)
}

fn flip_top(stack: &mut [Card]) {
    if let Some(card) = stack.last_mut() {
        card.face_up = true;
    }
}

#[cfg(test)]
mod tests;
