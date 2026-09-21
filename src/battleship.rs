//! Deterministic touch-first Battleship hunt.
use crate::undo::UndoStack;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattleshipFleet {
    #[default]
    Patrol,
    Armada,
}

impl BattleshipFleet {
    pub const ALL: [Self; 2] = [Self::Patrol, Self::Armada];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Patrol => "PATROL · 3 SHIPS",
            Self::Armada => "ARMADA · 4 SHIPS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battleship {
    pub ships: Vec<u8>,
    pub shots: Vec<Shot>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub streak: u16,
    #[serde(default)]
    pub best_streak: u16,
    #[serde(default)]
    pub score: u32,
    #[serde(default = "default_sonar_charges")]
    pub sonar_charges: u8,
    #[serde(default)]
    pub sonar_armed: bool,
    #[serde(default)]
    pub scanned: Vec<bool>,
    pub phase: BattleshipPhase,
    #[serde(default)]
    pub fleet: BattleshipFleet,
    #[serde(skip)]
    pub history: UndoStack<Self>,
}

impl Default for Battleship {
    fn default() -> Self {
        Self::new(0x0000_4241_5454_4C45)
    }
}

impl Battleship {
    pub fn new(seed: u64) -> Self {
        Self::new_with_fleet(seed, BattleshipFleet::default())
    }

    pub fn new_with_fleet(seed: u64, fleet: BattleshipFleet) -> Self {
        let mut ships = vec![0; CELLS];
        let layouts = [
            ([1, 2, 3], [14, 20], [28, 29]),
            ([7, 13, 19], [29, 30], [3, 4]),
            ([4, 10, 16], [27, 33], [19, 20]),
            ([31, 32, 33], [8, 14], [17, 23]),
        ];
        let (long_ship, scout_ship, cutter_ship) = layouts[(seed as usize) % layouts.len()];
        for cell in long_ship {
            ships[cell] = 1;
        }
        for cell in scout_ship {
            ships[cell] = 2;
        }
        for cell in cutter_ship {
            ships[cell] = 3;
        }
        if fleet == BattleshipFleet::Armada {
            let escort = match (seed as usize) % layouts.len() {
                0 => [35, 36],
                1 => [32, 33],
                2 => [34, 35],
                _ => [26, 27],
            };
            for cell in escort {
                if ships[cell] == 0 {
                    ships[cell] = 4;
                }
            }
        }
        Self {
            ships,
            shots: vec![Shot::Unknown; CELLS],
            moves: 0,
            seed,
            streak: 0,
            best_streak: 0,
            score: 0,
            sonar_charges: default_sonar_charges(),
            sonar_armed: false,
            scanned: vec![false; CELLS],
            phase: BattleshipPhase::Playing,
            fleet,
            history: UndoStack::default(),
        }
    }

    pub fn fire(&mut self, cell: usize) -> bool {
        if self.sonar_armed {
            return self.scan(cell);
        }
        if self.phase != BattleshipPhase::Playing
            || cell >= CELLS
            || self.shots[cell] != Shot::Unknown
        {
            return false;
        }
        let previous = self.clone_without_history();
        let ship = self.ships[cell];
        self.shots[cell] = if ship == 0 { Shot::Miss } else { Shot::Hit };
        self.moves = self.moves.saturating_add(1);
        if ship == 0 {
            self.streak = 0;
        } else {
            self.streak = self.streak.saturating_add(1);
            self.best_streak = self.best_streak.max(self.streak);
            self.score = self
                .score
                .saturating_add(10_u32.saturating_mul(u32::from(self.streak)));
            if self.ship_sunk(ship) {
                self.score = self.score.saturating_add(25);
            }
        }
        if self
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| **ship != 0)
            .all(|(index, _)| self.shots[index] == Shot::Hit)
        {
            self.phase = BattleshipPhase::Won;
        }
        self.history.push(previous);
        true
    }

    pub fn toggle_sonar(&mut self) -> bool {
        if self.phase != BattleshipPhase::Playing || self.sonar_charges == 0 {
            return false;
        }
        self.sonar_armed = !self.sonar_armed;
        true
    }

    pub fn scan(&mut self, cell: usize) -> bool {
        if cell >= CELLS || self.sonar_charges == 0 {
            return false;
        }
        let previous = self.clone_without_history();
        self.ensure_scanned_shape();
        let row = cell / SIDE;
        let col = cell % SIDE;
        for next_row in row.saturating_sub(1)..=(row + 1).min(SIDE - 1) {
            for next_col in col.saturating_sub(1)..=(col + 1).min(SIDE - 1) {
                self.scanned[next_row * SIDE + next_col] = true;
            }
        }
        self.sonar_charges = self.sonar_charges.saturating_sub(1);
        self.sonar_armed = false;
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
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == BattleshipPhase::Won
    }

    pub fn hits(&self) -> usize {
        self.shots.iter().filter(|shot| **shot == Shot::Hit).count()
    }

    pub fn ship_cells(&self) -> usize {
        self.ships.iter().filter(|ship| **ship != 0).count()
    }

    pub fn ship_count(&self) -> usize {
        self.ships.iter().copied().max().unwrap_or(0) as usize
    }

    pub fn sunk_ships(&self) -> usize {
        (1..=self.ship_count() as u8)
            .filter(|&ship| self.ship_sunk(ship))
            .count()
    }

    pub fn ship_sunk(&self, ship: u8) -> bool {
        ship != 0
            && self
                .ships
                .iter()
                .enumerate()
                .filter(|(_, value)| **value == ship)
                .all(|(index, _)| self.shots[index] == Shot::Hit)
    }

    pub fn is_scanned(&self, cell: usize) -> bool {
        self.scanned.get(cell).copied().unwrap_or(false)
    }

    pub fn contact_count(&self) -> usize {
        (0..CELLS)
            .filter(|&cell| {
                self.is_scanned(cell) && self.shots[cell] == Shot::Unknown && self.ships[cell] != 0
            })
            .count()
    }

    pub fn hint_cell(&self) -> Option<usize> {
        if self.phase != BattleshipPhase::Playing {
            return None;
        }
        if let Some(contact) = (0..CELLS).find(|&cell| {
            self.is_scanned(cell) && self.ships[cell] != 0 && self.shots[cell] == Shot::Unknown
        }) {
            return Some(contact);
        }
        for index in 0..CELLS {
            if self.shots[index] != Shot::Hit {
                continue;
            }
            let row = index / SIDE;
            let col = index % SIDE;
            for neighbor in [
                (col + 1 < SIDE).then_some(row * SIDE + col + 1),
                (row + 1 < SIDE).then_some((row + 1) * SIDE + col),
                col.checked_sub(1).map(|next| row * SIDE + next),
                row.checked_sub(1).map(|next| next * SIDE + col),
            ]
            .into_iter()
            .flatten()
            {
                if self.shots[neighbor] == Shot::Unknown {
                    return Some(neighbor);
                }
            }
        }
        (0..CELLS)
            .find(|&index| {
                (index / SIDE + index % SIDE).is_multiple_of(2)
                    && self.shots[index] == Shot::Unknown
            })
            .or_else(|| self.shots.iter().position(|shot| *shot == Shot::Unknown))
    }

    pub fn ensure_scanned_shape(&mut self) {
        if self.scanned.len() != CELLS {
            self.scanned.resize(CELLS, false);
        }
    }

    pub fn clone_without_history(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

pub const fn default_sonar_charges() -> u8 {
    2
}
