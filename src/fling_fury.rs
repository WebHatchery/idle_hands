//! Touch-first physics puzzle with a sling, breakable blocks, and falling targets.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 32;
pub const HEIGHT: u8 = 18;
pub const GROUND_Y: f32 = 14.0;
pub const LEVEL_COUNT: u8 = 3;
pub const STEP_INTERVAL: f32 = 0.016;
pub const GRAVITY: f32 = 10.5;
pub const DEFAULT_SHOTS: u8 = 5;

pub const LEVEL_NAMES: [&str; LEVEL_COUNT as usize] =
    ["COPPER YARD", "STACKED WORKS", "TOWER RUSH"];

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FlingBlock {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub health: u8,
    #[serde(default)]
    pub vx: f32,
    #[serde(default)]
    pub vy: f32,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub angular_velocity: f32,
    #[serde(default)]
    pub knocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FlingTarget {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
    #[serde(default)]
    pub falling: bool,
    #[serde(default)]
    pub vx: f32,
    #[serde(default)]
    pub vy: f32,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub score_awarded: bool,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub blocks: Vec<FlingBlock>,
    pub targets: Vec<FlingTarget>,
    pub shot: Option<FlingShot>,
    pub angle: u16,
    pub power: u8,
    pub shots_remaining: u8,
    pub score: u16,
    pub moves: u16,
    pub status: FlingStatus,
    pub seed: u64,
    pub paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingFury {
    pub blocks: Vec<FlingBlock>,
    pub targets: Vec<FlingTarget>,
    pub shot: Option<FlingShot>,
    pub angle: u16,
    pub power: u8,
    #[serde(default = "default_shots", alias = "birds")]
    pub shots_remaining: u8,
    pub score: u16,
    pub moves: u16,
    pub status: FlingStatus,
    pub seed: u64,
    pub paused: bool,
    /// The cabinet's variant button uses the level index for this game.
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    pub undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    pub elapsed: f32,
}

pub fn default_shots() -> u8 {
    DEFAULT_SHOTS
}

impl Default for FlingFury {
    fn default() -> Self {
        Self::new(0xF11A_0001)
    }
}

impl FlingFury {
    pub fn new(seed: u64) -> Self {
        Self::new_at_level(seed, 0)
    }

    pub fn new_at_level(seed: u64, mode: u8) -> Self {
        let mode = mode % LEVEL_COUNT;
        let (blocks, targets) = level_layout(mode);
        Self {
            blocks,
            targets,
            shot: None,
            angle: 35,
            power: 70,
            shots_remaining: DEFAULT_SHOTS,
            score: 0,
            moves: 0,
            status: FlingStatus::Playing,
            seed,
            paused: false,
            mode,
            undo: None,
            elapsed: 0.,
        }
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
            || self.shots_remaining == 0
        {
            return false;
        }
        self.snapshot();
        let radians = f32::from(self.angle).to_radians();
        let speed = f32::from(self.power) * 0.19;
        self.shot = Some(FlingShot {
            x: 5.,
            y: 12.5,
            vx: radians.cos() * speed,
            vy: -radians.sin() * speed,
        });
        self.shots_remaining -= 1;
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
        self.shots_remaining = snapshot.shots_remaining;
        self.score = snapshot.score;
        self.moves = snapshot.moves;
        self.status = snapshot.status;
        self.seed = snapshot.seed;
        self.paused = snapshot.paused;
        self.elapsed = 0.;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_at_level(seed, self.mode);
    }

    pub fn next_level(&mut self) -> bool {
        if self.status != FlingStatus::Won {
            return false;
        }
        let next = (self.mode + 1) % LEVEL_COUNT;
        *self = Self::new_at_level(self.seed.wrapping_add(1), next);
        true
    }

    pub fn cycle_mode(&mut self) {
        let next = (self.mode + 1) % LEVEL_COUNT;
        *self = Self::new_at_level(self.seed.wrapping_add(1), next);
    }

    pub const fn level_number(&self) -> u8 {
        self.mode + 1
    }

    pub const fn level_count() -> u8 {
        LEVEL_COUNT
    }

    pub fn mode_label(&self) -> &'static str {
        LEVEL_NAMES[self.mode as usize]
    }

    pub fn stars(&self) -> u8 {
        if self.status != FlingStatus::Won {
            return 0;
        }
        match self.moves {
            0..=2 => 3,
            3..=4 => 2,
            _ => 1,
        }
    }

    pub fn advance_one(&mut self, dt: f32) {
        if let Some(mut shot) = self.shot {
            shot.x += shot.vx * dt;
            shot.y += shot.vy * dt;
            shot.vy += GRAVITY * dt;

            let mut ended = shot.x < -1. || shot.x > f32::from(WIDTH) + 1. || shot.y > 20.;
            let mut target_hit = false;
            for target in &mut self.targets {
                if target.alive
                    && !target.falling
                    && distance(shot.x, shot.y, target.x, target.y) < 0.8
                {
                    target_hit = true;
                    topple_target(target, shot.vx * 0.36, shot.vy * 0.18 - 1.4);
                    self.score = self.score.saturating_add(100);
                }
            }
            if target_hit {
                shot.vx *= 0.78;
                shot.vy = -shot.vy.abs() * 0.5;
            }

            let mut block_hit = false;
            for block in &mut self.blocks {
                if block.health > 0 && in_block(shot.x, shot.y, *block) {
                    let was_intact = block.health == 1;
                    block.health = block.health.saturating_sub(1);
                    block.knocked = true;
                    block.vx += shot.vx * 0.24;
                    block.vy = block.vy.min(-shot.vy.abs() * 0.14 - 1.0);
                    block.angular_velocity += shot.vx * 0.12;
                    if was_intact {
                        self.score = self.score.saturating_add(25);
                    } else {
                        self.score = self.score.saturating_add(15);
                    }
                    block_hit = true;
                    break;
                }
            }
            if block_hit {
                shot.vx *= -0.52;
                shot.vy = -shot.vy.abs() * 0.5;
                shot.x -= shot.vx * dt;
            }

            if shot.y >= GROUND_Y - 0.35 {
                shot.y = GROUND_Y - 0.35;
                shot.vy = -shot.vy.abs() * 0.46;
                if shot.vy.abs() < 1.8 {
                    ended = true;
                }
            }
            self.shot = if ended { None } else { Some(shot) };
        }

        self.advance_blocks(dt);
        self.advance_targets(dt);
        self.resolve_block_contacts();
        self.resolve_target_contacts();
        self.finish_if_ready();
    }

    pub fn advance_blocks(&mut self, dt: f32) {
        for block in &mut self.blocks {
            if !block.knocked {
                continue;
            }
            block.vy += GRAVITY * dt;
            block.x += block.vx * dt;
            block.y += block.vy * dt;
            block.rotation += block.angular_velocity * dt;
            block.vx *= 0.998;
            block.angular_velocity *= 0.995;

            let floor = GROUND_Y - block.h;
            if block.y >= floor {
                block.y = floor;
                block.vy = -block.vy.abs() * 0.22;
                block.vx *= 0.86;
                block.angular_velocity *= 0.72;
                if block.vy.abs() < 0.3 {
                    block.vy = 0.;
                }
            }
        }
    }

    pub fn advance_targets(&mut self, dt: f32) {
        for target in &mut self.targets {
            if !target.alive || !target.falling {
                continue;
            }
            target.vy += GRAVITY * dt;
            target.x += target.vx * dt;
            target.y += target.vy * dt;
            target.rotation += target.vx * dt * 0.75;
            target.vx *= 0.998;
            if target.y >= GROUND_Y - 0.55 {
                target.y = GROUND_Y - 0.55;
                target.alive = false;
                target.falling = false;
            }
        }
    }

    pub fn resolve_block_contacts(&mut self) {
        let len = self.blocks.len();
        for source_index in 0..len {
            if !self.blocks[source_index].knocked {
                continue;
            }
            let source = self.blocks[source_index];
            for target_index in 0..len {
                if source_index == target_index || self.blocks[target_index].knocked {
                    continue;
                }
                let target = self.blocks[target_index];
                if rectangles_overlap(expanded_block(source, 0.2), expanded_block(target, 0.1)) {
                    let direction = if target.x >= source.x { 1. } else { -1. };
                    let block = &mut self.blocks[target_index];
                    block.knocked = true;
                    block.vx += direction * (source.vx.abs() * 0.35 + 0.7);
                    block.vy = -1.1;
                    block.angular_velocity += direction * 0.35;
                }
            }
        }
    }

    pub fn resolve_target_contacts(&mut self) {
        for block in &self.blocks {
            if !block.knocked {
                continue;
            }
            let block_area = expanded_block(*block, 0.25);
            for target in &mut self.targets {
                if target.alive && !target.falling && point_in_rect(target.x, target.y, block_area)
                {
                    topple_target(target, block.vx * 0.3, block.vy.min(-1.3));
                    self.score = self.score.saturating_add(100);
                }
            }
        }
    }

    pub fn finish_if_ready(&mut self) {
        if self.targets.iter().all(|target| !target.alive) {
            self.status = FlingStatus::Won;
            self.paused = false;
            self.shot = None;
        } else if self.shot.is_none()
            && self.shots_remaining == 0
            && self.targets.iter().all(|target| !target.falling)
        {
            self.status = FlingStatus::Lost;
            self.paused = false;
        }
    }

    pub fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            blocks: self.blocks.clone(),
            targets: self.targets.clone(),
            shot: self.shot,
            angle: self.angle,
            power: self.power,
            shots_remaining: self.shots_remaining,
            score: self.score,
            moves: self.moves,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

pub fn level_layout(mode: u8) -> (Vec<FlingBlock>, Vec<FlingTarget>) {
    match mode {
        1 => (
            vec![
                block(19., 12., 2., 2., 2),
                block(22., 10., 2., 4., 2),
                block(25., 8., 2., 6., 3),
                block(28., 12., 2., 2., 2),
                block(21., 8., 6., 1., 3),
            ],
            vec![
                target(20., 11.2),
                target(23., 9.2),
                target(26., 7.2),
                target(29., 11.2),
            ],
        ),
        2 => (
            vec![
                block(19., 12., 2., 2., 2),
                block(22., 10., 2., 4., 2),
                block(24., 8., 2., 6., 3),
                block(26., 6., 2., 8., 3),
                block(28., 12., 2., 2., 2),
                block(22., 6., 6., 1., 3),
            ],
            vec![
                target(20., 11.2),
                target(23., 9.2),
                target(25., 7.2),
                target(27., 5.2),
                target(29., 11.2),
            ],
        ),
        _ => (
            vec![
                block(20., 12., 3., 2., 2),
                block(24., 10., 2., 4., 3),
                block(27., 12., 3., 2., 2),
            ],
            vec![target(21.5, 11.2), target(25., 9.2), target(28.5, 11.2)],
        ),
    }
}

pub fn block(x: f32, y: f32, w: f32, h: f32, health: u8) -> FlingBlock {
    FlingBlock {
        x,
        y,
        w,
        h,
        health,
        vx: 0.,
        vy: 0.,
        rotation: 0.,
        angular_velocity: 0.,
        knocked: false,
    }
}

pub fn target(x: f32, y: f32) -> FlingTarget {
    FlingTarget {
        x,
        y,
        alive: true,
        falling: false,
        vx: 0.,
        vy: 0.,
        rotation: 0.,
        score_awarded: false,
    }
}

pub fn topple_target(target: &mut FlingTarget, vx: f32, vy: f32) {
    target.falling = true;
    target.vx = vx;
    target.vy = vy;
    target.rotation = if vx >= 0. { -0.1 } else { 0.1 };
    target.score_awarded = true;
}

pub fn distance(x: f32, y: f32, other_x: f32, other_y: f32) -> f32 {
    ((x - other_x).powi(2) + (y - other_y).powi(2)).sqrt()
}

pub fn in_block(x: f32, y: f32, block: FlingBlock) -> bool {
    point_in_rect(
        x,
        y,
        Rect {
            x: block.x - 0.35,
            y: block.y - 0.35,
            w: block.w + 0.7,
            h: block.h + 0.7,
        },
    )
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub fn expanded_block(block: FlingBlock, margin: f32) -> Rect {
    Rect {
        x: block.x - margin,
        y: block.y - margin,
        w: block.w + margin * 2.,
        h: block.h + margin * 2.,
    }
}

pub fn point_in_rect(x: f32, y: f32, rect: Rect) -> bool {
    x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h
}

pub fn rectangles_overlap(left: Rect, right: Rect) -> bool {
    left.x < right.x + right.w
        && left.x + left.w > right.x
        && left.y < right.y + right.h
        && left.y + left.h > right.y
}
