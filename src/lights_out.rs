//! Deterministic touch-first Lights Out rules.

use serde::{Deserialize, Serialize};

pub const SIZE: usize = 5;
const CELLS: usize = SIZE * SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightsOutStatus {
    Playing,
    Won,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightsDifficulty {
    #[default]
    Classic,
    Dense,
}

impl LightsDifficulty {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC 12",
            Self::Dense => "DENSE 20",
        }
    }

    const fn scramble_presses(self) -> usize {
        match self {
            Self::Classic => 12,
            Self::Dense => 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightsOut {
    pub cells: [bool; CELLS],
    pub moves: u16,
    pub seed: u64,
    pub status: LightsOutStatus,
    #[serde(default)]
    pub difficulty: LightsDifficulty,
    #[serde(default)]
    pub par: u8,
    #[serde(default)]
    pub guide: bool,
    #[serde(skip)]
    history: Vec<([bool; CELLS], u16, LightsOutStatus)>,
}

impl Default for LightsOut {
    fn default() -> Self {
        Self::new(0x1D1E_1075)
    }
}

impl LightsOut {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, LightsDifficulty::Classic)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: LightsDifficulty) -> Self {
        let mut game = Self {
            cells: [false; CELLS],
            moves: 0,
            seed,
            status: LightsOutStatus::Playing,
            difficulty,
            par: 0,
            guide: false,
            history: Vec::new(),
        };
        let mut source = seed;
        for _ in 0..difficulty.scramble_presses() {
            source = next_seed(source);
            game.toggle_pattern((source as usize) % CELLS);
        }
        if !game.cells.iter().any(|cell| *cell) {
            game.toggle_pattern(CELLS / 2);
        }
        game.par = game.minimum_solution().len() as u8;
        game
    }

    pub fn press(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status == LightsOutStatus::Won {
            return false;
        }
        self.history.push((self.cells, self.moves, self.status));
        self.toggle_pattern(index);
        self.moves = self.moves.saturating_add(1);
        if self.cells.iter().all(|cell| !cell) {
            self.status = LightsOutStatus::Won;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, moves, status)) = self.history.pop() {
            self.cells = cells;
            self.moves = moves;
            self.status = status;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }

    pub fn set_difficulty(&mut self, difficulty: LightsDifficulty, seed: u64) {
        *self = Self::new_with_difficulty(seed, difficulty);
    }

    pub fn toggle_guide(&mut self) {
        self.guide = !self.guide;
    }

    pub fn lit_count(&self) -> usize {
        self.cells.iter().filter(|cell| **cell).count()
    }

    pub fn displayed_par(&self) -> usize {
        if self.par == 0 {
            self.minimum_solution().len()
        } else {
            self.par as usize
        }
    }

    pub fn hint_move(&self) -> Option<usize> {
        if self.status == LightsOutStatus::Won {
            return None;
        }
        self.minimum_solution().into_iter().next()
    }

    pub fn optimal_contains(&self, index: usize) -> bool {
        self.guide && self.minimum_solution().contains(&index)
    }

    pub fn minimum_solution(&self) -> Vec<usize> {
        let mut best: Option<Vec<usize>> = None;
        for first_row in 0_u32..(1 << SIZE) {
            let mut cells = self.cells;
            let mut presses = Vec::new();
            for column in 0..SIZE {
                if first_row & (1 << column) != 0 {
                    toggle_cells(&mut cells, column);
                    presses.push(column);
                }
            }
            for row in 1..SIZE {
                for column in 0..SIZE {
                    if cells[(row - 1) * SIZE + column] {
                        let index = row * SIZE + column;
                        toggle_cells(&mut cells, index);
                        presses.push(index);
                    }
                }
            }
            if cells.iter().all(|cell| !cell)
                && best
                    .as_ref()
                    .is_none_or(|current| presses.len() < current.len())
            {
                best = Some(presses);
            }
        }
        best.unwrap_or_default()
    }

    fn toggle_pattern(&mut self, index: usize) {
        toggle_cells(&mut self.cells, index);
    }
}

fn toggle_cells(cells: &mut [bool; CELLS], index: usize) {
    let row = index / SIZE;
    let column = index % SIZE;
    for (row_delta, column_delta) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        let next_row = row as isize + row_delta;
        let next_column = column as isize + column_delta;
        if (0..SIZE as isize).contains(&next_row) && (0..SIZE as isize).contains(&next_column) {
            let next = next_row as usize * SIZE + next_column as usize;
            cells[next] = !cells[next];
        }
    }
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
#[path = "../tests/legacy/lights_out/tests.rs"]
mod tests;
