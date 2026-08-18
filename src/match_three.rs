//! Deterministic touch-first Match Three swap-and-clear puzzle.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 7;
pub const COLORS: u8 = 5;
const CELLS: usize = SIDE * SIDE;
const EMPTY: u8 = u8::MAX;
const TARGET_SCORE: u16 = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchThreeDifficulty {
    Standard,
    Hard,
    Expert,
}

impl Default for MatchThreeDifficulty {
    fn default() -> Self {
        Self::Standard
    }
}

impl MatchThreeDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    fn settings(self) -> (usize, u8, u16) {
        match self {
            Self::Standard => (7, 5, 120),
            Self::Hard => (8, 6, 240),
            Self::Expert => (9, 7, 360),
        }
    }
}

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
    #[serde(default)]
    pub difficulty: MatchThreeDifficulty,
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
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, MatchThreeDifficulty::Standard)
    }

    pub fn new_with_difficulty(mut seed: u64, difficulty: MatchThreeDifficulty) -> Self {
        let (side, colors, _) = difficulty.settings();
        let mut cells = vec![0; side * side];
        for index in 0..side * side {
            seed = next_seed(seed);
            let start = (seed % colors as u64) as u8;
            cells[index] = (0..colors)
                .map(|offset| (start + offset) % colors)
                .find(|&color| !creates_match_for_side(&cells, index, color, side))
                .unwrap_or(start);
        }
        Self {
            cells,
            selected: None,
            score: 0,
            moves: 0,
            seed,
            difficulty,
            phase: MatchThreePhase::Playing,
            undo: None,
        }
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.phase != MatchThreePhase::Playing || index >= self.cells.len() {
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
        if !adjacent_for_side(first, index, self.side()) {
            return false;
        }
        let previous = self.clone_without_undo();
        self.cells.swap(first, index);
        if find_matches_for_side(&self.cells, self.side())
            .iter()
            .all(|&matched| !matched)
        {
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
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }
    pub fn won(&self) -> bool {
        self.phase == MatchThreePhase::Won
    }

    pub fn hint_swap(&self) -> Option<(usize, usize)> {
        if self.phase != MatchThreePhase::Playing {
            return None;
        }
        let mut best: Option<(u16, usize, usize)> = None;
        for first in 0..self.cells.len() {
            for second in [first + 1, first + self.side()] {
                if second >= self.cells.len() || !adjacent_for_side(first, second, self.side()) {
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
            let matches = find_matches_for_side(&self.cells, self.side());
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
            let side = self.side();
            for col in 0..side {
                let mut filled: Vec<u8> = (0..side)
                    .rev()
                    .filter_map(|row| {
                        let value = self.cells[row * side + col];
                        (value != EMPTY).then_some(value)
                    })
                    .collect();
                for row in (0..side).rev() {
                    self.cells[row * side + col] = filled.pop().unwrap_or_else(|| {
                        self.seed = next_seed(self.seed);
                        (self.seed % self.color_count() as u64) as u8
                    });
                }
            }
        }
        if self.score >= self.target_score() {
            self.phase = MatchThreePhase::Won;
        }
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    pub fn side(&self) -> usize {
        self.difficulty.settings().0
    }

    pub fn color_count(&self) -> u8 {
        self.difficulty.settings().1
    }

    pub fn target_score(&self) -> u16 {
        self.difficulty.settings().2
    }
}

fn creates_match(cells: &[u8], index: usize, color: u8) -> bool {
    creates_match_for_side(cells, index, color, SIDE)
}

fn creates_match_for_side(cells: &[u8], index: usize, color: u8, side: usize) -> bool {
    let col = index % side;
    let row = index / side;
    (col >= 2 && cells[index - 1] == color && cells[index - 2] == color)
        || (row >= 2 && cells[index - side] == color && cells[index - side * 2] == color)
}

fn adjacent(first: usize, second: usize) -> bool {
    adjacent_for_side(first, second, SIDE)
}

fn adjacent_for_side(first: usize, second: usize, side: usize) -> bool {
    let row_delta = (first / side).abs_diff(second / side);
    let col_delta = (first % side).abs_diff(second % side);
    row_delta + col_delta == 1
}

fn find_matches(cells: &[u8]) -> Vec<bool> {
    find_matches_for_side(cells, SIDE)
}

fn find_matches_for_side(cells: &[u8], side: usize) -> Vec<bool> {
    let cells_count = side * side;
    let mut matches = vec![false; cells_count];
    for row in 0..side {
        let mut start = 0;
        while start < side {
            let color = cells[row * side + start];
            let mut end = start + 1;
            while end < side && cells[row * side + end] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for col in start..end {
                    matches[row * side + col] = true;
                }
            }
            start = end;
        }
    }
    for col in 0..side {
        let mut start = 0;
        while start < side {
            let color = cells[start * side + col];
            let mut end = start + 1;
            while end < side && cells[end * side + col] == color {
                end += 1;
            }
            if color != EMPTY && end - start >= 3 {
                for row in start..end {
                    matches[row * side + col] = true;
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
