//! Deterministic Kalah-style Mancala with a local opponent.

use serde::{Deserialize, Serialize};

const PLAYER_STORE: usize = 6;
const OPPONENT_START: usize = 7;
const OPPONENT_STORE: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MancalaPhase {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiLevel {
    Gentle,
    Sharp,
    Expert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MancalaVariant {
    Quick,
    Classic,
    Grand,
}

impl MancalaVariant {
    pub fn starting_stones(self) -> u8 {
        match self {
            Self::Quick => 3,
            Self::Classic => 4,
            Self::Grand => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovePreview {
    pub store_gain: u8,
    pub captured: u8,
    pub extra_turn: bool,
}

#[derive(Debug, Clone, Copy)]
struct SowResult {
    last: usize,
    captured: u8,
}

fn default_ai_level() -> AiLevel {
    AiLevel::Sharp
}

fn default_variant() -> MancalaVariant {
    MancalaVariant::Classic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mancala {
    pub pits: Vec<u8>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default = "default_ai_level")]
    pub ai_level: AiLevel,
    #[serde(default = "default_variant")]
    pub variant: MancalaVariant,
    #[serde(default)]
    pub captured_stones: u16,
    #[serde(default)]
    pub extra_turns: u16,
    pub phase: MancalaPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for Mancala {
    fn default() -> Self {
        Self::new(0x4D41_4E43_4100)
    }
}

impl Mancala {
    pub fn new(seed: u64) -> Self {
        Self::new_with_variant(seed, MancalaVariant::Classic)
    }

    pub fn new_with_variant(seed: u64, variant: MancalaVariant) -> Self {
        let mut pits = vec![variant.starting_stones(); 14];
        pits[PLAYER_STORE] = 0;
        pits[OPPONENT_STORE] = 0;
        Self {
            pits,
            moves: 0,
            seed,
            ai_level: AiLevel::Sharp,
            variant,
            captured_stones: 0,
            extra_turns: 0,
            phase: MancalaPhase::Playing,
            undo: None,
        }
    }

    pub fn play(&mut self, pit: usize) -> bool {
        if self.phase != MancalaPhase::Playing || pit >= PLAYER_STORE || self.pits[pit] == 0 {
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        let result = self.sow(pit, true);
        self.captured_stones = self
            .captured_stones
            .saturating_add(u16::from(result.captured));
        if result.last == PLAYER_STORE {
            self.extra_turns = self.extra_turns.saturating_add(1);
        }
        self.moves = self.moves.saturating_add(1);
        if self.side_empty(true) {
            self.finish();
        } else if result.last != PLAYER_STORE {
            self.cpu_turn();
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
        let ai_level = self.ai_level;
        let variant = self.variant;
        *self = Self::new_with_variant(seed, variant);
        self.ai_level = ai_level;
    }

    pub fn set_ai_level(&mut self, ai_level: AiLevel) {
        self.ai_level = ai_level;
    }

    pub fn set_variant(&mut self, variant: MancalaVariant, seed: u64) {
        let ai_level = self.ai_level;
        *self = Self::new_with_variant(seed, variant);
        self.ai_level = ai_level;
    }

    pub fn won(&self) -> bool {
        self.phase == MancalaPhase::Won
    }

    pub fn hint_pit(&self) -> Option<usize> {
        if self.phase != MancalaPhase::Playing {
            return None;
        }
        let mut best: Option<(i32, usize)> = None;
        for pit in 0..PLAYER_STORE {
            if self.pits[pit] == 0 {
                continue;
            }
            let preview = self.move_preview(pit)?;
            let score = i32::from(preview.extra_turn) * 1_000
                + i32::from(preview.captured) * 140
                + i32::from(preview.store_gain) * 100;
            if best.is_none_or(|(best_score, _)| score > best_score) {
                best = Some((score, pit));
            }
        }
        best.map(|(_, pit)| pit)
    }

    pub fn move_preview(&self, pit: usize) -> Option<MovePreview> {
        if self.phase != MancalaPhase::Playing || pit >= PLAYER_STORE || self.pits[pit] == 0 {
            return None;
        }
        let mut trial = self.clone_without_undo();
        let before_store = trial.pits[PLAYER_STORE];
        let result = trial.sow(pit, true);
        Some(MovePreview {
            store_gain: trial.pits[PLAYER_STORE] - before_store,
            captured: result.captured,
            extra_turn: result.last == PLAYER_STORE,
        })
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    fn sow(&mut self, pit: usize, player: bool) -> SowResult {
        let mut stones = self.pits[pit];
        self.pits[pit] = 0;
        let store = if player { PLAYER_STORE } else { OPPONENT_STORE };
        let forbidden = if player { OPPONENT_STORE } else { PLAYER_STORE };
        let start = if player { 0 } else { OPPONENT_START };
        let end = if player { PLAYER_STORE } else { OPPONENT_STORE };
        let mut index = pit;
        while stones > 0 {
            index = (index + 1) % 14;
            if index == forbidden {
                continue;
            }
            self.pits[index] += 1;
            stones -= 1;
        }
        let mut captured = 0;
        if index >= start && index < end && self.pits[index] == 1 {
            let opposite = 12 - index;
            if self.pits[opposite] > 0 {
                captured = self.pits[opposite] + 1;
                self.pits[store] += captured;
                self.pits[opposite] = 0;
                self.pits[index] = 0;
            }
        }
        SowResult {
            last: index,
            captured,
        }
    }

    fn cpu_turn(&mut self) {
        loop {
            let available: Vec<usize> = (OPPONENT_START..OPPONENT_STORE)
                .filter(|&pit| self.pits[pit] > 0)
                .collect();
            if available.is_empty() {
                self.finish();
                return;
            }
            let pit = match self.ai_level {
                AiLevel::Gentle => available[0],
                AiLevel::Sharp => available
                    .iter()
                    .copied()
                    .max_by_key(|&pit| self.cpu_move_value(pit, 0))
                    .unwrap_or(available[0]),
                AiLevel::Expert => available
                    .iter()
                    .copied()
                    .max_by_key(|&pit| self.cpu_move_value(pit, 2))
                    .unwrap_or(available[0]),
            };
            let result = self.sow(pit, false);
            if self.side_empty(false) {
                self.finish();
                return;
            }
            if result.last != OPPONENT_STORE {
                return;
            }
        }
    }

    fn cpu_move_value(&self, pit: usize, depth: u8) -> i32 {
        let mut trial = self.clone_without_undo();
        let before_store = trial.pits[OPPONENT_STORE];
        let result = trial.sow(pit, false);
        let gain = i32::from(trial.pits[OPPONENT_STORE] - before_store);
        let mut value = gain * 100
            + i32::from(result.captured) * 40
            + i32::from(result.last == OPPONENT_STORE) * 1_000;
        if depth > 0 && result.last == OPPONENT_STORE && !trial.side_empty(false) {
            value += (OPPONENT_START..OPPONENT_STORE)
                .filter(|&next| trial.pits[next] > 0)
                .map(|next| trial.cpu_move_value(next, depth - 1))
                .max()
                .unwrap_or(0)
                / 2;
        } else if depth > 0 {
            value -= trial.best_player_reply_value() / 2;
        }
        value
    }

    fn best_player_reply_value(&self) -> i32 {
        (0..PLAYER_STORE)
            .filter_map(|pit| self.move_preview(pit))
            .map(|preview| {
                i32::from(preview.extra_turn) * 800
                    + i32::from(preview.captured) * 140
                    + i32::from(preview.store_gain) * 100
            })
            .max()
            .unwrap_or(0)
    }

    fn side_empty(&self, player: bool) -> bool {
        if player {
            self.pits[..PLAYER_STORE].iter().all(|&stones| stones == 0)
        } else {
            self.pits[OPPONENT_START..OPPONENT_STORE]
                .iter()
                .all(|&stones| stones == 0)
        }
    }

    fn finish(&mut self) {
        let player_remaining: u8 = self.pits[..PLAYER_STORE].iter().sum();
        let opponent_remaining: u8 = self.pits[OPPONENT_START..OPPONENT_STORE].iter().sum();
        self.pits[PLAYER_STORE] += player_remaining;
        self.pits[OPPONENT_STORE] += opponent_remaining;
        self.pits[..PLAYER_STORE].fill(0);
        self.pits[OPPONENT_START..OPPONENT_STORE].fill(0);
        self.phase = if self.pits[PLAYER_STORE] > self.pits[OPPONENT_STORE] {
            MancalaPhase::Won
        } else {
            MancalaPhase::Lost
        };
    }
}

#[cfg(test)]
mod tests;
