//! Deterministic touch-first Peg Solitaire on the classic 33-hole cross board.

use serde::{Deserialize, Serialize};

const SIZE: usize = 7;
const CELLS: usize = SIZE * SIZE;
const CENTER: usize = 3 * SIZE + 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hole {
    Empty,
    Peg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PegSolitaireStatus {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PegVariant {
    #[default]
    Classic,
    Corner,
}

impl PegVariant {
    pub const ALL: [Self; 2] = [Self::Classic, Self::Corner];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · CENTER FINISH",
            Self::Corner => "CORNER · OFFSET FINISH",
        }
    }

    const fn starting_empty(self) -> usize {
        match self {
            Self::Classic => CENTER,
            Self::Corner => CENTER,
        }
    }

    const fn winning_hole(self) -> usize {
        match self {
            Self::Classic => CENTER,
            Self::Corner => 2 * SIZE + 2,
        }
    }
}

type Snapshot = (Vec<Hole>, PegSolitaireStatus, u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PegSolitaire {
    pub cells: Vec<Hole>,
    pub status: PegSolitaireStatus,
    pub moves: u16,
    pub seed: u64,
    pub selected: Option<usize>,
    #[serde(default)]
    pub variant: PegVariant,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for PegSolitaire {
    fn default() -> Self {
        Self::new(0x0FAC_E733)
    }
}

impl PegSolitaire {
    pub fn new(seed: u64) -> Self {
        Self::new_with_variant(seed, PegVariant::default())
    }

    pub fn new_with_variant(seed: u64, variant: PegVariant) -> Self {
        let mut cells = vec![Hole::Empty; CELLS];
        for (index, cell) in cells.iter_mut().enumerate() {
            if valid_hole(index) {
                *cell = Hole::Peg;
            }
        }
        cells[variant.starting_empty()] = Hole::Empty;
        Self {
            cells,
            status: PegSolitaireStatus::Playing,
            moves: 0,
            seed,
            selected: None,
            variant,
            undo: None,
        }
    }

    pub fn tap(&mut self, square: usize) -> bool {
        if square >= CELLS || !valid_hole(square) || self.status != PegSolitaireStatus::Playing {
            return false;
        }
        if let Some(from) = self.selected {
            if self.move_peg(from, square) {
                return true;
            }
            if self.cells[square] == Hole::Peg {
                self.selected = Some(square);
                return true;
            }
            return false;
        }
        if self.cells[square] == Hole::Peg {
            self.selected = Some(square);
            return true;
        }
        false
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, status, moves)) = self.undo.take() {
            self.cells = cells;
            self.status = status;
            self.moves = moves;
            self.selected = None;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.status != PegSolitaireStatus::Playing {
            return None;
        }
        (0..CELLS).find_map(|from| self.targets(from).into_iter().next().map(|to| (from, to)))
    }

    pub fn targets(&self, from: usize) -> Vec<usize> {
        if from >= CELLS || !valid_hole(from) || self.cells[from] != Hole::Peg {
            return Vec::new();
        }
        let row = from / SIZE;
        let column = from % SIZE;
        let mut targets = Vec::new();
        for (row_step, column_step) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let middle_row = row as isize + row_step;
            let middle_column = column as isize + column_step;
            let target_row = row as isize + row_step * 2;
            let target_column = column as isize + column_step * 2;
            if !(0..SIZE as isize).contains(&target_row)
                || !(0..SIZE as isize).contains(&target_column)
                || !(0..SIZE as isize).contains(&middle_row)
                || !(0..SIZE as isize).contains(&middle_column)
            {
                continue;
            }
            let middle = middle_row as usize * SIZE + middle_column as usize;
            let target = target_row as usize * SIZE + target_column as usize;
            if valid_hole(target)
                && self.cells[middle] == Hole::Peg
                && self.cells[target] == Hole::Empty
            {
                targets.push(target);
            }
        }
        targets
    }

    fn move_peg(&mut self, from: usize, to: usize) -> bool {
        if !self.targets(from).contains(&to) {
            return false;
        }
        self.undo = Some((self.cells.clone(), self.status, self.moves));
        let middle = (from / SIZE + to / SIZE) / 2 * SIZE + (from % SIZE + to % SIZE) / 2;
        self.cells[from] = Hole::Empty;
        self.cells[middle] = Hole::Empty;
        self.cells[to] = Hole::Peg;
        self.moves = self.moves.saturating_add(1);
        self.selected = None;
        self.resolve();
        true
    }

    fn resolve(&mut self) {
        let pegs = self.cells.iter().filter(|hole| **hole == Hole::Peg).count();
        if pegs == 1 && self.cells[self.variant.winning_hole()] == Hole::Peg {
            self.status = PegSolitaireStatus::Won;
        } else if !(0..CELLS).any(|from| !self.targets(from).is_empty()) {
            self.status = PegSolitaireStatus::Stuck;
        }
    }
}

pub fn valid_hole(index: usize) -> bool {
    let row = index / SIZE;
    let column = index % SIZE;
    row < SIZE && column < SIZE && ((2..=4).contains(&row) || (2..=4).contains(&column))
}

#[cfg(test)]
mod tests;
