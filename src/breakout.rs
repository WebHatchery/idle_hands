//! Deterministic real-time Breakout with visible paddle controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i8 = 16;
pub const HEIGHT: i8 = 12;
const BRICK_ROWS: i8 = 4;
const BRICK_COUNT: usize = WIDTH as usize * BRICK_ROWS as usize;
const PHYSICS_STEP: f32 = 1. / 120.;
#[cfg(test)]
const LEGACY_STEP: f32 = 0.25;
const PADDLE_SPEED: f32 = 7.;
const BALL_SPEED: f32 = 5.;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddleMove {
    Left,
    #[default]
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
    #[serde(default)]
    pub control: PaddleMove,
    #[serde(default)]
    pub paused: bool,
    #[serde(skip)]
    undo: Option<Snapshot>,
    #[serde(skip)]
    elapsed: f32,
    #[serde(skip)]
    precise_paddle: f32,
    #[serde(skip)]
    precise_ball_x: f32,
    #[serde(skip)]
    precise_ball_y: f32,
    #[serde(skip)]
    runtime_initialized: bool,
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
            control: PaddleMove::Stay,
            paused: false,
            undo: None,
            elapsed: 0.,
            precise_paddle: f32::from(WIDTH / 2),
            precise_ball_x: f32::from(WIDTH / 2),
            precise_ball_y: f32::from(HEIGHT - 3),
            runtime_initialized: true,
        }
    }

    pub fn set_control(&mut self, movement: PaddleMove) -> bool {
        if self.status != BreakoutStatus::Playing {
            return false;
        }
        self.control = movement;
        true
    }

    #[cfg(test)]
    pub fn step(&mut self, movement: PaddleMove) -> bool {
        if !self.set_control(movement) {
            return false;
        }
        self.ensure_runtime();
        self.capture_undo();
        self.simulate_for(LEGACY_STEP);
        self.sync_persisted();
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != BreakoutStatus::Playing || self.paused {
            return false;
        }
        self.ensure_runtime();
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= PHYSICS_STEP && self.status == BreakoutStatus::Playing {
            if !advanced {
                self.capture_undo();
            }
            self.elapsed -= PHYSICS_STEP;
            self.simulate_step(PHYSICS_STEP);
            advanced = true;
        }
        if advanced {
            self.sync_persisted();
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != BreakoutStatus::Playing {
            return false;
        }
        self.paused = !self.paused;
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    pub fn hint_move(&self) -> Option<PaddleMove> {
        if self.status != BreakoutStatus::Playing {
            return None;
        }
        let projected = self.projected_ball_x();
        Some(if projected < self.paddle {
            PaddleMove::Left
        } else if projected > self.paddle {
            PaddleMove::Right
        } else {
            PaddleMove::Stay
        })
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
            self.elapsed = 0.;
            self.runtime_initialized = false;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn ball_position(&self) -> (f32, f32) {
        if self.runtime_initialized {
            (self.precise_ball_x, self.precise_ball_y)
        } else {
            (f32::from(self.ball_x), f32::from(self.ball_y))
        }
    }

    pub fn paddle_position(&self) -> f32 {
        if self.runtime_initialized {
            self.precise_paddle
        } else {
            f32::from(self.paddle)
        }
    }

    fn ensure_runtime(&mut self) {
        if self.runtime_initialized {
            if (f32::from(self.paddle) - self.precise_paddle).abs() > 0.6
                || (f32::from(self.ball_x) - self.precise_ball_x).abs() > 0.6
                || (f32::from(self.ball_y) - self.precise_ball_y).abs() > 0.6
            {
                self.precise_paddle = f32::from(self.paddle);
                self.precise_ball_x = f32::from(self.ball_x);
                self.precise_ball_y = f32::from(self.ball_y);
            }
            return;
        }
        self.precise_paddle = f32::from(self.paddle);
        self.precise_ball_x = f32::from(self.ball_x);
        self.precise_ball_y = f32::from(self.ball_y);
        self.runtime_initialized = true;
    }

    fn capture_undo(&mut self) {
        self.sync_persisted();
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
    }

    #[cfg(test)]
    fn simulate_for(&mut self, duration: f32) {
        let mut remaining = duration;
        while remaining > 0. && self.status == BreakoutStatus::Playing {
            let step = remaining.min(PHYSICS_STEP);
            self.simulate_step(step);
            remaining -= step;
        }
    }

    fn simulate_step(&mut self, dt: f32) {
        let paddle_delta = match self.control {
            PaddleMove::Left => -1.,
            PaddleMove::Stay => 0.,
            PaddleMove::Right => 1.,
        };
        self.precise_paddle = (self.precise_paddle + paddle_delta * PADDLE_SPEED * dt)
            .clamp(2., f32::from(WIDTH - 3));
        self.precise_ball_x += f32::from(self.velocity_x) * BALL_SPEED * dt;
        self.precise_ball_y += f32::from(self.velocity_y) * BALL_SPEED * dt;
        self.moves = self.moves.saturating_add(1);

        if self.precise_ball_x <= 0. {
            self.precise_ball_x = 0.;
            self.velocity_x = self.velocity_x.abs();
        } else if self.precise_ball_x >= f32::from(WIDTH - 1) {
            self.precise_ball_x = f32::from(WIDTH - 1);
            self.velocity_x = -self.velocity_x.abs();
        }
        if self.precise_ball_y <= 0. {
            self.precise_ball_y = 0.;
            self.velocity_y = self.velocity_y.abs();
        }

        let brick_row = self.precise_ball_y.floor() as i8 - 1;
        let brick_column = self.precise_ball_x.floor() as i8;
        if (0..BRICK_ROWS).contains(&brick_row)
            && (0..WIDTH).contains(&brick_column)
            && self.bricks[brick_row as usize * WIDTH as usize + brick_column as usize]
        {
            let index = brick_row as usize * WIDTH as usize + brick_column as usize;
            self.bricks[index] = false;
            self.score = self.score.saturating_add(1);
            let moving_down = self.velocity_y > 0;
            self.velocity_y = -self.velocity_y;
            self.precise_ball_y = if moving_down {
                f32::from(brick_row + 1) - 0.01
            } else {
                f32::from(brick_row + 2) + 0.01
            };
            if self.bricks.iter().all(|brick| !brick) {
                self.status = BreakoutStatus::Won;
                return;
            }
        }

        if self.precise_ball_y >= f32::from(HEIGHT - 1) && self.velocity_y > 0 {
            if (self.precise_ball_x - self.precise_paddle).abs() <= 2. {
                self.precise_ball_y = f32::from(HEIGHT - 1) - 0.01;
                self.velocity_y = -self.velocity_y.abs();
            } else {
                self.status = BreakoutStatus::Lost;
            }
        }
    }

    fn sync_persisted(&mut self) {
        self.paddle = self.precise_paddle.round() as i8;
        self.ball_x = self.precise_ball_x.round() as i8;
        self.ball_y = self.precise_ball_y.round() as i8;
    }

    fn projected_ball_x(&self) -> i8 {
        let mut x = self.ball_x;
        let mut velocity = self.velocity_x;
        let steps = (HEIGHT - 1 - self.ball_y).max(0);
        for _ in 0..steps {
            x += velocity;
            if x <= 0 || x >= WIDTH - 1 {
                velocity = -velocity;
                x = x.clamp(0, WIDTH - 1);
            }
        }
        x
    }
}

#[cfg(test)]
mod tests;
