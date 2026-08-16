//! Deterministic, first-tap-safe Minesweeper rules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell {
    Hidden,
    Mine,
    FlaggedMine,
    Revealed(u8),
    Flagged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MineStatus {
    Ready,
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinePreset {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}

impl MinePreset {
    pub const ALL: [Self; 4] = [
        Self::Beginner,
        Self::Intermediate,
        Self::Expert,
        Self::Custom,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Beginner => "BEGINNER",
            Self::Intermediate => "INTERMEDIATE",
            Self::Expert => "EXPERT",
            Self::Custom => "CUSTOM",
        }
    }
    pub fn dimensions(self) -> (usize, usize, usize) {
        match self {
            Self::Beginner => (9, 9, 10),
            Self::Intermediate => (16, 16, 40),
            Self::Expert => (30, 16, 99),
            Self::Custom => (12, 12, 20),
        }
    }
    pub fn index(self) -> usize {
        match self {
            Self::Beginner => 0,
            Self::Intermediate => 1,
            Self::Expert => 2,
            Self::Custom => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minesweeper {
    pub preset: MinePreset,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub cells: Vec<Cell>,
    pub seed: u64,
    pub first_reveal: bool,
    pub status: MineStatus,
    pub elapsed_seconds: f32,
}

impl Minesweeper {
    pub fn beginner(seed: u64) -> Self {
        Self::new(MinePreset::Beginner, seed)
    }
    pub fn custom(width: usize, height: usize, mines: usize, seed: u64) -> Self {
        let width = width.clamp(5, 30);
        let height = height.clamp(5, 24);
        let mines = mines.clamp(1, width * height - 9);
        Self {
            preset: MinePreset::Custom,
            width,
            height,
            mines,
            cells: vec![Cell::Hidden; width * height],
            seed,
            first_reveal: false,
            status: MineStatus::Ready,
            elapsed_seconds: 0.0,
        }
    }
    pub fn new(preset: MinePreset, seed: u64) -> Self {
        let (width, height, mines) = preset.dimensions();
        Self {
            preset,
            cells: vec![Cell::Hidden; width * height],
            width,
            height,
            mines,
            seed,
            first_reveal: false,
            status: MineStatus::Ready,
            elapsed_seconds: 0.0,
        }
    }
    pub fn tick(&mut self, dt: f32) {
        if matches!(self.status, MineStatus::Playing) {
            self.elapsed_seconds = (self.elapsed_seconds + dt).min(9999.0);
        }
    }
    pub fn elapsed_whole_seconds(&self) -> u32 {
        self.elapsed_seconds.floor() as u32
    }
    pub fn reveal(&mut self, index: usize) -> bool {
        if index >= self.cells.len()
            || !matches!(self.status, MineStatus::Ready | MineStatus::Playing)
            || !matches!(self.cells[index], Cell::Hidden)
        {
            return false;
        }
        if !self.first_reveal {
            self.place_mines(index);
            self.first_reveal = true;
            self.status = MineStatus::Playing;
        }
        if self.is_mine(index) {
            self.cells[index] = Cell::Revealed(9);
            self.status = MineStatus::Lost;
            return true;
        }
        self.flood_reveal(index);
        self.check_win();
        true
    }
    pub fn toggle_flag(&mut self, index: usize) -> bool {
        if index >= self.cells.len()
            || !matches!(self.status, MineStatus::Ready | MineStatus::Playing)
        {
            return false;
        }
        match self.cells[index] {
            Cell::Hidden => self.cells[index] = Cell::Flagged,
            Cell::Flagged => self.cells[index] = Cell::Hidden,
            Cell::Mine => self.cells[index] = Cell::FlaggedMine,
            Cell::FlaggedMine => self.cells[index] = Cell::Mine,
            Cell::Revealed(_) => return false,
        };
        true
    }
    pub fn chord(&mut self, index: usize) -> bool {
        let Cell::Revealed(number) = self.cells.get(index).copied().unwrap_or(Cell::Hidden) else {
            return false;
        };
        let flagged = self
            .neighbors(index)
            .filter(|&i| matches!(self.cells[i], Cell::Flagged | Cell::FlaggedMine))
            .count();
        if flagged != number as usize {
            return false;
        }
        let targets: Vec<usize> = self
            .neighbors(index)
            .filter(|&i| matches!(self.cells[i], Cell::Hidden))
            .collect();
        let mut changed = false;
        for target in targets {
            changed |= self.reveal(target);
        }
        changed
    }
    pub fn adjacent_mines(&self, index: usize) -> u8 {
        self.neighbors(index).filter(|&i| self.is_mine(i)).count() as u8
    }
    pub fn flagged_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| matches!(cell, Cell::Flagged | Cell::FlaggedMine))
            .count()
    }

    pub fn hint_move(&self) -> Option<(usize, bool)> {
        if matches!(self.status, MineStatus::Won | MineStatus::Lost) {
            return None;
        }
        if !self.first_reveal {
            return Some(((self.height / 2) * self.width + self.width / 2, true));
        }
        for index in 0..self.cells.len() {
            let Cell::Revealed(number) = self.cells[index] else {
                continue;
            };
            let neighbors: Vec<_> = self.neighbors(index).collect();
            let flagged = neighbors
                .iter()
                .filter(|&&neighbor| {
                    matches!(self.cells[neighbor], Cell::Flagged | Cell::FlaggedMine)
                })
                .count();
            let hidden: Vec<_> = neighbors
                .into_iter()
                .filter(|&neighbor| matches!(self.cells[neighbor], Cell::Hidden))
                .collect();
            if flagged == number as usize {
                if let Some(&safe) = hidden.first() {
                    return Some((safe, true));
                }
            } else if flagged + hidden.len() == number as usize {
                if let Some(&mine) = hidden.first() {
                    return Some((mine, false));
                }
            }
        }
        None
    }
    fn place_mines(&mut self, safe: usize) {
        let safe_zone: Vec<usize> = std::iter::once(safe).chain(self.neighbors(safe)).collect();
        let mut placed = 0;
        while placed < self.mines {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442690888963407);
            let index = (self.seed as usize) % self.cells.len();
            if safe_zone.contains(&index) || self.is_mine(index) {
                continue;
            }
            self.cells[index] = Cell::Mine;
            placed += 1;
        }
    }
    fn is_mine(&self, index: usize) -> bool {
        matches!(self.cells[index], Cell::Mine | Cell::FlaggedMine)
    }
    fn flood_reveal(&mut self, start: usize) {
        let mut queue = vec![start];
        while let Some(index) = queue.pop() {
            if !matches!(self.cells[index], Cell::Hidden) || self.is_mine(index) {
                continue;
            }
            let count = self.adjacent_mines(index);
            self.cells[index] = Cell::Revealed(count);
            if count == 0 {
                queue.extend(
                    self.neighbors(index)
                        .filter(|&neighbor| matches!(self.cells[neighbor], Cell::Hidden)),
                );
            }
        }
    }
    fn check_win(&mut self) {
        let safe = self
            .cells
            .iter()
            .filter(|cell| matches!(cell, Cell::Revealed(value) if *value < 9))
            .count();
        if safe + self.mines == self.cells.len() {
            self.status = MineStatus::Won;
        }
    }
    fn neighbors(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        let x = index % self.width;
        let y = index / self.width;
        (-1i32..=1)
            .flat_map(move |dy| (-1i32..=1).map(move |dx| (x as i32 + dx, y as i32 + dy)))
            .filter(move |&(nx, ny)| {
                !(nx == x as i32 && ny == y as i32)
                    && nx >= 0
                    && ny >= 0
                    && nx < self.width as i32
                    && ny < self.height as i32
            })
            .map(move |(nx, ny)| ny as usize * self.width + nx as usize)
    }
}

#[cfg(test)]
mod tests;
