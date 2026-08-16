//! Deterministic touch-first Checkers with mandatory captures and a bounded reply.

use serde::{Deserialize, Serialize};

const SIZE: usize = 8;
const CELLS: usize = SIZE * SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Red,
    Yellow,
}

impl Side {
    fn other(self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Piece {
    Empty,
    RedMan,
    RedKing,
    YellowMan,
    YellowKing,
}

impl Piece {
    fn side(self) -> Option<Side> {
        match self {
            Self::RedMan | Self::RedKing => Some(Side::Red),
            Self::YellowMan | Self::YellowKing => Some(Side::Yellow),
            Self::Empty => None,
        }
    }

    fn is_king(self) -> bool {
        matches!(self, Self::RedKing | Self::YellowKing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckersStatus {
    Playing,
    Won(Side),
    Draw,
}

type Snapshot = (Vec<Piece>, Side, CheckersStatus, u16, u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkers {
    pub cells: Vec<Piece>,
    pub turn: Side,
    pub status: CheckersStatus,
    pub moves: u16,
    pub seed: u64,
    pub selected: Option<usize>,
    #[serde(default)]
    forced_capture: Option<usize>,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for Checkers {
    fn default() -> Self {
        Self::new(0xC1_EC_7E)
    }
}

impl Checkers {
    pub fn new(seed: u64) -> Self {
        let mut cells = vec![Piece::Empty; CELLS];
        for row in 0..3 {
            for column in 0..SIZE {
                if (row + column) % 2 == 1 {
                    cells[row * SIZE + column] = Piece::YellowMan;
                }
                if ((SIZE - 1 - row) + column) % 2 == 1 {
                    cells[(SIZE - 1 - row) * SIZE + column] = Piece::RedMan;
                }
            }
        }
        Self {
            cells,
            turn: Side::Red,
            status: CheckersStatus::Playing,
            moves: 0,
            seed,
            selected: None,
            forced_capture: None,
            undo: None,
        }
    }

    pub fn tap(&mut self, square: usize) -> bool {
        if square >= CELLS || self.status != CheckersStatus::Playing {
            return false;
        }
        if let Some(from) = self.selected {
            if self.try_move(from, square) {
                return true;
            }
            if self.forced_capture.is_none() && self.cells[square].side() == Some(self.turn) {
                self.selected = Some(square);
                self.forced_capture = None;
                return true;
            }
            return false;
        }
        if self.cells[square].side() == Some(self.turn) && !self.targets(square).is_empty() {
            self.selected = Some(square);
            self.forced_capture = None;
            return true;
        }
        false
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, turn, status, moves, seed)) = self.undo.take() {
            self.cells = cells;
            self.turn = turn;
            self.status = status;
            self.moves = moves;
            self.seed = seed;
            self.selected = None;
            self.forced_capture = None;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn hint_move(&self) -> Option<(usize, usize)> {
        if self.status != CheckersStatus::Playing || self.turn != Side::Red {
            return None;
        }
        let mut fallback = None;
        for from in 0..CELLS {
            if self.cells[from].side() != Some(Side::Red) {
                continue;
            }
            let targets = self.targets(from);
            if targets.is_empty() {
                continue;
            }
            let capture = (from / SIZE).abs_diff(targets[0] / SIZE) == 2;
            if capture {
                return Some((from, targets[0]));
            }
            fallback.get_or_insert((from, targets[0]));
        }
        fallback
    }

    pub fn targets(&self, from: usize) -> Vec<usize> {
        if from >= CELLS || self.cells[from].side() != Some(self.turn) {
            return Vec::new();
        }
        let captures = self.capture_targets(from);
        if !captures.is_empty() || self.capture_available_for(self.turn) {
            captures
        } else {
            self.step_targets(from)
        }
    }

    fn try_move(&mut self, from: usize, to: usize) -> bool {
        if !self.targets(from).contains(&to) {
            return false;
        }
        let snapshot = self.undo.get_or_insert_with(|| {
            (
                self.cells.clone(),
                self.turn,
                self.status,
                self.moves,
                self.seed,
            )
        });
        let _ = snapshot;
        let capture = self.apply_move(from, to);
        if capture && !self.capture_targets(to).is_empty() {
            self.selected = Some(to);
            self.forced_capture = Some(to);
            return true;
        }
        self.selected = None;
        self.forced_capture = None;
        self.turn = self.turn.other();
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        if self.status == CheckersStatus::Playing && self.turn == Side::Yellow {
            self.ai_turn();
        }
        true
    }

    fn apply_move(&mut self, from: usize, to: usize) -> bool {
        let piece = self.cells[from];
        let from_row = from / SIZE;
        let to_row = to / SIZE;
        let capture = from_row.abs_diff(to_row) == 2;
        self.cells[from] = Piece::Empty;
        self.cells[to] = piece;
        if capture {
            let middle = ((from_row + to_row) / 2) * SIZE + ((from % SIZE + to % SIZE) / 2);
            self.cells[middle] = Piece::Empty;
        }
        if to_row == 0 && piece == Piece::RedMan {
            self.cells[to] = Piece::RedKing;
        } else if to_row == SIZE - 1 && piece == Piece::YellowMan {
            self.cells[to] = Piece::YellowKing;
        }
        capture
    }

    fn ai_turn(&mut self) {
        while let Some((from, to)) = self.best_ai_move() {
            let capture = self.apply_move(from, to);
            self.moves = self.moves.saturating_add(1);
            self.resolve();
            if self.status != CheckersStatus::Playing
                || !capture
                || self.capture_targets(to).is_empty()
            {
                break;
            }
        }
        if self.status == CheckersStatus::Playing {
            self.turn = Side::Red;
            self.resolve();
        }
    }

    fn best_ai_move(&self) -> Option<(usize, usize)> {
        let mut candidates = Vec::new();
        for from in 0..CELLS {
            if self.cells[from].side() == Some(Side::Yellow) {
                for to in self.targets_for(Side::Yellow, from) {
                    candidates.push((from, to));
                }
            }
        }
        candidates.sort_by_key(|&(from, to)| {
            let capture = (from / SIZE).abs_diff(to / SIZE) == 2;
            (!capture, to.abs_diff(28), to)
        });
        candidates.into_iter().next()
    }

    fn targets_for(&self, side: Side, from: usize) -> Vec<usize> {
        if from >= CELLS || self.cells[from].side() != Some(side) {
            return Vec::new();
        }
        let captures = self.capture_targets_for(side, from);
        if !captures.is_empty() || self.capture_available_for(side) {
            captures
        } else {
            self.step_targets_for(side, from)
        }
    }

    fn capture_available_for(&self, side: Side) -> bool {
        (0..CELLS).any(|from| {
            self.cells[from].side() == Some(side)
                && !self.capture_targets_for(side, from).is_empty()
        })
    }

    fn capture_targets(&self, from: usize) -> Vec<usize> {
        self.capture_targets_for(self.turn, from)
    }

    fn capture_targets_for(&self, side: Side, from: usize) -> Vec<usize> {
        self.targets_in_directions(side, from, true)
    }

    fn step_targets(&self, from: usize) -> Vec<usize> {
        self.step_targets_for(self.turn, from)
    }

    fn step_targets_for(&self, side: Side, from: usize) -> Vec<usize> {
        self.targets_in_directions(side, from, false)
    }

    fn targets_in_directions(&self, side: Side, from: usize, capture: bool) -> Vec<usize> {
        let piece = self.cells[from];
        let row = from / SIZE;
        let column = from % SIZE;
        let mut targets = Vec::new();
        let directions = if piece.is_king() {
            vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]
        } else if side == Side::Red {
            vec![(-1, -1), (-1, 1)]
        } else {
            vec![(1, -1), (1, 1)]
        };
        for (row_step, column_step) in directions {
            let target_row = row as isize + if capture { row_step * 2 } else { row_step };
            let target_column = column as isize
                + if capture {
                    column_step * 2
                } else {
                    column_step
                };
            if !(0..SIZE as isize).contains(&target_row)
                || !(0..SIZE as isize).contains(&target_column)
            {
                continue;
            }
            let target = target_row as usize * SIZE + target_column as usize;
            if self.cells[target] != Piece::Empty {
                continue;
            }
            if capture {
                let middle = (row as isize + row_step) as usize * SIZE
                    + (column as isize + column_step) as usize;
                if self.cells[middle].side() == Some(side.other()) {
                    targets.push(target);
                }
            } else {
                targets.push(target);
            }
        }
        targets
    }

    fn resolve(&mut self) {
        let red = self
            .cells
            .iter()
            .any(|piece| piece.side() == Some(Side::Red));
        let yellow = self
            .cells
            .iter()
            .any(|piece| piece.side() == Some(Side::Yellow));
        if !red {
            self.status = CheckersStatus::Won(Side::Yellow);
        } else if !yellow {
            self.status = CheckersStatus::Won(Side::Red);
        } else if self.moves >= 200 {
            self.status = CheckersStatus::Draw;
        } else if !self.has_any_move(self.turn) {
            self.status = CheckersStatus::Won(self.turn.other());
        }
    }

    fn has_any_move(&self, side: Side) -> bool {
        (0..CELLS).any(|from| {
            self.cells[from].side() == Some(side) && !self.targets_for(side, from).is_empty()
        })
    }
}

#[cfg(test)]
mod tests;
