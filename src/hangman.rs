//! Deterministic touch-first Hangman rules.

use serde::{Deserialize, Serialize};

pub const WORDS: [&str; 6] = ["QUIET", "CABINET", "PAUSE", "SHELF", "GARDEN", "MOMENT"];
const MAX_WRONG: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HangmanStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hangman {
    pub word: String,
    pub guessed: [bool; 26],
    pub wrong: [bool; 26],
    pub wrong_count: u8,
    pub moves: u16,
    pub status: HangmanStatus,
    pub seed: u64,
}

impl Default for Hangman {
    fn default() -> Self {
        Self::new(0x0BAD_5EED)
    }
}

impl Hangman {
    pub fn new(seed: u64) -> Self {
        let word = WORDS[(seed as usize) % WORDS.len()].to_owned();
        Self {
            word,
            guessed: [false; 26],
            wrong: [false; 26],
            wrong_count: 0,
            moves: 0,
            status: HangmanStatus::Playing,
            seed,
        }
    }

    pub fn guess(&mut self, letter: u8) -> bool {
        if letter >= 26 || self.status != HangmanStatus::Playing || self.guessed[letter as usize] {
            return false;
        }
        self.guessed[letter as usize] = true;
        self.moves += 1;
        let character = b'A' + letter;
        if self.word.bytes().any(|candidate| candidate == character) {
            if self
                .word
                .bytes()
                .all(|candidate| self.guessed[(candidate - b'A') as usize])
            {
                self.status = HangmanStatus::Won;
            }
        } else {
            self.wrong[letter as usize] = true;
            self.wrong_count += 1;
            if self.wrong_count >= MAX_WRONG {
                self.status = HangmanStatus::Lost;
            }
        }
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn is_revealed(&self, character: u8) -> bool {
        character < 26 && self.guessed[character as usize]
    }
}

#[cfg(test)]
mod tests;
