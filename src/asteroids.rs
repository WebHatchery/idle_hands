//! Touch-first, deterministic Asteroids.

use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 20;
pub const HEIGHT: u8 = 14;
const TARGET_SCORE: u16 = 120;
const STEP_INTERVAL: f32 = 0.12;

fn default_target_score() -> u16 {
    TARGET_SCORE
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipDirection {
    Left,
    #[default]
    Stay,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsteroidsStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asteroid {
    pub x: u8,
    pub y: u8,
    pub size: u8,
    pub drift: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shot {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone)]
struct Snapshot {
    ship_x: u8,
    asteroids: Vec<Asteroid>,
    shot: Option<Shot>,
    score: u16,
    moves: u16,
    lives: u8,
    status: AsteroidsStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asteroids {
    pub ship_x: u8,
    pub asteroids: Vec<Asteroid>,
    pub shot: Option<Shot>,
    pub score: u16,
    #[serde(default = "default_target_score")]
    pub target_score: u16,
    pub moves: u16,
    pub lives: u8,
    pub status: AsteroidsStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
    #[serde(skip)]
    control: ShipDirection,
}

impl Default for Asteroids {
    fn default() -> Self {
        Self::new(0xA57E_0001)
    }
}

impl Asteroids {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_target(seed, content.balance.arcade.asteroids_target_score)
    }

    pub fn new_with_target(seed: u64, target_score: u16) -> Self {
        let mut game = Self {
            ship_x: WIDTH / 2,
            asteroids: Vec::new(),
            shot: None,
            score: 0,
            target_score,
            moves: 0,
            lives: 3,
            status: AsteroidsStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
            control: ShipDirection::Stay,
        };
        game.spawn_field();
        game
    }

    pub fn set_control(&mut self, direction: ShipDirection) {
        if self.status == AsteroidsStatus::Playing {
            self.control = direction;
        }
    }

    pub fn fire(&mut self) -> bool {
        if self.status != AsteroidsStatus::Playing || self.paused || self.shot.is_some() {
            return false;
        }
        self.snapshot();
        self.shot = Some(Shot {
            x: self.ship_x,
            y: HEIGHT - 2,
        });
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != AsteroidsStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == AsteroidsStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.advance_one();
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != AsteroidsStatus::Playing {
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
        self.asteroids = snapshot.asteroids;
        self.shot = snapshot.shot;
        self.score = snapshot.score;
        self.moves = snapshot.moves;
        self.lives = snapshot.lives;
        self.status = snapshot.status;
        self.seed = snapshot.seed;
        self.paused = snapshot.paused;
        self.elapsed = 0.;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        let mode = self.mode;
        *self = Self::new_with_target(seed, self.target_score);
        self.mode = mode;
        self.spawn_field();
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
        self.spawn_field();
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "DRIFT"
        } else {
            "DENSE"
        }
    }

    pub const fn target_score() -> u16 {
        TARGET_SCORE
    }

    fn advance_one(&mut self) {
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        match self.control {
            ShipDirection::Left => self.ship_x = (self.ship_x + WIDTH - 1) % WIDTH,
            ShipDirection::Stay => {}
            ShipDirection::Right => self.ship_x = (self.ship_x + 1) % WIDTH,
        }
        self.advance_shot();
        let mut hit_ship = false;
        for asteroid in &mut self.asteroids {
            asteroid.x = (asteroid.x + WIDTH).wrapping_add_signed(asteroid.drift) % WIDTH;
            asteroid.y = asteroid.y.saturating_add(1);
            if asteroid.y >= HEIGHT - 1 {
                asteroid.y = 0;
                if asteroid.x.abs_diff(self.ship_x) <= asteroid.size {
                    hit_ship = true;
                }
            }
        }
        if hit_ship {
            self.hit_ship();
        }
    }

    fn advance_shot(&mut self) {
        let Some(mut shot) = self.shot else {
            return;
        };
        if shot.y == 0 {
            self.shot = None;
            return;
        }
        shot.y -= 1;
        let hit = self.asteroids.iter().position(|asteroid| {
            asteroid.x.abs_diff(shot.x) <= asteroid.size && asteroid.y == shot.y
        });
        if let Some(index) = hit {
            let asteroid = self.asteroids[index];
            self.score = self.score.saturating_add(u16::from(asteroid.size) * 10);
            self.shot = None;
            if asteroid.size > 1 {
                self.asteroids[index] = Asteroid {
                    x: asteroid.x,
                    y: asteroid.y,
                    size: asteroid.size - 1,
                    drift: -asteroid.drift,
                };
            } else {
                self.asteroids[index] = self.next_asteroid();
            }
            if self.score >= self.target_score {
                self.status = AsteroidsStatus::Won;
                self.paused = false;
            }
        } else {
            self.shot = Some(shot);
        }
    }

    fn hit_ship(&mut self) {
        self.lives = self.lives.saturating_sub(1);
        if self.lives == 0 {
            self.status = AsteroidsStatus::Lost;
            self.paused = false;
        }
    }

    fn spawn_field(&mut self) {
        let count = if self.mode == 0 { 8 } else { 10 };
        self.asteroids = (0..count)
            .map(|index| self.next_asteroid_at(index))
            .collect();
    }

    fn next_asteroid(&mut self) -> Asteroid {
        self.next_asteroid_at(self.moves as usize + self.asteroids.len())
    }

    fn next_asteroid_at(&mut self, index: usize) -> Asteroid {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        Asteroid {
            x: ((self.seed % u64::from(WIDTH)) as u8 + index as u8) % WIDTH,
            y: 1 + ((self.seed >> 8) % u64::from(HEIGHT - 5)) as u8,
            size: 1 + (index as u8 % 3),
            drift: if self.seed & 1 == 0 { -1 } else { 1 },
        }
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            ship_x: self.ship_x,
            asteroids: self.asteroids.clone(),
            shot: self.shot,
            score: self.score,
            moves: self.moves,
            lives: self.lives,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

#[cfg(test)]
mod tests;
