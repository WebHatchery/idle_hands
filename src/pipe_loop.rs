//! Deterministic touch-first Pipe Loop rotation puzzle.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 5;
const CELLS: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipePhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeLoop {
    pub pipes: Vec<u8>,
    pub solution: Vec<u8>,
    pub moves: u16,
    pub seed: u64,
    pub phase: PipePhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for PipeLoop {
    fn default() -> Self {
        Self::new(0x0050_4950_4553)
    }
}

impl PipeLoop {
    pub fn new(mut seed: u64) -> Self {
        let solution = solved_pipes();
        let mut pipes = solution.clone();
        for pipe in &mut pipes {
            seed = next_seed(seed);
            for _ in 0..(seed % 4) {
                *pipe = rotate_mask(*pipe);
            }
        }
        if pipes == solution {
            pipes[0] = rotate_mask(pipes[0]);
        }
        Self {
            pipes,
            solution,
            moves: 0,
            seed,
            phase: PipePhase::Playing,
            undo: None,
        }
    }

    pub fn rotate(&mut self, index: usize) -> bool {
        if self.phase != PipePhase::Playing || index >= CELLS {
            return false;
        }
        let previous = self.clone_without_undo();
        self.pipes[index] = rotate_mask(self.pipes[index]);
        self.moves = self.moves.saturating_add(1);
        self.undo = Some(Box::new(previous));
        if self.pipes == self.solution {
            self.phase = PipePhase::Won;
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
        self.phase == PipePhase::Won
    }

    pub fn hint_rotation(&self) -> Option<(usize, u8)> {
        if self.phase != PipePhase::Playing {
            return None;
        }
        self.pipes.iter().zip(&self.solution).enumerate().find_map(
            |(index, (&current, &solution))| {
                if current == solution {
                    return None;
                }
                let mut rotated = current;
                for count in 1..=3 {
                    rotated = rotate_mask(rotated);
                    if rotated == solution {
                        return Some((index, count));
                    }
                }
                None
            },
        )
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

fn solved_pipes() -> Vec<u8> {
    let mut pipes = vec![0; CELLS];
    for row in 0..SIDE {
        for col in 0..SIDE {
            let index = row * SIDE + col;
            if col > 0 {
                pipes[index] |= 8;
            }
            if col + 1 < SIDE {
                pipes[index] |= 2;
            }
            if row % 2 == 0 && col == SIDE - 1 && row + 1 < SIDE {
                pipes[index] |= 4;
            }
            if row % 2 == 1 && col == 0 && row + 1 < SIDE {
                pipes[index] |= 4;
            }
            if row > 0 && ((row - 1) % 2 == 0 && col == SIDE - 1 || (row - 1) % 2 == 1 && col == 0)
            {
                pipes[index] |= 1;
            }
        }
    }
    pipes
}

fn rotate_mask(mask: u8) -> u8 {
    ((mask << 1) & 0x0F) | ((mask >> 3) & 1)
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
