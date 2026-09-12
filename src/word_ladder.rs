//! Deterministic five-letter word ladders.
use crate::undo::UndoStack;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

pub const WORD_LENGTH: usize = 5;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LadderMode {
    #[default]
    Direct,
    Scenic,
}

impl LadderMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Direct => "DIRECT",
            Self::Scenic => "SCENIC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordLadderPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordLadder {
    pub start: String,
    pub target: String,
    #[serde(default)]
    pub waypoint: String,
    #[serde(default)]
    pub waypoint_reached: bool,
    #[serde(default)]
    pub mode: LadderMode,
    #[serde(default)]
    pub par: u16,
    pub current: String,
    pub guesses: Vec<String>,
    pub moves: u16,
    pub seed: u64,
    pub phase: WordLadderPhase,
    pub message: String,
    #[serde(default)]
    pub dictionary: Vec<String>,
    #[serde(default)]
    pub puzzles: Vec<crate::content::WordLadderPuzzle>,
    #[serde(skip)]
    history: UndoStack<Self>,
}

impl Default for WordLadder {
    fn default() -> Self {
        Self::new(0x4C41_4444_4552)
    }
}

impl WordLadder {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_config(
            seed,
            LadderMode::Direct,
            &content.words.word_ladder.dictionary,
            &content.words.word_ladder.puzzles,
        )
    }
    pub fn new_with_mode(seed: u64, mode: LadderMode) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_config(
            seed,
            mode,
            &content.words.word_ladder.dictionary,
            &content.words.word_ladder.puzzles,
        )
    }

    pub fn new_with_mode_config(
        seed: u64,
        mode: LadderMode,
        dictionary: &[String],
        puzzles: &[crate::content::WordLadderPuzzle],
    ) -> Self {
        let puzzle = &puzzles[(seed as usize) % puzzles.len()];
        let direct = shortest_path(dictionary, &puzzle.start, &puzzle.target)
            .map_or(0, |path| path.len().saturating_sub(1));
        let scenic = shortest_path(dictionary, &puzzle.start, &puzzle.waypoint)
            .map_or(0, |path| path.len().saturating_sub(1))
            + shortest_path(dictionary, &puzzle.waypoint, &puzzle.target)
                .map_or(0, |path| path.len().saturating_sub(1));
        Self {
            start: puzzle.start.clone(),
            target: puzzle.target.clone(),
            waypoint: puzzle.waypoint.clone(),
            waypoint_reached: false,
            mode,
            par: if mode == LadderMode::Scenic {
                scenic
            } else {
                direct
            } as u16,
            current: String::new(),
            guesses: Vec::new(),
            moves: 0,
            seed,
            phase: WordLadderPhase::Playing,
            message: "Change one letter at a time".into(),
            dictionary: dictionary.to_vec(),
            puzzles: puzzles.to_vec(),
            history: UndoStack::default(),
        }
    }
    pub fn tap_letter(&mut self, letter: u8) -> bool {
        if self.phase != WordLadderPhase::Playing
            || letter >= 26
            || self.current.len() >= WORD_LENGTH
        {
            return false;
        }
        self.current.push((b'A' + letter) as char);
        true
    }
    pub fn backspace(&mut self) -> bool {
        self.phase == WordLadderPhase::Playing && self.current.pop().is_some()
    }
    pub fn submit(&mut self) -> bool {
        if self.phase != WordLadderPhase::Playing || self.current.len() != WORD_LENGTH {
            self.message = "Fill all five letters first".into();
            return false;
        }
        let guess = self.current.clone();
        if !self.dictionary.iter().any(|word| word == &guess) {
            self.message = "That word is not in the dictionary".into();
            return false;
        }
        if self.guesses.last() == Some(&guess)
            || !one_away(self.guesses.last().map_or(&self.start, |word| word), &guess)
        {
            self.message = "Change exactly one letter from the last word".into();
            return false;
        }
        if self.mode == LadderMode::Scenic && !self.waypoint_reached && guess == self.target {
            self.message = format!("Reach {} before the target", self.waypoint);
            return false;
        }
        let previous = self.clone_without_history();
        self.guesses.push(guess.clone());
        self.current.clear();
        self.moves = self.moves.saturating_add(1);
        if guess == self.waypoint {
            self.waypoint_reached = true;
        }
        if guess == self.target {
            self.phase = WordLadderPhase::Won;
            self.message = format!("Ladder complete in {} moves", self.moves);
        } else {
            self.message = if self.mode == LadderMode::Scenic && self.waypoint_reached {
                "Waypoint reached — now climb to the target".into()
            } else {
                "Good step — keep climbing".into()
            };
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
        *self = self.with_catalog(seed, self.mode, &self.dictionary, &self.puzzles);
    }
    pub fn set_mode(&mut self, mode: LadderMode, seed: u64) {
        *self = self.with_catalog(seed, mode, &self.dictionary, &self.puzzles);
    }
    pub fn hint_word(&self) -> Option<String> {
        if self.phase != WordLadderPhase::Playing {
            return None;
        }
        self.route()
            .and_then(|path| path.get(1).map(|word| (*word).to_owned()))
    }

    pub fn remaining_steps(&self) -> usize {
        self.route().map_or(0, |path| path.len().saturating_sub(1))
    }

    pub fn legal_step_count(&self) -> usize {
        let from = self.guesses.last().map_or(&self.start, |word| word);
        self.dictionary
            .iter()
            .filter(|word| word.as_str() != from && one_away(from, word))
            .count()
    }

    pub fn current_difference_count(&self) -> usize {
        let from = self.guesses.last().map_or(&self.start, |word| word);
        from.bytes()
            .zip(self.current.bytes())
            .filter(|(left, right)| left != right)
            .count()
    }

    fn route(&self) -> Option<Vec<&str>> {
        let from = self
            .guesses
            .last()
            .map_or(self.start.as_str(), |word| word.as_str());
        let objective = if self.mode == LadderMode::Scenic && !self.waypoint_reached {
            self.waypoint.as_str()
        } else {
            self.target.as_str()
        };
        shortest_path(&self.dictionary, from, objective)
    }

    fn clone_without_history(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }

    fn with_catalog(
        &self,
        seed: u64,
        mode: LadderMode,
        dictionary: &[String],
        puzzles: &[crate::content::WordLadderPuzzle],
    ) -> Self {
        Self::new_with_mode_config(seed, mode, dictionary, puzzles)
    }
}

