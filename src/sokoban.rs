//! Deterministic compact Sokoban rules.

use crate::state::Direction;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;
const CELLS: usize = WIDTH * HEIGHT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SokobanPhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sokoban {
    pub tiles: Vec<u8>,
    pub player: usize,
    pub crates: u8,
    pub moves: u16,
    pub seed: u64,
    pub phase: SokobanPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for Sokoban {
    fn default() -> Self {
        Self::new(0x50C0_BA0B)
    }
}

impl Sokoban {
    pub fn new(seed: u64) -> Self {
        let mut tiles = vec![0; CELLS];
        let rows = [
            "########", "# .    #", "# $    #", "#   $ .#", "#      #", "#  @   #", "#      #",
            "########",
        ];
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
            seed,
            phase: SokobanPhase::Playing,
            undo: None,
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
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
        self.undo = Some(Box::new(previous));
        self.player = next;
        self.moves = self.moves.saturating_add(1);
        if pushed && self.tiles.iter().filter(|tile| **tile == 4).count() == self.crates as usize {
            self.phase = SokobanPhase::Won;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.take() else {
            return false;
        };
        *self = *previous;
        true
    }

    pub fn won(&self) -> bool {
        self.phase == SokobanPhase::Won
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.won() {
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

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    fn is_crate(&self, index: usize) -> bool {
        matches!(self.tiles[index], 3 | 4)
    }

    fn is_target(&self, index: usize) -> bool {
        matches!(self.tiles[index], 2 | 4)
    }

    fn neighbor(&self, index: usize, direction: Direction) -> Option<usize> {
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
}

#[cfg(test)]
mod tests;
