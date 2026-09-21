//! Randomized Sudoku board state for the first collection puzzle.

use macroquad_toolkit::rng::{random_u64, SeededRng};
use serde::{Deserialize, Serialize};

pub const PUZZLE: &str =
    "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
pub const EASY: &str =
    "534670000672195000098300060850760003420803001710020006960000280200419005300080079";
pub const HARD: &str =
    "005300000800000020070010500400005300010070006003200080060500009004000030000009700";

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

    pub fn source(self) -> &'static str {
        match self {
            Self::Easy => EASY,
            Self::Medium => PUZZLE,
            Self::Hard => HARD,
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
    pub history: Vec<(Vec<u8>, Vec<u16>, SudokuStatus, u32)>,
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
        Self::with_difficulty_and_seed(difficulty, random_u64())
    }
    pub fn with_difficulty_and_seed(difficulty: SudokuDifficulty, seed: u64) -> Self {
        let mut rng = SeededRng::new(seed);
        let puzzle = randomized_puzzle(difficulty.source(), &mut rng);
        assert_eq!(
            count_solutions(&puzzle, 2),
            1,
            "Generated Sudoku puzzle is not unique"
        );
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

    pub fn hint_move(&self) -> Option<(usize, u8)> {
        if self.status == SudokuStatus::Won {
            return None;
        }
        let mut solved = [0u8; 81];
        solved.copy_from_slice(&self.values);
        if !solve_first(&mut solved) {
            return None;
        }
        (0..81)
            .find(|&index| self.values[index] == 0)
            .map(|index| (index, solved[index]))
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
    pub fn peers(left: usize, right: usize) -> bool {
        left / 9 == right / 9
            || left % 9 == right % 9
            || (left / 27 == right / 27 && left % 9 / 3 == right % 9 / 3)
    }
    pub fn check_win(&mut self) {
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
    pub fn push_history(&mut self) {
        self.history.push((
            self.values.clone(),
            self.notes.clone(),
            self.status,
            self.moves,
        ));
    }
}

pub fn count_solutions(puzzle: &[u8], limit: u8) -> u8 {
    if puzzle.len() != 81 || limit == 0 {
        return 0;
    }
    let mut board = [0u8; 81];
    board.copy_from_slice(puzzle);
    solve_count(&mut board, 0, limit)
}

pub fn randomized_puzzle(source: &str, rng: &mut SeededRng) -> Vec<u8> {
    let source: Vec<u8> = source.bytes().map(|digit| digit - b'0').collect();
    assert_eq!(source.len(), 81, "Sudoku source must contain 81 cells");
    let rows = shuffled_units(rng);
    let columns = shuffled_units(rng);
    let mut digits: Vec<u8> = (1..=9).collect();
    rng.shuffle(&mut digits);
    let mut puzzle = vec![0; 81];
    for (row, &source_row) in rows.iter().enumerate() {
        for (column, &source_column) in columns.iter().enumerate() {
            let value = source[source_row * 9 + source_column];
            puzzle[row * 9 + column] = if value == 0 {
                0
            } else {
                digits[value as usize - 1]
            };
        }
    }
    puzzle
}

pub fn shuffled_units(rng: &mut SeededRng) -> Vec<usize> {
    let mut units: Vec<Vec<usize>> = (0..3)
        .map(|unit| {
            let mut members = (0..3).map(|offset| unit * 3 + offset).collect::<Vec<_>>();
            rng.shuffle(&mut members);
            members
        })
        .collect();
    rng.shuffle(&mut units);
    units.into_iter().flatten().collect()
}

pub fn solve_count(board: &mut [u8; 81], found: u8, limit: u8) -> u8 {
    if found >= limit {
        return found;
    }
    let mut best_index = None;
    let mut best_candidates = [0u8; 9];
    let mut best_count = 10;
    for index in 0..81 {
        if board[index] != 0 {
            continue;
        }
        let mut candidates = [0u8; 9];
        let mut count = 0;
        for value in 1..=9 {
            if valid_on_board(board, index, value) {
                candidates[count] = value;
                count += 1;
            }
        }
        if count == 0 {
            return found;
        }
        if count < best_count {
            best_index = Some(index);
            best_candidates = candidates;
            best_count = count;
        }
    }
    let Some(index) = best_index else {
        return found + 1;
    };
    let mut found = found;
    for value in best_candidates.into_iter().take(best_count) {
        board[index] = value;
        found = solve_count(board, found, limit);
        board[index] = 0;
        if found >= limit {
            break;
        }
    }
    found
}

pub fn solve_first(board: &mut [u8; 81]) -> bool {
    let mut best_index = None;
    let mut best_candidates = [0u8; 9];
    let mut best_count = 10;
    for index in 0..81 {
        if board[index] != 0 {
            continue;
        }
        let mut candidates = [0u8; 9];
        let mut count = 0;
        for value in 1..=9 {
            if valid_on_board(board, index, value) {
                candidates[count] = value;
                count += 1;
            }
        }
        if count == 0 {
            return false;
        }
        if count < best_count {
            best_index = Some(index);
            best_candidates = candidates;
            best_count = count;
        }
    }
    let Some(index) = best_index else {
        return true;
    };
    for value in best_candidates.into_iter().take(best_count) {
        board[index] = value;
        if solve_first(board) {
            return true;
        }
        board[index] = 0;
    }
    false
}

pub fn valid_on_board(board: &[u8; 81], index: usize, value: u8) -> bool {
    (0..81).all(|other| other == index || board[other] != value || !Sudoku::peers(index, other))
}
