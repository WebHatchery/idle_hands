//! Deterministic touch-first Mastermind rules.

use serde::{Deserialize, Serialize};

const PEGS: usize = 4;
const ROWS: usize = 10;
const EMPTY: u8 = 255;

type MastermindSnapshot = (
    [[u8; PEGS]; ROWS],
    [u8; ROWS],
    [u8; ROWS],
    [u8; PEGS],
    u8,
    MastermindStatus,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MastermindStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mastermind {
    pub secret: [u8; PEGS],
    pub guesses: [[u8; PEGS]; ROWS],
    pub exact: [u8; ROWS],
    pub partial: [u8; ROWS],
    pub current: [u8; PEGS],
    pub row: u8,
    pub seed: u64,
    pub status: MastermindStatus,
    #[serde(skip)]
    undo: Option<MastermindSnapshot>,
}

impl Default for Mastermind {
    fn default() -> Self {
        Self::new(0x1D1E_6A57)
    }
}

impl Mastermind {
    pub fn new(seed: u64) -> Self {
        let mut source = seed;
        let mut secret = [0; PEGS];
        for peg in &mut secret {
            source = next_seed(source);
            *peg = (source % 6) as u8;
        }
        Self {
            secret,
            guesses: [[EMPTY; PEGS]; ROWS],
            exact: [0; ROWS],
            partial: [0; ROWS],
            current: [EMPTY; PEGS],
            row: 0,
            seed,
            status: MastermindStatus::Playing,
            undo: None,
        }
    }

    pub fn pick(&mut self, color: u8) -> bool {
        if color >= 6 || self.status != MastermindStatus::Playing {
            return false;
        }
        if let Some(slot) = self.current.iter().position(|peg| *peg == EMPTY) {
            self.undo = Some(self.snapshot());
            self.current[slot] = color;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) -> bool {
        if self.status != MastermindStatus::Playing || self.current == [EMPTY; PEGS] {
            return false;
        }
        self.undo = Some(self.snapshot());
        self.current = [EMPTY; PEGS];
        true
    }

    pub fn submit(&mut self) -> bool {
        if self.status != MastermindStatus::Playing || self.current.contains(&EMPTY) {
            return false;
        }
        self.undo = Some(self.snapshot());
        let row = self.row as usize;
        self.guesses[row] = self.current;
        let (exact, partial) = score_guess(&self.secret, &self.current);
        self.exact[row] = exact;
        self.partial[row] = partial;
        self.row += 1;
        self.current = [EMPTY; PEGS];
        if exact == PEGS as u8 {
            self.status = MastermindStatus::Won;
        } else if self.row as usize == ROWS {
            self.status = MastermindStatus::Lost;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.undo.take() {
            self.restore(snapshot);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn snapshot(&self) -> MastermindSnapshot {
        (
            self.guesses,
            self.exact,
            self.partial,
            self.current,
            self.row,
            self.status,
        )
    }

    fn restore(&mut self, snapshot: MastermindSnapshot) {
        (
            self.guesses,
            self.exact,
            self.partial,
            self.current,
            self.row,
            self.status,
        ) = snapshot;
    }
}

pub fn score_guess(secret: &[u8; PEGS], guess: &[u8; PEGS]) -> (u8, u8) {
    let exact = secret.iter().zip(guess).filter(|(a, b)| a == b).count() as u8;
    let mut secret_counts = [0u8; 6];
    let mut guess_counts = [0u8; 6];
    for (secret, guess) in secret.iter().zip(guess) {
        if secret != guess {
            secret_counts[*secret as usize] += 1;
            guess_counts[*guess as usize] += 1;
        }
    }
    let partial = (0..6)
        .map(|color| secret_counts[color].min(guess_counts[color]))
        .sum();
    (exact, partial)
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
