//! Deterministic touch-first 15-puzzle rules.

use serde::{Deserialize, Serialize};

const CELLS: usize = 16;
const SIDE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlidingStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SlidingVariant {
    #[default]
    Classic,
    Wanderer,
    Marathon,
}

impl SlidingVariant {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Wanderer, Self::Marathon];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · 80 SHUFFLES",
            Self::Wanderer => "WANDERER · 40 SHUFFLES",
            Self::Marathon => "MARATHON · 140 SHUFFLES",
        }
    }

    const fn scramble_steps(self) -> usize {
        match self {
            Self::Classic => 80,
            Self::Wanderer => 40,
            Self::Marathon => 140,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlidingPuzzle {
    pub cells: [u8; CELLS],
    pub moves: u16,
    pub seed: u64,
    pub status: SlidingStatus,
    #[serde(default)]
    pub variant: SlidingVariant,
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
        Self::new_with_variant(seed, SlidingVariant::default())
    }

    pub fn new_with_variant(seed: u64, variant: SlidingVariant) -> Self {
        let mut game = Self {
            cells: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
            moves: 0,
            seed,
            status: SlidingStatus::Playing,
            variant,
            undo: None,
        };
        let mut source = seed;
        let mut previous = CELLS - 1;
        for _ in 0..variant.scramble_steps() {
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

    pub fn hint_move(&self) -> Option<usize> {
        if self.status == SlidingStatus::Won {
            return None;
        }
        let blank = self.blank();
        let mut best = None;
        let mut best_distance = usize::MAX;
        for tile in neighbors(blank) {
            let mut preview = self.cells;
            preview.swap(blank, tile);
            let distance = board_distance(&preview);
            if distance < best_distance {
                best = Some(tile);
                best_distance = distance;
            }
        }
        best
    }

    fn blank(&self) -> usize {
        // All constructors and validated saves contain exactly one blank tile.
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

fn board_distance(cells: &[u8; CELLS]) -> usize {
    cells
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != 0)
        .map(|(index, value)| {
            let target = (*value as usize) - 1;
            let row = index / SIDE;
            let column = index % SIDE;
            let target_row = target / SIDE;
            let target_column = target % SIDE;
            row.abs_diff(target_row) + column.abs_diff(target_column)
        })
        .sum()
}

#[cfg(test)]
#[path = "../tests/legacy/sliding_puzzle/tests.rs"]
mod tests;
