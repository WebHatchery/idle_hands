//! Seeded 2048 rules with selectable board sizes and single-step undo state.

use crate::domain::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Game2048Size {
    #[default]
    Four,
    Five,
}

impl Game2048Size {
    pub const ALL: [Self; 2] = [Self::Four, Self::Five];

    pub const fn dimension(self) -> usize {
        match self {
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Four => "4 × 4",
            Self::Five => "5 × 5",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game2048 {
    #[serde(default)]
    pub board_size: Game2048Size,
    pub cells: Vec<u16>,
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<(Vec<u16>, u32, u64)>,
}

impl Default for Game2048 {
    fn default() -> Self {
        Self::new(0x1D1E_2048)
    }
}

impl Game2048 {
    pub fn new(seed: u64) -> Self {
        Self::new_with_size(seed, Game2048Size::Four)
    }

    pub fn new_with_size(seed: u64, board_size: Game2048Size) -> Self {
        let dimension = board_size.dimension();
        let mut game = Self {
            board_size,
            cells: vec![0; dimension * dimension],
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
        let before = self.cells.clone();
        let before_score = self.score;
        let before_seed = self.seed;
        let mut changed = false;
        let dimension = self.board_size.dimension();
        for line in 0..dimension {
            let indices = match direction {
                Direction::Left => (0..dimension)
                    .map(|offset| line * dimension + offset)
                    .collect::<Vec<_>>(),
                Direction::Right => (0..dimension)
                    .rev()
                    .map(|offset| line * dimension + offset)
                    .collect::<Vec<_>>(),
                Direction::Up => (0..dimension)
                    .map(|offset| line + offset * dimension)
                    .collect::<Vec<_>>(),
                Direction::Down => (0..dimension)
                    .rev()
                    .map(|offset| line + offset * dimension)
                    .collect::<Vec<_>>(),
            };
            let values: Vec<u16> = indices
                .iter()
                .map(|&i| self.cells[i])
                .filter(|&v| v != 0)
                .collect();
            let mut merged = Vec::with_capacity(dimension);
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

    pub fn can_move(&self) -> bool {
        let dimension = self.board_size.dimension();
        self.cells.contains(&0)
            || (0..dimension).any(|r| {
                (0..dimension - 1)
                    .any(|c| self.cells[r * dimension + c] == self.cells[r * dimension + c + 1])
            })
            || (0..dimension - 1).any(|r| {
                (0..dimension)
                    .any(|c| self.cells[r * dimension + c] == self.cells[(r + 1) * dimension + c])
            })
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
#[path = "../tests/legacy/game_2048_tests.rs"]
mod tests;
