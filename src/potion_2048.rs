//! Deterministic potion-themed 2048 rules.

use crate::state::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PotionDifficulty {
    Standard,
    Hard,
    Expert,
}

impl Default for PotionDifficulty {
    fn default() -> Self {
        Self::Standard
    }
}

impl PotionDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    fn settings(self) -> (usize, u16) {
        match self {
            Self::Standard => (4, 4096),
            Self::Hard => (5, 8192),
            Self::Expert => (6, 16384),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion2048 {
    pub cells: Vec<u16>,
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: PotionDifficulty,
    #[serde(skip)]
    undo: Option<(Vec<u16>, u32, u64)>,
}

impl Default for Potion2048 {
    fn default() -> Self {
        Self::new(0xB071_2048)
    }
}

impl Potion2048 {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, PotionDifficulty::Standard)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: PotionDifficulty) -> Self {
        let side = difficulty.settings().0;
        let mut game = Self {
            cells: vec![0; side * side],
            score: 0,
            best: 0,
            seed,
            difficulty,
            undo: None,
        };
        game.spawn();
        game.spawn();
        game
    }
    pub fn move_in(&mut self, direction: Direction) -> bool {
        let before = self.cells.clone();
        let before_score = self.score;
        let before_seed = self.seed;
        let mut changed = false;
        let side = self.side();
        for line in 0..side {
            let indices = line_indices(side, line, direction);
            let values: Vec<u16> = indices
                .iter()
                .map(|&i| self.cells[i])
                .filter(|&v| v != 0)
                .collect();
            let mut merged = Vec::with_capacity(4);
            let mut index = 0;
            while index < values.len() {
                if index + 1 < values.len() && values[index] == values[index + 1] {
                    merged.push(values[index] * 2);
                    self.score = self.score.saturating_add(values[index] as u32 * 2);
                    index += 2;
                } else {
                    merged.push(values[index]);
                    index += 1;
                }
            }
            for (slot, &cell) in indices.iter().enumerate() {
                let value = merged.get(slot).copied().unwrap_or(0);
                if self.cells[cell] != value {
                    changed = true;
                }
                self.cells[cell] = value;
            }
        }
        if changed {
            self.undo = Some((before, before_score, before_seed));
            self.spawn();
            self.best = self.best.max(self.score);
        }
        changed
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.won() {
            return None;
        }
        let mut best = None;
        let mut best_gain = 0;
        let mut best_empty = 0;
        for direction in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ] {
            let mut candidate = self.clone();
            let score = candidate.score;
            if !candidate.move_in(direction) {
                continue;
            }
            let gain = candidate.score - score;
            let empty = candidate.cells.iter().filter(|&&value| value == 0).count();
            if best.is_none() || gain > best_gain || (gain == best_gain && empty > best_empty) {
                best = Some(direction);
                best_gain = gain;
                best_empty = empty;
            }
        }
        best
    }
    pub fn undo(&mut self) -> bool {
        if let Some((cells, score, seed)) = self.undo.take() {
            self.cells = cells;
            self.score = score;
            self.seed = seed;
            true
        } else {
            false
        }
    }
    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }
    pub fn won(&self) -> bool {
        self.cells.iter().any(|&value| value >= self.target())
    }
    fn spawn(&mut self) {
        let empty: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some(i))
            .collect();
        if empty.is_empty() {
            return;
        }
        self.seed = next_seed(self.seed);
        let index = empty[(self.seed as usize) % empty.len()];
        self.seed = next_seed(self.seed);
        self.cells[index] = if self.seed & 7 == 0 { 4 } else { 2 };
    }

    pub fn side(&self) -> usize {
        self.difficulty.settings().0
    }

    pub fn target(&self) -> u16 {
        self.difficulty.settings().1
    }
}

fn line_indices(side: usize, line: usize, direction: Direction) -> Vec<usize> {
    let indices: Vec<usize> = match direction {
        Direction::Left => (0..side).map(|offset| line * side + offset).collect(),
        Direction::Right => (0..side).rev().map(|offset| line * side + offset).collect(),
        Direction::Up => (0..side).map(|offset| line + offset * side).collect(),
        Direction::Down => (0..side).rev().map(|offset| line + offset * side).collect(),
    };
    indices
}
fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
