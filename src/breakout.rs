//! Deterministic real-time Breakout with visible paddle controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i8 = 16;
pub const HEIGHT: i8 = 12;
const BRICK_ROWS: i8 = 4;
const BRICK_COUNT: usize = WIDTH as usize * BRICK_ROWS as usize;
const TARGET_LEVEL: u8 = 3;
const STARTING_LIVES: u8 = 3;
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

#[derive(Debug, Clone)]
struct Snapshot {
    bricks: Vec<bool>,
    brick_health: Vec<u8>,
    paddle: i8,
    ball_x: i8,
    ball_y: i8,
    velocity_x: i8,
    velocity_y: i8,
    score: u16,
    moves: u16,
    lives: u8,
    level: u8,
    status: BreakoutStatus,
    paused: bool,
    serve_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakout {
    pub bricks: Vec<bool>,
    #[serde(default)]
    pub brick_health: Vec<u8>,
    pub paddle: i8,
    pub ball_x: i8,
    pub ball_y: i8,
    pub velocity_x: i8,
    pub velocity_y: i8,
    pub score: u16,
    pub moves: u16,
    #[serde(default = "default_lives")]
    pub lives: u8,
    #[serde(default = "default_level")]
    pub level: u8,
    pub status: BreakoutStatus,
    pub seed: u64,
    #[serde(default)]
    pub control: PaddleMove,
    #[serde(default)]
    pub paused: bool,
    #[serde(default)]
    pub serve_ready: bool,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
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
        Self::new_with_level(seed, 1)
    }

