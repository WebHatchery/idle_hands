//! Deterministic potion-themed 2048 rules.

use crate::domain::Direction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PotionDifficulty {
    #[default]
    Standard,
    Hard,
    Expert,
}

impl PotionDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    fn settings(self) -> (usize, u16) {
        match self {
            Self::Standard => (4, 4096),
            Self::Hard => (5, 8192),
            Self::Expert => (6, 16384),
        }
    }

    pub const fn catalyst_chain(self) -> u8 {
        match self {
            Self::Standard => 3,
            Self::Hard => 4,
            Self::Expert => 5,
        }
    }
}

#[derive(Debug, Clone)]
struct Snapshot {
    cells: Vec<u16>,
    score: u32,
    seed: u64,
    combo: u8,
    best_combo: u8,
    catalysts_brewed: u16,
    last_merges: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion2048 {
    pub cells: Vec<u16>,
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: PotionDifficulty,
    #[serde(default)]
    pub combo: u8,
    #[serde(default)]
    pub best_combo: u8,
    #[serde(default)]
    pub catalysts_brewed: u16,
    #[serde(default)]
    pub last_merges: u8,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for Potion2048 {
    fn default() -> Self {
        Self::new(0xB071_2048)
    }
}

impl Potion2048 {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, PotionDifficulty::Standard)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: PotionDifficulty) -> Self {
        let side = difficulty.settings().0;
        let mut game = Self {
            cells: vec![0; side * side],
            score: 0,
            best: 0,
            seed,
            difficulty,
            combo: 0,
            best_combo: 0,
            catalysts_brewed: 0,
            last_merges: 0,
            undo: None,
        };
        game.spawn(false);
        game.spawn(false);
        game
    }
    pub fn move_in(&mut self, direction: Direction) -> bool {
        let before = self.cells.clone();
        let before_score = self.score;
        let before_seed = self.seed;
        let before_combo = self.combo;
        let before_best_combo = self.best_combo;
        let before_catalysts = self.catalysts_brewed;
        let before_last_merges = self.last_merges;
        let mut changed = false;
        let mut raw_gain = 0_u32;
        let mut merge_count = 0_u8;
        let side = self.side();
        for line in 0..side {
            let indices = line_indices(side, line, direction);
            let values: Vec<u16> = indices
                .iter()
                .map(|&i| self.cells[i])
                .filter(|&v| v != 0)
                .collect();
            let mut merged = Vec::with_capacity(4);
            let mut index = 0;
            while index < values.len() {
                let result = (index + 1 < values.len())
                    .then(|| reaction(values[index], values[index + 1]))
                    .flatten();
                if let Some(result) = result {
                    merged.push(result);
                    raw_gain = raw_gain.saturating_add(u32::from(result));
                    merge_count = merge_count.saturating_add(1);
                    index += 2;
                } else {
                    merged.push(values[index]);
                    index += 1;
                }
            }
            for (slot, &cell) in indices.iter().enumerate() {
                let value = merged.get(slot).copied().unwrap_or(0);
                if self.cells[cell] != value {
                    changed = true;
                }
                self.cells[cell] = value;
            }
        }
        if changed {
            self.undo = Some(Snapshot {
                cells: before,
                score: before_score,
                seed: before_seed,
                combo: before_combo,
                best_combo: before_best_combo,
                catalysts_brewed: before_catalysts,
                last_merges: before_last_merges,
            });
            if merge_count > 0 {
                self.combo = self.combo.saturating_add(1);
                self.best_combo = self.best_combo.max(self.combo);
                self.score = self
                    .score
                    .saturating_add(raw_gain.saturating_mul(u32::from(self.combo)));
            } else {
                self.combo = 0;
            }
            self.last_merges = merge_count;
            let catalyst =
                self.combo > 0 && self.combo.is_multiple_of(self.difficulty.catalyst_chain());
            if catalyst {
                self.catalysts_brewed = self.catalysts_brewed.saturating_add(1);
            }
            self.spawn(catalyst);
            self.best = self.best.max(self.score);
        }
        changed
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.won() {
            return None;
        }
        let mut best = None;
        let mut best_gain = 0;
        let mut best_empty = 0;
        for direction in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ] {
            let mut candidate = self.clone();
            let score = candidate.score;
            if !candidate.move_in(direction) {
                continue;
            }
            let gain = candidate.score - score;
            let empty = candidate.cells.iter().filter(|&&value| value == 0).count();
            if best.is_none() || gain > best_gain || (gain == best_gain && empty > best_empty) {
                best = Some(direction);
                best_gain = gain;
                best_empty = empty;
            }
        }
        best
    }
    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.undo.take() {
            self.cells = snapshot.cells;
            self.score = snapshot.score;
            self.seed = snapshot.seed;
            self.combo = snapshot.combo;
            self.best_combo = snapshot.best_combo;
            self.catalysts_brewed = snapshot.catalysts_brewed;
            self.last_merges = snapshot.last_merges;
            true
        } else {
            false
        }
    }
    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }
    pub fn won(&self) -> bool {
        self.cells.iter().any(|&value| value >= self.target())
    }
    fn spawn(&mut self, catalyst: bool) {
        let empty: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some(i))
            .collect();
        if empty.is_empty() {
            return;
        }
        self.seed = next_seed(self.seed);
        let index = empty[(self.seed as usize) % empty.len()];
        self.seed = next_seed(self.seed);
        self.cells[index] = if catalyst {
            1
        } else if self.seed & 7 == 0 {
            4
        } else {
            2
        };
    }

    pub fn side(&self) -> usize {
        self.difficulty.settings().0
    }

    pub fn target(&self) -> u16 {
        self.difficulty.settings().1
    }
}

fn reaction(first: u16, second: u16) -> Option<u16> {
    if first == second {
        Some(first.saturating_mul(2))
    } else if first == 1 || second == 1 {
        Some(first.max(second).saturating_mul(2))
    } else {
        None
    }
}

fn line_indices(side: usize, line: usize, direction: Direction) -> Vec<usize> {
    let indices: Vec<usize> = match direction {
        Direction::Left => (0..side).map(|offset| line * side + offset).collect(),
        Direction::Right => (0..side).rev().map(|offset| line * side + offset).collect(),
        Direction::Up => (0..side).map(|offset| line + offset * side).collect(),
        Direction::Down => (0..side).rev().map(|offset| line + offset * side).collect(),
    };
    indices
}
fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
