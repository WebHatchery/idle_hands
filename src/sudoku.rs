//! Deterministic Sudoku board state for the first collection puzzle.

use serde::{Deserialize, Serialize};

const PUZZLE: &str =
    "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SudokuStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku {
    pub puzzle: Vec<u8>,
    pub values: Vec<u8>,
    pub notes: Vec<u16>,
    pub selected: Option<usize>,
    pub status: SudokuStatus,
}
impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}
impl Sudoku {
    pub fn new() -> Self {
        let puzzle: Vec<u8> = PUZZLE.bytes().map(|digit| digit - b'0').collect();
        Self {
            values: puzzle.clone(),
            puzzle,
            notes: vec![0; 81],
            selected: None,
            status: SudokuStatus::Playing,
        }
    }
    pub fn is_given(&self, index: usize) -> bool {
        self.puzzle.get(index).copied().unwrap_or(0) != 0
    }
    pub fn select(&mut self, index: usize) -> bool {
        if index < 81 {
            self.selected = Some(index);
            true
        } else {
            false
        }
    }
    pub fn place(&mut self, index: usize, value: u8) -> bool {
        if index >= 81
            || self.is_given(index)
            || !(1..=9).contains(&value)
            || !self.is_valid(index, value)
        {
            return false;
        }
        self.values[index] = value;
        self.notes[index] = 0;
        self.check_win();
        true
    }
    pub fn erase(&mut self, index: usize) -> bool {
        if index >= 81 || self.is_given(index) {
            return false;
        }
        self.values[index] = 0;
        self.check_win();
        true
    }
    pub fn toggle_note(&mut self, index: usize, value: u8) -> bool {
        if index >= 81
            || self.is_given(index)
            || !(1..=9).contains(&value)
            || self.values[index] != 0
        {
            return false;
        }
        self.notes[index] ^= 1 << value;
        true
    }
    pub fn conflicts(&self, index: usize) -> Vec<usize> {
        if index >= 81 || self.values[index] == 0 {
            return Vec::new();
        }
        let value = self.values[index];
        (0..81)
            .filter(|&other| {
                other != index && self.values[other] == value && Self::peers(index, other)
            })
            .collect()
    }
    pub fn is_valid(&self, index: usize, value: u8) -> bool {
        (0..81).all(|other| {
            other == index || self.values[other] != value || !Self::peers(index, other)
        })
    }
    fn peers(left: usize, right: usize) -> bool {
        left / 9 == right / 9
            || left % 9 == right % 9
            || (left / 27 == right / 27 && left % 9 / 3 == right % 9 / 3)
    }
    fn check_win(&mut self) {
        self.status = if self.values.iter().all(|&value| value != 0)
            && (0..81).all(|index| self.is_valid(index, self.values[index]))
        {
            SudokuStatus::Won
        } else {
            SudokuStatus::Playing
        };
    }
}

#[cfg(test)]
mod tests;
