//! Deterministic touch-first Maze Walk navigation puzzle.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::state::Direction;

pub const SIDE: usize = 7;
const CELLS: usize = SIDE * SIDE;
const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MazePhase {
    Playing,
    Won,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeWalk {
    pub walls: Vec<u8>,
    pub player: usize,
    pub goal: usize,
    pub moves: u16,
    pub seed: u64,
    pub phase: MazePhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
}

impl Default for MazeWalk {
    fn default() -> Self {
        Self::new(0x0000_4D41_5A45)
    }
}

impl MazeWalk {
    pub fn new(mut seed: u64) -> Self {
        let mut walls = vec![0; CELLS];
        for row in 0..SIDE {
            for col in 0..SIDE {
                let index = row * SIDE + col;
                if row == 0 {
                    walls[index] |= UP;
                }
                if row + 1 == SIDE {
                    walls[index] |= DOWN;
                }
                if col == 0 {
                    walls[index] |= LEFT;
                }
                if col + 1 == SIDE {
                    walls[index] |= RIGHT;
                }
            }
        }
        for _ in 0..18 {
            seed = next_seed(seed);
            let row = (seed as usize) % (SIDE - 1);
            seed = next_seed(seed);
            let col = (seed as usize) % (SIDE - 1);
            seed = next_seed(seed);
            let horizontal = seed & 1 == 0;
            let first = row * SIDE + col;
            let second = if horizontal { first + 1 } else { first + SIDE };
            if is_route_edge(first, second) {
                continue;
            }
            let (first_bit, second_bit) = if horizontal {
                (RIGHT, LEFT)
            } else {
                (DOWN, UP)
            };
            walls[first] |= first_bit;
            walls[second] |= second_bit;
        }
        Self {
            walls,
            player: 0,
            goal: CELLS - 1,
            moves: 0,
            seed,
            phase: MazePhase::Playing,
            undo: None,
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        if self.phase != MazePhase::Playing {
            return false;
        }
        let bit = direction_bit(direction);
        if self.walls[self.player] & bit != 0 {
            return false;
        }
        let next = neighbor(self.player, direction).expect("open edge has a neighbor");
        let previous = self.clone_without_undo();
        self.player = next;
        self.moves = self.moves.saturating_add(1);
        self.undo = Some(Box::new(previous));
        if self.player == self.goal {
            self.phase = MazePhase::Won;
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

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }
    pub fn won(&self) -> bool {
        self.phase == MazePhase::Won
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.phase != MazePhase::Playing {
            return None;
        }
        let mut queue = VecDeque::from([(self.player, None)]);
        let mut visited = [false; CELLS];
        while let Some((index, first)) = queue.pop_front() {
            if visited[index] {
                continue;
            }
            visited[index] = true;
            for direction in [
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Up,
            ] {
                if self.walls[index] & direction_bit(direction) != 0 {
                    continue;
                }
                let next = neighbor(index, direction).expect("open edge has a neighbor");
                let first = first.or(Some(direction));
                if next == self.goal {
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
}

fn direction_bit(direction: Direction) -> u8 {
    match direction {
        Direction::Up => UP,
        Direction::Right => RIGHT,
        Direction::Down => DOWN,
        Direction::Left => LEFT,
    }
}

fn neighbor(index: usize, direction: Direction) -> Option<usize> {
    let row = index / SIDE;
    let col = index % SIDE;
    match direction {
        Direction::Up => row.checked_sub(1).map(|next| next * SIDE + col),
        Direction::Right => (col + 1 < SIDE).then_some(index + 1),
        Direction::Down => (row + 1 < SIDE).then_some(index + SIDE),
        Direction::Left => col.checked_sub(1).map(|next| row * SIDE + next),
    }
}

fn is_route_edge(first: usize, second: usize) -> bool {
    let first_row = first / SIDE;
    let first_col = first % SIDE;
    let second_row = second / SIDE;
    let second_col = second % SIDE;
    (first_row == 0 && second_row == 0) || (first_col == SIDE - 1 && second_col == SIDE - 1)
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
