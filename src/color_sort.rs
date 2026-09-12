//! Solvable touch-first Color Sort tube puzzle.
use crate::undo::UndoStack;

use serde::{Deserialize, Serialize};

use crate::data::ColorSortConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ColorSortDifficulty {
    #[default]
    Standard,
    Hard,
    Expert,
}

impl ColorSortDifficulty {
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
pub enum ColorSortPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PourPreview {
    pub count: usize,
    pub stacks_match: bool,
    pub completes_tube: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSort {
    pub tubes: Vec<Vec<u8>>,
    pub selected: Option<usize>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: ColorSortDifficulty,
    #[serde(default = "default_capacity")]
    capacity: usize,
    #[serde(default = "default_colors")]
    colors: u8,
    #[serde(default = "default_tube_count")]
    tube_count: usize,
    #[serde(default = "default_scramble_steps")]
    scramble_steps: usize,
    #[serde(default)]
    pub last_poured: u8,
    #[serde(default)]
    pub combo: u16,
    #[serde(default)]
    pub best_combo: u16,
    #[serde(default)]
    pub points: u32,
    pub phase: ColorSortPhase,
    #[serde(skip)]
    history: UndoStack<Self>,
}

impl Default for ColorSort {
    fn default() -> Self {
        Self::new(0x0043_4F4C_4F52)
    }
}

impl ColorSort {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, ColorSortDifficulty::Standard)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: ColorSortDifficulty) -> Self {
        Self::new_with_config(seed, difficulty, &ColorSortConfig::default())
    }

    pub fn new_with_config(
        seed: u64,
        difficulty: ColorSortDifficulty,
        config: &ColorSortConfig,
    ) -> Self {
        // GameData validation guarantees one row for every difficulty variant.
        let settings = config
            .difficulties
            .get(difficulty.index())
            .expect("validated Color Sort difficulty configuration");
        Self::new_with_settings(
            seed,
            difficulty,
            config.capacity,
            settings.colors,
            settings.tubes,
            settings.scramble_steps,
        )
    }

    fn new_with_settings(
        mut seed: u64,
        difficulty: ColorSortDifficulty,
        capacity: usize,
        colors: u8,
        tube_count: usize,
        scramble_steps: usize,
    ) -> Self {
        let mut tubes = vec![Vec::new(); tube_count];
        for color in 0..colors {
            tubes[color as usize] = vec![color; capacity];
        }
        for attempt in 0..8 {
            let mut candidate = tubes.clone();
            let mut candidates_seed = seed;
            for _ in 0..scramble_steps {
                let moves = reverse_moves(&candidate, capacity);
                if moves.is_empty() {
                    break;
                }
                candidates_seed = next_seed(candidates_seed);
                let (source, destination, count) = moves[candidates_seed as usize % moves.len()];
                for _ in 0..count {
                    // The reverse move count was produced from this tube's length.
                    let value = candidate[source].pop().expect("reverse move counted");
                    candidate[destination].push(value);
                }
            }
            if !is_solved(&candidate, capacity) && candidate.iter().any(|tube| is_mixed(tube)) {
                tubes = candidate;
                seed = candidates_seed;
                break;
            }
            seed = next_seed(seed ^ attempt as u64);
        }
        Self {
            tubes,
            selected: None,
            moves: 0,
            seed,
            difficulty,
            capacity,
            colors,
            tube_count,
            scramble_steps,
            last_poured: 0,
            combo: 0,
            best_combo: 0,
            points: 0,
            phase: ColorSortPhase::Playing,
            history: UndoStack::default(),
        }
    }

    pub fn tap_tube(&mut self, tube: usize) -> bool {
        if self.phase != ColorSortPhase::Playing || tube >= self.tubes.len() {
            return false;
        }
        let Some(source) = self.selected else {
            if self.tubes[tube].is_empty() || self.is_complete_tube(tube) {
                return false;
            }
            self.selected = Some(tube);
            return true;
        };
        if source == tube {
            self.selected = None;
            return true;
        }
        let Some(preview) = self.pour_preview(source, tube) else {
            return false;
        };
        let mut previous = self.clone_without_undo();
        previous.selected = None;
        for _ in 0..preview.count {
            // The preview count is computed from the source tube's current run.
            let value = self.tubes[source].pop().expect("run counted");
            self.tubes[tube].push(value);
        }
        self.moves = self.moves.saturating_add(1);
        self.last_poured = preview.count.min(u8::MAX as usize) as u8;
        self.combo = if preview.stacks_match {
            self.combo.saturating_add(1).max(1)
        } else {
            1
        };
        self.best_combo = self.best_combo.max(self.combo);
        self.points = self
            .points
            .saturating_add((preview.count as u32).saturating_mul(u32::from(self.combo)))
            .saturating_add(u32::from(preview.completes_tube) * 10);
        self.selected = None;
        if self.is_solved() {
            self.phase = ColorSortPhase::Won;
        }
        self.history.push(previous);
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.history.pop() else {
            return false;
        };
        let history = std::mem::take(&mut self.history);
        *self = previous;
        self.history = history;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_settings(
            seed,
            self.difficulty,
            self.capacity,
            self.colors,
            self.tube_count,
            self.scramble_steps,
        );
    }

