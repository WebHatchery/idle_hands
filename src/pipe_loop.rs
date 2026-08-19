//! Deterministic touch-first Pipe Loop rotation puzzle.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 5;
const CELLS: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipePhase {
    Playing,
    Won,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipePattern {
    #[default]
    Serpent,
    Trunk,
}

impl PipePattern {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Serpent => "SERPENT",
            Self::Trunk => "TRUNK",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeLoop {
    pub pipes: Vec<u8>,
    pub solution: Vec<u8>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub pattern: PipePattern,
    #[serde(default)]
    pub par: u16,
    pub phase: PipePhase,
    #[serde(skip)]
    history: Vec<Box<Self>>,
}

impl Default for PipeLoop {
    fn default() -> Self {
        Self::new(0x0050_4950_4553)
    }
}

impl PipeLoop {
    pub fn new(seed: u64) -> Self {
        Self::new_with_pattern(seed, PipePattern::Serpent)
    }

    pub fn new_with_pattern(mut seed: u64, pattern: PipePattern) -> Self {
        let original_seed = seed;
        let solution = solved_pipes(pattern);
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
        let par = pipes
            .iter()
            .zip(&solution)
            .map(|(&current, &target)| u16::from(rotation_distance(current, target)))
            .sum();
        Self {
            pipes,
            solution,
            moves: 0,
            seed: original_seed,
            pattern,
            par,
            phase: PipePhase::Playing,
            history: Vec::new(),
        }
    }

    pub fn rotate(&mut self, index: usize) -> bool {
        if self.phase != PipePhase::Playing
            || index >= CELLS
            || rotate_mask(self.pipes[index]) == self.pipes[index]
        {
            return false;
        }
        let previous = self.clone_without_history();
        self.pipes[index] = rotate_mask(self.pipes[index]);
        self.moves = self.moves.saturating_add(1);
        if self.is_complete_network() {
            self.phase = PipePhase::Won;
        }
        self.history.push(Box::new(previous));
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.history.pop() else {
            return false;
        };
        let history = std::mem::take(&mut self.history);
        *self = *previous;
        self.history = history;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_pattern(seed, self.pattern);
    }

    pub fn set_pattern(&mut self, pattern: PipePattern, seed: u64) {
        *self = Self::new_with_pattern(seed, pattern);
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

    pub fn powered_mask(&self) -> Vec<bool> {
        let mut powered = vec![false; CELLS];
        let mut frontier = vec![0usize];
        powered[0] = true;
        while let Some(index) = frontier.pop() {
            for (bit, neighbor, reciprocal) in neighbors(index) {
                if self.pipes[index] & bit != 0
                    && self.pipes[neighbor] & reciprocal != 0
                    && !powered[neighbor]
                {
                    powered[neighbor] = true;
                    frontier.push(neighbor);
                }
            }
        }
        powered
    }

    pub fn connected_count(&self) -> usize {
        self.powered_mask()
            .into_iter()
            .filter(|value| *value)
            .count()
    }

    pub fn leak_count(&self) -> usize {
        (0..CELLS)
            .map(|index| {
                [1, 2, 4, 8]
                    .into_iter()
                    .filter(|&bit| self.has_leak(index, bit))
                    .count()
            })
            .sum()
    }

    pub fn has_leak(&self, index: usize, bit: u8) -> bool {
        if index >= CELLS || self.pipes[index] & bit == 0 {
            return false;
        }
        let reciprocal = match bit {
            1 => 4,
            2 => 8,
            4 => 1,
            8 => 2,
            _ => return false,
        };
        neighbor_for(index, bit).is_none_or(|neighbor| self.pipes[neighbor] & reciprocal == 0)
    }

    pub fn is_complete_network(&self) -> bool {
        self.connected_count() == CELLS && self.leak_count() == 0
    }

    fn clone_without_history(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

fn solved_pipes(pattern: PipePattern) -> Vec<u8> {
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
            match pattern {
                PipePattern::Serpent => {
                    if row % 2 == 0 && col == SIDE - 1 && row + 1 < SIDE {
                        pipes[index] |= 4;
                    }
                    if row % 2 == 1 && col == 0 && row + 1 < SIDE {
                        pipes[index] |= 4;
                    }
                    if row > 0
                        && ((row - 1) % 2 == 0 && col == SIDE - 1 || (row - 1) % 2 == 1 && col == 0)
                    {
                        pipes[index] |= 1;
                    }
                }
                PipePattern::Trunk if col == SIDE / 2 => {
                    if row > 0 {
                        pipes[index] |= 1;
                    }
                    if row + 1 < SIDE {
                        pipes[index] |= 4;
                    }
                }
                PipePattern::Trunk => {}
            }
        }
    }
    pipes
}

fn neighbor_for(index: usize, bit: u8) -> Option<usize> {
    let row = index / SIDE;
    let col = index % SIDE;
    match bit {
        1 => row.checked_sub(1).map(|next| next * SIDE + col),
        2 if col + 1 < SIDE => Some(index + 1),
        4 if row + 1 < SIDE => Some(index + SIDE),
        8 => col.checked_sub(1).map(|_| index - 1),
        _ => None,
    }
}

fn neighbors(index: usize) -> Vec<(u8, usize, u8)> {
    [(1, 4), (2, 8), (4, 1), (8, 2)]
        .into_iter()
        .filter_map(|(bit, reciprocal)| {
            neighbor_for(index, bit).map(|neighbor| (bit, neighbor, reciprocal))
        })
        .collect()
}

fn rotation_distance(mut current: u8, target: u8) -> u8 {
    for count in 0..4 {
        if current == target {
            return count;
        }
        current = rotate_mask(current);
    }
    0
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
