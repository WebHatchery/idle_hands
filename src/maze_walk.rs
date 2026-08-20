//! Deterministic touch-first Maze Walk navigation puzzle.
use crate::undo::UndoStack;

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::domain::Direction;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MazeMode {
    #[default]
    Explorer,
    Fog,
}

impl MazeMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Explorer => "EXPLORE",
            Self::Fog => "FOG",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeWalk {
    pub walls: Vec<u8>,
    pub player: usize,
    pub goal: usize,
    #[serde(default)]
    pub beacons: Vec<usize>,
    #[serde(default)]
    pub collected: Vec<usize>,
    #[serde(default)]
    pub visited: Vec<bool>,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub mode: MazeMode,
    #[serde(default)]
    pub par: u16,
    pub phase: MazePhase,
    #[serde(skip)]
    history: UndoStack<Self>,
}

impl Default for MazeWalk {
    fn default() -> Self {
        Self::new(0x0000_4D41_5A45)
    }
}

impl MazeWalk {
    pub fn new(seed: u64) -> Self {
        Self::new_with_mode(seed, MazeMode::Explorer)
    }

    pub fn new_with_mode(mut seed: u64, mode: MazeMode) -> Self {
        let original_seed = seed;
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
        let candidates = [SIDE + 1, SIDE * 3 + 2, SIDE * 4 + 4, SIDE * 5 + 1];
        let first = (original_seed as usize) % candidates.len();
        let mut second = ((original_seed >> 3) as usize + 2) % candidates.len();
        if second == first {
            second = (second + 1) % candidates.len();
        }
        let beacons = vec![candidates[first], candidates[second]];
        let par = route_par(&walls, &beacons, CELLS - 1);
        let mut visited = vec![false; CELLS];
        visited[0] = true;
        Self {
            walls,
            player: 0,
            goal: CELLS - 1,
            beacons,
            collected: Vec::new(),
            visited,
            moves: 0,
            seed: original_seed,
            mode,
            par,
            phase: MazePhase::Playing,
            history: UndoStack::default(),
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
        let previous = self.clone_without_history();
        self.player = next;
        self.moves = self.moves.saturating_add(1);
        self.ensure_visited_shape();
        self.visited[next] = true;
        if self.beacons.contains(&next) && !self.collected.contains(&next) {
            self.collected.push(next);
        }
        if self.player == self.goal && self.collected.len() == self.beacons.len() {
            self.phase = MazePhase::Won;
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

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_mode(seed, self.mode);
    }
    pub fn set_mode(&mut self, mode: MazeMode, seed: u64) {
        *self = Self::new_with_mode(seed, mode);
    }
    pub fn won(&self) -> bool {
        self.phase == MazePhase::Won
    }

    pub fn hint_direction(&self) -> Option<Direction> {
        if self.phase != MazePhase::Playing {
            return None;
        }
        let target = self.next_objective()?;
        shortest_first(&self.walls, self.player, target)
    }

    pub fn next_objective(&self) -> Option<usize> {
        self.beacons
            .iter()
            .copied()
            .filter(|beacon| !self.collected.contains(beacon))
            .min_by_key(|&beacon| shortest_distance(&self.walls, self.player, beacon))
            .or(Some(self.goal))
    }

    pub fn distance_to_objective(&self) -> usize {
        self.next_objective().map_or(0, |target| {
            shortest_distance(&self.walls, self.player, target)
        })
    }

    pub fn is_visible(&self, index: usize) -> bool {
        if self.mode == MazeMode::Explorer || self.visited.get(index).copied().unwrap_or(false) {
            return true;
        }
        let row = index / SIDE;
        let col = index % SIDE;
        let player_row = self.player / SIDE;
        let player_col = self.player % SIDE;
        row.abs_diff(player_row) + col.abs_diff(player_col) == 1
    }

    pub fn can_step(&self, direction: Direction) -> bool {
        self.phase == MazePhase::Playing && self.walls[self.player] & direction_bit(direction) == 0
    }

    fn ensure_visited_shape(&mut self) {
        if self.visited.len() != CELLS {
            self.visited.resize(CELLS, false);
        }
    }

    fn clone_without_history(&self) -> Self {
        let mut copy = self.clone();
        copy.history.clear();
        copy
    }
}

fn shortest_first(walls: &[u8], start: usize, target: usize) -> Option<Direction> {
    let mut queue = VecDeque::from([(start, None)]);
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
            if walls[index] & direction_bit(direction) != 0 {
                continue;
            }
            let next = neighbor(index, direction).expect("open edge has a neighbor");
            let first = first.or(Some(direction));
            if next == target {
                return first;
            }
            queue.push_back((next, first));
        }
    }
    None
}

fn shortest_distance(walls: &[u8], start: usize, target: usize) -> usize {
    if start == target {
        return 0;
    }
    let mut queue = VecDeque::from([(start, 0usize)]);
    let mut visited = [false; CELLS];
    while let Some((index, distance)) = queue.pop_front() {
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
            if walls[index] & direction_bit(direction) != 0 {
                continue;
            }
            let next = neighbor(index, direction).expect("open edge has a neighbor");
            if next == target {
                return distance + 1;
            }
            queue.push_back((next, distance + 1));
        }
    }
    usize::MAX / 2
}

fn route_par(walls: &[u8], beacons: &[usize], goal: usize) -> u16 {
    let first = shortest_distance(walls, 0, beacons[0])
        + shortest_distance(walls, beacons[0], beacons[1])
        + shortest_distance(walls, beacons[1], goal);
    let second = shortest_distance(walls, 0, beacons[1])
        + shortest_distance(walls, beacons[1], beacons[0])
        + shortest_distance(walls, beacons[0], goal);
    first.min(second).min(u16::MAX as usize) as u16
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
