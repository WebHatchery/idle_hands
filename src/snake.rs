//! Deterministic turn-based Snake with visible touch direction controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i32 = 16;
pub const HEIGHT: i32 = 12;
const TARGET_SCORE: u16 = 20;

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
    #[serde(skip)]
    undo: Option<Snapshot>,
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
            undo: None,
        };
        game.food = game.next_food();
        game
    }

    pub fn step(&mut self, direction: SnakeDirection) -> bool {
        if self.status != SnakeStatus::Playing || direction.opposite(self.direction) {
            return false;
        }
        self.undo = Some((
            self.body.clone(),
            self.direction,
            self.food,
            self.score,
            self.moves,
            self.status,
            self.seed,
        ));
        self.direction = direction;
        let head = self.body[0] as i32;
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

    pub fn undo(&mut self) -> bool {
        if let Some((body, direction, food, score, moves, status, seed)) = self.undo.take() {
            self.body = body;
            self.direction = direction;
            self.food = food;
            self.score = score;
            self.moves = moves;
            self.status = status;
            self.seed = seed;
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
}

#[cfg(test)]
mod tests;
