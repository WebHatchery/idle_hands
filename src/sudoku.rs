//! Deterministic Sudoku board state for the first collection puzzle.

use serde::{Deserialize, Serialize};

const PUZZLE: &str =
    "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
const EASY: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419000000000000";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SudokuStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SudokuDifficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}
impl SudokuDifficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Medium, Self::Hard];
    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "EASY",
            Self::Medium => "MEDIUM",
            Self::Hard => "HARD",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku {
    #[serde(default)]
    pub difficulty: SudokuDifficulty,
    pub puzzle: Vec<u8>,
    pub values: Vec<u8>,
    pub notes: Vec<u16>,
    pub selected: Option<usize>,
    pub status: SudokuStatus,
    #[serde(default)]
    pub moves: u32,
    #[serde(default)]
    pub best_moves: Option<u32>,
    #[serde(skip)]
    history: Vec<(Vec<u8>, Vec<u16>, SudokuStatus, u32)>,
}
impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}
impl Sudoku {
    pub fn new() -> Self {
        Self::with_difficulty(SudokuDifficulty::Medium)
    }
    pub fn with_difficulty(difficulty: SudokuDifficulty) -> Self {
        let source = match difficulty {
            SudokuDifficulty::Easy => EASY,
            SudokuDifficulty::Medium | SudokuDifficulty::Hard => PUZZLE,
        };
        let puzzle: Vec<u8> = source.bytes().map(|digit| digit - b'0').collect();
        Self {
            difficulty,
            values: puzzle.clone(),
            puzzle,
            notes: vec![0; 81],
            selected: None,
            status: SudokuStatus::Playing,
            moves: 0,
            best_moves: None,
            history: Vec::new(),
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
        self.push_history();
        self.values[index] = value;
        self.notes[index] = 0;
        self.moves += 1;
        self.check_win();
        true
    }
    pub fn erase(&mut self, index: usize) -> bool {
        if index >= 81 || self.is_given(index) {
            return false;
        }
        self.push_history();
        self.values[index] = 0;
        self.moves += 1;
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
        self.push_history();
        self.notes[index] ^= 1 << value;
        self.moves += 1;
        true
    }
    pub fn undo(&mut self) -> bool {
        if let Some((values, notes, status, moves)) = self.history.pop() {
            self.values = values;
            self.notes = notes;
            self.status = status;
            self.moves = moves;
            true
        } else {
            false
        }
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
        if self.status == SudokuStatus::Won {
            self.best_moves = Some(
                self.best_moves
                    .map_or(self.moves, |best| best.min(self.moves)),
            );
        }
    }
    fn push_history(&mut self) {
        self.history.push((
            self.values.clone(),
            self.notes.clone(),
            self.status,
            self.moves,
        ));
    }
}

#[cfg(test)]
mod tests;
