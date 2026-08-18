//! Solvable touch-first Color Sort tube puzzle.

use serde::{Deserialize, Serialize};

pub const TUBES: usize = 6;
pub const COLORS: u8 = 4;
pub const CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSortDifficulty {
    Standard,
    Hard,
    Expert,
}

impl Default for ColorSortDifficulty {
    fn default() -> Self {
        Self::Standard
    }
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

    fn settings(self) -> (u8, usize, usize) {
        match self {
            Self::Standard => (4, 6, 52),
            Self::Hard => (5, 7, 78),
            Self::Expert => (6, 8, 108),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSortPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSort {
    pub tubes: Vec<Vec<u8>>,
    pub selected: Option<usize>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: ColorSortDifficulty,
    pub phase: ColorSortPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
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

    pub fn new_with_difficulty(mut seed: u64, difficulty: ColorSortDifficulty) -> Self {
        let (colors, tube_count, scramble_steps) = difficulty.settings();
        let mut tubes = vec![Vec::new(); tube_count];
        for color in 0..colors {
            tubes[color as usize] = vec![color; CAPACITY];
        }
        for attempt in 0..8 {
            let mut candidate = tubes.clone();
            let mut candidates_seed = seed;
            for _ in 0..scramble_steps {
                let moves = reverse_moves(&candidate, CAPACITY);
                if moves.is_empty() {
                    break;
                }
                candidates_seed = next_seed(candidates_seed);
                let (source, destination, count) = moves[candidates_seed as usize % moves.len()];
                for _ in 0..count {
                    let value = candidate[source].pop().expect("reverse move counted");
                    candidate[destination].push(value);
                }
            }
            if !is_solved(&candidate, CAPACITY) && candidate.iter().any(|tube| is_mixed(tube)) {
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
            phase: ColorSortPhase::Playing,
            undo: None,
        }
    }

    pub fn tap_tube(&mut self, tube: usize) -> bool {
        if self.phase != ColorSortPhase::Playing || tube >= self.tubes.len() {
            return false;
        }
        let Some(source) = self.selected else {
            if self.tubes[tube].is_empty() {
                return false;
            }
            self.selected = Some(tube);
            return true;
        };
        if source == tube {
            self.selected = None;
            return true;
        }
        let Some(&color) = self.tubes[source].last() else {
            self.selected = None;
            return false;
        };
        if self.tubes[tube].len() == CAPACITY
            || self.tubes[tube].last().is_some_and(|&top| top != color)
        {
            return false;
        }
        let run = self.tubes[source]
            .iter()
            .rev()
            .take_while(|&&value| value == color)
            .count();
        let count = run.min(CAPACITY - self.tubes[tube].len());
        let previous = self.clone_without_undo();
        for _ in 0..count {
            let value = self.tubes[source].pop().expect("run counted");
            self.tubes[tube].push(value);
        }
        self.undo = Some(Box::new(previous));
        self.moves = self.moves.saturating_add(1);
        self.selected = None;
        if self.is_solved() {
            self.phase = ColorSortPhase::Won;
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
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }

    pub fn won(&self) -> bool {
        self.phase == ColorSortPhase::Won
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.phase != ColorSortPhase::Playing {
            return None;
        }
        let mut best: Option<(i32, usize, usize)> = None;
        for source in 0..self.tubes.len() {
            if self.tubes[source].is_empty() {
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
        is_solved(&self.tubes, CAPACITY)
    }

    fn progress_score(&self) -> i32 {
        let completed = self
            .tubes
            .iter()
            .filter(|tube| tube.len() == CAPACITY && tube.windows(2).all(|pair| pair[0] == pair[1]))
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
        copy.undo = None;
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
        for destination in 0..tubes.len() {
            if source == destination
                || tubes[destination].len() == capacity
                || tubes[destination].last().is_some_and(|&top| top == color)
            {
                continue;
            }
            let room = capacity - tubes[destination].len();
            for count in 1..=run.min(room) {
                moves.push((source, destination, count));
            }
        }
    }
    moves
}

fn is_solved(tubes: &[Vec<u8>], capacity: usize) -> bool {
    tubes.iter().all(|tube| {
        tube.is_empty() || (tube.len() == capacity && !is_mixed(tube))
    })
}

fn is_mixed(tube: &[u8]) -> bool {
    tube.windows(2).any(|pair| pair[0] != pair[1])
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
