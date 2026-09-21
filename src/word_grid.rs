//! Deterministic touch-first five-letter Word Grid.
use crate::undo::UndoStack;

use serde::{Deserialize, Serialize};

pub const WORD_LENGTH: usize = 5;
pub const MAX_GUESSES: usize = 6;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordGridMode {
    #[default]
    Classic,
    Hard,
}

impl WordGridMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC",
            Self::Hard => "HARD",
        }
    }
}

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
    #[serde(default)]
    pub mode: WordGridMode,
    #[serde(default)]
    pub notice: String,
    pub phase: WordGridPhase,
    #[serde(default)]
    pub dictionary: Vec<String>,
    #[serde(default = "default_max_guesses")]
    pub max_guesses: usize,
    #[serde(skip)]
    pub history: UndoStack<Self>,
}

impl Default for WordGrid {
    fn default() -> Self {
        Self::new(0x0057_4F52_4453)
    }
}

impl WordGrid {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_config(
            seed,
            WordGridMode::Classic,
            &content.words.word_grid,
            content.balance.word_games.word_grid_max_guesses,
        )
    }

    pub fn new_with_mode(seed: u64, mode: WordGridMode) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_config(
            seed,
            mode,
            &content.words.word_grid,
            content.balance.word_games.word_grid_max_guesses,
        )
    }

    pub fn new_with_mode_config(
        seed: u64,
        mode: WordGridMode,
        dictionary: &[String],
        max_guesses: usize,
    ) -> Self {
        let target = dictionary[(seed as usize) % dictionary.len()].clone();
        Self {
            target,
            guesses: Vec::new(),
            feedback: Vec::new(),
            current: String::new(),
            used: [LetterState::Unknown; 26],
            moves: 0,
            seed,
            mode,
            notice: String::new(),
            phase: WordGridPhase::Playing,
            dictionary: dictionary.to_vec(),
            max_guesses,
            history: UndoStack::default(),
        }
    }

    pub fn tap_letter(&mut self, letter: u8) -> bool {
        if self.phase != WordGridPhase::Playing || letter >= 26 || self.current.len() >= WORD_LENGTH
        {
            return false;
        }
        self.current.push((b'A' + letter) as char);
        self.notice.clear();
        true
    }

    pub fn backspace(&mut self) -> bool {
        if self.phase != WordGridPhase::Playing || self.current.pop().is_none() {
            return false;
        }
        self.notice.clear();
        true
    }

    pub fn submit(&mut self) -> bool {
        if self.phase != WordGridPhase::Playing || self.current.len() != WORD_LENGTH {
            return false;
        }
        if self.mode == WordGridMode::Hard {
            if let Some(message) = self.hard_violation(&self.current) {
                self.notice = message;
                return false;
            }
        }
        let previous = self.clone_without_history();
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
        self.notice.clear();
        if guess == self.target {
            self.phase = WordGridPhase::Won;
        } else if self.guesses.len() >= self.max_guesses {
            self.phase = WordGridPhase::Lost;
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
        *self = Self::new_with_mode(seed, self.mode);
    }

    pub fn set_mode(&mut self, mode: WordGridMode, seed: u64) {
        *self = Self::new_with_mode(seed, mode);
    }

    pub fn won(&self) -> bool {
        self.phase == WordGridPhase::Won
    }

    pub fn hint_word(&self) -> Option<String> {
        if self.phase != WordGridPhase::Playing {
            return None;
        }
        let remaining = self.remaining_words();
        self.dictionary
            .iter()
            .map(String::as_str)
            .filter(|candidate| *candidate != self.target)
            .filter(|candidate| !self.guesses.iter().any(|guess| guess == candidate))
            .max_by_key(|candidate| probe_score(candidate, &remaining))
            .map(str::to_owned)
    }

    pub fn remaining_words(&self) -> Vec<&str> {
        self.dictionary
            .iter()
            .map(String::as_str)
            .filter(|candidate| {
                self.guesses
                    .iter()
                    .enumerate()
                    .all(|(index, guess)| score_guess(candidate, guess) == self.feedback[index])
            })
            .collect()
    }

    pub fn hard_violation(&self, guess: &str) -> Option<String> {
        if guess.len() != WORD_LENGTH || self.guesses.is_empty() {
            return None;
        }
        let bytes = guess.as_bytes();
        let mut required = [0u8; 26];
        for (prior, feedback) in self.guesses.iter().zip(&self.feedback) {
            let prior_bytes = prior.as_bytes();
            let mut row_required = [0u8; 26];
            for index in 0..WORD_LENGTH {
                match feedback[index] {
                    LetterState::Correct if bytes[index] != prior_bytes[index] => {
                        return Some(format!(
                            "HARD: keep {} in slot {}",
                            prior_bytes[index] as char,
                            index + 1
                        ));
                    }
                    LetterState::Present if bytes[index] == prior_bytes[index] => {
                        return Some(format!(
                            "HARD: move {} out of slot {}",
                            prior_bytes[index] as char,
                            index + 1
                        ));
                    }
                    LetterState::Present | LetterState::Correct => {
                        row_required[(prior_bytes[index] - b'A') as usize] += 1;
                    }
                    LetterState::Unknown | LetterState::Absent => {}
                }
            }
            for index in 0..26 {
                required[index] = required[index].max(row_required[index]);
            }
        }
        for (letter, &count) in required.iter().enumerate() {
            let actual = bytes
                .iter()
                .filter(|&&value| value == b'A' + letter as u8)
                .count();
            if actual < count as usize {
                return Some(format!("HARD: include {}", (b'A' + letter as u8) as char));
            }
        }
        None
    }

    pub fn clone_without_history(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

pub fn probe_score(probe: &str, remaining: &[&str]) -> usize {
    let mut seen = [false; 26];
    probe
        .bytes()
        .filter_map(|letter| {
            let index = (letter - b'A') as usize;
            if std::mem::replace(&mut seen[index], true) {
                None
            } else {
                Some(
                    remaining
                        .iter()
                        .filter(|word| word.as_bytes().contains(&letter))
                        .count(),
                )
            }
        })
        .sum()
}

pub fn score_guess(target: &str, guess: &str) -> [LetterState; WORD_LENGTH] {
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

pub fn stronger(old: LetterState, new: LetterState) -> LetterState {
    if state_rank(new) > state_rank(old) {
        new
    } else {
        old
    }
}

pub fn state_rank(state: LetterState) -> u8 {
    match state {
        LetterState::Unknown => 0,
        LetterState::Absent => 1,
        LetterState::Present => 2,
        LetterState::Correct => 3,
    }
}

pub fn default_max_guesses() -> usize {
    MAX_GUESSES
}
