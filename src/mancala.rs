//! Deterministic Kalah-style Mancala with a quiet local opponent.

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mancala {
    pub pits: Vec<u8>,
    pub moves: u16,
    pub seed: u64,
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
        let mut pits = vec![4; 14];
        pits[PLAYER_STORE] = 0;
        pits[OPPONENT_STORE] = 0;
        Self {
            pits,
            moves: 0,
            seed,
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
        let last = self.sow(pit, true);
        self.moves = self.moves.saturating_add(1);
        if self.side_empty(true) {
            self.finish();
        } else if last != PLAYER_STORE {
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
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == MancalaPhase::Won
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    fn sow(&mut self, pit: usize, player: bool) -> usize {
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
        if index >= start && index < end && self.pits[index] == 1 {
            let opposite = 12 - index;
            if self.pits[opposite] > 0 {
                self.pits[store] += self.pits[opposite] + 1;
                self.pits[opposite] = 0;
                self.pits[index] = 0;
            }
        }
        index
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
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let pit = available[(self.seed as usize) % available.len()];
            let last = self.sow(pit, false);
            if self.side_empty(false) {
                self.finish();
                return;
            }
            if last != OPPONENT_STORE {
                return;
            }
        }
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
