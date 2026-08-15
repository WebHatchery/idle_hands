//! Deterministic touch-first Connect Four with a bounded local opponent.

use serde::{Deserialize, Serialize};

const COLUMNS: usize = 7;
const ROWS: usize = 6;
const CELLS: usize = COLUMNS * ROWS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disc {
    Empty,
    Red,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectFourStatus {
    Playing,
    Won(Disc),
    Draw,
}

type Snapshot = (Vec<Disc>, ConnectFourStatus, u8, u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectFour {
    pub cells: Vec<Disc>,
    pub status: ConnectFourStatus,
    pub moves: u8,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for ConnectFour {
    fn default() -> Self {
        Self::new(0xC0_4F_00)
    }
}

impl ConnectFour {
    pub fn new(seed: u64) -> Self {
        Self {
            cells: vec![Disc::Empty; CELLS],
            status: ConnectFourStatus::Playing,
            moves: 0,
            seed,
            undo: None,
        }
    }

    pub fn drop(&mut self, column: usize) -> bool {
        if self.status != ConnectFourStatus::Playing || !self.place(column, Disc::Red) {
            return false;
        }
        if self.status == ConnectFourStatus::Playing {
            self.ai_move();
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, status, moves, seed)) = self.undo.take() {
            self.cells = cells;
            self.status = status;
            self.moves = moves;
            self.seed = seed;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn column_full(&self, column: usize) -> bool {
        column >= COLUMNS || self.cells[column] != Disc::Empty
    }

    fn place(&mut self, column: usize, disc: Disc) -> bool {
        if column >= COLUMNS {
            return false;
        }
        let Some(row) = (0..ROWS)
            .rev()
            .find(|row| self.cells[row * COLUMNS + column] == Disc::Empty)
        else {
            return false;
        };
        if self.undo.is_none() && disc == Disc::Red {
            self.undo = Some((self.cells.clone(), self.status, self.moves, self.seed));
        }
        self.cells[row * COLUMNS + column] = disc;
        self.moves += 1;
        self.resolve(disc);
        true
    }

    fn ai_move(&mut self) {
        let column = self
            .winning_column(Disc::Yellow)
            .or_else(|| self.winning_column(Disc::Red))
            .or_else(|| (!self.column_full(3)).then_some(3))
            .or_else(|| {
                [3, 2, 4, 1, 5, 0, 6]
                    .into_iter()
                    .find(|&column| !self.column_full(column))
            });
        if let Some(column) = column {
            self.place(column, Disc::Yellow);
        }
    }

    fn winning_column(&self, disc: Disc) -> Option<usize> {
        (0..COLUMNS).find(|&column| {
            if self.column_full(column) {
                return false;
            }
            let mut cells = self.cells.clone();
            let row = (0..ROWS)
                .rev()
                .find(|row| cells[row * COLUMNS + column] == Disc::Empty)
                .unwrap();
            cells[row * COLUMNS + column] = disc;
            has_four(&cells, disc)
        })
    }

    fn resolve(&mut self, disc: Disc) {
        if has_four(&self.cells, disc) {
            self.status = ConnectFourStatus::Won(disc);
        } else if self.cells.iter().all(|cell| *cell != Disc::Empty) {
            self.status = ConnectFourStatus::Draw;
        }
    }
}

fn has_four(cells: &[Disc], disc: Disc) -> bool {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            for (row_step, column_step) in [(0isize, 1isize), (1, 0), (1, 1), (1, -1)] {
                if (0..4).all(|offset| {
                    let target_row = row as isize + row_step * offset;
                    let target_column = column as isize + column_step * offset;
                    target_row >= 0
                        && target_row < ROWS as isize
                        && target_column >= 0
                        && target_column < COLUMNS as isize
                        && cells[target_row as usize * COLUMNS + target_column as usize] == disc
                }) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests;
