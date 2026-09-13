//! Deterministic adjacent-pair Number Match rules.
use crate::undo::UndoStack;

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 6;
const CELLS: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumberMatchPhase {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkRule {
    Neighbors,
    Lines,
    Diagonals,
}

fn default_rule() -> LinkRule {
    LinkRule::Neighbors
}

fn default_remixes() -> u8 {
    2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberMatch {
    pub cells: Vec<u8>,
    pub selected: Option<usize>,
    pub moves: u16,
    pub score: u16,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub combo: u16,
    #[serde(default)]
    pub best_combo: u16,
    #[serde(default = "default_rule")]
    pub rule: LinkRule,
    #[serde(default = "default_remixes")]
    pub remixes_left: u8,
    pub seed: u64,
    pub phase: NumberMatchPhase,
    #[serde(skip)]
    history: UndoStack<Self>,
}

impl Default for NumberMatch {
    fn default() -> Self {
        Self::new(0x4E55_4D42_4552)
    }
}

impl NumberMatch {
    pub fn new(seed: u64) -> Self {
        Self::new_with_rule(seed, LinkRule::Neighbors)
    }

    pub fn new_with_rule(seed: u64, rule: LinkRule) -> Self {
        let mut cells = vec![0; CELLS];
        let mut pair = 0;
        for block_row in 0..(SIDE / 2) {
            for block_col in 0..(SIDE / 2) {
                let vertical = random_word(seed, pair) & 1 == 1;
                for offset in 0..2 {
                    let row = block_row * 2;
                    let col = block_col * 2;
                    let (first, second) = if vertical {
                        (row * SIDE + col + offset, (row + 1) * SIDE + col + offset)
                    } else {
                        ((row + offset) * SIDE + col, (row + offset) * SIDE + col + 1)
                    };
                    let value = ((random_word(seed, pair + 17) % 9) + 1) as u8;
                    cells[first] = value;
                    cells[second] = if pair % 3 == 0 { 10 - value } else { value };
                    pair += 1;
                }
            }
        }
        Self {
            cells,
            selected: None,
            moves: 0,
            score: 0,
            points: 0,
            combo: 0,
            best_combo: 0,
            rule,
            remixes_left: 2,
            seed,
            phase: NumberMatchPhase::Playing,
            history: UndoStack::default(),
        }
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.phase != NumberMatchPhase::Playing || index >= CELLS || self.cells[index] == 0 {
            return false;
        }
        let Some(previous) = self.selected else {
            self.selected = Some(index);
            return true;
        };
        if previous == index {
            self.selected = None;
            return true;
        }
        if !self.can_pair(previous, index) {
            self.selected = Some(index);
            self.combo = 0;
            return true;
        }
        let mut snapshot = self.clone_without_undo();
        snapshot.selected = None;
        self.cells[previous] = 0;
        self.cells[index] = 0;
        self.selected = None;
        self.moves = self.moves.saturating_add(1);
        self.score = self.score.saturating_add(1);
        self.combo = self.combo.saturating_add(1);
        self.best_combo = self.best_combo.max(self.combo);
        self.points = self
            .points
            .saturating_add(10_u32.saturating_mul(u32::from(self.combo)));
        if self.cells.iter().all(|&value| value == 0) {
            self.phase = NumberMatchPhase::Won;
        } else if self.hint_pair().is_none() {
            self.phase = NumberMatchPhase::Stuck;
        }
        self.history.push(snapshot);
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.history.pop() else {
            return false;
        };
        let history = std::mem::take(&mut self.history);
        *self = previous;
        self.history = history;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_rule(seed, self.rule);
    }

    pub fn set_rule(&mut self, rule: LinkRule, seed: u64) {
        *self = Self::new_with_rule(seed, rule);
    }

    pub fn remix(&mut self) -> bool {
        if self.phase == NumberMatchPhase::Won || self.remixes_left == 0 {
            return false;
        }
        let remaining = self.cells.iter().filter(|&&value| value != 0).count();
        if remaining == 0 || remaining % 2 != 0 {
            return false;
        }
        let snapshot = self.clone_without_undo();
        self.cells.fill(0);
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        for pair in 0..(remaining / 2) {
            let value = ((random_word(self.seed, pair) % 9) + 1) as u8;
            self.cells[pair * 2] = value;
            self.cells[pair * 2 + 1] = if pair % 2 == 0 { value } else { 10 - value };
        }
        self.selected = None;
        self.combo = 0;
        self.remixes_left -= 1;
        self.phase = NumberMatchPhase::Playing;
        self.history.push(snapshot);
        true
    }

    pub fn won(&self) -> bool {
        self.phase == NumberMatchPhase::Won
    }

    pub fn hint_pair(&self) -> Option<(usize, usize)> {
        if self.phase != NumberMatchPhase::Playing {
            return None;
        }
        for first in 0..CELLS {
            if self.cells[first] == 0 {
                continue;
            }
            for second in (first + 1)..CELLS {
                if self.can_pair(first, second) {
                    return Some((first, second));
                }
            }
        }
        None
    }

    pub fn can_pair(&self, first: usize, second: usize) -> bool {
        if first >= CELLS
            || second >= CELLS
            || first == second
            || self.cells[first] == 0
            || self.cells[second] == 0
            || !self.valid_pair(first, second)
        {
            return false;
        }
        let first_row = first / SIDE;
        let first_col = first % SIDE;
        let second_row = second / SIDE;
        let second_col = second % SIDE;
        let row_distance = first_row.abs_diff(second_row);
        let col_distance = first_col.abs_diff(second_col);
        if row_distance + col_distance == 1 {
            return true;
        }
        match self.rule {
            LinkRule::Neighbors => false,
            LinkRule::Lines => {
                (first_row == second_row || first_col == second_col)
                    && self.path_is_clear(first, second)
            }
            LinkRule::Diagonals => {
                (first_row == second_row || first_col == second_col || row_distance == col_distance)
                    && self.path_is_clear(first, second)
            }
        }
    }

    fn valid_pair(&self, first: usize, second: usize) -> bool {
        self.cells[first] == self.cells[second] || self.cells[first] + self.cells[second] == 10
    }

    fn path_is_clear(&self, first: usize, second: usize) -> bool {
        let first_row = first / SIDE;
        let first_col = first % SIDE;
        let second_row = second / SIDE;
        let second_col = second % SIDE;
        let row_step = step(first_row, second_row);
        let col_step = step(first_col, second_col);
        let mut row = first_row as isize + row_step;
        let mut col = first_col as isize + col_step;
        while row != second_row as isize || col != second_col as isize {
            if self.cells[row as usize * SIDE + col as usize] != 0 {
                return false;
            }
            row += row_step;
            col += col_step;
        }
        true
    }

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

fn step(from: usize, to: usize) -> isize {
    match from.cmp(&to) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

fn random_word(seed: u64, index: usize) -> u64 {
    seed.wrapping_add(index as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        ^ seed.rotate_left((index % 63) as u32 + 1)
}

#[cfg(test)]
#[path = "../tests/legacy/number_match/tests.rs"]
mod tests;
