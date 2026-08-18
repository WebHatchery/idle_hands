//! Deterministic Flood It rules with a bounded move target.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 8;
pub const COLORS: u8 = 6;
const CELLS: usize = SIDE * SIDE;
const MOVE_LIMIT: u16 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloodDifficulty {
    Standard,
    Hard,
    Expert,
}

impl Default for FloodDifficulty {
    fn default() -> Self {
        Self::Standard
    }
}

impl FloodDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    pub fn settings(self) -> (usize, u8, u16) {
        match self {
            Self::Standard => (8, 6, 24),
            Self::Hard => (10, 7, 32),
            Self::Expert => (12, 8, 40),
        }
    }
}

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
    #[serde(default)]
    pub difficulty: FloodDifficulty,
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
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, FloodDifficulty::Standard)
    }

    pub fn new_with_difficulty(mut seed: u64, difficulty: FloodDifficulty) -> Self {
        let (side, colors, _) = difficulty.settings();
        let mut cells = Vec::with_capacity(side * side);
        for _ in 0..side * side {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            cells.push((seed % colors as u64) as u8);
        }
        Self {
            active_color: cells[0],
            cells,
            moves: 0,
            seed,
            difficulty,
            phase: FloodPhase::Playing,
            undo: None,
        }
    }

    pub fn choose(&mut self, color: u8) -> bool {
        if self.phase != FloodPhase::Playing
            || color >= self.color_count()
            || color == self.active_color
        {
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        let old_color = self.active_color;
        self.active_color = color;
        let mut queue = VecDeque::from([0usize]);
        let mut visited = vec![false; self.cells.len()];
        while let Some(index) = queue.pop_front() {
            if visited[index] || self.cells[index] != old_color {
                continue;
            }
            visited[index] = true;
            self.cells[index] = color;
            for neighbor in neighbors(index, self.side()) {
                if !visited[neighbor] && self.cells[neighbor] == old_color {
                    queue.push_back(neighbor);
                }
            }
        }
        self.moves = self.moves.saturating_add(1);
        if self.cells.iter().all(|&cell| cell == color) {
            self.phase = FloodPhase::Won;
        } else if self.moves >= self.move_limit() {
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

    pub fn hint_color(&self) -> Option<u8> {
        if self.phase != FloodPhase::Playing {
            return None;
        }
        let before = origin_region_size(&self.cells, self.side());
        let mut best: Option<(i32, u8)> = None;
        for color in 0..self.color_count() {
            if color == self.active_color {
                continue;
            }
            let mut trial = self.clone_without_undo();
            trial.choose(color);
            let gain = origin_region_size(&trial.cells, self.side()) - before;
            let score = (trial.won() as i32) * 10_000 + gain as i32;
            if best.is_none_or(|(best_score, _)| score > best_score) {
                best = Some((score, color));
            }
        }
        best.map(|(_, color)| color)
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    pub fn side(&self) -> usize {
        self.difficulty.settings().0
    }

    pub fn color_count(&self) -> u8 {
        self.difficulty.settings().1
    }

    pub fn move_limit(&self) -> u16 {
        self.difficulty.settings().2
    }
}

fn origin_region_size(cells: &[u8], side: usize) -> usize {
    let color = cells[0];
    let mut queue = VecDeque::from([0usize]);
    let mut visited = vec![false; cells.len()];
    let mut size = 0;
    while let Some(index) = queue.pop_front() {
        if visited[index] || cells[index] != color {
            continue;
        }
        visited[index] = true;
        size += 1;
        for neighbor in neighbors(index, side) {
            if !visited[neighbor] && cells[neighbor] == color {
                queue.push_back(neighbor);
            }
        }
    }
    size
}

fn neighbors(index: usize, side: usize) -> impl Iterator<Item = usize> {
    let row = index / side;
    let col = index % side;
    [
        row.checked_sub(1).map(|next| next * side + col),
        (row + 1 < side).then_some((row + 1) * side + col),
        col.checked_sub(1).map(|next| row * side + next),
        (col + 1 < side).then_some(row * side + col + 1),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod tests;
