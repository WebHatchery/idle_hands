//! Deterministic touch-first Battleship hunt.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 6;
pub const CELLS: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shot {
    Unknown,
    Miss,
    Hit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleshipPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battleship {
    pub ships: Vec<u8>,
    pub shots: Vec<Shot>,
    pub moves: u16,
    pub seed: u64,
    pub phase: BattleshipPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for Battleship {
    fn default() -> Self {
        Self::new(0x0000_4241_5454_4C45)
    }
}

impl Battleship {
    pub fn new(seed: u64) -> Self {
        let mut ships = vec![0; CELLS];
        let layouts = [
            ([1, 2, 3], [14, 20]),
            ([7, 13, 19], [29, 30]),
            ([4, 10, 16], [27, 33]),
            ([31, 32, 33], [8, 14]),
        ];
        let (long_ship, scout_ship) = layouts[(seed as usize) % layouts.len()];
        for cell in long_ship {
            ships[cell] = 1;
        }
        for cell in scout_ship {
            ships[cell] = 2;
        }
        Self {
            ships,
            shots: vec![Shot::Unknown; CELLS],
            moves: 0,
            seed,
            phase: BattleshipPhase::Playing,
            undo: None,
        }
    }

    pub fn fire(&mut self, cell: usize) -> bool {
        if self.phase != BattleshipPhase::Playing
            || cell >= CELLS
            || self.shots[cell] != Shot::Unknown
        {
            return false;
        }
        let previous = self.clone_without_undo();
        self.shots[cell] = if self.ships[cell] == 0 {
            Shot::Miss
        } else {
            Shot::Hit
        };
        self.moves = self.moves.saturating_add(1);
        self.undo = Some(Box::new(previous));
        if self
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| **ship != 0)
            .all(|(index, _)| self.shots[index] == Shot::Hit)
        {
            self.phase = BattleshipPhase::Won;
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
        self.phase == BattleshipPhase::Won
    }

    pub fn hits(&self) -> usize {
        self.shots.iter().filter(|shot| **shot == Shot::Hit).count()
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }
}

#[cfg(test)]
mod tests;
