//! Deterministic touch-first five-letter Word Grid.

use serde::{Deserialize, Serialize};

pub const WORD_LENGTH: usize = 5;
pub const MAX_GUESSES: usize = 6;
pub const WORDS: [&str; 8] = [
    "STILL", "SHELF", "PAUSE", "GAMES", "SMALL", "WORDS", "MOTIF", "CABIN",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LetterState {
    Unknown,
    Absent,
    Present,
    Correct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordGridPhase {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordGrid {
    pub target: String,
    pub guesses: Vec<String>,
    pub feedback: Vec<[LetterState; WORD_LENGTH]>,
    pub current: String,
    pub used: [LetterState; 26],
    pub moves: u16,
    pub seed: u64,
    pub phase: WordGridPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for WordGrid {
    fn default() -> Self {
        Self::new(0x0057_4F52_4453)
    }
}

impl WordGrid {
    pub fn new(seed: u64) -> Self {
        let target = WORDS[(seed as usize) % WORDS.len()].to_owned();
        Self {
            target,
            guesses: Vec::new(),
            feedback: Vec::new(),
            current: String::new(),
            used: [LetterState::Unknown; 26],
            moves: 0,
            seed,
            phase: WordGridPhase::Playing,
            undo: None,
        }
    }

    pub fn tap_letter(&mut self, letter: u8) -> bool {
        if self.phase != WordGridPhase::Playing || letter >= 26 || self.current.len() >= WORD_LENGTH
        {
            return false;
        }
        self.current.push((b'A' + letter) as char);
        true
    }

    pub fn backspace(&mut self) -> bool {
        if self.phase != WordGridPhase::Playing || self.current.pop().is_none() {
            return false;
        }
        true
    }

    pub fn submit(&mut self) -> bool {
        if self.phase != WordGridPhase::Playing || self.current.len() != WORD_LENGTH {
            return false;
        }
        let previous = self.clone_without_undo();
        let guess = self.current.clone();
        let result = score_guess(&self.target, &guess);
        for (index, state) in result.iter().enumerate() {
            let letter = (guess.as_bytes()[index] - b'A') as usize;
            self.used[letter] = stronger(self.used[letter], *state);
        }
        self.guesses.push(guess.clone());
        self.feedback.push(result);
        self.current.clear();
        self.moves = self.moves.saturating_add(1);
        self.undo = Some(Box::new(previous));
        if guess == self.target {
            self.phase = WordGridPhase::Won;
        } else if self.guesses.len() >= MAX_GUESSES {
            self.phase = WordGridPhase::Lost;
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
        self.phase == WordGridPhase::Won
    }

    pub fn hint_word(&self) -> Option<&'static str> {
        if self.phase != WordGridPhase::Playing {
            return None;
        }
        WORDS
            .iter()
            .copied()
            .filter(|candidate| *candidate != self.target)
            .filter(|candidate| !self.guesses.iter().any(|guess| guess == candidate))
            .find(|candidate| {
                self.guesses
                    .iter()
                    .enumerate()
                    .all(|(index, guess)| score_guess(candidate, guess) == self.feedback[index])
            })
            .or_else(|| {
                WORDS.iter().copied().find(|candidate| {
                    *candidate != self.target
                        && !self.guesses.iter().any(|guess| guess == candidate)
                })
            })
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

fn score_guess(target: &str, guess: &str) -> [LetterState; WORD_LENGTH] {
    let target_bytes = target.as_bytes();
    let guess_bytes = guess.as_bytes();
    let mut result = [LetterState::Absent; WORD_LENGTH];
    let mut remaining = [0u8; 26];
    for (index, &letter) in target_bytes.iter().enumerate() {
        if guess_bytes[index] == letter {
            result[index] = LetterState::Correct;
        } else {
            remaining[(letter - b'A') as usize] += 1;
        }
    }
    for (index, &letter) in guess_bytes.iter().enumerate() {
        if result[index] == LetterState::Correct {
            continue;
        }
        let slot = (letter - b'A') as usize;
        if remaining[slot] > 0 {
            result[index] = LetterState::Present;
            remaining[slot] -= 1;
        }
    }
    result
}

fn stronger(old: LetterState, new: LetterState) -> LetterState {
    if state_rank(new) > state_rank(old) {
        new
    } else {
        old
    }
}

fn state_rank(state: LetterState) -> u8 {
    match state {
        LetterState::Unknown => 0,
        LetterState::Absent => 1,
        LetterState::Present => 2,
        LetterState::Correct => 3,
    }
}

#[cfg(test)]
mod tests;
