//! Deterministic Reversi rules with a bounded local opponent.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiLevel {
    Gentle,
    Sharp,
    TwoPlayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReversiStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reversi {
    pub board: Vec<u8>,
    pub turn: u8,
    pub passes: u8,
    pub status: ReversiStatus,
    pub winner: Option<u8>,
    pub ai_level: AiLevel,
    pub seed: u64,
    pub moves: u32,
}

impl Default for Reversi {
    fn default() -> Self {
        Self::new(0xAE_7E_01, AiLevel::Gentle)
    }
}

impl Reversi {
    pub fn new(seed: u64, ai_level: AiLevel) -> Self {
        let mut board = vec![0; 64];
        board[3 * 8 + 3] = 2;
        board[4 * 8 + 4] = 2;
        board[3 * 8 + 4] = 1;
        board[4 * 8 + 3] = 1;
        Self {
            board,
            turn: 1,
            passes: 0,
            status: ReversiStatus::Playing,
            winner: None,
            ai_level,
            seed,
            moves: 0,
        }
    }
    pub fn legal_moves(&self, player: u8) -> Vec<usize> {
        (0..64)
            .filter(|index| self.captures(*index, player).next().is_some())
            .collect()
    }
    pub fn place(&mut self, index: usize) -> bool {
        self.turn == 1 && self.place_current(index)
    }
    pub fn place_current(&mut self, index: usize) -> bool {
        if self.status != ReversiStatus::Playing
            || !matches!(self.turn, 1 | 2)
            || !self.apply_move(index, self.turn)
        {
            return false;
        }
        self.turn = 3 - self.turn;
        self.passes = 0;
        self.moves += 1;
        self.finish_or_continue();
        true
    }
    pub fn pass(&mut self) -> bool {
        if self.status != ReversiStatus::Playing || !self.legal_moves(self.turn).is_empty() {
            return false;
        }
        self.passes += 1;
        self.turn = 3 - self.turn;
        self.finish_or_continue();
        true
    }
    pub fn ai_move(&mut self) -> bool {
        if self.status != ReversiStatus::Playing
            || self.turn != 2
            || self.ai_level == AiLevel::TwoPlayer
        {
            return false;
        }
        let moves = self.legal_moves(2);
        if moves.is_empty() {
            return self.pass();
        }
        let chosen = match self.ai_level {
            AiLevel::Gentle => moves[0],
            // The empty-move case returned above, so the iterator has a value.
            AiLevel::Sharp => moves
                .into_iter()
                .max_by_key(|index| self.move_value(*index))
                .unwrap(),
            AiLevel::TwoPlayer => return false,
        };
        self.apply_move(chosen, 2);
        self.turn = 1;
        self.passes = 0;
        self.moves += 1;
        self.finish_or_continue();
        true
    }
    pub fn score(&self, player: u8) -> usize {
        self.board.iter().filter(|piece| **piece == player).count()
    }

    pub fn hint_move(&self) -> Option<usize> {
        if self.status != ReversiStatus::Playing || self.turn != 1 {
            return None;
        }
        self.legal_moves(1).into_iter().max_by_key(|&index| {
            let row = index / 8;
            let column = index % 8;
            let corner = matches!(index, 0 | 7 | 56 | 63);
            let edge = row == 0 || row == 7 || column == 0 || column == 7;
            (
                corner,
                edge,
                self.captures(index, 1).count(),
                std::cmp::Reverse(index),
            )
        })
    }
    fn apply_move(&mut self, index: usize, player: u8) -> bool {
        if index >= 64 || self.board[index] != 0 {
            return false;
        }
        let captures = self.captures(index, player).collect::<Vec<_>>();
        if captures.is_empty() {
            return false;
        }
        self.board[index] = player;
        for captured in captures {
            self.board[captured] = player;
        }
        true
    }
    fn captures(&self, index: usize, player: u8) -> impl Iterator<Item = usize> + '_ {
        let mut captured = Vec::new();
        if index < 64 && self.board[index] == 0 {
            for (dr, dc) in DIRECTIONS {
                let mut row = index / 8;
                let mut col = index % 8;
                let mut line = Vec::new();
                loop {
                    let next_row = row as isize + dr;
                    let next_col = col as isize + dc;
                    if !(0..8).contains(&next_row) || !(0..8).contains(&next_col) {
                        break;
                    }
                    row = next_row as usize;
                    col = next_col as usize;
                    match self.board[row * 8 + col] {
                        piece if piece == 3 - player => line.push(row * 8 + col),
                        piece if piece == player => {
                            if !line.is_empty() {
                                captured.extend(line);
                            }
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
        captured.into_iter()
    }
    fn move_value(&self, index: usize) -> usize {
        self.captures(index, 2).count() * 10
            + usize::from(index == 0 || index == 7 || index == 56 || index == 63) * 100
    }
    fn finish_or_continue(&mut self) {
        if self.board.iter().all(|piece| *piece != 0)
            || (self.legal_moves(1).is_empty() && self.legal_moves(2).is_empty())
            || self.passes >= 2
        {
            self.status = ReversiStatus::Won;
            self.winner = match self.score(1).cmp(&self.score(2)) {
                std::cmp::Ordering::Greater => Some(1),
                std::cmp::Ordering::Less => Some(2),
                std::cmp::Ordering::Equal => Some(0),
            };
        }
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
#[path = "../tests/legacy/reversi/tests.rs"]
mod tests;
