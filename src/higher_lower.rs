//! Deterministic touch-first Higher or Lower card guessing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Guess {
    Higher,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HigherLowerStatus {
    Playing,
    Won,
    Lost,
}

type Snapshot = (u8, u8, u16, u16, HigherLowerStatus, u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HigherLower {
    pub current: u8,
    pub next: u8,
    pub score: u16,
    pub moves: u16,
    pub status: HigherLowerStatus,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for HigherLower {
    fn default() -> Self {
        Self::new(0xC4AD_0001)
    }
}

impl HigherLower {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            current: ((seed % 13) + 1) as u8,
            next: 1,
            score: 0,
            moves: 0,
            status: HigherLowerStatus::Playing,
            seed,
            undo: None,
        };
        game.next = game.draw();
        game
    }

    pub fn guess(&mut self, guess: Guess) -> bool {
        if self.status != HigherLowerStatus::Playing {
            return false;
        }
        self.undo = Some((
            self.current,
            self.next,
            self.score,
            self.moves,
            self.status,
            self.seed,
        ));
        self.moves = self.moves.saturating_add(1);
        let correct = match guess {
            Guess::Higher => self.next >= self.current,
            Guess::Lower => self.next <= self.current,
        };
        self.current = self.next;
        if correct {
            self.score = self.score.saturating_add(1);
            if self.score >= 10 {
                self.status = HigherLowerStatus::Won;
            } else {
                self.next = self.draw();
            }
        } else {
            self.status = HigherLowerStatus::Lost;
        }
        true
    }

    pub fn hint_guess(&self) -> Option<Guess> {
        if self.status != HigherLowerStatus::Playing {
            return None;
        }
        let higher_or_equal = 14 - self.current;
        let lower_or_equal = self.current;
        Some(if higher_or_equal >= lower_or_equal {
            Guess::Higher
        } else {
            Guess::Lower
        })
    }

    pub fn undo(&mut self) -> bool {
        if let Some((current, next, score, moves, status, seed)) = self.undo.take() {
            self.current = current;
            self.next = next;
            self.score = score;
            self.moves = moves;
            self.status = status;
            self.seed = seed;
            true
        } else {
            false
        }
    }
    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }
    fn draw(&mut self) -> u8 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.seed % 13) + 1) as u8
    }
}

#[cfg(test)]
mod tests;
