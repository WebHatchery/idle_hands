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

pub const FOCUS_WINDOW: usize = 9;
pub const VARIANT_COUNT: u8 = 4;

pub fn visible_size(size: usize, zoomed: bool) -> usize {
    if zoomed {
        size.min(FOCUS_WINDOW)
    } else {
        size
    }
}

pub fn focus_origin(size: usize, zoomed: bool, focus: (usize, usize)) -> (usize, usize) {
    if !zoomed {
        return (0, 0);
    }
    let window = visible_size(size, true);
    (
        focus.0.min(size.saturating_sub(window)),
        focus.1.min(size.saturating_sub(window)),
    )
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
    pub best_moves: Option<u32>,
    #[serde(default)]
    pub variant: u8,
    pub status: NonogramStatus,
    #[serde(skip)]
    history: Vec<(Vec<NonogramMark>, u32, NonogramStatus)>,
}

impl Default for Nonogram {
    fn default() -> Self {
        Self::new(NonogramPreset::Small)
    }
}
impl Nonogram {
    pub fn new(preset: NonogramPreset) -> Self {
        Self::new_with_variant(preset, 0)
    }

    pub fn new_with_seed(preset: NonogramPreset, seed: u64) -> Self {
        Self::new_with_variant(preset, variant_from_seed(seed))
    }

    pub fn new_with_variant(preset: NonogramPreset, variant: u8) -> Self {
        let size = preset.size();
        let variant = variant % VARIANT_COUNT;
        let solution: Vec<bool> = (0..size * size)
            .map(|index| {
                let x = index % size;
                let y = index / size;
                match variant {
                    0 => x == y || x + y + 1 == size || (y == size / 2 && x.is_multiple_of(3)),
                    1 => x == 0 || y == 0 || x + 1 == size || y + 1 == size || x == y,
                    2 => {
                        let center = size / 2;
                        x.abs_diff(center) + y.abs_diff(center) <= center
                    }
                    _ => (x + y).is_multiple_of(2) || x == size / 2 || y == size / 2,
                }
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
            best_moves: None,
            variant,
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
        self.history
            .push((self.marks.clone(), self.moves, self.status));
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
        if let Some((marks, moves, status)) = self.history.pop() {
            self.marks = marks;
            self.moves = moves;
            self.status = status;
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

    pub fn hint_cell(&self) -> Option<(usize, bool)> {
        if self.status == NonogramStatus::Won {
            return None;
        }
        self.marks
            .iter()
            .position(|mark| *mark == NonogramMark::Empty)
            .map(|index| (index, self.solution[index]))
    }
    fn check_win(&mut self) {
        if self
            .marks
            .iter()
            .enumerate()
            .all(|(index, mark)| (*mark == NonogramMark::Filled) == self.solution[index])
        {
            self.status = NonogramStatus::Won;
            self.best_moves = Some(
                self.best_moves
                    .map_or(self.moves, |best| best.min(self.moves)),
            );
        }
    }
}

fn variant_from_seed(seed: u64) -> u8 {
    let mixed = seed ^ seed.rotate_left(17) ^ 0x9E37_79B9_7F4A_7C15;
    (mixed.wrapping_mul(0xBF58_476D_1CE4_E5B9) % u64::from(VARIANT_COUNT)) as u8
}

pub fn stroke_indices(size: usize, start: (usize, usize), end: (usize, usize)) -> Vec<usize> {
    if size == 0 || start.0 >= size || start.1 >= size || end.0 >= size || end.1 >= size {
        return Vec::new();
    }
    if start.0.abs_diff(end.0) >= start.1.abs_diff(end.1) {
        let (from, to) = if start.0 <= end.0 {
            (start.0, end.0)
        } else {
            (end.0, start.0)
        };
        (from..=to).map(|x| start.1 * size + x).collect()
    } else {
        let (from, to) = if start.1 <= end.1 {
            (start.1, end.1)
        } else {
            (end.1, start.1)
        };
        (from..=to).map(|y| y * size + start.0).collect()
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
