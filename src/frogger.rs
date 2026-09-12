//! Touch-first, deterministic Frogger.

use crate::domain::Direction;
use serde::{Deserialize, Serialize};

pub const WIDTH: u8 = 12;
pub const HEIGHT: u8 = 12;
const TARGET_CROSSINGS: u8 = 3;
const STEP_INTERVAL: f32 = 0.20;

fn default_target_crossings() -> u8 {
    TARGET_CROSSINGS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FroggerStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Car {
    pub row: u8,
    pub x: u8,
    pub length: u8,
    pub direction: i8,
}

#[derive(Debug, Clone)]
struct Snapshot {
    player_row: u8,
    player_column: u8,
    cars: Vec<Car>,
    crossings: u8,
    score: u16,
    moves: u16,
    lives: u8,
    status: FroggerStatus,
    seed: u64,
    paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frogger {
    pub player_row: u8,
    pub player_column: u8,
    pub cars: Vec<Car>,
    pub crossings: u8,
    #[serde(default = "default_target_crossings")]
    pub target_crossings: u8,
    pub score: u16,
    pub moves: u16,
    pub lives: u8,
    pub status: FroggerStatus,
    pub seed: u64,
    pub paused: bool,
    #[serde(default)]
    pub mode: u8,
    #[serde(skip)]
    undo: Option<Box<Snapshot>>,
    #[serde(skip)]
    elapsed: f32,
}

impl Default for Frogger {
    fn default() -> Self {
        Self::new(0xF906_0001)
    }
}

impl Frogger {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_target(seed, content.balance.arcade.frogger_target_crossings)
    }

    pub fn new_with_target(seed: u64, target_crossings: u8) -> Self {
        let mut game = Self {
            player_row: HEIGHT - 1,
            player_column: WIDTH / 2,
            cars: Vec::new(),
            crossings: 0,
            target_crossings,
            score: 0,
            moves: 0,
            lives: 3,
            status: FroggerStatus::Playing,
            seed,
            paused: false,
            mode: 0,
            undo: None,
            elapsed: 0.,
        };
        game.build_lanes();
        game
    }

    pub fn move_player(&mut self, direction: Direction) -> bool {
        if self.status != FroggerStatus::Playing || self.paused {
            return false;
        }
        let (row_delta, column_delta) = match direction {
            Direction::Up => (-1_i8, 0_i8),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        let next_row = self.player_row as i8 + row_delta;
        let next_column = self.player_column as i8 + column_delta;
        if !(0..HEIGHT as i8).contains(&next_row) || !(0..WIDTH as i8).contains(&next_column) {
            return false;
        }
        self.snapshot();
        self.player_row = next_row as u8;
        self.player_column = next_column as u8;
        self.moves = self.moves.saturating_add(1);
        self.resolve_position();
        true
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != FroggerStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= STEP_INTERVAL && self.status == FroggerStatus::Playing {
            self.elapsed -= STEP_INTERVAL;
            self.snapshot();
            self.moves = self.moves.saturating_add(1);
            for car in &mut self.cars {
                car.x = (i16::from(car.x) + i16::from(car.direction)).rem_euclid(i16::from(WIDTH))
                    as u8;
            }
            self.resolve_position();
            advanced = true;
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != FroggerStatus::Playing {
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
        self.player_row = snapshot.player_row;
        self.player_column = snapshot.player_column;
        self.cars = snapshot.cars;
        self.crossings = snapshot.crossings;
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
        *self = Self::new_with_target(seed, self.target_crossings);
        self.mode = mode;
        self.build_lanes();
    }

    pub fn cycle_mode(&mut self) {
        let seed = self.seed.wrapping_add(1);
        let mode = (self.mode + 1) % 2;
        *self = Self::new(seed);
        self.mode = mode;
        self.build_lanes();
    }

    pub const fn mode_label(&self) -> &'static str {
        if self.mode == 0 {
            "CLASSIC"
        } else {
            "RUSH"
        }
    }

    pub const fn target_crossings() -> u8 {
        TARGET_CROSSINGS
    }

    fn resolve_position(&mut self) {
        if self.player_row == 0 {
            self.crossings = self.crossings.saturating_add(1);
            self.score = self.score.saturating_add(25);
            if self.crossings >= self.target_crossings {
                self.status = FroggerStatus::Won;
                self.paused = false;
            } else {
                self.player_row = HEIGHT - 1;
                self.player_column = WIDTH / 2;
            }
            return;
        }
        if self
            .cars
            .iter()
            .any(|car| car.row == self.player_row && car_contains(car, self.player_column))
        {
            self.lives = self.lives.saturating_sub(1);
            if self.lives == 0 {
                self.status = FroggerStatus::Lost;
                self.paused = false;
            } else {
                self.player_row = HEIGHT - 1;
                self.player_column = WIDTH / 2;
            }
        }
    }

    fn build_lanes(&mut self) {
        self.cars.clear();
        for (lane, row) in [2, 4, 6, 8, 10].into_iter().enumerate() {
            let direction = if lane.is_multiple_of(2) { 1 } else { -1 };
            let first_x = self.next_random() % WIDTH;
            let second_x = (self.next_random() % WIDTH + 5) % WIDTH;
            self.cars.push(Car {
                row,
                x: first_x,
                length: 2 + (lane as u8 % 2) + self.mode,
                direction,
            });
            self.cars.push(Car {
                row,
                x: second_x,
                length: 2,
                direction,
            });
        }
    }

    fn next_random(&mut self) -> u8 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.seed >> 24) as u8
    }

    fn snapshot(&mut self) {
        self.undo = Some(Box::new(Snapshot {
            player_row: self.player_row,
            player_column: self.player_column,
            cars: self.cars.clone(),
            crossings: self.crossings,
            score: self.score,
            moves: self.moves,
            lives: self.lives,
            status: self.status,
            seed: self.seed,
            paused: self.paused,
        }));
    }
}

fn car_contains(car: &Car, column: u8) -> bool {
    (0..car.length).any(|offset| (car.x + offset) % WIDTH == column)
}

#[cfg(test)]
mod tests;
