//! Touch-first artillery puzzle with a destructible terrain silhouette.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 32;
pub const HEIGHT: u8 = 18;
const STEP_INTERVAL: f32 = 0.03;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CannonStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CannonShot {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Clone)]
struct Snapshot {
    terrain: Vec<u8>,
    target_health: u8,
    angle: u16,
    power: u8,
    shot: Option<CannonShot>,
    score: u16,
    moves: u16,
    status: CannonStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCannon {
    pub terrain: Vec<u8>,
    pub target_x: u8,
    pub target_health: u8,
    pub angle: u16,
    pub power: u8,
    pub shot: Option<CannonShot>,
    pub score: u16,
    pub moves: u16,
    pub status: CannonStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
}

impl Default for TerrainCannon {
    fn default() -> Self {
        Self::new(0xCA77_0001)
    }
}

impl TerrainCannon {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            terrain: vec![14; usize::from(WIDTH)],
            target_x: WIDTH - 6,
            target_health: 3,
            angle: 45,
            power: 60,
            shot: None,
            score: 0,
            moves: 0,
            status: CannonStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
        };
        game.build_terrain();
        game
    }

    pub fn adjust_angle(&mut self, delta: i16) -> bool {
        if self.status != CannonStatus::Playing || self.shot.is_some() {
            return false;
        }
        self.angle = (i32::from(self.angle) + i32::from(delta)).clamp(15, 75) as u16;
        true
    }

    pub fn adjust_power(&mut self, delta: i16) -> bool {
        if self.status != CannonStatus::Playing || self.shot.is_some() {
            return false;
        }
        self.power = (i16::from(self.power) + delta).clamp(30, 90) as u8;
        true
    }

    pub fn fire(&mut self) -> bool {
        if self.status != CannonStatus::Playing || self.paused || self.shot.is_some() {
            return false;
        }
        self.snapshot();
        let radians = f32::from(self.angle).to_radians();
        let speed = f32::from(self.power) * 0.12;
        let x = 4.5;
        let y = f32::from(self.terrain[4]) - 0.7;
        self.shot = Some(CannonShot {
            x,
            y,
            vx: radians.cos() * speed,
            vy: -radians.sin() * speed,
        });
        self.moves = self.moves.saturating_add(1);
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != CannonStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == CannonStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one(STEP_INTERVAL);
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != CannonStatus::Playing {
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
        self.terrain = snapshot.terrain;
        self.target_health = snapshot.target_health;
        self.angle = snapshot.angle;
        self.power = snapshot.power;
        self.shot = snapshot.shot;
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
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
        self.build_terrain();
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "HILLS"
        } else {
            "CANYON"
        }
    }

    fn advance_one(&mut self, dt: f32) {
        let Some(mut shot) = self.shot else {
            return;
        };
        shot.x += shot.vx * dt;
        shot.y += shot.vy * dt;
        shot.vy += 0.72 * dt;
        let out_of_bounds =
            shot.x < 0. || shot.x >= f32::from(WIDTH) || shot.y >= f32::from(HEIGHT);
        if out_of_bounds {
            self.shot = None;
            return;
        }
        let column = shot.x.floor() as usize;
        if shot.x >= f32::from(self.target_x) - 1.3
            && shot.x <= f32::from(self.target_x) + 1.3
            && shot.y >= f32::from(self.terrain[column]) - 2.5
        {
            self.target_health = self.target_health.saturating_sub(1);
            self.score = self.score.saturating_add(100);
            self.shot = None;
            if self.target_health == 0 {
                self.status = CannonStatus::Won;
                self.paused = false;
            }
        } else if shot.y >= f32::from(self.terrain[column]) {
            self.carve(column);
            self.shot = None;
        } else {
            self.shot = Some(shot);
        }
    }

    fn carve(&mut self, center: usize) {
        for offset in -2_i32..=2 {
            let index = center as i32 + offset;
            if !(0..i32::from(WIDTH)).contains(&index) {
                continue;
            }
            let amount = if offset == 0 { 3 } else { 1 };
            self.terrain[index as usize] = self.terrain[index as usize]
                .saturating_add(amount)
                .min(HEIGHT - 1);
        }
    }

    fn build_terrain(&mut self) {
        for (index, height) in self.terrain.iter_mut().enumerate() {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let wave = ((index as i32 - 12).unsigned_abs() % 4) as u8;
            *height = if self.mode == 0 {
                12 + ((self.seed >> 8) as u8 % 4) + wave.min(2)
            } else {
                11 + ((self.seed >> 8) as u8 % 3) + if index % 7 == 0 { 3 } else { 0 }
            };
        }
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            terrain: self.terrain.clone(),
            target_health: self.target_health,
            angle: self.angle,
            power: self.power,
            shot: self.shot,
            score: self.score,
            moves: self.moves,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

#[cfg(test)]
#[path = "../tests/legacy/terrain_cannon/tests.rs"]
mod tests;
