//! Deterministic compact Sokoban rules.
use crate::undo::UndoStack;

use crate::domain::Direction;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
pub const CELLS: usize = WIDTH * HEIGHT;
pub const LEVEL_COUNT: u8 = 6;
pub const PAR_MOVES: [u16; LEVEL_COUNT as usize] = [10, 12, 20, 2, 8, 8];

pub const LEVELS: [[&str; HEIGHT]; LEVEL_COUNT as usize] = [
    [
        "########", "# .    #", "# $    #", "#   $ .#", "#      #", "#  @   #", "#      #",
        "########",
    ],
    [
        "########", "#  .   #", "#  $   #", "#  @   #", "# . $  #", "#      #", "#      #",
        "########",
    ],
    [
        "########", "# .    #", "# $ $ .#", "#   @  #", "# .    #", "#      #", "#      #",
        "########",
    ],
    [
        "########", "#  .   #", "#  $   #", "#  @   #", "#      #", "#      #", "#      #",
        "########",
    ],
    [
        "########", "# . .  #", "# $ $  #", "#  @   #", "#      #", "#      #", "#      #",
        "########",
    ],
    [
        "########", "#  ..  #", "#  $$  #", "#   @  #", "#      #", "#      #", "#      #",
        "########",
    ],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SokobanPhase {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sokoban {
    pub tiles: Vec<u8>,
    pub player: usize,
    pub crates: u8,
    pub moves: u16,
    #[serde(default)]
    pub pushes: u16,
    pub seed: u64,
    #[serde(default)]
    pub level: u8,
    pub phase: SokobanPhase,
    #[serde(skip)]
    pub history: UndoStack<Self>,
}

impl Default for Sokoban {
    fn default() -> Self {
        Self::new(0x50C0_BA0B)
    }
}

impl Sokoban {
    pub fn new(seed: u64) -> Self {
        Self::new_with_level(seed, 0)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        Self::new_with_level(seed, level_from_seed(seed))
    }

    pub fn new_with_level(seed: u64, level: u8) -> Self {
        let mut tiles = vec![0; CELLS];
        let level = level % LEVEL_COUNT;
        let rows = LEVELS[level as usize];
        let mut player = 0;
        let mut crates = 0;
        for (row, line) in rows.iter().enumerate() {
            for (col, symbol) in line.bytes().enumerate() {
                let index = row * WIDTH + col;
                tiles[index] = match symbol {
                    b'#' => 0,
                    b'.' => 2,
                    b'$' => {
                        crates += 1;
                        3
                    }
                    b'@' => {
                        player = index;
                        1
                    }
                    _ => 1,
                };
            }
        }
        Self {
            tiles,
            player,
            crates,
            moves: 0,
            pushes: 0,
            seed,
            level,
            phase: SokobanPhase::Playing,
            history: UndoStack::default(),
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_level(seed, self.level);
    }

    pub fn reset_next(&mut self, seed: u64) {
        *self = Self::new_with_level(seed, (self.level + 1) % LEVEL_COUNT);
    }

    pub fn par_moves(&self) -> u16 {
        PAR_MOVES[self.level as usize % PAR_MOVES.len()]
    }

    pub fn clear_rank(&self) -> &'static str {
        if !self.won() {
            "—"
        } else if self.moves <= self.par_moves() {
            "GOLD"
        } else if self.moves <= self.par_moves().saturating_add(4) {
            "SILVER"
        } else {
            "BRONZE"
        }
    }

    pub fn move_in(&mut self, direction: Direction) -> bool {
        if self.phase != SokobanPhase::Playing {
            return false;
        }
        let Some(next) = self.neighbor(self.player, direction) else {
            return false;
        };
        if self.tiles[next] == 0 {
            return false;
        }
        let previous = self.clone_without_undo();
        let mut pushed = false;
        if self.is_crate(next) {
            let Some(beyond) = self.neighbor(next, direction) else {
                return false;
            };
            if self.tiles[beyond] == 0 || self.is_crate(beyond) {
                return false;
            }
            let target = self.tiles[beyond] == 2;
            self.tiles[beyond] = if target { 4 } else { 3 };
            self.tiles[next] = if self.is_target(next) { 2 } else { 1 };
            pushed = true;
        }
        self.player = next;
        self.moves = self.moves.saturating_add(1);
        if pushed {
            self.pushes = self.pushes.saturating_add(1);
            if self.tiles.iter().filter(|tile| **tile == 4).count() == self.crates as usize {
                self.phase = SokobanPhase::Won;
            } else if self.has_deadlock() {
                self.phase = SokobanPhase::Stuck;
            }
        }
        self.history.push(previous);
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

    pub fn won(&self) -> bool {
        self.phase == SokobanPhase::Won
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.phase != SokobanPhase::Playing {
            return None;
        }
        let mut queue = VecDeque::from([(self.clone_without_undo(), None)]);
        let mut seen = Vec::new();
        while let Some((state, first)) = queue.pop_front() {
            let key = (state.player, state.tiles.clone());
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            for direction in [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ] {
                let mut next = state.clone_without_undo();
                if !next.move_in(direction) {
                    continue;
                }
                let first = first.or(Some(direction));
                if next.won() {
                    return first;
                }
                queue.push_back((next, first));
            }
        }
        None
    }

    pub fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }

    pub fn is_crate(&self, index: usize) -> bool {
        matches!(self.tiles[index], 3 | 4)
    }

    pub fn is_target(&self, index: usize) -> bool {
        matches!(self.tiles[index], 2 | 4)
    }

    pub fn neighbor(&self, index: usize, direction: Direction) -> Option<usize> {
        let row = index / WIDTH;
        let col = index % WIDTH;
        let (row, col) = match direction {
            Direction::Up => (row.checked_sub(1)?, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.checked_sub(1)?),
        };
        (row < HEIGHT && col < WIDTH).then_some(row * WIDTH + col)
    }

    pub fn is_deadlocked_crate(&self, index: usize) -> bool {
        if !self.is_crate(index) || self.is_target(index) {
            return false;
        }
        let vertical_wall = [Direction::Up, Direction::Down]
            .into_iter()
            .any(|direction| {
                self.neighbor(index, direction)
                    .is_none_or(|cell| self.tiles[cell] == 0)
            });
        let horizontal_wall = [Direction::Left, Direction::Right]
            .into_iter()
            .any(|direction| {
                self.neighbor(index, direction)
                    .is_none_or(|cell| self.tiles[cell] == 0)
            });
        vertical_wall && horizontal_wall
    }

    pub fn has_deadlock(&self) -> bool {
        (0..self.tiles.len()).any(|index| self.is_deadlocked_crate(index))
    }
}

pub fn level_from_seed(seed: u64) -> u8 {
    ((seed ^ seed.rotate_left(23)) % u64::from(LEVEL_COUNT)) as u8
}
