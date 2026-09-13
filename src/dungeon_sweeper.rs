//! Deterministic dungeon pathfinding with Minesweeper-style trap clues.

use serde::{Deserialize, Serialize};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DungeonDifficulty {
    Explorer,
    #[default]
    Delver,
    Peril,
}

impl DungeonDifficulty {
    pub const ALL: [Self; 3] = [Self::Explorer, Self::Delver, Self::Peril];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Explorer => "EXPLORER",
            Self::Delver => "DELVER",
            Self::Peril => "PERIL",
        }
    }

    const fn traps(self) -> usize {
        match self {
            Self::Explorer => 10,
            Self::Delver => 12,
            Self::Peril => 15,
        }
    }

    const fn hearts(self) -> u8 {
        match self {
            Self::Explorer => 3,
            Self::Delver => 2,
            Self::Peril => 1,
        }
    }

    const fn relics(self) -> u8 {
        match self {
            Self::Explorer => 2,
            Self::Delver | Self::Peril => 3,
        }
    }
}

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

#[derive(Debug, Clone)]
struct Snapshot {
    cells: Vec<DungeonCell>,
    seed: u64,
    first_reveal: bool,
    moves: u16,
    status: DungeonStatus,
    hearts: u8,
    relics: Vec<usize>,
    collected_relics: Vec<usize>,
}

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
    #[serde(default)]
    pub difficulty: DungeonDifficulty,
    #[serde(default = "default_hearts")]
    pub hearts: u8,
    #[serde(default)]
    pub relics: Vec<usize>,
    #[serde(default)]
    pub collected_relics: Vec<usize>,
    #[serde(default)]
    pub required_relics: u8,
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
        Self::new_with_difficulty(seed, DungeonDifficulty::Delver)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: DungeonDifficulty) -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            traps: difficulty.traps(),
            cells: vec![DungeonCell::Hidden; WIDTH * HEIGHT],
            exit: WIDTH * HEIGHT - 1,
            moves: 0,
            status: DungeonStatus::Ready,
            seed,
            first_reveal: false,
            difficulty,
            hearts: difficulty.hearts(),
            relics: Vec::new(),
            collected_relics: Vec::new(),
            required_relics: difficulty.relics(),
            history: Vec::new(),
        }
    }

    pub fn relics_found(&self) -> u8 {
        self.collected_relics.len().min(u8::MAX as usize) as u8
    }

    pub fn relic_total(&self) -> u8 {
        self.required_relics
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
            self.snapshot();
            self.place_traps(index);
            self.place_relics(index);
            self.first_reveal = true;
            self.status = DungeonStatus::Playing;
        } else {
            self.snapshot();
        }
        self.moves = self.moves.saturating_add(1);
        if matches!(self.cells[index], DungeonCell::Trap) {
            self.cells[index] = DungeonCell::Revealed(9);
            self.hearts = self.hearts.saturating_sub(1);
            if self.hearts == 0 {
                self.status = DungeonStatus::Lost;
            }
        } else {
            self.cells[index] = DungeonCell::Revealed(self.adjacent_traps(index));
            self.collect_relic_at(index);
            self.finish_if_ready();
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
                self.hearts = self.hearts.saturating_sub(1);
                if self.hearts == 0 {
                    self.status = DungeonStatus::Lost;
                    break;
                }
                continue;
            }
            self.cells[neighbor] = DungeonCell::Revealed(self.adjacent_traps(neighbor));
            self.collect_relic_at(neighbor);
            self.finish_if_ready();
            if self.status == DungeonStatus::Won {
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
        self.snapshot();
        match self.cells[index] {
            DungeonCell::Hidden => self.cells[index] = DungeonCell::Flagged,
            DungeonCell::Trap => self.cells[index] = DungeonCell::FlaggedTrap,
            DungeonCell::Flagged => self.cells[index] = DungeonCell::Hidden,
            DungeonCell::FlaggedTrap => self.cells[index] = DungeonCell::Trap,
            _ => {
                self.history.pop();
                return false;
            }
        }
        true
    }

    pub fn flagged_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| matches!(cell, DungeonCell::Flagged | DungeonCell::FlaggedTrap))
            .count()
    }

    pub fn hint_cell(&self) -> Option<usize> {
        if !matches!(self.status, DungeonStatus::Ready | DungeonStatus::Playing) {
            return None;
        }
        if !self.first_reveal {
            return self.cells.iter().enumerate().find_map(|(index, cell)| {
                (index != self.exit && *cell == DungeonCell::Hidden).then_some(index)
            });
        }
        if self.relics_found() >= self.relic_total() {
            return (!matches!(self.cells[self.exit], DungeonCell::Revealed(_)))
                .then_some(self.exit);
        }
        if let Some(relic) = self.relics.iter().copied().find(|relic| {
            !self.collected_relics.contains(relic)
                && matches!(self.cells[*relic], DungeonCell::Hidden)
        }) {
            return Some(relic);
        }
        self.cells.iter().enumerate().find_map(|(index, cell)| {
            (matches!(cell, DungeonCell::Hidden) && index != self.exit && !self.is_trap(index))
                .then_some(index)
        })
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
        if let Some(snapshot) = self.history.pop() {
            self.cells = snapshot.cells;
            self.seed = snapshot.seed;
            self.first_reveal = snapshot.first_reveal;
            self.moves = snapshot.moves;
            self.status = snapshot.status;
            self.hearts = snapshot.hearts;
            self.relics = snapshot.relics;
            self.collected_relics = snapshot.collected_relics;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }

    fn place_traps(&mut self, safe: usize) {
        let mut placed = 0;
        while placed < self.traps {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let index = (self.seed as usize) % self.cells.len();
            if index == safe
                || index == self.exit
                || !matches!(self.cells[index], DungeonCell::Hidden)
            {
                continue;
            }
            self.cells[index] = DungeonCell::Trap;
            placed += 1;
        }
    }

    fn place_relics(&mut self, safe: usize) {
        while self.relics.len() < self.required_relics as usize {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let index = (self.seed as usize) % self.cells.len();
            if index == safe
                || index == self.exit
                || !matches!(self.cells[index], DungeonCell::Hidden)
                || self.relics.contains(&index)
            {
                continue;
            }
            self.relics.push(index);
        }
    }

    fn collect_relic_at(&mut self, index: usize) {
        if self.relics.contains(&index) && !self.collected_relics.contains(&index) {
            self.collected_relics.push(index);
        }
    }

    fn finish_if_ready(&mut self) {
        if self.relics_found() >= self.relic_total()
            && matches!(self.cells[self.exit], DungeonCell::Revealed(_))
        {
            self.status = DungeonStatus::Won;
        }
    }

    fn is_trap(&self, index: usize) -> bool {
        matches!(
            self.cells[index],
            DungeonCell::Trap | DungeonCell::FlaggedTrap
        )
    }

    fn snapshot(&mut self) {
        self.history.push(Snapshot {
            cells: self.cells.clone(),
            seed: self.seed,
            first_reveal: self.first_reveal,
            moves: self.moves,
            status: self.status,
            hearts: self.hearts,
            relics: self.relics.clone(),
            collected_relics: self.collected_relics.clone(),
        });
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

fn default_hearts() -> u8 {
    2
}

#[cfg(test)]
#[path = "../tests/legacy/dungeon_sweeper/tests.rs"]
mod tests;
