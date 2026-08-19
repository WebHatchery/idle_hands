//! Deterministic Flood It rules with a bounded move target.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::FloodItConfig;

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

    fn index(self) -> usize {
        match self {
            Self::Standard => 0,
            Self::Hard => 1,
            Self::Expert => 2,
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
    #[serde(default = "default_side")]
    side: usize,
    #[serde(default = "default_colors")]
    colors: u8,
    #[serde(default = "default_move_limit")]
    move_limit: u16,
    #[serde(default)]
    pub last_gain: u16,
    #[serde(default)]
    pub combo: u16,
    #[serde(default)]
    pub best_combo: u16,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub momentum: u8,
    #[serde(default)]
    pub surges: u8,
    pub phase: FloodPhase,
    #[serde(skip)]
    history: Vec<Box<Self>>,
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

    pub fn new_with_difficulty(seed: u64, difficulty: FloodDifficulty) -> Self {
        Self::new_with_config(seed, difficulty, &FloodItConfig::default())
    }

    pub fn new_with_config(seed: u64, difficulty: FloodDifficulty, config: &FloodItConfig) -> Self {
        let settings = config
            .difficulties
            .get(difficulty.index())
            .expect("validated Flood It difficulty configuration");
        Self::new_with_settings(
            seed,
            difficulty,
            settings.side,
            settings.colors,
            settings.move_limit,
        )
    }

    fn new_with_settings(
        mut seed: u64,
        difficulty: FloodDifficulty,
        side: usize,
        colors: u8,
        move_limit: u16,
    ) -> Self {
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
            side,
            colors,
            move_limit,
            last_gain: 0,
            combo: 0,
            best_combo: 0,
            points: 0,
            momentum: 0,
            surges: 0,
            phase: FloodPhase::Playing,
            history: Vec::new(),
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
        let before = self.region_size();
        self.apply_color(color);
        let gain = self.region_size().saturating_sub(before);
        self.record_growth(gain, true);
        self.moves = self.moves.saturating_add(1);
        self.finish_after_move();
        self.history.push(Box::new(previous));
        true
    }

    pub fn use_surge(&mut self) -> bool {
        if self.phase != FloodPhase::Playing || self.surges == 0 {
            return false;
        }
        let Some(color) = self.hint_color() else {
            return false;
        };
        let previous = self.clone_without_undo();
        let before = self.region_size();
        self.apply_color(color);
        let gain = self.region_size().saturating_sub(before);
        self.surges -= 1;
        self.record_growth(gain, false);
        if self.cells.iter().all(|&cell| cell == color) {
            self.phase = FloodPhase::Won;
        }
        self.history.push(Box::new(previous));
        true
    }

    fn apply_color(&mut self, color: u8) {
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
    }

    fn finish_after_move(&mut self) {
        if self.cells.iter().all(|&cell| cell == self.active_color) {
            self.phase = FloodPhase::Won;
        } else if self.moves >= self.move_limit() {
            self.phase = FloodPhase::Lost;
        }
    }

    fn record_growth(&mut self, gain: usize, earn_surge: bool) {
        self.last_gain = gain.min(u16::MAX as usize) as u16;
        if gain > 0 {
            self.combo = self.combo.saturating_add(1);
            self.best_combo = self.best_combo.max(self.combo);
        } else {
            self.combo = 0;
        }
        self.points = self
            .points
            .saturating_add((gain as u32).saturating_mul(u32::from(self.combo.max(1))));
        if !earn_surge {
            return;
        }
        if gain >= self.side / 2 {
            self.momentum = self.momentum.saturating_add(1);
            if self.momentum >= 3 {
                self.surges = self.surges.saturating_add(1).min(2);
                self.momentum = 0;
            }
        } else {
            self.momentum = 0;
        }
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
        *self = Self::new_with_settings(
            seed,
            self.difficulty,
            self.side,
            self.colors,
            self.move_limit,
        );
    }

    pub fn won(&self) -> bool {
        self.phase == FloodPhase::Won
    }

    pub fn hint_color(&self) -> Option<u8> {
        if self.phase != FloodPhase::Playing {
            return None;
        }
        let before = self.region_size();
        let mut best: Option<(i32, u8)> = None;
        for color in 0..self.color_count() {
            if color == self.active_color {
                continue;
            }
            let gain = self.preview_gain(color);
            let score = i32::from(before + gain == self.cells.len()) * 10_000 + gain as i32;
            if best.is_none_or(|(best_score, _)| score > best_score) {
                best = Some((score, color));
            }
        }
        best.map(|(_, color)| color)
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }

    pub fn region_size(&self) -> usize {
        origin_region_size(&self.cells, self.side())
    }

    pub fn region_mask(&self) -> Vec<bool> {
        origin_region_mask(&self.cells, self.side())
    }

    pub fn preview_gain(&self, color: u8) -> usize {
        if color >= self.color_count() || color == self.active_color {
            return 0;
        }
        let before = self.region_size();
        let mut trial = self.clone_without_undo();
        trial.apply_color(color);
        trial.region_size().saturating_sub(before)
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn color_count(&self) -> u8 {
        self.colors
    }

    pub fn move_limit(&self) -> u16 {
        self.move_limit
    }
}

fn origin_region_size(cells: &[u8], side: usize) -> usize {
    origin_region_mask(cells, side)
        .into_iter()
        .filter(|included| *included)
        .count()
}

fn origin_region_mask(cells: &[u8], side: usize) -> Vec<bool> {
    let color = cells[0];
    let mut queue = VecDeque::from([0usize]);
    let mut visited = vec![false; cells.len()];
    while let Some(index) = queue.pop_front() {
        if visited[index] || cells[index] != color {
            continue;
        }
        visited[index] = true;
        for neighbor in neighbors(index, side) {
            if !visited[neighbor] && cells[neighbor] == color {
                queue.push_back(neighbor);
            }
        }
    }
    visited
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

fn default_side() -> usize {
    FloodItConfig::default().difficulties[0].side
}

fn default_colors() -> u8 {
    FloodItConfig::default().difficulties[0].colors
}

fn default_move_limit() -> u16 {
    FloodItConfig::default().difficulties[0].move_limit
}

#[cfg(test)]
mod tests;
