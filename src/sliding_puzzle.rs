//! Deterministic touch-first 15-puzzle rules.

use serde::{Deserialize, Serialize};

const CELLS: usize = 16;
const SIDE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlidingStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlidingPuzzle {
    pub cells: [u8; CELLS],
    pub moves: u16,
    pub seed: u64,
    pub status: SlidingStatus,
    #[serde(skip)]
    undo: Option<([u8; CELLS], u16, SlidingStatus)>,
}

impl Default for SlidingPuzzle {
    fn default() -> Self {
        Self::new(0x1D1E_1500)
    }
}

impl SlidingPuzzle {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            cells: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
            moves: 0,
            seed,
            status: SlidingStatus::Playing,
            undo: None,
        };
        let mut source = seed;
        let mut previous = CELLS - 1;
        for _ in 0..80 {
            source = next_seed(source);
            let blank = game.blank();
            let neighbors = neighbors(blank);
            let mut choice = (source as usize) % neighbors.len();
            if neighbors[choice] == previous && neighbors.len() > 1 {
                choice = (choice + 1) % neighbors.len();
            }
            previous = blank;
            game.swap_blank(neighbors[choice]);
        }
        if game.is_solved() {
            game.swap_blank(14);
        }
        game
    }

    pub fn move_tile(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status == SlidingStatus::Won {
            return false;
        }
        if !neighbors(self.blank()).contains(&index) {
            return false;
        }
        self.undo = Some((self.cells, self.moves, self.status));
        self.swap_blank(index);
        self.moves = self.moves.saturating_add(1);
        if self.is_solved() {
            self.status = SlidingStatus::Won;
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

    fn blank(&self) -> usize {
        self.cells.iter().position(|cell| *cell == 0).unwrap()
    }

    fn swap_blank(&mut self, tile: usize) {
        let blank = self.blank();
        self.cells.swap(blank, tile);
    }

    fn is_solved(&self) -> bool {
        self.cells == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
    }
}

fn neighbors(index: usize) -> Vec<usize> {
    let row = index / SIDE;
    let column = index % SIDE;
    let mut result = Vec::with_capacity(4);
    if row > 0 {
        result.push(index - SIDE);
    }
    if row + 1 < SIDE {
        result.push(index + SIDE);
    }
    if column > 0 {
        result.push(index - 1);
    }
    if column + 1 < SIDE {
        result.push(index + 1);
    }
    result
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
