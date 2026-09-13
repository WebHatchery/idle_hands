//! Touch-first, deterministic maze chase inspired by classic pellet runs.

use crate::domain::Direction;
use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 19;
pub const HEIGHT: u8 = 15;
const STEP_INTERVAL: f32 = 0.16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MunchStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ghost {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
}

#[derive(Debug, Clone)]
struct Snapshot {
    player: usize,
    ghosts: Vec<Ghost>,
    pellets: Vec<bool>,
    score: u16,
    moves: u16,
    lives: u8,
    status: MunchStatus,
    paused: bool,
    seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MunchMaze {
    pub player: usize,
    pub ghosts: Vec<Ghost>,
    pub walls: Vec<bool>,
    pub pellets: Vec<bool>,
    pub score: u16,
    pub moves: u16,
    pub lives: u8,
    pub status: MunchStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
    #[serde(skip, default = "default_direction")]
    direction: Direction,
    #[serde(skip, default = "default_direction")]
    desired: Direction,
}

impl Default for MunchMaze {
    fn default() -> Self {
        Self::new(0xC0DE_0001)
    }
}

impl MunchMaze {
    pub fn new(seed: u64) -> Self {
        let walls = build_walls();
        let mut pellets = walls.iter().map(|wall| !wall).collect::<Vec<_>>();
        let player = cell(1, 1);
        pellets[player] = false;
        let ghosts = vec![
            Ghost {
                x: 17,
                y: 1,
                kind: 0,
            },
            Ghost {
                x: 17,
                y: 13,
                kind: 1,
            },
            Ghost {
                x: 9,
                y: 13,
                kind: 2,
            },
            Ghost {
                x: 9,
                y: 1,
                kind: 3,
            },
        ];
        for ghost in &ghosts {
            pellets[cell(ghost.x, ghost.y)] = false;
        }
        Self {
            player,
            ghosts,
            walls,
            pellets,
            score: 0,
            moves: 0,
            lives: 3,
            status: MunchStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
            direction: Direction::Right,
            desired: Direction::Right,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.status == MunchStatus::Playing {
            self.desired = direction;
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != MunchStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == MunchStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one();
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != MunchStatus::Playing {
            return false;
        }
        self.paused = !self.paused;
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo.take() else {
            return false;
        };
        self.player = snapshot.player;
        self.ghosts = snapshot.ghosts;
        self.pellets = snapshot.pellets;
        self.score = snapshot.score;
        self.moves = snapshot.moves;
        self.lives = snapshot.lives;
        self.status = snapshot.status;
        self.paused = snapshot.paused;
        self.seed = snapshot.seed;
        self.elapsed = 0.;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        let mode = self.mode;
        *self = Self::new(seed);
        self.mode = mode;
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "PATROL"
        } else {
            "PURSUIT"
        }
    }

    pub fn is_wall(&self, x: u8, y: u8) -> bool {
        x >= WIDTH || y >= HEIGHT || self.walls[cell(x, y)]
    }

    fn advance_one(&mut self) {
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        if can_step(self.player, self.desired, &self.walls) {
            self.direction = self.desired;
        }
        if can_step(self.player, self.direction, &self.walls) {
            self.player = next_cell(self.player, self.direction);
        }
        if self.pellets[self.player] {
            self.pellets[self.player] = false;
            self.score = self.score.saturating_add(10);
        }
        for index in 0..self.ghosts.len() {
            let direction = self.ghost_direction(index);
            let current = cell(self.ghosts[index].x, self.ghosts[index].y);
            if can_step(current, direction, &self.walls) {
                let next = next_cell(current, direction);
                self.ghosts[index].x = (next % usize::from(WIDTH)) as u8;
                self.ghosts[index].y = (next / usize::from(WIDTH)) as u8;
            }
        }
        if self
            .ghosts
            .iter()
            .any(|ghost| cell(ghost.x, ghost.y) == self.player)
        {
            self.lives = self.lives.saturating_sub(1);
            if self.lives == 0 {
                self.status = MunchStatus::Lost;
                self.paused = false;
            } else {
                self.player = cell(1, 1);
            }
        } else if self.pellets.iter().all(|pellet| !*pellet) {
            self.status = MunchStatus::Won;
            self.paused = false;
        }
    }

    fn ghost_direction(&self, index: usize) -> Direction {
        let ghost = self.ghosts[index];
        let choices = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let mut best = Direction::Left;
        let mut best_distance = usize::MAX;
        for (offset, direction) in choices.into_iter().enumerate() {
            if !can_step(cell(ghost.x, ghost.y), direction, &self.walls) {
                continue;
            }
            let next = next_cell(cell(ghost.x, ghost.y), direction);
            let x = next % usize::from(WIDTH);
            let y = next / usize::from(WIDTH);
            let player_x = self.player % usize::from(WIDTH);
            let player_y = self.player / usize::from(WIDTH);
            let distance = x.abs_diff(player_x) + y.abs_diff(player_y) + (offset + index) % 2;
            if distance < best_distance {
                best_distance = distance;
                best = direction;
            }
        }
        best
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            player: self.player,
            ghosts: self.ghosts.clone(),
            pellets: self.pellets.clone(),
            score: self.score,
            moves: self.moves,
            lives: self.lives,
            status: self.status,
            paused: self.paused,
            seed: self.seed,
        }));
    }
}

fn build_walls() -> Vec<bool> {
    let mut walls = vec![false; usize::from(WIDTH) * usize::from(HEIGHT)];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let border = x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1;
            let horizontal = matches!(y, 3 | 7 | 11) && x > 1 && x < WIDTH - 2 && x % 6 != 3;
            let vertical = matches!(x, 5 | 13) && y > 1 && y < HEIGHT - 2 && y % 4 != 2;
            walls[cell(x, y)] = border || horizontal || vertical;
        }
    }
    for (x, y) in [(1, 1), (17, 1), (17, 13), (9, 13), (9, 1)] {
        walls[cell(x, y)] = false;
    }
    walls
}

fn cell(x: u8, y: u8) -> usize {
    usize::from(y) * usize::from(WIDTH) + usize::from(x)
}

fn can_step(position: usize, direction: Direction, walls: &[bool]) -> bool {
    let x = position % usize::from(WIDTH);
    let y = position / usize::from(WIDTH);
    let (next_x, next_y) = match direction {
        Direction::Up => (x, y.saturating_sub(1)),
        Direction::Right => (x.saturating_add(1), y),
        Direction::Down => (x, y.saturating_add(1)),
        Direction::Left => (x.saturating_sub(1), y),
    };
    next_x < usize::from(WIDTH)
        && next_y < usize::from(HEIGHT)
        && !walls[next_y * usize::from(WIDTH) + next_x]
}

fn next_cell(position: usize, direction: Direction) -> usize {
    let x = position % usize::from(WIDTH);
    let y = position / usize::from(WIDTH);
    let (next_x, next_y) = match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    };
    next_y * usize::from(WIDTH) + next_x
}

fn default_direction() -> Direction {
    Direction::Right
}

#[cfg(test)]
#[path = "../tests/legacy/munch_maze/tests.rs"]
mod tests;
