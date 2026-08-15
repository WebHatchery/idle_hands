//! Deterministic touch-first Nim with a bounded local opponent.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NimStatus {
    Playing,
    Won,
    Lost,
}

type Snapshot = ([u8; 3], u16, NimStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nim {
    pub seed: u64,
    pub heaps: [u8; 3],
    pub selected_heap: Option<usize>,
    pub moves: u16,
    pub status: NimStatus,
    undo: Option<Snapshot>,
}

impl Default for Nim {
    fn default() -> Self {
        Self::new(0x004E_494D)
    }
}

impl Nim {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            seed,
            heaps: [0; 3],
            selected_heap: None,
            moves: 0,
            status: NimStatus::Playing,
            undo: None,
        };
        game.reset(seed);
        game
    }

    pub fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.heaps = [
            3 + (seed as u8 % 3),
            4 + ((seed >> 8) as u8 % 3),
            5 + ((seed >> 16) as u8 % 3),
        ];
        self.selected_heap = None;
        self.moves = 0;
        self.status = NimStatus::Playing;
        self.undo = None;
    }

    pub fn select_heap(&mut self, heap: usize) {
        if self.status == NimStatus::Playing && heap < self.heaps.len() && self.heaps[heap] > 0 {
            self.selected_heap = Some(heap);
        }
    }

    pub fn take(&mut self, amount: u8) -> bool {
        let Some(heap) = self.selected_heap else {
            return false;
        };
        if self.status != NimStatus::Playing
            || amount == 0
            || amount > 3
            || amount > self.heaps[heap]
        {
            return false;
        }
        self.undo = Some((self.heaps, self.moves, self.status));
        self.heaps[heap] -= amount;
        self.selected_heap = None;
        self.moves = self.moves.saturating_add(1);
        if self.heaps == [0; 3] {
            self.status = NimStatus::Won;
            return true;
        }
        self.ai_move();
        true
    }

    pub fn undo(&mut self) {
        if let Some((heaps, moves, status)) = self.undo.take() {
            self.heaps = heaps;
            self.moves = moves;
            self.status = status;
            self.selected_heap = None;
        }
    }

    pub fn won(&self) -> bool {
        self.status == NimStatus::Won
    }

    fn ai_move(&mut self) {
        let xor = self.heaps.iter().fold(0, |total, heap| total ^ heap);
        let mut choice = None;
        for (index, &heap) in self.heaps.iter().enumerate() {
            let target = heap ^ xor;
            if target < heap && heap - target <= 3 {
                choice = Some((index, heap - target));
                break;
            }
        }
        let (heap, amount) = choice.unwrap_or_else(|| {
            self.heaps
                .iter()
                .enumerate()
                .find(|(_, heap)| **heap > 0)
                .map(|(index, heap)| (index, (*heap).min(3)))
                .unwrap_or((0, 0))
        });
        if amount > 0 {
            self.heaps[heap] -= amount;
        }
        if self.heaps == [0; 3] {
            self.status = NimStatus::Lost;
        }
    }
}

#[cfg(test)]
mod tests;
