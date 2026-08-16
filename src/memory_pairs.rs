//! Deterministic touch-first Memory/Pairs rules.

use serde::{Deserialize, Serialize};

const CELLS: usize = 16;
const PAIRS: usize = CELLS / 2;

type MemoryUndo = (
    [MemoryCard; CELLS],
    [Option<usize>; 2],
    bool,
    u8,
    u16,
    MemoryStatus,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCard {
    pub pair: u8,
    pub face_up: bool,
    pub matched: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPairs {
    pub cards: [MemoryCard; CELLS],
    pub selected: [Option<usize>; 2],
    pub mismatch_waiting: bool,
    pub matched_pairs: u8,
    pub moves: u16,
    pub seed: u64,
    pub status: MemoryStatus,
    #[serde(skip)]
    undo: Option<MemoryUndo>,
}

impl Default for MemoryPairs {
    fn default() -> Self {
        Self::new(0x1D1E_5045)
    }
}

impl MemoryPairs {
    pub fn new(seed: u64) -> Self {
        let mut pairs = [0u8; CELLS];
        for (index, pair) in pairs.iter_mut().enumerate() {
            *pair = (index / 2) as u8;
        }
        let mut source = seed;
        for index in (1..CELLS).rev() {
            source = next_seed(source);
            let swap = (source as usize) % (index + 1);
            pairs.swap(index, swap);
        }
        Self {
            cards: pairs.map(|pair| MemoryCard {
                pair,
                face_up: false,
                matched: false,
            }),
            selected: [None, None],
            mismatch_waiting: false,
            matched_pairs: 0,
            moves: 0,
            seed,
            status: MemoryStatus::Playing,
            undo: None,
        }
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status == MemoryStatus::Won || self.cards[index].matched {
            return false;
        }
        if self.mismatch_waiting {
            for selected in self.selected.into_iter().flatten() {
                self.cards[selected].face_up = false;
            }
            self.selected = [None, None];
            self.mismatch_waiting = false;
        }
        if self.selected.contains(&Some(index)) {
            return false;
        }
        self.undo = Some((
            self.cards,
            self.selected,
            self.mismatch_waiting,
            self.matched_pairs,
            self.moves,
            self.status,
        ));
        self.cards[index].face_up = true;
        if let Some(first) = self.selected[0] {
            self.selected[1] = Some(index);
            self.moves = self.moves.saturating_add(1);
            if self.cards[first].pair == self.cards[index].pair {
                self.cards[first].matched = true;
                self.cards[index].matched = true;
                self.matched_pairs += 1;
                self.selected = [None, None];
                if self.matched_pairs as usize == PAIRS {
                    self.status = MemoryStatus::Won;
                }
            } else {
                self.mismatch_waiting = true;
            }
        } else {
            self.selected[0] = Some(index);
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cards, selected, mismatch_waiting, matched_pairs, moves, status)) =
            self.undo.take()
        {
            self.cards = cards;
            self.selected = selected;
            self.mismatch_waiting = mismatch_waiting;
            self.matched_pairs = matched_pairs;
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

    pub fn hint_pair(&self) -> Option<(usize, usize)> {
        if self.status == MemoryStatus::Won {
            return None;
        }
        for first in 0..CELLS {
            if self.cards[first].matched {
                continue;
            }
            if let Some(second) = ((first + 1)..CELLS).find(|&index| {
                !self.cards[index].matched && self.cards[index].pair == self.cards[first].pair
            }) {
                return Some((first, second));
            }
        }
        None
    }
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
