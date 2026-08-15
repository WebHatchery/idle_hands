//! Deterministic adjacent-pair Number Match rules.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 6;
const CELLS: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumberMatchPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberMatch {
    pub cells: Vec<u8>,
    pub selected: Option<usize>,
    pub moves: u16,
    pub score: u16,
    pub seed: u64,
    pub phase: NumberMatchPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for NumberMatch {
    fn default() -> Self {
        Self::new(0x4E55_4D42_4552)
    }
}

impl NumberMatch {
    pub fn new(seed: u64) -> Self {
        let mut cells = vec![0; CELLS];
        for pair in 0..(CELLS / 2) {
            let first = ((seed.wrapping_add(pair as u64) % 9) + 1) as u8;
            let second = if pair % 3 == 0 { 10 - first } else { first };
            cells[pair * 2] = first;
            cells[pair * 2 + 1] = second;
        }
        Self {
            cells,
            selected: None,
            moves: 0,
            score: 0,
            seed,
            phase: NumberMatchPhase::Playing,
            undo: None,
        }
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.phase != NumberMatchPhase::Playing || index >= CELLS || self.cells[index] == 0 {
            return false;
        }
        let Some(previous) = self.selected else {
            self.selected = Some(index);
            return true;
        };
        if previous == index {
            self.selected = None;
            return true;
        }
        if !self.adjacent(previous, index) || !self.valid_pair(previous, index) {
            self.selected = Some(index);
            return true;
        }
        let snapshot = self.clone_without_undo();
        self.undo = Some(Box::new(snapshot));
        self.cells[previous] = 0;
        self.cells[index] = 0;
        self.selected = None;
        self.moves = self.moves.saturating_add(1);
        self.score = self.score.saturating_add(1);
        if self.cells.iter().all(|&value| value == 0) {
            self.phase = NumberMatchPhase::Won;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.take() else {
            return false;
        };
        *self = *previous;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == NumberMatchPhase::Won
    }

    fn adjacent(&self, first: usize, second: usize) -> bool {
        let first_row = first / SIDE;
        let first_col = first % SIDE;
        let second_row = second / SIDE;
        let second_col = second % SIDE;
        first_row.abs_diff(second_row) + first_col.abs_diff(second_col) == 1
    }

    fn valid_pair(&self, first: usize, second: usize) -> bool {
        self.cells[first] == self.cells[second] || self.cells[first] + self.cells[second] == 10
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

#[cfg(test)]
mod tests;
