//! Deterministic touch-first word search with endpoint selection.

use serde::{Deserialize, Serialize};

const SIZE: usize = 10;
const WORD_COUNT: usize = 6;
pub const WORDS: [&str; WORD_COUNT] = ["STILL", "SHELF", "CARD", "PAUSE", "GAMES", "DREAM"];
const PLACEMENTS: [(usize, usize, isize, isize); WORD_COUNT] = [
    (0, 0, 0, 1),
    (2, 9, 1, 0),
    (9, 0, 0, 1),
    (8, 2, 0, 1),
    (0, 7, 1, 0),
    (3, 1, 1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordSearchStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSearch {
    pub cells: Vec<u8>,
    pub found: [bool; WORD_COUNT],
    pub selected_start: Option<usize>,
    pub moves: u16,
    pub status: WordSearchStatus,
    pub seed: u64,
}

impl Default for WordSearch {
    fn default() -> Self {
        Self::new(0x0057_0D5E)
    }
}

impl WordSearch {
    pub fn new(seed: u64) -> Self {
        let mut cells = vec![0u8; SIZE * SIZE];
        let mut rng = seed;
        for cell in &mut cells {
            rng = next_seed(rng);
            *cell = (rng % 26) as u8;
        }
        for (word, &(row, column, row_step, column_step)) in WORDS.iter().zip(PLACEMENTS.iter()) {
            for (offset, letter) in word.bytes().enumerate() {
                let target_row = (row as isize + row_step * offset as isize) as usize;
                let target_column = (column as isize + column_step * offset as isize) as usize;
                cells[target_row * SIZE + target_column] = letter - b'A';
            }
        }
        Self {
            cells,
            found: [false; WORD_COUNT],
            selected_start: None,
            moves: 0,
            status: WordSearchStatus::Playing,
            seed,
        }
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= self.cells.len() || self.status == WordSearchStatus::Won {
            return false;
        }
        let Some(start) = self.selected_start else {
            self.selected_start = Some(index);
            return true;
        };
        self.selected_start = None;
        let Some(word) = self.matching_word(start, index) else {
            return false;
        };
        if !self.found[word] {
            self.found[word] = true;
            self.moves += 1;
            if self.found.iter().all(|is_found| *is_found) {
                self.status = WordSearchStatus::Won;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        self.selected_start = None;
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn hint_word(&self) -> Option<(usize, usize, usize)> {
        if self.status == WordSearchStatus::Won {
            return None;
        }
        self.found.iter().position(|found| !found).map(|word| {
            let (row, column, row_step, column_step) = PLACEMENTS[word];
            let end_offset = WORDS[word].len() as isize - 1;
            let end_row = (row as isize + row_step * end_offset) as usize;
            let end_column = (column as isize + column_step * end_offset) as usize;
            (word, row * SIZE + column, end_row * SIZE + end_column)
        })
    }

    pub fn cell_found(&self, index: usize) -> bool {
        PLACEMENTS
            .iter()
            .enumerate()
            .any(|(word, &(row, column, row_step, column_step))| {
                self.found[word]
                    && (0..WORDS[word].len()).any(|offset| {
                        let target_row = (row as isize + row_step * offset as isize) as usize;
                        let target_column =
                            (column as isize + column_step * offset as isize) as usize;
                        target_row * SIZE + target_column == index
                    })
            })
    }

    fn matching_word(&self, start: usize, end: usize) -> Option<usize> {
        let start_row = start / SIZE;
        let start_column = start % SIZE;
        let end_row = end / SIZE;
        let end_column = end % SIZE;
        let row_delta = end_row as isize - start_row as isize;
        let column_delta = end_column as isize - start_column as isize;
        let steps = row_delta.abs().max(column_delta.abs());
        if steps == 0
            || (row_delta != 0 && column_delta != 0 && row_delta.abs() != column_delta.abs())
        {
            return None;
        }
        let row_step = row_delta.signum();
        let column_step = column_delta.signum();
        WORDS.iter().enumerate().find_map(|(word, letters)| {
            if letters.len() != steps as usize + 1 {
                return None;
            }
            let matches_forward = letters.bytes().enumerate().all(|(offset, letter)| {
                self.cells[((start_row as isize + row_step * offset as isize) as usize) * SIZE
                    + (start_column as isize + column_step * offset as isize) as usize]
                    == letter - b'A'
            });
            let matches_reverse = letters.bytes().rev().enumerate().all(|(offset, letter)| {
                self.cells[((start_row as isize + row_step * offset as isize) as usize) * SIZE
                    + (start_column as isize + column_step * offset as isize) as usize]
                    == letter - b'A'
            });
            (matches_forward || matches_reverse).then_some(word)
        })
    }
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
