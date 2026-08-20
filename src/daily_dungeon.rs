//! A deterministic daily route through a hidden six-by-six dungeon floor.

use crate::state::Direction;
use serde::{Deserialize, Serialize};

const SIZE: usize = 6;
const CELLS: usize = SIZE * SIZE;
const RUNES: usize = 3;
const START_HEARTS: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DailyTile {
    Floor,
    Trap,
    Rune,
    Exit,
    Spring,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DailyRule {
    #[default]
    Wayfinder,
    Forager,
    Daredevil,
}

impl DailyRule {
    pub const ALL: [Self; 3] = [Self::Wayfinder, Self::Forager, Self::Daredevil];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Wayfinder => "WAYFINDER",
            Self::Forager => "FORAGER",
            Self::Daredevil => "DAREDEVIL",
        }
    }

    const fn trap_count(self) -> usize {
        match self {
            Self::Wayfinder => 8,
            Self::Forager => 7,
            Self::Daredevil => 9,
        }
    }

    const fn spring_count(self) -> usize {
        match self {
            Self::Forager => 2,
            Self::Wayfinder | Self::Daredevil => 1,
        }
    }

    const fn starting_scouts(self) -> u8 {
        match self {
            Self::Wayfinder => 3,
            Self::Forager => 2,
            Self::Daredevil => 1,
        }
    }

    const fn bravery_score(self) -> u32 {
        match self {
            Self::Daredevil => 4,
            Self::Wayfinder | Self::Forager => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DailyPhase {
    Exploring,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyHint {
    Scout,
    Move(Direction),
}

type Snapshot = (
    usize,
    Vec<DailyTile>,
    Vec<bool>,
    u8,
    u8,
    u8,
    u16,
    u32,
    u64,
    DailyPhase,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyDungeon {
    pub challenge: u32,
    pub player: usize,
    pub tiles: Vec<DailyTile>,
    pub revealed: Vec<bool>,
    pub hearts: u8,
    #[serde(default = "default_scouts")]
    pub scouts: u8,
    pub runes_found: u8,
    pub moves: u16,
    pub score: u32,
    pub seed: u64,
    pub phase: DailyPhase,
    #[serde(default)]
    pub rule: DailyRule,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for DailyDungeon {
    fn default() -> Self {
        Self::new(0xDA11_0001)
    }
}

impl DailyDungeon {
    pub fn new(seed: u64) -> Self {
        let rule = match seed % 3 {
            0 => DailyRule::Wayfinder,
            1 => DailyRule::Forager,
            _ => DailyRule::Daredevil,
        };
        Self::new_with_rule(seed, rule)
    }

    pub fn new_with_rule(seed: u64, rule: DailyRule) -> Self {
        let mut dungeon = Self {
            challenge: seed as u32 % 10_000,
            player: 0,
            tiles: vec![DailyTile::Floor; CELLS],
            revealed: vec![false; CELLS],
            hearts: START_HEARTS,
            scouts: rule.starting_scouts(),
            runes_found: 0,
            moves: 0,
            score: 0,
            seed,
            phase: DailyPhase::Exploring,
            rule,
            history: Vec::new(),
        };
        dungeon.place_tiles();
        dungeon.revealed[0] = true;
        dungeon
    }

    pub const fn size() -> usize {
        SIZE
    }

    pub const fn rune_total() -> u8 {
        RUNES as u8
    }

    pub fn move_in(&mut self, direction: Direction) -> bool {
        if self.phase != DailyPhase::Exploring {
            return false;
        }
        let Some(destination) = self.destination(direction) else {
            return false;
        };
        self.snapshot();
        let unexplored = !self.revealed[destination];
        self.player = destination;
        self.moves = self.moves.saturating_add(1);
        self.revealed[destination] = true;
        if unexplored {
            self.score = self.score.saturating_add(self.rule.bravery_score());
        }
        match self.tiles[destination] {
            DailyTile::Trap => {
                self.hearts = self.hearts.saturating_sub(1);
                self.tiles[destination] = DailyTile::Floor;
                self.score = self.score.saturating_add(2);
                if self.hearts == 0 {
                    self.phase = DailyPhase::Lost;
                }
            }
            DailyTile::Rune => {
                self.runes_found = self.runes_found.saturating_add(1);
                self.tiles[destination] = DailyTile::Floor;
                self.score = self.score.saturating_add(15);
            }
            DailyTile::Exit if self.runes_found == RUNES as u8 => {
                self.phase = DailyPhase::Won;
                self.score = self
                    .score
                    .saturating_add(50 + u32::from(self.hearts) * 10 + u32::from(self.scouts) * 5);
            }
            DailyTile::Spring => {
                self.hearts = self.hearts.saturating_add(1).min(START_HEARTS);
                self.tiles[destination] = DailyTile::Floor;
                self.score = self.score.saturating_add(8);
            }
            _ => {}
        }
        true
    }

    pub fn scout(&mut self) -> bool {
        if self.phase != DailyPhase::Exploring || self.scouts == 0 {
            return false;
        }
        let neighbors = Self::neighbors(self.player);
        if !neighbors
            .iter()
            .flatten()
            .any(|position| !self.revealed[*position])
        {
            return false;
        }
        self.snapshot();
        self.scouts -= 1;
        self.moves = self.moves.saturating_add(1);
        for position in neighbors.into_iter().flatten() {
            self.revealed[position] = true;
        }
        true
    }

    pub fn hint_action(&self) -> Option<DailyHint> {
        if self.phase != DailyPhase::Exploring {
            return None;
        }
        if self.scouts > 0
            && Self::neighbors(self.player)
                .iter()
                .flatten()
                .any(|position| !self.revealed[*position])
        {
            return Some(DailyHint::Scout);
        }
        let spring = (self.hearts < START_HEARTS).then(|| {
            self.tiles
                .iter()
                .enumerate()
                .filter(|(index, tile)| self.revealed[*index] && matches!(tile, DailyTile::Spring))
                .min_by_key(|(index, _)| Self::distance(self.player, *index))
                .map(|(index, _)| index)
        });
        let target = spring.flatten().unwrap_or_else(|| {
            if self.runes_found < RUNES as u8 {
                self.tiles
                    .iter()
                    .enumerate()
                    .filter(|(_, tile)| matches!(tile, DailyTile::Rune))
                    .min_by_key(|(index, _)| Self::distance(self.player, *index))
                    .map(|(index, _)| index)
                    .unwrap_or(CELLS - 1)
            } else {
                CELLS - 1
            }
        });
        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|direction| {
            self.destination(direction).map(|destination| {
                let known_trap =
                    self.revealed[destination] && self.tiles[destination] == DailyTile::Trap;
                (direction, known_trap, Self::distance(destination, target))
            })
        })
        .min_by_key(|(_, known_trap, distance)| (*known_trap, *distance))
        .map(|(direction, _, _)| DailyHint::Move(direction))
    }

    pub fn undo(&mut self) -> bool {
        if let Some((
            player,
            tiles,
            revealed,
            hearts,
            scouts,
            runes_found,
            moves,
            score,
            seed,
            phase,
        )) = self.history.pop()
        {
            self.player = player;
            self.tiles = tiles;
            self.revealed = revealed;
            self.hearts = hearts;
            self.scouts = scouts;
            self.runes_found = runes_found;
            self.moves = moves;
            self.score = score;
            self.seed = seed;
            self.phase = phase;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == DailyPhase::Won
    }

    fn destination(&self, direction: Direction) -> Option<usize> {
        let row = self.player / SIZE;
        let column = self.player % SIZE;
        match direction {
            Direction::Up if row > 0 => Some(self.player - SIZE),
            Direction::Right if column + 1 < SIZE => Some(self.player + 1),
            Direction::Down if row + 1 < SIZE => Some(self.player + SIZE),
            Direction::Left if column > 0 => Some(self.player - 1),
            _ => None,
        }
    }

    fn distance(first: usize, second: usize) -> usize {
        let first_row = first / SIZE;
        let first_column = first % SIZE;
        let second_row = second / SIZE;
        let second_column = second % SIZE;
        first_row.abs_diff(second_row) + first_column.abs_diff(second_column)
    }

    fn neighbors(position: usize) -> [Option<usize>; 4] {
        let row = position / SIZE;
        let column = position % SIZE;
        [
            (row > 0).then_some(position.saturating_sub(SIZE)),
            (column > 0).then_some(position.saturating_sub(1)),
            (row + 1 < SIZE).then_some(position + SIZE),
            (column + 1 < SIZE).then_some(position + 1),
        ]
    }

    fn place_tiles(&mut self) {
        let mut occupied = vec![0, CELLS - 1];
        self.tiles[CELLS - 1] = DailyTile::Exit;
        for _ in 0..RUNES {
            let position = self.open_position(&occupied);
            occupied.push(position);
            self.tiles[position] = DailyTile::Rune;
        }
        for _ in 0..self.rule.spring_count() {
            let position = self.open_position(&occupied);
            occupied.push(position);
            self.tiles[position] = DailyTile::Spring;
        }
        for _ in 0..self.rule.trap_count() {
            let position = self.open_position(&occupied);
            occupied.push(position);
            self.tiles[position] = DailyTile::Trap;
        }
    }

    fn open_position(&mut self, occupied: &[usize]) -> usize {
        loop {
            let position = (self.next_random() as usize) % CELLS;
            if !occupied.contains(&position) {
                return position;
            }
        }
    }

    fn next_random(&mut self) -> u64 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.seed
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.player,
            self.tiles.clone(),
            self.revealed.clone(),
            self.hearts,
            self.scouts,
            self.runes_found,
            self.moves,
            self.score,
            self.seed,
            self.phase,
        ));
    }
}

fn default_scouts() -> u8 {
    2
}

#[cfg(test)]
mod tests;