fn shortest_path<'a>(dictionary: &'a [String], start: &str, target: &str) -> Option<Vec<&'a str>> {
    let start = dictionary
        .iter()
        .find(|word| word.as_str() == start)
        .map(String::as_str)?;
    let target = dictionary
        .iter()
        .find(|word| word.as_str() == target)
        .map(String::as_str)?;
    let mut queue = VecDeque::from([start]);
    let mut previous: HashMap<&str, Option<&str>> = HashMap::from([(start, None)]);
    while let Some(word) = queue.pop_front() {
        if word == target {
            let mut path = Vec::new();
            let mut cursor = Some(word);
            while let Some(next) = cursor {
                path.push(
                    dictionary
                        .iter()
                        .find(|candidate| candidate.as_str() == next)?
                        .as_str(),
                );
                cursor = previous[next];
            }
            path.reverse();
            return Some(path);
        }
        for candidate in dictionary.iter().map(String::as_str) {
            if !previous.contains_key(candidate) && one_away(word, candidate) {
                previous.insert(candidate, Some(word));
                queue.push_back(candidate);
            }
        }
    }
    None
}

fn one_away(a: &str, b: &str) -> bool {
    a.bytes()
        .zip(b.bytes())
        .filter(|(left, right)| left != right)
        .count()
        == 1
}

#[cfg(test)]
mod tests;
