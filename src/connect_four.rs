//! Deterministic touch-first Connect Four with a bounded local opponent.

use serde::{Deserialize, Serialize};

pub const COLUMNS: usize = 7;
pub const ROWS: usize = 6;
pub const CELLS: usize = COLUMNS * ROWS;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiLevel {
    Gentle,
    Sharp,
    Expert,
}

pub fn default_ai_level() -> AiLevel {
    AiLevel::Sharp
}

pub type Snapshot = (Vec<Disc>, ConnectFourStatus, u8, u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectFour {
    pub cells: Vec<Disc>,
    pub status: ConnectFourStatus,
    pub moves: u8,
    pub seed: u64,
    #[serde(default = "default_ai_level")]
    pub ai_level: AiLevel,
    #[serde(skip)]
    pub undo: Option<Snapshot>,
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
            ai_level: AiLevel::Sharp,
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
        let ai_level = self.ai_level;
        *self = Self::new(seed);
        self.ai_level = ai_level;
    }

    pub fn set_ai_level(&mut self, ai_level: AiLevel) {
        self.ai_level = ai_level;
    }

    pub fn hint_column(&self) -> Option<usize> {
        if self.status != ConnectFourStatus::Playing {
            return None;
        }
        self.winning_column(Disc::Red)
            .or_else(|| self.winning_column(Disc::Yellow))
            .or_else(|| (!self.column_full(3)).then_some(3))
            .or_else(|| {
                [3, 2, 4, 1, 5, 0, 6]
                    .into_iter()
                    .find(|&column| !self.column_full(column))
            })
    }

    pub fn column_full(&self, column: usize) -> bool {
        column >= COLUMNS || self.cells[column] != Disc::Empty
    }

    pub fn place(&mut self, column: usize, disc: Disc) -> bool {
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

    pub fn ai_move(&mut self) {
        let column = match self.ai_level {
            AiLevel::Gentle => (0..COLUMNS).find(|&column| !self.column_full(column)),
            AiLevel::Sharp => self
                .winning_column(Disc::Yellow)
                .or_else(|| self.winning_column(Disc::Red))
                .or_else(|| (!self.column_full(3)).then_some(3))
                .or_else(|| {
                    [3, 2, 4, 1, 5, 0, 6]
                        .into_iter()
                        .find(|&column| !self.column_full(column))
                }),
            AiLevel::Expert => self.expert_column(),
        };
        if let Some(column) = column {
            self.place(column, Disc::Yellow);
        }
    }

    pub fn winning_column(&self, disc: Disc) -> Option<usize> {
        (0..COLUMNS).find(|&column| {
            if self.column_full(column) {
                return false;
            }
            let mut cells = self.cells.clone();
            // `column_full` above proves that this bounded column has a slot.
            let row = (0..ROWS)
                .rev()
                .find(|row| cells[row * COLUMNS + column] == Disc::Empty)
                .unwrap();
            cells[row * COLUMNS + column] = disc;
            has_four(&cells, disc)
        })
    }

    pub fn resolve(&mut self, disc: Disc) {
        if has_four(&self.cells, disc) {
            self.status = ConnectFourStatus::Won(disc);
        } else if self.cells.iter().all(|cell| *cell != Disc::Empty) {
            self.status = ConnectFourStatus::Draw;
        }
    }

    pub fn expert_column(&self) -> Option<usize> {
        let mut best = None;
        let mut best_score = i32::MIN;
        for column in 0..COLUMNS {
            let Some(cells) = drop_disc(&self.cells, column, Disc::Yellow) else {
                continue;
            };
            let score = connect_four_minimax(&cells, Disc::Red, 3);
            if score > best_score {
                best_score = score;
                best = Some(column);
            }
        }
        best
    }
}

pub fn drop_disc(cells: &[Disc], column: usize, disc: Disc) -> Option<Vec<Disc>> {
    if column >= COLUMNS {
        return None;
    }
    let mut next = cells.to_vec();
    let row = (0..ROWS)
        .rev()
        .find(|row| next[row * COLUMNS + column] == Disc::Empty)?;
    next[row * COLUMNS + column] = disc;
    Some(next)
}

pub fn connect_four_minimax(cells: &[Disc], turn: Disc, depth: u8) -> i32 {
    if has_four(cells, Disc::Yellow) {
        return 10_000 + i32::from(depth);
    }
    if has_four(cells, Disc::Red) {
        return -10_000 - i32::from(depth);
    }
    if depth == 0 || cells.iter().all(|cell| *cell != Disc::Empty) {
        return board_value(cells);
    }
    let maximizing = turn == Disc::Yellow;
    let mut score = if maximizing { i32::MIN } else { i32::MAX };
    for column in [3, 2, 4, 1, 5, 0, 6] {
        let Some(next) = drop_disc(cells, column, turn) else {
            continue;
        };
        let value = connect_four_minimax(
            &next,
            if maximizing { Disc::Red } else { Disc::Yellow },
            depth - 1,
        );
        score = if maximizing {
            score.max(value)
        } else {
            score.min(value)
        };
    }
    score
}

pub fn board_value(cells: &[Disc]) -> i32 {
    cells
        .iter()
        .enumerate()
        .map(|(index, disc)| match disc {
            Disc::Yellow => 4 - (index % COLUMNS).abs_diff(3) as i32,
            Disc::Red => -(4 - (index % COLUMNS).abs_diff(3) as i32),
            Disc::Empty => 0,
        })
        .sum()
}

pub fn has_four(cells: &[Disc], disc: Disc) -> bool {
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
