//! Deterministic touch-first Color Sort tube puzzle.

use serde::{Deserialize, Serialize};

pub const TUBES: usize = 6;
pub const COLORS: u8 = 4;
const CAPACITY: usize = 4;
const SCRAMBLE_STEPS: usize = 36;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSortPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSort {
    pub tubes: [Vec<u8>; TUBES],
    pub selected: Option<usize>,
    pub moves: u16,
    pub seed: u64,
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
    pub fn new(mut seed: u64) -> Self {
        let mut tubes = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        for color in 0..COLORS {
            tubes[color as usize] = vec![color; CAPACITY];
        }
        for _ in 0..SCRAMBLE_STEPS {
            seed = next_seed(seed);
            let destination = (seed as usize) % TUBES;
            seed = next_seed(seed);
            let source = (seed as usize) % TUBES;
            let top = tubes[destination].last().copied();
            let below_top = tubes[destination]
                .get(tubes[destination].len().saturating_sub(2))
                .copied();
            if source == destination
                || tubes[destination].is_empty()
                || tubes[source].len() == CAPACITY
                || (tubes[destination].len() > 1 && top != below_top)
            {
                continue;
            }
            let color = tubes[destination].pop().expect("destination checked");
            tubes[source].push(color);
        }
        Self {
            tubes,
            selected: None,
            moves: 0,
            seed,
            phase: ColorSortPhase::Playing,
            undo: None,
        }
    }

    pub fn tap_tube(&mut self, tube: usize) -> bool {
        if self.phase != ColorSortPhase::Playing || tube >= TUBES {
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
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == ColorSortPhase::Won
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.phase != ColorSortPhase::Playing {
            return None;
        }
        let mut best: Option<(i32, usize, usize)> = None;
        for source in 0..TUBES {
            if self.tubes[source].is_empty() {
                continue;
            }
            for destination in 0..TUBES {
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
        self.tubes.iter().all(|tube| {
            tube.is_empty()
                || (tube.len() == CAPACITY && tube.windows(2).all(|pair| pair[0] == pair[1]))
        })
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

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
