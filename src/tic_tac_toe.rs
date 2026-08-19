//! Deterministic touch-first Tic-Tac-Toe with a bounded local opponent.

use serde::{Deserialize, Serialize};

const CELLS: usize = 9;
const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mark {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicTacToeStatus {
    Playing,
    Won(Mark),
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiLevel {
    Gentle,
    Sharp,
    Expert,
}

fn default_ai_level() -> AiLevel {
    AiLevel::Sharp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicTacToe {
    pub cells: [Mark; CELLS],
    pub status: TicTacToeStatus,
    pub moves: u8,
    pub seed: u64,
    #[serde(default = "default_ai_level")]
    pub ai_level: AiLevel,
    #[serde(skip)]
    undo: Option<([Mark; CELLS], TicTacToeStatus, u8, u64)>,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self::new(0x1D1E_3AC3)
    }
}

impl TicTacToe {
    pub fn new(seed: u64) -> Self {
        Self {
            cells: [Mark::Empty; CELLS],
            status: TicTacToeStatus::Playing,
            moves: 0,
            seed,
            ai_level: AiLevel::Sharp,
            undo: None,
        }
    }

    pub fn place(&mut self, index: usize) -> bool {
        if index >= CELLS || self.status != TicTacToeStatus::Playing {
            return false;
        }
        if self.cells[index] != Mark::Empty {
            return false;
        }
        self.undo = Some((self.cells, self.status, self.moves, self.seed));
        self.cells[index] = Mark::X;
        self.moves += 1;
        self.resolve();
        if self.status == TicTacToeStatus::Playing {
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

    pub fn hint_move(&self) -> Option<usize> {
        if self.status != TicTacToeStatus::Playing {
            return None;
        }
        self.winning_move(Mark::X)
            .or_else(|| self.winning_move(Mark::O))
            .or_else(|| (self.cells[4] == Mark::Empty).then_some(4))
            .or_else(|| {
                [0, 2, 6, 8]
                    .into_iter()
                    .find(|&index| self.cells[index] == Mark::Empty)
            })
            .or_else(|| self.cells.iter().position(|mark| *mark == Mark::Empty))
    }

    fn ai_move(&mut self) {
        let index = match self.ai_level {
            AiLevel::Gentle => self.cells.iter().position(|mark| *mark == Mark::Empty),
            AiLevel::Sharp => self
                .winning_move(Mark::O)
                .or_else(|| self.winning_move(Mark::X))
                .or_else(|| (self.cells[4] == Mark::Empty).then_some(4))
                .or_else(|| {
                    [0, 2, 6, 8]
                        .into_iter()
                        .find(|&index| self.cells[index] == Mark::Empty)
                })
                .or_else(|| self.cells.iter().position(|mark| *mark == Mark::Empty)),
            AiLevel::Expert => self.expert_move(),
        };
        if let Some(index) = index {
            self.cells[index] = Mark::O;
            self.moves += 1;
            self.resolve();
        }
    }

    fn winning_move(&self, mark: Mark) -> Option<usize> {
        self.cells.iter().enumerate().find_map(|(index, cell)| {
            if *cell != Mark::Empty {
                return None;
            }
            let mut cells = self.cells;
            cells[index] = mark;
            has_won(&cells, mark).then_some(index)
        })
    }

    fn expert_move(&self) -> Option<usize> {
        let mut best = None;
        let mut best_score = i32::MIN;
        for index in 0..CELLS {
            if self.cells[index] != Mark::Empty {
                continue;
            }
            let mut cells = self.cells;
            cells[index] = Mark::O;
            let score = minimax(&mut cells, Mark::X, 0);
            if score > best_score {
                best_score = score;
                best = Some(index);
            }
        }
        best
    }

    fn resolve(&mut self) {
        if let Some(mark) = [Mark::X, Mark::O]
            .into_iter()
            .find(|&mark| has_won(&self.cells, mark))
        {
            self.status = TicTacToeStatus::Won(mark);
        } else if self.cells.iter().all(|mark| *mark != Mark::Empty) {
            self.status = TicTacToeStatus::Draw;
        }
    }
}

fn minimax(cells: &mut [Mark; CELLS], turn: Mark, depth: i32) -> i32 {
    if has_won(cells, Mark::O) {
        return 10 - depth;
    }
    if has_won(cells, Mark::X) {
        return depth - 10;
    }
    if cells.iter().all(|mark| *mark != Mark::Empty) {
        return 0;
    }
    let maximizing = turn == Mark::O;
    let mut score = if maximizing { i32::MIN } else { i32::MAX };
    for index in 0..CELLS {
        if cells[index] != Mark::Empty {
            continue;
        }
        cells[index] = turn;
        let next = minimax(cells, if maximizing { Mark::X } else { Mark::O }, depth + 1);
        cells[index] = Mark::Empty;
        score = if maximizing {
            score.max(next)
        } else {
            score.min(next)
        };
    }
    score
}

fn has_won(cells: &[Mark; CELLS], mark: Mark) -> bool {
    LINES
        .iter()
        .any(|line| line.iter().all(|&index| cells[index] == mark))
}

#[cfg(test)]
mod tests;
