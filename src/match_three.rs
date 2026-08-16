//! Deterministic touch-first Match Three swap-and-clear puzzle.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 7;
pub const COLORS: u8 = 5;
const CELLS: usize = SIDE * SIDE;
const EMPTY: u8 = u8::MAX;
const TARGET_SCORE: u16 = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchThreePhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchThree {
    pub cells: Vec<u8>,
    pub selected: Option<usize>,
    pub score: u16,
    pub moves: u16,
    pub seed: u64,
    pub phase: MatchThreePhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for MatchThree {
    fn default() -> Self {
        Self::new(0x004D_4154_4348)
    }
}

impl MatchThree {
    pub fn new(mut seed: u64) -> Self {
        let mut cells = vec![0; CELLS];
        for index in 0..CELLS {
            seed = next_seed(seed);
            let start = (seed % COLORS as u64) as u8;
            cells[index] = (0..COLORS)
                .map(|offset| (start + offset) % COLORS)
                .find(|&color| !creates_match(&cells, index, color))
                .unwrap_or(start);
        }
        Self {
            cells,
            selected: None,
            score: 0,
            moves: 0,
            seed,
            phase: MatchThreePhase::Playing,
            undo: None,
        }
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.phase != MatchThreePhase::Playing || index >= CELLS {
            return false;
        }
        let Some(first) = self.selected else {
            self.selected = Some(index);
            return true;
        };
        if first == index {
            self.selected = None;
            return true;
        }
        if !adjacent(first, index) {
            return false;
        }
        let previous = self.clone_without_undo();
        self.cells.swap(first, index);
        if find_matches(&self.cells).iter().all(|&matched| !matched) {
            self.cells.swap(first, index);
            return false;
        }
        self.undo = Some(Box::new(previous));
        self.selected = None;
        self.moves = self.moves.saturating_add(1);
        self.resolve();
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
        self.phase == MatchThreePhase::Won
    }

    pub fn hint_swap(&self) -> Option<(usize, usize)> {
        if self.phase != MatchThreePhase::Playing {
            return None;
        }
        let mut best: Option<(u16, usize, usize)> = None;
        for first in 0..CELLS {
            for second in [first + 1, first + SIDE] {
                if second >= CELLS || !adjacent(first, second) {
                    continue;
                }
                let mut candidate = self.clone_without_undo();
                candidate.selected = None;
                let before = candidate.score;
                if !candidate.tap(first) || !candidate.tap(second) {
                    continue;
                }
                let gain = candidate.score.saturating_sub(before);
                if best.is_none_or(|(best_gain, _, _)| gain > best_gain) {
                    best = Some((gain, first, second));
                }
            }
        }
        best.map(|(_, first, second)| (first, second))
    }

    fn resolve(&mut self) {
        loop {
            let matches = find_matches(&self.cells);
            let removed = matches.iter().filter(|&&matched| matched).count();
            if removed == 0 {
                break;
            }
            self.score = self
                .score
                .saturating_add((removed as u16).saturating_mul(10));
            for (index, matched) in matches.into_iter().enumerate() {
                if matched {
                    self.cells[index] = EMPTY;
                }
            }
            for col in 0..SIDE {
                let mut filled: Vec<u8> = (0..SIDE)
                    .rev()
                    .filter_map(|row| {
                        let value = self.cells[row * SIDE + col];
                        (value != EMPTY).then_some(value)
                    })
                    .collect();
                for row in (0..SIDE).rev() {
                    self.cells[row * SIDE + col] = filled.pop().unwrap_or_else(|| {
                        self.seed = next_seed(self.seed);
                        (self.seed % COLORS as u64) as u8
                    });
                }
            }
        }
        if self.score >= TARGET_SCORE {
            self.phase = MatchThreePhase::Won;
        }
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

fn creates_match(cells: &[u8], index: usize, color: u8) -> bool {
    let col = index % SIDE;
    let row = index / SIDE;
    (col >= 2 && cells[index - 1] == color && cells[index - 2] == color)
        || (row >= 2 && cells[index - SIDE] == color && cells[index - SIDE * 2] == color)
}

fn adjacent(first: usize, second: usize) -> bool {
    let row_delta = (first / SIDE).abs_diff(second / SIDE);
    let col_delta = (first % SIDE).abs_diff(second % SIDE);
    row_delta + col_delta == 1
}

fn find_matches(cells: &[u8]) -> Vec<bool> {
    let mut matches = vec![false; CELLS];
    for row in 0..SIDE {
        let mut start = 0;
        while start < SIDE {
            let color = cells[row * SIDE + start];
            let mut end = start + 1;
            while end < SIDE && cells[row * SIDE + end] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for col in start..end {
                    matches[row * SIDE + col] = true;
                }
            }
            start = end;
        }
    }
    for col in 0..SIDE {
        let mut start = 0;
        while start < SIDE {
            let color = cells[start * SIDE + col];
            let mut end = start + 1;
            while end < SIDE && cells[end * SIDE + col] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for row in start..end {
                    matches[row * SIDE + col] = true;
                }
            }
            start = end;
        }
    }
    matches
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