    pub fn new_with_level(seed: u64, level: u8) -> Self {
        let mut game = Self {
            bricks: vec![false; BRICK_COUNT],
            brick_health: vec![0; BRICK_COUNT],
            paddle: WIDTH / 2,
            ball_x: WIDTH / 2,
            ball_y: HEIGHT - 3,
            velocity_x: if seed & 1 == 0 { 1 } else { -1 },
            velocity_y: -1,
            score: 0,
            moves: 0,
            lives: STARTING_LIVES,
            level: level.clamp(1, TARGET_LEVEL),
            status: BreakoutStatus::Playing,
            seed,
            control: PaddleMove::Stay,
            paused: false,
            serve_ready: false,
            undo: None,
            elapsed: 0.,
            precise_paddle: f32::from(WIDTH / 2),
            precise_ball_x: f32::from(WIDTH / 2),
            precise_ball_y: f32::from(HEIGHT - 3),
            runtime_initialized: true,
        };
        game.build_wall();
        game
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
        if self.serve_ready {
            self.serve_ready = false;
            self.paused = false;
        } else {
            self.paused = !self.paused;
        }
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    pub fn hint_move(&self) -> Option<PaddleMove> {
        if self.status != BreakoutStatus::Playing || self.serve_ready {
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
        if let Some(snapshot) = self.undo.take() {
            self.bricks = snapshot.bricks;
            self.brick_health = snapshot.brick_health;
            self.paddle = snapshot.paddle;
            self.ball_x = snapshot.ball_x;
            self.ball_y = snapshot.ball_y;
            self.velocity_x = snapshot.velocity_x;
            self.velocity_y = snapshot.velocity_y;
            self.score = snapshot.score;
            self.moves = snapshot.moves;
            self.lives = snapshot.lives;
            self.level = snapshot.level;
            self.status = snapshot.status;
            self.paused = snapshot.paused;
            self.serve_ready = snapshot.serve_ready;
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

    pub const fn target_level() -> u8 {
        TARGET_LEVEL
    }

    pub fn remaining_bricks(&self) -> usize {
        self.bricks.iter().filter(|brick| **brick).count()
    }

    pub fn brick_health(&self, index: usize) -> u8 {
        self.brick_health
            .get(index)
            .copied()
            .unwrap_or_else(|| u8::from(self.bricks.get(index).copied().unwrap_or(false)))
    }

    fn ensure_runtime(&mut self) {
        self.ensure_brick_health();
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
        self.undo = Some(Box::new(Snapshot {
            bricks: self.bricks.clone(),
            brick_health: self.brick_health.clone(),
            paddle: self.paddle,
            ball_x: self.ball_x,
            ball_y: self.ball_y,
            velocity_x: self.velocity_x,
            velocity_y: self.velocity_y,
            score: self.score,
            moves: self.moves,
            lives: self.lives,
            level: self.level,
            status: self.status,
            paused: self.paused,
            serve_ready: self.serve_ready,
        }));
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
        let ball_speed = BALL_SPEED + f32::from(self.level.saturating_sub(1)) * 0.65;
        self.precise_ball_x += f32::from(self.velocity_x) * ball_speed * dt;
        self.precise_ball_y += f32::from(self.velocity_y) * ball_speed * dt;
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
            self.damage_brick(index);
            let moving_down = self.velocity_y > 0;
            self.velocity_y = -self.velocity_y;
            self.precise_ball_y = if moving_down {
                f32::from(brick_row + 1) - 0.01
            } else {
                f32::from(brick_row + 2) + 0.01
            };
            if self.remaining_bricks() == 0 {
                self.complete_wall();
                return;
            }
        }

        if self.precise_ball_y >= f32::from(HEIGHT - 1) && self.velocity_y > 0 {
            if (self.precise_ball_x - self.precise_paddle).abs() <= 2. {
                let impact = self.precise_ball_x - self.precise_paddle;
                self.precise_ball_y = f32::from(HEIGHT - 1) - 0.01;
                self.velocity_y = -self.velocity_y.abs();
                if impact < -0.6 {
                    self.velocity_x = -1;
                } else if impact > 0.6 {
                    self.velocity_x = 1;
                }
            } else {
                self.lives = self.lives.saturating_sub(1);
                if self.lives == 0 {
                    self.status = BreakoutStatus::Lost;
                } else {
                    self.prepare_serve();
                }
            }
        }
    }

    fn damage_brick(&mut self, index: usize) {
        self.ensure_brick_health();
        let Some(health) = self.brick_health.get_mut(index) else {
            return;
        };
        *health = health.saturating_sub(1);
        self.score = self.score.saturating_add(1);
        self.bricks[index] = *health > 0;
    }

    fn complete_wall(&mut self) {
        if self.level >= TARGET_LEVEL {
            self.status = BreakoutStatus::Won;
            self.paused = false;
            self.serve_ready = false;
            return;
        }
        self.level = self.level.saturating_add(1);
        self.build_wall();
        self.prepare_serve();
    }

    fn prepare_serve(&mut self) {
        self.paddle = WIDTH / 2;
        self.ball_x = WIDTH / 2;
        self.ball_y = HEIGHT - 3;
        self.velocity_x = if self.seed.wrapping_add(u64::from(self.level)) & 1 == 0 {
            1
        } else {
            -1
        };
        self.velocity_y = -1;
        self.control = PaddleMove::Stay;
        self.paused = true;
        self.serve_ready = true;
        self.elapsed = 0.;
        self.precise_paddle = f32::from(self.paddle);
        self.precise_ball_x = f32::from(self.ball_x);
        self.precise_ball_y = f32::from(self.ball_y);
        self.runtime_initialized = true;
    }

    fn build_wall(&mut self) {
        self.bricks.resize(BRICK_COUNT, false);
        self.brick_health.resize(BRICK_COUNT, 0);
        let offset = (self.seed % WIDTH as u64) as usize;
        for row in 0..BRICK_ROWS as usize {
            for column in 0..WIDTH as usize {
                let health = wall_health(self.level, row, column, offset);
                let index = row * WIDTH as usize + column;
                self.brick_health[index] = health;
                self.bricks[index] = health > 0;
            }
        }
    }

    fn ensure_brick_health(&mut self) {
        if self.brick_health.len() == self.bricks.len() {
            return;
        }
        self.brick_health = self.bricks.iter().map(|brick| u8::from(*brick)).collect();
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

fn wall_health(level: u8, row: usize, column: usize, offset: usize) -> u8 {
    match level {
        1 => 1,
        2 => {
            let shifted = (column + offset) % WIDTH as usize;
            if (row + shifted).is_multiple_of(5) {
                0
            } else if row == 0 || shifted.is_multiple_of(4) {
                2
            } else {
                1
            }
        }
        _ => {
            let shifted = (column + offset) % WIDTH as usize;
            if row == 0 || row == BRICK_ROWS as usize - 1 {
                if shifted.is_multiple_of(3) {
                    3
                } else {
                    2
                }
            } else if shifted % 4 == 1 || shifted % 4 == 2 {
                2
            } else {
                0
            }
        }
    }
}

const fn default_lives() -> u8 {
    STARTING_LIVES
}

const fn default_level() -> u8 {
    1
}

#[cfg(test)]
mod tests;
