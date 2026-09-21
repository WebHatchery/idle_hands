//! Deterministic touch-first Memory/Pairs rules.

use serde::{Deserialize, Serialize};

pub const CELLS: usize = 16;
pub const PAIRS: usize = CELLS / 2;

#[derive(Debug, Clone, Copy)]
pub struct MemoryUndo {
    pub cards: [MemoryCard; CELLS],
    pub selected: [Option<usize>; 2],
    pub mismatch_waiting: bool,
    pub matched_pairs: u8,
    pub moves: u16,
    pub status: MemoryStatus,
    pub seen: [bool; CELLS],
    pub score: u32,
    pub combo: u8,
    pub best_combo: u8,
    pub mistakes: u8,
    pub peeks: u8,
    pub peeked: [Option<usize>; 2],
    pub peek_waiting: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCard {
    pub pair: u8,
    pub face_up: bool,
    pub matched: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MemoryVariant {
    #[default]
    Classic,
    Focus,
    Rush,
}

impl MemoryVariant {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Focus, Self::Rush];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · 1 PEEK",
            Self::Focus => "FOCUS · NO PEEK",
            Self::Rush => "RUSH · DOUBLE PENALTY",
        }
    }

    pub const fn peeks(self) -> u8 {
        match self {
            Self::Classic | Self::Rush => 1,
            Self::Focus => 0,
        }
    }

    pub const fn mismatch_penalty(self) -> u32 {
        match self {
            Self::Rush => 10,
            Self::Classic | Self::Focus => 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPairs {
    pub cards: [MemoryCard; CELLS],
    pub selected: [Option<usize>; 2],
    pub mismatch_waiting: bool,
    pub matched_pairs: u8,
    pub moves: u16,
    pub seed: u64,
    pub status: MemoryStatus,
    #[serde(default)]
    pub variant: MemoryVariant,
    #[serde(default)]
    pub seen: [bool; CELLS],
    #[serde(default)]
    pub score: u32,
    #[serde(default)]
    pub combo: u8,
    #[serde(default)]
    pub best_combo: u8,
    #[serde(default)]
    pub mistakes: u8,
    #[serde(default = "default_peeks")]
    pub peeks: u8,
    #[serde(default)]
    pub peeked: [Option<usize>; 2],
    #[serde(default)]
    pub peek_waiting: bool,
    #[serde(skip)]
    pub history: Vec<MemoryUndo>,
}

impl Default for MemoryPairs {
    fn default() -> Self {
        Self::new(0x1D1E_5045)
    }
}

impl MemoryPairs {
    pub fn new(seed: u64) -> Self {
        Self::new_with_variant(seed, MemoryVariant::default())
    }

    pub fn new_with_variant(seed: u64, variant: MemoryVariant) -> Self {
        let mut pairs = [0u8; CELLS];
        for (index, pair) in pairs.iter_mut().enumerate() {
            *pair = (index / 2) as u8;
        }
        let mut source = seed;
        for index in (1..CELLS).rev() {
            source = next_seed(source);
            let swap = (source as usize) % (index + 1);
            pairs.swap(index, swap);
        }
        Self {
            cards: pairs.map(|pair| MemoryCard {
                pair,
                face_up: false,
                matched: false,
            }),
            selected: [None, None],
            mismatch_waiting: false,
            matched_pairs: 0,
            moves: 0,
            seed,
            status: MemoryStatus::Playing,
            variant,
            seen: [false; CELLS],
            score: 0,
            combo: 0,
            best_combo: 0,
            mistakes: 0,
            peeks: variant.peeks(),
            peeked: [None, None],
            peek_waiting: false,
            history: Vec::new(),
        }
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status == MemoryStatus::Won || self.cards[index].matched {
            return false;
        }
        if self.selected.contains(&Some(index)) && !self.peek_waiting {
            return false;
        }
        self.snapshot();
        if self.mismatch_waiting {
            for selected in self.selected.into_iter().flatten() {
                self.cards[selected].face_up = false;
            }
            self.selected = [None, None];
            self.mismatch_waiting = false;
        }
        if self.peek_waiting {
            for peeked in self.peeked.into_iter().flatten() {
                if !self.cards[peeked].matched {
                    self.cards[peeked].face_up = false;
                }
            }
            self.peeked = [None, None];
            self.peek_waiting = false;
        }
        self.cards[index].face_up = true;
        self.seen[index] = true;
        if let Some(first) = self.selected[0] {
            self.selected[1] = Some(index);
            self.moves = self.moves.saturating_add(1);
            if self.cards[first].pair == self.cards[index].pair {
                self.cards[first].matched = true;
                self.cards[index].matched = true;
                self.matched_pairs += 1;
                self.combo = self.combo.saturating_add(1);
                self.best_combo = self.best_combo.max(self.combo);
                self.score = self.score.saturating_add(20 * u32::from(self.combo));
                self.selected = [None, None];
                if self.matched_pairs as usize == PAIRS {
                    self.status = MemoryStatus::Won;
                }
            } else {
                self.mismatch_waiting = true;
                self.combo = 0;
                self.mistakes = self.mistakes.saturating_add(1);
                self.score = self.score.saturating_sub(self.variant.mismatch_penalty());
            }
        } else {
            self.selected[0] = Some(index);
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.history.pop() {
            self.cards = snapshot.cards;
            self.selected = snapshot.selected;
            self.mismatch_waiting = snapshot.mismatch_waiting;
            self.matched_pairs = snapshot.matched_pairs;
            self.moves = snapshot.moves;
            self.status = snapshot.status;
            self.seen = snapshot.seen;
            self.score = snapshot.score;
            self.combo = snapshot.combo;
            self.best_combo = snapshot.best_combo;
            self.mistakes = snapshot.mistakes;
            self.peeks = snapshot.peeks;
            self.peeked = snapshot.peeked;
            self.peek_waiting = snapshot.peek_waiting;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn hint_pair(&self) -> Option<(usize, usize)> {
        if self.status == MemoryStatus::Won {
            return None;
        }
        for first in 0..CELLS {
            if self.cards[first].matched || !self.seen[first] {
                continue;
            }
            if let Some(second) = ((first + 1)..CELLS).find(|&index| {
                !self.cards[index].matched
                    && self.seen[index]
                    && self.cards[index].pair == self.cards[first].pair
            }) {
                return Some((first, second));
            }
        }
        None
    }

    pub fn hint_choice(&self) -> Option<usize> {
        if let Some(first) = self.selected[0] {
            if let Some(index) = (0..CELLS).find(|&index| {
                index != first
                    && self.seen[index]
                    && !self.cards[index].matched
                    && self.cards[index].pair == self.cards[first].pair
            }) {
                return Some(index);
            }
        }
        self.hint_pair()
            .map(|(first, _)| first)
            .or_else(|| (0..CELLS).find(|&index| !self.cards[index].matched && !self.seen[index]))
    }

    pub fn peek(&mut self) -> bool {
        if self.status != MemoryStatus::Playing || self.peeks == 0 || self.peek_waiting {
            return false;
        }
        let choices: Vec<usize> = (0..CELLS)
            .filter(|&index| {
                !self.cards[index].matched && !self.cards[index].face_up && !self.seen[index]
            })
            .collect();
        if choices.len() < 2 {
            return false;
        }
        self.snapshot();
        self.peeks -= 1;
        self.peeked = [Some(choices[0]), Some(choices[1])];
        for index in choices.into_iter().take(2) {
            self.cards[index].face_up = true;
            self.seen[index] = true;
        }
        self.peek_waiting = true;
        true
    }

    pub fn seen_count(&self) -> usize {
        self.seen.iter().filter(|seen| **seen).count()
    }

    pub fn snapshot(&mut self) {
        self.history.push(MemoryUndo {
            cards: self.cards,
            selected: self.selected,
            mismatch_waiting: self.mismatch_waiting,
            matched_pairs: self.matched_pairs,
            moves: self.moves,
            status: self.status,
            seen: self.seen,
            score: self.score,
            combo: self.combo,
            best_combo: self.best_combo,
            mistakes: self.mistakes,
            peeks: self.peeks,
            peeked: self.peeked,
            peek_waiting: self.peek_waiting,
        });
    }
}

pub const fn default_peeks() -> u8 {
    1
}

pub fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}
