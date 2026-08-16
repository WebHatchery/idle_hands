//! Seeded 2048 rules and its single-step undo state.

use crate::state::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game2048 {
    pub cells: [u16; 16],
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<([u16; 16], u32, u64)>,
}

impl Default for Game2048 {
    fn default() -> Self {
        Self::new(0x1D1E_2048)
    }
}

impl Game2048 {
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
            let mut i = 0;
            while i < values.len() {
                if i + 1 < values.len() && values[i] == values[i + 1] {
                    merged.push(values[i] * 2);
                    self.score += values[i] as u32 * 2;
                    i += 2;
                } else {
                    merged.push(values[i]);
                    i += 1;
                }
            }
            for (slot, &index) in indices.iter().enumerate() {
                let value = merged.get(slot).copied().unwrap_or(0);
                if self.cells[index] != value {
                    changed = true;
                }
                self.cells[index] = value;
            }
        }
        if changed {
            self.undo = Some((before, before_score, before_seed));
            self.spawn();
            self.best = self.best.max(self.score);
        }
        changed
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

    pub fn can_undo(&self) -> bool {
        self.undo.is_some()
    }

    #[allow(dead_code)]
    pub fn can_move(&self) -> bool {
        self.cells.contains(&0)
            || (0..4).any(|r| (0..3).any(|c| self.cells[r * 4 + c] == self.cells[r * 4 + c + 1]))
            || (0..3).any(|r| (0..4).any(|c| self.cells[r * 4 + c] == self.cells[(r + 1) * 4 + c]))
    }

    pub fn won(&self) -> bool {
        self.cells.iter().any(|&v| v >= 2048)
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .into_iter()
        .find(|&direction| {
            let mut preview = self.clone();
            preview.move_in(direction)
        })
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
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442690888963407);
        let index = empty[(self.seed as usize) % empty.len()];
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442690888963407);
        self.cells[index] = if self.seed & 7 == 0 { 4 } else { 2 };
    }
}

#[cfg(test)]
#[path = "game_2048_tests.rs"]
mod tests;
