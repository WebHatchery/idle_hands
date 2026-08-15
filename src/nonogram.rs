//! Deterministic Nonogram catalog boards and mark rules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonogramPreset {
    Small,
    Medium,
    Large,
}
impl NonogramPreset {
    pub const ALL: [Self; 3] = [Self::Small, Self::Medium, Self::Large];
    pub fn size(self) -> usize {
        match self {
            Self::Small => 5,
            Self::Medium => 10,
            Self::Large => 15,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Small => "5 × 5",
            Self::Medium => "10 × 10",
            Self::Large => "15 × 15",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonogramMark {
    Empty,
    Filled,
    Crossed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonogramMode {
    Fill,
    Cross,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonogramStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonogram {
    pub preset: NonogramPreset,
    pub size: usize,
    pub solution: Vec<bool>,
    pub marks: Vec<NonogramMark>,
    pub row_clues: Vec<Vec<u8>>,
    pub column_clues: Vec<Vec<u8>>,
    pub mode: NonogramMode,
    pub selected: Option<usize>,
    pub moves: u32,
    pub status: NonogramStatus,
    #[serde(skip)]
    history: Vec<Vec<NonogramMark>>,
}

impl Default for Nonogram {
    fn default() -> Self {
        Self::new(NonogramPreset::Small)
    }
}
impl Nonogram {
    pub fn new(preset: NonogramPreset) -> Self {
        let size = preset.size();
        let solution: Vec<bool> = (0..size * size)
            .map(|index| {
                let x = index % size;
                let y = index / size;
                x == y || x + y + 1 == size || (y == size / 2 && x % 3 == 0)
            })
            .collect();
        let row_clues = (0..size)
            .map(|row| clues((0..size).map(|x| solution[row * size + x])))
            .collect();
        let column_clues = (0..size)
            .map(|column| clues((0..size).map(|y| solution[y * size + column])))
            .collect();
        Self {
            preset,
            size,
            solution,
            marks: vec![NonogramMark::Empty; size * size],
            row_clues,
            column_clues,
            mode: NonogramMode::Fill,
            selected: None,
            moves: 0,
            status: NonogramStatus::Playing,
            history: Vec::new(),
        }
    }
    pub fn select(&mut self, index: usize) -> bool {
        if index < self.marks.len() {
            self.selected = Some(index);
            true
        } else {
            false
        }
    }
    pub fn toggle(&mut self, index: usize) -> bool {
        if index >= self.marks.len() || self.status == NonogramStatus::Won {
            return false;
        }
        self.history.push(self.marks.clone());
        self.marks[index] = match (self.mode, self.marks[index]) {
            (NonogramMode::Fill, NonogramMark::Filled)
            | (NonogramMode::Cross, NonogramMark::Crossed) => NonogramMark::Empty,
            (NonogramMode::Fill, _) => NonogramMark::Filled,
            (NonogramMode::Cross, _) => NonogramMark::Crossed,
        };
        self.moves += 1;
        self.check_win();
        true
    }
    pub fn undo(&mut self) -> bool {
        if let Some(marks) = self.history.pop() {
            self.marks = marks;
            self.status = NonogramStatus::Playing;
            true
        } else {
            false
        }
    }
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            NonogramMode::Fill => NonogramMode::Cross,
            NonogramMode::Cross => NonogramMode::Fill,
        };
    }
    fn check_win(&mut self) {
        if self
            .marks
            .iter()
            .enumerate()
            .all(|(index, mark)| (*mark == NonogramMark::Filled) == self.solution[index])
        {
            self.status = NonogramStatus::Won;
        }
    }
}

fn clues<I: Iterator<Item = bool>>(cells: I) -> Vec<u8> {
    let mut result = Vec::new();
    let mut run = 0;
    for filled in cells {
        if filled {
            run += 1;
        } else if run > 0 {
            result.push(run);
            run = 0;
        }
    }
    if run > 0 {
        result.push(run);
    }
    if result.is_empty() {
        result.push(0);
    }
    result
}

#[cfg(test)]
mod tests;
