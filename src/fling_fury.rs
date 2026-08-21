//! Touch-first physics shooter with a sling, breakable blocks, and targets.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 32;
pub const HEIGHT: u8 = 18;
const STEP_INTERVAL: f32 = 0.03;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlingStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FlingShot {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlingBlock {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub health: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlingTarget {
    pub x: u8,
    pub y: u8,
    pub alive: bool,
}

#[derive(Debug, Clone)]
struct Snapshot {
    blocks: Vec<FlingBlock>,
    targets: Vec<FlingTarget>,
    shot: Option<FlingShot>,
    angle: u16,
    power: u8,
    birds: u8,
    score: u16,
    moves: u16,
    status: FlingStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingFury {
    pub blocks: Vec<FlingBlock>,
    pub targets: Vec<FlingTarget>,
    pub shot: Option<FlingShot>,
    pub angle: u16,
    pub power: u8,
    pub birds: u8,
    pub score: u16,
    pub moves: u16,
    pub status: FlingStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
}

impl Default for FlingFury {
    fn default() -> Self {
        Self::new(0xF11A_0001)
    }
}

impl FlingFury {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            blocks: vec![
                FlingBlock {
                    x: 20,
                    y: 11,
                    w: 2,
                    h: 3,
                    health: 2,
                },
                FlingBlock {
                    x: 23,
                    y: 9,
                    w: 2,
                    h: 5,
                    health: 2,
                },
                FlingBlock {
                    x: 26,
                    y: 11,
                    w: 2,
                    h: 3,
                    health: 2,
                },
            ],
            targets: vec![
                FlingTarget {
                    x: 21,
                    y: 10,
                    alive: true,
                },
                FlingTarget {
                    x: 25,
                    y: 8,
                    alive: true,
                },
            ],
            shot: None,
            angle: 35,
            power: 70,
            birds: 5,
            score: 0,
            moves: 0,
            status: FlingStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
        };
        game.apply_mode();
        game
    }

    pub fn adjust_angle(&mut self, delta: i16) -> bool {
        if self.status != FlingStatus::Playing || self.shot.is_some() {
            return false;
        }
        self.angle = (i32::from(self.angle) + i32::from(delta)).clamp(15, 70) as u16;
        true
    }

    pub fn adjust_power(&mut self, delta: i16) -> bool {
        if self.status != FlingStatus::Playing || self.shot.is_some() {
            return false;
        }
        self.power = (i16::from(self.power) + delta).clamp(35, 95) as u8;
        true
    }

    pub fn fire(&mut self) -> bool {
        if self.status != FlingStatus::Playing
            || self.paused
            || self.shot.is_some()
            || self.birds == 0
        {
            return false;
        }
        self.snapshot();
        let radians = f32::from(self.angle).to_radians();
        let speed = f32::from(self.power) * 0.11;
        self.shot = Some(FlingShot {
            x: 5.,
            y: 12.,
            vx: radians.cos() * speed,
            vy: -radians.sin() * speed,
        });
        self.birds -= 1;
        self.moves = self.moves.saturating_add(1);
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != FlingStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == FlingStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one(STEP_INTERVAL);
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != FlingStatus::Playing {
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
        self.blocks = snapshot.blocks;
        self.targets = snapshot.targets;
        self.shot = snapshot.shot;
        self.angle = snapshot.angle;
        self.power = snapshot.power;
        self.birds = snapshot.birds;
        self.score = snapshot.score;
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
        self.apply_mode();
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
        self.apply_mode();
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "WOODPILE"
        } else {
            "IRON FORT"
        }
    }

    fn advance_one(&mut self, dt: f32) {
        let Some(mut shot) = self.shot else {
            if self.targets.iter().all(|target| !target.alive) {
                self.status = FlingStatus::Won;
            } else if self.birds == 0 {
                self.status = FlingStatus::Lost;
            }
            return;
        };
        shot.x += shot.vx * dt;
        shot.y += shot.vy * dt;
        shot.vy += 1.05 * dt;
        let mut ended = shot.x < 0. || shot.x >= f32::from(WIDTH) || shot.y >= 14.;
        for target in &mut self.targets {
            if target.alive
                && distance(shot.x, shot.y, f32::from(target.x), f32::from(target.y)) < 1.4
            {
                target.alive = false;
                self.score = self.score.saturating_add(100);
                shot.vx *= 0.78;
                shot.vy *= -0.55;
            }
        }
        for block in &mut self.blocks {
            if block.health > 0 && in_block(shot.x, shot.y, *block) {
                block.health -= 1;
                self.score = self.score.saturating_add(20);
                shot.vx *= 0.72;
                shot.vy *= -0.55;
                ended = false;
            }
        }
        if shot.y >= 14. && shot.vy.abs() < 1.0 {
            ended = true;
        } else if shot.y >= 14. {
            shot.y = 14.;
            shot.vy *= -0.45;
        }
        if self.targets.iter().all(|target| !target.alive) {
            self.status = FlingStatus::Won;
            self.paused = false;
            self.shot = None;
        } else if ended {
            self.shot = None;
            if self.birds == 0 {
                self.status = FlingStatus::Lost;
                self.paused = false;
            }
        } else {
            self.shot = Some(shot);
        }
    }

    fn apply_mode(&mut self) {
        if self.mode == 1 {
            self.blocks.push(FlingBlock {
                x: 28,
                y: 10,
                w: 2,
                h: 4,
                health: 3,
            });
            self.targets.push(FlingTarget {
                x: 29,
                y: 8,
                alive: true,
            });
        }
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            blocks: self.blocks.clone(),
            targets: self.targets.clone(),
            shot: self.shot,
            angle: self.angle,
            power: self.power,
            birds: self.birds,
            score: self.score,
            moves: self.moves,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

fn distance(x: f32, y: f32, other_x: f32, other_y: f32) -> f32 {
    ((x - other_x).powi(2) + (y - other_y).powi(2)).sqrt()
}

fn in_block(x: f32, y: f32, block: FlingBlock) -> bool {
    x >= f32::from(block.x) - 0.4
        && x <= f32::from(block.x + block.w) + 0.4
        && y >= f32::from(block.y) - 0.4
        && y <= f32::from(block.y + block.h) + 0.4
}

#[cfg(test)]
mod tests;
