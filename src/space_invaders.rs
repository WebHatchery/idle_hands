//! Touch-first, deterministic Space Invaders.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 20;
pub const HEIGHT: u8 = 12;
const TARGET_WAVE: u8 = 3;
const STEP_INTERVAL: f32 = 0.10;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipDirection {
    Left,
    #[default]
    Stay,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceInvadersStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invader {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
    pub alive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shot {
    pub x: u8,
    pub y: u8,
    pub downward: bool,
}

#[derive(Debug, Clone)]
struct Snapshot {
    ship_x: u8,
    invaders: Vec<Invader>,
    player_shot: Option<Shot>,
    enemy_shot: Option<Shot>,
    score: u16,
    moves: u16,
    lives: u8,
    wave: u8,
    status: SpaceInvadersStatus,
    seed: u64,
    paused: bool,
    formation_direction: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceInvaders {
    pub ship_x: u8,
    pub invaders: Vec<Invader>,
    pub player_shot: Option<Shot>,
    pub enemy_shot: Option<Shot>,
    pub score: u16,
    pub moves: u16,
    pub lives: u8,
    pub wave: u8,
    pub status: SpaceInvadersStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    formation_direction: i8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
    #[serde(skip)]
    control: ShipDirection,
}

impl Default for SpaceInvaders {
    fn default() -> Self {
        Self::new(0x5ACE_0001)
    }
}

impl SpaceInvaders {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            ship_x: WIDTH / 2,
            invaders: Vec::new(),
            player_shot: None,
            enemy_shot: None,
            score: 0,
            moves: 0,
            lives: 3,
            wave: 1,
            status: SpaceInvadersStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            formation_direction: 1,
            undo: None,
            elapsed: 0.,
            control: ShipDirection::Stay,
        };
        game.build_wave();
        game
    }

    pub fn set_control(&mut self, direction: ShipDirection) {
        if self.status == SpaceInvadersStatus::Playing {
            self.control = direction;
        }
    }

    pub fn fire(&mut self) -> bool {
        if self.status != SpaceInvadersStatus::Playing || self.paused || self.player_shot.is_some()
        {
            return false;
        }
        self.snapshot();
        self.player_shot = Some(Shot {
            x: self.ship_x,
            y: HEIGHT - 2,
            downward: false,
        });
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != SpaceInvadersStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == SpaceInvadersStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one();
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != SpaceInvadersStatus::Playing {
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
        self.ship_x = snapshot.ship_x;
        self.invaders = snapshot.invaders;
        self.player_shot = snapshot.player_shot;
        self.enemy_shot = snapshot.enemy_shot;
        self.score = snapshot.score;
        self.moves = snapshot.moves;
        self.lives = snapshot.lives;
        self.wave = snapshot.wave;
        self.status = snapshot.status;
        self.seed = snapshot.seed;
        self.paused = snapshot.paused;
        self.formation_direction = snapshot.formation_direction;
        self.elapsed = 0.;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        let mode = self.mode;
        *self = Self::new(seed);
        self.mode = mode;
        self.build_wave();
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
        self.build_wave();
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "CADET"
        } else {
            "ACE"
        }
    }

    pub const fn target_wave() -> u8 {
        TARGET_WAVE
    }

    fn advance_one(&mut self) {
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        match self.control {
            ShipDirection::Left => self.ship_x = self.ship_x.saturating_sub(1).max(1),
            ShipDirection::Stay => {}
            ShipDirection::Right => self.ship_x = self.ship_x.saturating_add(1).min(WIDTH - 2),
        }
        self.advance_player_shot();
        self.advance_enemy_shot();
        if self.moves.is_multiple_of(3) {
            self.move_formation();
        }
        if self.enemy_shot.is_none() && self.moves.is_multiple_of(8) {
            self.enemy_shot = self
                .invaders
                .iter()
                .rev()
                .find(|invader| invader.alive)
                .map(|invader| Shot {
                    x: invader.x,
                    y: invader.y.saturating_add(1),
                    downward: true,
                });
        }
    }

    fn advance_player_shot(&mut self) {
        let Some(mut shot) = self.player_shot else {
            return;
        };
        if shot.y == 0 {
            self.player_shot = None;
            return;
        }
        shot.y -= 1;
        if let Some(invader) = self
            .invaders
            .iter_mut()
            .find(|invader| invader.alive && invader.x == shot.x && invader.y == shot.y)
        {
            invader.alive = false;
            self.score = self.score.saturating_add(10);
            self.player_shot = None;
            if self.invaders.iter().all(|invader| !invader.alive) {
                if self.wave >= TARGET_WAVE {
                    self.status = SpaceInvadersStatus::Won;
                    self.paused = false;
                } else {
                    self.wave = self.wave.saturating_add(1);
                    self.build_wave();
                }
            }
        } else {
            self.player_shot = Some(shot);
        }
    }

    fn advance_enemy_shot(&mut self) {
        let Some(mut shot) = self.enemy_shot else {
            return;
        };
        shot.y = shot.y.saturating_add(1);
        if shot.y >= HEIGHT {
            self.enemy_shot = None;
        } else if shot.x == self.ship_x && shot.y >= HEIGHT - 2 {
            self.enemy_shot = None;
            self.lives = self.lives.saturating_sub(1);
            if self.lives == 0 {
                self.status = SpaceInvadersStatus::Lost;
                self.paused = false;
            }
        } else {
            self.enemy_shot = Some(shot);
        }
    }

    fn move_formation(&mut self) {
        let edge = self
            .invaders
            .iter()
            .filter(|invader| invader.alive)
            .any(|invader| {
                (self.formation_direction < 0 && invader.x <= 1)
                    || (self.formation_direction > 0 && invader.x >= WIDTH - 2)
            });
        if edge {
            self.formation_direction = -self.formation_direction;
            for invader in &mut self.invaders {
                invader.y = invader.y.saturating_add(1);
            }
        } else {
            for invader in &mut self.invaders {
                invader.x = invader.x.saturating_add_signed(self.formation_direction);
            }
        }
        if self
            .invaders
            .iter()
            .any(|invader| invader.alive && invader.y >= HEIGHT - 2)
        {
            self.status = SpaceInvadersStatus::Lost;
            self.paused = false;
        }
    }

    fn build_wave(&mut self) {
        self.invaders.clear();
        self.player_shot = None;
        self.enemy_shot = None;
        self.formation_direction = 1;
        let rows = if self.mode == 0 { 4 } else { 5 };
        for row in 0..rows {
            for column in 0..8 {
                self.invaders.push(Invader {
                    x: 2 + column * 2,
                    y: 1 + row,
                    kind: (row + column + self.wave) % 3,
                    alive: true,
                });
            }
        }
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            ship_x: self.ship_x,
            invaders: self.invaders.clone(),
            player_shot: self.player_shot,
            enemy_shot: self.enemy_shot,
            score: self.score,
            moves: self.moves,
            lives: self.lives,
            wave: self.wave,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
            formation_direction: self.formation_direction,
        }));
    }
}

#[cfg(test)]
mod tests;