    pub fn won(&self) -> bool {
        self.phase == ColorSortPhase::Won
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn completed_tubes(&self) -> usize {
        (0..self.tubes.len())
            .filter(|&tube| self.is_complete_tube(tube))
            .count()
    }

    pub fn is_complete_tube(&self, tube: usize) -> bool {
        self.tubes
            .get(tube)
            .is_some_and(|contents| contents.len() == self.capacity && !is_mixed(contents))
    }

    pub fn pour_preview(&self, source: usize, destination: usize) -> Option<PourPreview> {
        if source >= self.tubes.len()
            || destination >= self.tubes.len()
            || source == destination
            || self.tubes[source].is_empty()
            || self.is_complete_tube(source)
            || self.tubes[destination].len() == self.capacity
        {
            return None;
        }
        let color = *self.tubes[source].last()?;
        if self.tubes[destination]
            .last()
            .is_some_and(|&top| top != color)
        {
            return None;
        }
        let run = self.tubes[source]
            .iter()
            .rev()
            .take_while(|&&value| value == color)
            .count();
        let count = run.min(self.capacity - self.tubes[destination].len());
        let stacks_match = !self.tubes[destination].is_empty();
        Some(PourPreview {
            count,
            stacks_match,
            completes_tube: self.tubes[destination].len() + count == self.capacity,
        })
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.phase != ColorSortPhase::Playing {
            return None;
        }
        let mut best: Option<(i32, usize, usize)> = None;
        for source in 0..self.tubes.len() {
            if self.tubes[source].is_empty() || self.is_complete_tube(source) {
                continue;
            }
            for destination in 0..self.tubes.len() {
                if source == destination {
                    continue;
                }
                let mut trial = self.clone_without_undo();
                if !trial.tap_tube(source) || !trial.tap_tube(destination) {
                    continue;
                }
                let score = trial.progress_score();
                if best.is_none_or(|(best_score, _, _)| score > best_score) {
                    best = Some((score, source, destination));
                }
            }
        }
        best.map(|(_, source, destination)| (source, destination))
    }

    fn is_solved(&self) -> bool {
        is_solved(&self.tubes, self.capacity)
    }

    fn progress_score(&self) -> i32 {
        let completed = self
            .tubes
            .iter()
            .filter(|tube| {
                tube.len() == self.capacity && tube.windows(2).all(|pair| pair[0] == pair[1])
            })
            .count() as i32;
        let uniform = self
            .tubes
            .iter()
            .filter(|tube| !tube.is_empty() && tube.windows(2).all(|pair| pair[0] == pair[1]))
            .count() as i32;
        let empty = self.tubes.iter().filter(|tube| tube.is_empty()).count() as i32;
        completed * 10_000 + uniform * 100 + empty
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

fn reverse_moves(tubes: &[Vec<u8>], capacity: usize) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::new();
    for source in 0..tubes.len() {
        let Some(&color) = tubes[source].last() else {
            continue;
        };
        let run = tubes[source]
            .iter()
            .rev()
            .take_while(|&&value| value == color)
            .count();
        for (destination, tube) in tubes.iter().enumerate() {
            if source == destination
                || tube.len() == capacity
                || tube.last().is_some_and(|&top| top == color)
            {
                continue;
            }
            let room = capacity - tube.len();
            for count in 1..=run.min(room) {
                moves.push((source, destination, count));
            }
        }
    }
    moves
}

fn is_solved(tubes: &[Vec<u8>], capacity: usize) -> bool {
    tubes
        .iter()
        .all(|tube| tube.is_empty() || (tube.len() == capacity && !is_mixed(tube)))
}

fn is_mixed(tube: &[u8]) -> bool {
    tube.windows(2).any(|pair| pair[0] != pair[1])
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

fn default_capacity() -> usize {
    ColorSortConfig::default().capacity
}

fn default_colors() -> u8 {
    ColorSortConfig::default().difficulties[0].colors
}

fn default_tube_count() -> usize {
    ColorSortConfig::default().difficulties[0].tubes
}

fn default_scramble_steps() -> usize {
    ColorSortConfig::default().difficulties[0].scramble_steps
}

#[cfg(test)]
mod tests;
