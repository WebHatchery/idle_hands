//! Deterministic Flood It rules with a bounded move target.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 8;
pub const COLORS: u8 = 6;
const CELLS: usize = SIDE * SIDE;
const MOVE_LIMIT: u16 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloodPhase {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodIt {
    pub cells: Vec<u8>,
    pub active_color: u8,
    pub moves: u16,
    pub seed: u64,
    pub phase: FloodPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for FloodIt {
    fn default() -> Self {
        Self::new(0x0046_4C4F_4F44_4954)
    }
}

impl FloodIt {
    pub fn new(mut seed: u64) -> Self {
        let mut cells = Vec::with_capacity(CELLS);
        for _ in 0..CELLS {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            cells.push((seed % COLORS as u64) as u8);
        }
        Self {
            active_color: cells[0],
            cells,
            moves: 0,
            seed,
            phase: FloodPhase::Playing,
            undo: None,
        }
    }

    pub fn choose(&mut self, color: u8) -> bool {
        if self.phase != FloodPhase::Playing || color >= COLORS || color == self.active_color {
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        let old_color = self.active_color;
        self.active_color = color;
        let mut queue = VecDeque::from([0usize]);
        let mut visited = [false; CELLS];
        while let Some(index) = queue.pop_front() {
            if visited[index] || self.cells[index] != old_color {
                continue;
            }
            visited[index] = true;
            self.cells[index] = color;
            for neighbor in neighbors(index) {
                if !visited[neighbor] && self.cells[neighbor] == old_color {
                    queue.push_back(neighbor);
                }
            }
        }
        self.moves = self.moves.saturating_add(1);
        if self.cells.iter().all(|&cell| cell == color) {
            self.phase = FloodPhase::Won;
        } else if self.moves >= MOVE_LIMIT {
            self.phase = FloodPhase::Lost;
        }
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
        self.phase == FloodPhase::Won
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

fn neighbors(index: usize) -> impl Iterator<Item = usize> {
    let row = index / SIDE;
    let col = index % SIDE;
    [
        row.checked_sub(1).map(|next| next * SIDE + col),
        (row + 1 < SIDE).then_some((row + 1) * SIDE + col),
        col.checked_sub(1).map(|next| row * SIDE + next),
        (col + 1 < SIDE).then_some(row * SIDE + col + 1),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod tests;
