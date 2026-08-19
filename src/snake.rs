//! Deterministic real-time Snake with visible touch direction controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i32 = 16;
pub const HEIGHT: i32 = 12;
const TARGET_SCORE: u16 = 20;
const MOVE_INTERVAL: f32 = 0.16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnakeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl SnakeDirection {
    fn opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnakeStatus {
    Playing,
    Won,
    Lost,
}

type Snapshot = (Vec<u16>, SnakeDirection, u16, u16, u16, SnakeStatus, u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snake {
    pub body: Vec<u16>,
    pub direction: SnakeDirection,
    pub food: u16,
    pub score: u16,
    pub moves: u16,
    pub status: SnakeStatus,
    pub seed: u64,
    #[serde(default)]
    pub paused: bool,
    #[serde(skip)]
    undo: Option<Snapshot>,
    #[serde(skip)]
    elapsed: f32,
}

impl Default for Snake {
    fn default() -> Self {
        Self::new(0x5AAE_0001)
    }
}

impl Snake {
    pub fn new(seed: u64) -> Self {
        let center = (HEIGHT / 2 * WIDTH + WIDTH / 2) as u16;
        let mut game = Self {
            body: vec![center, center - 1, center - 2],
            direction: SnakeDirection::Right,
            food: 0,
            score: 0,
            moves: 0,
            status: SnakeStatus::Playing,
            seed,
            paused: false,
            undo: None,
            elapsed: 0.,
        };
        game.food = game.next_food();
        game
    }

    pub fn set_direction(&mut self, direction: SnakeDirection) -> bool {
        if self.status != SnakeStatus::Playing || direction.opposite(self.direction) {
            return false;
        }
        self.direction = direction;
        true
    }

    #[cfg(test)]
    pub fn step(&mut self, direction: SnakeDirection) -> bool {
        if !self.set_direction(direction) {
            return false;
        }
        self.advance_one()
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != SnakeStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= MOVE_INTERVAL && self.status == SnakeStatus::Playing {
            self.elapsed -= MOVE_INTERVAL;
            advanced |= self.advance_one();
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != SnakeStatus::Playing {
            return false;
        }
        self.paused = !self.paused;
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    fn advance_one(&mut self) -> bool {
        self.undo = Some((
            self.body.clone(),
            self.direction,
            self.food,
            self.score,
            self.moves,
            self.status,
            self.seed,
        ));
        let head = self.body[0] as i32;
        let row = head / WIDTH;
        let column = head % WIDTH;
        let (row_step, column_step) = match self.direction {
            SnakeDirection::Up => (-1, 0),
            SnakeDirection::Right => (0, 1),
            SnakeDirection::Down => (1, 0),
            SnakeDirection::Left => (0, -1),
        };
        let next_row = row + row_step;
        let next_column = column + column_step;
        self.moves = self.moves.saturating_add(1);
        if !(0..HEIGHT).contains(&next_row) || !(0..WIDTH).contains(&next_column) {
            self.status = SnakeStatus::Lost;
            return true;
        }
        let next = (next_row * WIDTH + next_column) as u16;
        let eating = next == self.food;
        if self.body.contains(&next) && (eating || self.body[..self.body.len() - 1].contains(&next))
        {
            self.status = SnakeStatus::Lost;
            return true;
        }
        self.body.insert(0, next);
        if eating {
            self.score = self.score.saturating_add(1);
            if self.score >= TARGET_SCORE {
                self.status = SnakeStatus::Won;
            } else {
                self.food = self.next_food();
            }
        } else {
            self.body.pop();
        }
        true
    }

    pub fn hint_direction(&self) -> Option<SnakeDirection> {
        if self.status != SnakeStatus::Playing {
            return None;
        }
        [
            SnakeDirection::Up,
            SnakeDirection::Right,
            SnakeDirection::Down,
            SnakeDirection::Left,
        ]
        .into_iter()
        .filter(|&direction| !direction.opposite(self.direction))
        .filter_map(|direction| {
            let next = self.next_cell(direction)?;
            self.is_safe(next)
                .then_some((direction, self.food_distance(next)))
        })
        .min_by_key(|(_, distance)| *distance)
        .map(|(direction, _)| direction)
    }

    pub fn undo(&mut self) -> bool {
        if let Some((body, direction, food, score, moves, status, seed)) = self.undo.take() {
            self.body = body;
            self.direction = direction;
            self.food = food;
            self.score = score;
            self.moves = moves;
            self.status = status;
            self.seed = seed;
            self.elapsed = 0.;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn next_food(&mut self) -> u16 {
        for _ in 0..(WIDTH * HEIGHT) {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let candidate = (self.seed % (WIDTH * HEIGHT) as u64) as u16;
            if !self.body.contains(&candidate) {
                return candidate;
            }
        }
        0
    }

    fn next_cell(&self, direction: SnakeDirection) -> Option<u16> {
        let head = self.body.first().copied()? as i32;
        let row = head / WIDTH;
        let column = head % WIDTH;
        let (row_step, column_step) = match direction {
            SnakeDirection::Up => (-1, 0),
            SnakeDirection::Right => (0, 1),
            SnakeDirection::Down => (1, 0),
            SnakeDirection::Left => (0, -1),
        };
        let next_row = row + row_step;
        let next_column = column + column_step;
        (0..HEIGHT)
            .contains(&next_row)
            .then_some(())
            .filter(|_| (0..WIDTH).contains(&next_column))
            .map(|_| (next_row * WIDTH + next_column) as u16)
    }

    fn is_safe(&self, next: u16) -> bool {
        let eating = next == self.food;
        !self.body.contains(&next) || (!eating && !self.body[..self.body.len() - 1].contains(&next))
    }

    fn food_distance(&self, next: u16) -> i32 {
        let row = i32::from(next) / WIDTH;
        let column = i32::from(next) % WIDTH;
        let food_row = i32::from(self.food) / WIDTH;
        let food_column = i32::from(self.food) % WIDTH;
        (row - food_row).abs() + (column - food_column).abs()
    }
}

#[cfg(test)]
mod tests;
