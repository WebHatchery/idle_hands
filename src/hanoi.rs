//! Deterministic touch-first Towers of Hanoi.

use serde::{Deserialize, Serialize};

const DISKS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HanoiPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hanoi {
    pub stacks: [Vec<u8>; 3],
    pub selected: Option<usize>,
    pub moves: u16,
    pub seed: u64,
    pub phase: HanoiPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for Hanoi {
    fn default() -> Self {
        Self::new(0x0048_414E_4F49)
    }
}

impl Hanoi {
    pub fn new(seed: u64) -> Self {
        Self {
            stacks: [vec![5, 4, 3, 2, 1], Vec::new(), Vec::new()],
            selected: None,
            moves: 0,
            seed,
            phase: HanoiPhase::Playing,
            undo: None,
        }
    }

    pub fn tap_peg(&mut self, peg: usize) -> bool {
        if self.phase != HanoiPhase::Playing || peg >= 3 {
            return false;
        }
        let Some(source) = self.selected else {
            if self.stacks[peg].is_empty() {
                return false;
            }
            self.selected = Some(peg);
            return true;
        };
        if source == peg {
            self.selected = None;
            return true;
        }
        let Some(&disk) = self.stacks[source].last() else {
            self.selected = None;
            return false;
        };
        if self.stacks[peg].last().is_some_and(|&top| top < disk) {
            self.selected = None;
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        self.stacks[source].pop();
        self.stacks[peg].push(disk);
        self.moves = self.moves.saturating_add(1);
        self.selected = None;
        if self.stacks[2].len() == DISKS {
            self.phase = HanoiPhase::Won;
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
        self.phase == HanoiPhase::Won
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

#[cfg(test)]
mod tests;
