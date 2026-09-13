//! Seeded one-suit Spider Solitaire with selection-based touch moves.

use crate::cards::Card;
use serde::{Deserialize, Serialize};

const COLUMNS: usize = 8;
const SUITS: usize = 8;
const RUN: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiderStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpiderMode {
    #[default]
    OneSuit,
    TwoSuit,
}

impl SpiderMode {
    pub const ALL: [Self; 2] = [Self::OneSuit, Self::TwoSuit];

    pub const fn label(self) -> &'static str {
        match self {
            Self::OneSuit => "ONE SUIT · STUDY",
            Self::TwoSuit => "TWO SUIT · TACTICAL",
        }
    }

    const fn suit_count(self) -> u8 {
        match self {
            Self::OneSuit => 1,
            Self::TwoSuit => 2,
        }
    }
}

type SpiderSnapshot = (Vec<Vec<Card>>, Vec<Card>, u8, u32, SpiderStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spider {
    pub tableau: Vec<Vec<Card>>,
    pub stock: Vec<Card>,
    pub selected: Option<(usize, usize)>,
    pub completed: u8,
    pub moves: u32,
    pub status: SpiderStatus,
    pub seed: u64,
    #[serde(default)]
    pub mode: SpiderMode,
    #[serde(skip)]
    history: Vec<SpiderSnapshot>,
}

impl Default for Spider {
    fn default() -> Self {
        Self::new(0x005A_1D3E)
    }
}

impl Spider {
    pub fn new(seed: u64) -> Self {
        Self::new_with_mode(seed, SpiderMode::default())
    }

    pub fn new_with_mode(seed: u64, mode: SpiderMode) -> Self {
        let mut deck = Vec::with_capacity(SUITS * RUN);
        for copy in 0..SUITS {
            for rank in 1..=RUN as u8 {
                deck.push(Card {
                    rank,
                    suit: (copy as u8) % mode.suit_count(),
                    face_up: false,
                });
            }
        }
        let mut rng = seed;
        for index in (1..deck.len()).rev() {
            rng = next_seed(rng);
            deck.swap(index, (rng as usize) % (index + 1));
        }
        let mut tableau = vec![Vec::new(); COLUMNS];
        let mut cursor = 0;
        for (column, stack) in tableau.iter_mut().enumerate() {
            let count = if column < 6 { 7 } else { 6 };
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
            status: SpiderStatus::Playing,
            seed: rng,
            mode,
            history: Vec::new(),
        }
    }

    pub fn select_column(&mut self, column: usize, depth: usize) -> bool {
        let Some(stack) = self.tableau.get(column) else {
            return false;
        };
        if depth >= stack.len() || !stack[depth].face_up || !is_run(&stack[depth..]) {
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
        self.moves += 1;
        self.remove_completed_run(destination);
        true
    }

    pub fn deal_stock(&mut self) -> bool {
        if self.stock.is_empty()
            || self.status != SpiderStatus::Playing
            || self.tableau.iter().any(Vec::is_empty)
        {
            return false;
        }
        self.selected = None;
        self.snapshot();
        for stack in &mut self.tableau {
            if let Some(mut card) = self.stock.pop() {
                card.face_up = true;
                stack.push(card);
            }
        }
        self.moves += 1;
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

    pub fn hint_move(&self) -> Option<(usize, usize, usize)> {
        if self.status == SpiderStatus::Won {
            return None;
        }
        for source in 0..COLUMNS {
            for depth in 0..self.tableau[source].len() {
                if !is_run(&self.tableau[source][depth..]) {
                    continue;
                }
                for destination in 0..COLUMNS {
                    if destination != source && self.can_place(destination, depth, source) {
                        return Some((source, depth, destination));
                    }
                }
            }
        }
        None
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
        let complete = if self.mode == SpiderMode::OneSuit {
            is_complete_run(&stack[start..])
        } else {
            is_complete_run_for_mode(&stack[start..])
        };
        if complete {
            stack.truncate(start);
            self.completed += 1;
            if self.completed == SUITS as u8 {
                self.status = SpiderStatus::Won;
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
    is_complete_run_for_mode(cards)
}

fn is_complete_run_for_mode(cards: &[Card]) -> bool {
    cards.len() == RUN
        && cards.iter().all(|card| card.face_up)
        && cards.windows(2).all(|pair| pair[0].suit == pair[1].suit)
        && cards
            .windows(2)
            .all(|pair| pair[0].rank == pair[1].rank + 1)
        && cards.first().is_some_and(|card| card.rank == 13)
        && cards.last().is_some_and(|card| card.rank == 1)
}

fn flip_top(stack: &mut [Card]) {
    if let Some(card) = stack.last_mut() {
        card.face_up = true;
    }
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
#[path = "../tests/legacy/spider/tests.rs"]
mod tests;
