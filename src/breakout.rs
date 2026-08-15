//! Deterministic turn-based Breakout with visible paddle controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i8 = 16;
pub const HEIGHT: i8 = 12;
const BRICK_ROWS: i8 = 4;
const BRICK_COUNT: usize = WIDTH as usize * BRICK_ROWS as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddleMove {
    Left,
    Stay,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakoutStatus {
    Playing,
    Won,
    Lost,
}

type Snapshot = (Vec<bool>, i8, i8, i8, i8, i8, u16, u16, BreakoutStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakout {
    pub bricks: Vec<bool>,
    pub paddle: i8,
    pub ball_x: i8,
    pub ball_y: i8,
    pub velocity_x: i8,
    pub velocity_y: i8,
    pub score: u16,
    pub moves: u16,
    pub status: BreakoutStatus,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for Breakout {
    fn default() -> Self {
        Self::new(0xBEEA_0001)
    }
}

impl Breakout {
    pub fn new(seed: u64) -> Self {
        Self {
            bricks: vec![true; BRICK_COUNT],
            paddle: WIDTH / 2,
            ball_x: WIDTH / 2,
            ball_y: HEIGHT - 3,
            velocity_x: if seed & 1 == 0 { 1 } else { -1 },
            velocity_y: -1,
            score: 0,
            moves: 0,
            status: BreakoutStatus::Playing,
            seed,
            undo: None,
        }
    }

    pub fn step(&mut self, movement: PaddleMove) -> bool {
        if self.status != BreakoutStatus::Playing {
            return false;
        }
        self.undo = Some((
            self.bricks.clone(),
            self.paddle,
            self.ball_x,
            self.ball_y,
            self.velocity_x,
            self.velocity_y,
            self.score,
            self.moves,
            self.status,
        ));
        self.paddle = (self.paddle
            + match movement {
                PaddleMove::Left => -1,
                PaddleMove::Stay => 0,
                PaddleMove::Right => 1,
            })
        .clamp(1, WIDTH - 2);
        self.ball_x += self.velocity_x;
        self.ball_y += self.velocity_y;
        self.moves = self.moves.saturating_add(1);
        if self.ball_x <= 0 || self.ball_x >= WIDTH - 1 {
            self.velocity_x = -self.velocity_x;
            self.ball_x = self.ball_x.clamp(0, WIDTH - 1);
        }
        if self.ball_y <= 0 {
            self.velocity_y = 1;
            self.ball_y = 0;
        }
        if self.ball_y >= HEIGHT - 1 {
            if (self.ball_x - self.paddle).abs() <= 2 {
                self.velocity_y = -1;
                self.ball_y = HEIGHT - 2;
            } else {
                self.status = BreakoutStatus::Lost;
                return true;
            }
        }
        if (1..=BRICK_ROWS).contains(&self.ball_y) {
            let index = (self.ball_y as usize - 1) * WIDTH as usize + self.ball_x as usize;
            if self.bricks[index] {
                self.bricks[index] = false;
                self.score = self.score.saturating_add(1);
                self.velocity_y = -self.velocity_y;
                if self.bricks.iter().all(|brick| !brick) {
                    self.status = BreakoutStatus::Won;
                }
            }
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((
            bricks,
            paddle,
            ball_x,
            ball_y,
            velocity_x,
            velocity_y,
            score,
            moves,
            status,
        )) = self.undo.take()
        {
            self.bricks = bricks;
            self.paddle = paddle;
            self.ball_x = ball_x;
            self.ball_y = ball_y;
            self.velocity_x = velocity_x;
            self.velocity_y = velocity_y;
            self.score = score;
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
}

#[cfg(test)]
mod tests;
