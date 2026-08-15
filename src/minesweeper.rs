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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minesweeper {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub cells: Vec<Cell>,
    pub seed: u64,
    pub first_reveal: bool,
    pub status: MineStatus,
}

impl Minesweeper {
    pub fn beginner(seed: u64) -> Self {
        Self {
            width: 9,
            height: 9,
            mines: 10,
            cells: vec![Cell::Hidden; 81],
            seed,
            first_reveal: false,
            status: MineStatus::Ready,
        }
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
