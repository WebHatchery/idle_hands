//! Deterministic potion-themed 2048 rules.

use crate::state::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion2048 {
    pub cells: [u16; 16],
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<([u16; 16], u32, u64)>,
}

impl Default for Potion2048 {
    fn default() -> Self {
        Self::new(0xB071_2048)
    }
}

impl Potion2048 {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            cells: [0; 16],
            score: 0,
            best: 0,
            seed,
            undo: None,
        };
        game.spawn();
        game.spawn();
        game
    }
    pub fn move_in(&mut self, direction: Direction) -> bool {
        let before = self.cells;
        let before_score = self.score;
        let before_seed = self.seed;
        let mut changed = false;
        for line in 0..4 {
            let indices = match direction {
                Direction::Left => [line * 4, line * 4 + 1, line * 4 + 2, line * 4 + 3],
                Direction::Right => [line * 4 + 3, line * 4 + 2, line * 4 + 1, line * 4],
                Direction::Up => [line, line + 4, line + 8, line + 12],
                Direction::Down => [line + 12, line + 8, line + 4, line],
            };
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
        *self = Self::new(seed);
    }
    pub fn won(&self) -> bool {
        self.cells.iter().any(|&value| value >= 4096)
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
}
fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
