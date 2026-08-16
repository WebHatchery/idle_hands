//! Deterministic five-letter word ladders.

use serde::{Deserialize, Serialize};

pub const WORD_LENGTH: usize = 5;
pub const WORDS: [&str; 18] = [
    "SLATE", "PLATE", "PLACE", "PLANE", "PLANK", "BLANK", "FLANK", "FLARE", "SHARE",
    "SHORE", "SCORE", "SCONE", "STONE", "SHONE", "PHONE", "PHONY", "LIGHT", "NIGHT",
];
pub const PUZZLES: [(&str, &str); 3] = [("SLATE", "BLANK"), ("SHARE", "STONE"), ("LIGHT", "NIGHT")];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordLadderPhase { Playing, Won }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordLadder {
    pub start: String,
    pub target: String,
    pub current: String,
    pub guesses: Vec<String>,
    pub moves: u16,
    pub seed: u64,
    pub phase: WordLadderPhase,
    pub message: String,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for WordLadder { fn default() -> Self { Self::new(0x4C41_4444_4552) } }

impl WordLadder {
    pub fn new(seed: u64) -> Self {
        let (start, target) = PUZZLES[(seed as usize) % PUZZLES.len()];
        Self { start: start.into(), target: target.into(), current: String::new(), guesses: Vec::new(), moves: 0, seed, phase: WordLadderPhase::Playing, message: "Change one letter at a time".into(), undo: None }
    }
    pub fn tap_letter(&mut self, letter: u8) -> bool {
        if self.phase != WordLadderPhase::Playing || letter >= 26 || self.current.len() >= WORD_LENGTH { return false; }
        self.current.push((b'A' + letter) as char); true
    }
    pub fn backspace(&mut self) -> bool { self.phase == WordLadderPhase::Playing && self.current.pop().is_some() }
    pub fn submit(&mut self) -> bool {
        if self.phase != WordLadderPhase::Playing || self.current.len() != WORD_LENGTH { self.message = "Fill all five letters first".into(); return false; }
        let guess = self.current.clone();
        if !WORDS.contains(&guess.as_str()) { self.message = "That word is not in the quiet dictionary".into(); return false; }
        let previous = self.clone_without_undo();
        if self.guesses.last() == Some(&guess) || !one_away(self.guesses.last().map_or(&self.start, |word| word), &guess) {
            self.message = "Change exactly one letter from the last word".into(); return false;
        }
        self.guesses.push(guess.clone()); self.current.clear(); self.moves = self.moves.saturating_add(1); self.undo = Some(Box::new(previous));
        if guess == self.target { self.phase = WordLadderPhase::Won; self.message = format!("Ladder complete in {} moves", self.moves); } else { self.message = "Good step — keep climbing".into(); }
        true
    }
    pub fn undo(&mut self) -> bool { let Some(previous) = self.undo.take() else { return false; }; *self = *previous; true }
    pub fn reset(&mut self, seed: u64) { *self = Self::new(seed); }
    pub fn hint_word(&self) -> Option<&'static str> {
        if self.phase != WordLadderPhase::Playing { return None; }
        let from = self.guesses.last().map_or(&self.start, |word| word);
        WORDS.iter().copied().find(|word| *word != self.target && *word != from && one_away(from, word))
            .or_else(|| WORDS.iter().copied().find(|word| *word == self.target && one_away(from, word)))
    }
    fn clone_without_undo(&self) -> Self { let mut copy = self.clone(); copy.undo = None; copy }
}

fn one_away(a: &str, b: &str) -> bool { a.bytes().zip(b.bytes()).filter(|(left, right)| left != right).count() == 1 }

#[cfg(test)]
mod tests;
