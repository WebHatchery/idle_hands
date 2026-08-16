//! Deterministic touch-first Lights Out rules.

use serde::{Deserialize, Serialize};

pub const SIZE: usize = 5;
const CELLS: usize = SIZE * SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightsOutStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightsOut {
    pub cells: [bool; CELLS],
    pub moves: u16,
    pub seed: u64,
    pub status: LightsOutStatus,
    #[serde(skip)]
    undo: Option<([bool; CELLS], u16, LightsOutStatus)>,
}

impl Default for LightsOut {
    fn default() -> Self {
        Self::new(0x1D1E_1075)
    }
}

impl LightsOut {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            cells: [false; CELLS],
            moves: 0,
            seed,
            status: LightsOutStatus::Playing,
            undo: None,
        };
        let mut source = seed;
        for _ in 0..12 {
            source = next_seed(source);
            game.toggle_pattern((source as usize) % CELLS);
        }
        if !game.cells.iter().any(|cell| *cell) {
            game.toggle_pattern(CELLS / 2);
        }
        game
    }

    pub fn press(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status == LightsOutStatus::Won {
            return false;
        }
        self.undo = Some((self.cells, self.moves, self.status));
        self.toggle_pattern(index);
        self.moves = self.moves.saturating_add(1);
        if self.cells.iter().all(|cell| !cell) {
            self.status = LightsOutStatus::Won;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, moves, status)) = self.undo.take() {
            self.cells = cells;
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

    pub fn hint_move(&self) -> Option<usize> {
        if self.status == LightsOutStatus::Won {
            return None;
        }
        (0..CELLS).min_by_key(|&index| {
            let mut preview = self.clone();
            preview.press(index);
            preview.cells.iter().filter(|cell| **cell).count()
        })
    }

    fn toggle_pattern(&mut self, index: usize) {
        let row = index / SIZE;
        let column = index % SIZE;
        for (row_delta, column_delta) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row as isize + row_delta;
            let next_column = column as isize + column_delta;
            if (0..SIZE as isize).contains(&next_row) && (0..SIZE as isize).contains(&next_column) {
                let next = next_row as usize * SIZE + next_column as usize;
                self.cells[next] = !self.cells[next];
            }
        }
    }
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
