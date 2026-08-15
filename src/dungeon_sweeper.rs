//! Deterministic dungeon pathfinding with Minesweeper-style trap clues.

use serde::{Deserialize, Serialize};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const TRAPS: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonCell {
    Hidden,
    Trap,
    Flagged,
    FlaggedTrap,
    Revealed(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonStatus {
    Ready,
    Playing,
    Won,
    Lost,
}

type Snapshot = (Vec<DungeonCell>, u64, bool, u16, DungeonStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonSweeper {
    pub width: usize,
    pub height: usize,
    pub traps: usize,
    pub cells: Vec<DungeonCell>,
    pub exit: usize,
    pub moves: u16,
    pub status: DungeonStatus,
    pub seed: u64,
    pub first_reveal: bool,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for DungeonSweeper {
    fn default() -> Self {
        Self::new(0xD0A6_0001)
    }
}

impl DungeonSweeper {
    pub fn new(seed: u64) -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            traps: TRAPS,
            cells: vec![DungeonCell::Hidden; WIDTH * HEIGHT],
            exit: WIDTH * HEIGHT - 1,
            moves: 0,
            status: DungeonStatus::Ready,
            seed,
            first_reveal: false,
            history: Vec::new(),
        }
    }

    pub fn reveal(&mut self, index: usize) -> bool {
        if index >= self.cells.len()
            || !matches!(self.status, DungeonStatus::Ready | DungeonStatus::Playing)
            || !matches!(
                self.cells[index],
                DungeonCell::Hidden | DungeonCell::Trap | DungeonCell::Revealed(_)
            )
        {
            return false;
        }
        if matches!(self.cells[index], DungeonCell::Revealed(_)) {
            return self.chord(index);
        }
        if !self.first_reveal {
            self.place_traps(index);
            self.first_reveal = true;
            self.status = DungeonStatus::Playing;
        }
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        if matches!(self.cells[index], DungeonCell::Trap) {
            self.cells[index] = DungeonCell::Revealed(9);
            self.status = DungeonStatus::Lost;
        } else {
            self.cells[index] = DungeonCell::Revealed(self.adjacent_traps(index));
            if index == self.exit {
                self.status = DungeonStatus::Won;
            }
        }
        true
    }

    pub fn chord(&mut self, index: usize) -> bool {
        if index >= self.cells.len()
            || self.status != DungeonStatus::Playing
            || !matches!(self.cells[index], DungeonCell::Revealed(_))
        {
            return false;
        }
        let clue = match self.cells[index] {
            DungeonCell::Revealed(value) => value,
            _ => unreachable!(),
        };
        let neighbors: Vec<_> = self.neighbors(index).collect();
        let flagged = neighbors
            .iter()
            .filter(|&&neighbor| {
                matches!(
                    self.cells[neighbor],
                    DungeonCell::Flagged | DungeonCell::FlaggedTrap
                )
            })
            .count() as u8;
        if flagged != clue {
            return false;
        }
        let hidden: Vec<_> = neighbors
            .into_iter()
            .filter(|&neighbor| {
                matches!(
                    self.cells[neighbor],
                    DungeonCell::Hidden | DungeonCell::Trap
                )
            })
            .collect();
        if hidden.is_empty() {
            return false;
        }
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        for neighbor in hidden {
            if matches!(self.cells[neighbor], DungeonCell::Trap) {
                self.cells[neighbor] = DungeonCell::Revealed(9);
                self.status = DungeonStatus::Lost;
                break;
            }
            self.cells[neighbor] = DungeonCell::Revealed(self.adjacent_traps(neighbor));
            if neighbor == self.exit {
                self.status = DungeonStatus::Won;
                break;
            }
        }
        true
    }

    pub fn toggle_flag(&mut self, index: usize) -> bool {
        if index >= self.cells.len()
            || !matches!(self.status, DungeonStatus::Ready | DungeonStatus::Playing)
        {
            return false;
        }
        match self.cells[index] {
            DungeonCell::Hidden => self.cells[index] = DungeonCell::Flagged,
            DungeonCell::Trap => self.cells[index] = DungeonCell::FlaggedTrap,
            DungeonCell::Flagged => self.cells[index] = DungeonCell::Hidden,
            DungeonCell::FlaggedTrap => self.cells[index] = DungeonCell::Trap,
            _ => return false,
        }
        true
    }

    pub fn flagged_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| matches!(cell, DungeonCell::Flagged | DungeonCell::FlaggedTrap))
            .count()
    }

    pub fn adjacent_traps(&self, index: usize) -> u8 {
        self.neighbors(index)
            .filter(|&neighbor| {
                matches!(
                    self.cells[neighbor],
                    DungeonCell::Trap | DungeonCell::FlaggedTrap
                )
            })
            .count() as u8
    }

    pub fn undo(&mut self) -> bool {
        if let Some((cells, seed, first_reveal, moves, status)) = self.history.pop() {
            self.cells = cells;
            self.seed = seed;
            self.first_reveal = first_reveal;
            self.moves = moves;
            self.status = status;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn place_traps(&mut self, safe: usize) {
        let mut placed = 0;
        while placed < self.traps {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let index = (self.seed as usize) % self.cells.len();
            if index == safe || index == self.exit || matches!(self.cells[index], DungeonCell::Trap)
            {
                continue;
            }
            self.cells[index] = DungeonCell::Trap;
            placed += 1;
        }
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.cells.clone(),
            self.seed,
            self.first_reveal,
            self.moves,
            self.status,
        ));
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
