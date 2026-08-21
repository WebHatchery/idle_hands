//! Touch-first paddle duel against a deterministic cabinet opponent.

use serde::{Deserialize, Serialize};

pub const WIDTH: f32 = 32.;
pub const HEIGHT: f32 = 18.;
const STEP_INTERVAL: f32 = 0.03;
const WIN_SCORE: u8 = 7;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddleMove {
    Up,
    #[default]
    Stay,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddleStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone)]
struct Snapshot {
    paddle_y: f32,
    cpu_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    player_score: u8,
    cpu_score: u8,
    moves: u16,
    status: PaddleStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddleDuel {
    pub paddle_y: f32,
    pub cpu_y: f32,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_vx: f32,
    pub ball_vy: f32,
    pub player_score: u8,
    pub cpu_score: u8,
    pub moves: u16,
    pub status: PaddleStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
    #[serde(skip)]
    control: PaddleMove,
}

impl Default for PaddleDuel {
    fn default() -> Self {
        Self::new(0xD0E1_0001)
    }
}

impl PaddleDuel {
    pub fn new(seed: u64) -> Self {
        Self {
            paddle_y: HEIGHT * 0.5,
            cpu_y: HEIGHT * 0.5,
            ball_x: WIDTH * 0.5,
            ball_y: HEIGHT * 0.5,
            ball_vx: 6.,
            ball_vy: 2.4,
            player_score: 0,
            cpu_score: 0,
            moves: 0,
            status: PaddleStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
            control: PaddleMove::Stay,
        }
    }

    pub fn set_control(&mut self, movement: PaddleMove) {
        if self.status == PaddleStatus::Playing {
            self.control = movement;
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != PaddleStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == PaddleStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one(STEP_INTERVAL);
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != PaddleStatus::Playing {
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
        self.paddle_y = snapshot.paddle_y;
        self.cpu_y = snapshot.cpu_y;
        self.ball_x = snapshot.ball_x;
        self.ball_y = snapshot.ball_y;
        self.ball_vx = snapshot.ball_vx;
        self.ball_vy = snapshot.ball_vy;
        self.player_score = snapshot.player_score;
        self.cpu_score = snapshot.cpu_score;
        self.moves = snapshot.moves;
        self.status = snapshot.status;
        self.seed = snapshot.seed;
        self.paused = snapshot.paused;
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
        self.ball_vx = if mode == 0 { 6. } else { 7.5 };
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "CASUAL"
        } else {
            "RALLY"
        }
    }

    fn advance_one(&mut self, dt: f32) {
        self.snapshot();
        match self.control {
            PaddleMove::Up => self.paddle_y -= 7. * dt,
            PaddleMove::Stay => {}
            PaddleMove::Down => self.paddle_y += 7. * dt,
        }
        self.paddle_y = self.paddle_y.clamp(2., HEIGHT - 2.);
        self.cpu_y += (self.ball_y - self.cpu_y).signum() * 3.2 * dt;
        self.cpu_y = self.cpu_y.clamp(2., HEIGHT - 2.);
        self.ball_x += self.ball_vx * dt;
        self.ball_y += self.ball_vy * dt;
        self.moves = self.moves.saturating_add(1);
        if !(1.0..=HEIGHT - 1.0).contains(&self.ball_y) {
            self.ball_y = self.ball_y.clamp(1., HEIGHT - 1.);
            self.ball_vy = -self.ball_vy;
        }
        if self.ball_vx < 0. && self.ball_x <= 2.0 && (self.ball_y - self.paddle_y).abs() <= 2.0 {
            self.ball_x = 2.0;
            self.ball_vx = self.ball_vx.abs() * 1.03;
            self.ball_vy += (self.ball_y - self.paddle_y) * 0.65;
        } else if self.ball_vx > 0.
            && self.ball_x >= WIDTH - 2.0
            && (self.ball_y - self.cpu_y).abs() <= 2.0
        {
            self.ball_x = WIDTH - 2.0;
            self.ball_vx = -self.ball_vx.abs() * 1.03;
            self.ball_vy += (self.ball_y - self.cpu_y) * 0.45;
        } else if self.ball_x < 0. {
            self.cpu_score = self.cpu_score.saturating_add(1);
            self.reset_ball(-1.);
        } else if self.ball_x > WIDTH {
            self.player_score = self.player_score.saturating_add(1);
            self.reset_ball(1.);
        }
        if self.player_score >= WIN_SCORE {
            self.status = PaddleStatus::Won;
            self.paused = false;
        } else if self.cpu_score >= WIN_SCORE {
            self.status = PaddleStatus::Lost;
            self.paused = false;
        }
    }

    fn reset_ball(&mut self, direction: f32) {
        self.ball_x = WIDTH * 0.5;
        self.ball_y = HEIGHT * 0.5;
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let vertical = (((self.seed >> 8) % 5) as f32 - 2.) * 0.9;
        self.ball_vx = direction * if self.mode == 0 { 6. } else { 7.5 };
        self.ball_vy = vertical;
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            paddle_y: self.paddle_y,
            cpu_y: self.cpu_y,
            ball_x: self.ball_x,
            ball_y: self.ball_y,
            ball_vx: self.ball_vx,
            ball_vy: self.ball_vy,
            player_score: self.player_score,
            cpu_score: self.cpu_score,
            moves: self.moves,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

#[cfg(test)]
mod tests;
