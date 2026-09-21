//! Deterministic real-time Snake with visible touch direction controls.

use serde::{Deserialize, Serialize};

pub const WIDTH: i32 = 16;
pub const HEIGHT: i32 = 12;
pub const TARGET_SCORE: u16 = 20;
pub const MOVE_INTERVAL: f32 = 0.20;
pub const GARDEN_ROCKS: usize = 12;

pub fn default_target_score() -> u16 {
    TARGET_SCORE
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnakeMode {
    #[default]
    Classic,
    Wrap,
    Garden,
}

impl SnakeMode {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Wrap, Self::Garden];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC",
            Self::Wrap => "WRAP",
            Self::Garden => "GARDEN",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoodKind {
    #[default]
    Berry,
    Gold,
}

impl FoodKind {
    pub const fn value(self) -> u16 {
        match self {
            Self::Berry => 1,
            Self::Gold => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnakeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl SnakeDirection {
    pub fn opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnakeStatus {
    Playing,
    Won,
    Lost,
}

pub type Snapshot = (
    Vec<u16>,
    SnakeDirection,
    u16,
    FoodKind,
    u16,
    u16,
    SnakeStatus,
    u64,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snake {
    pub body: Vec<u16>,
    pub direction: SnakeDirection,
    pub food: u16,
    #[serde(default)]
    pub food_kind: FoodKind,
    #[serde(default)]
    pub mode: SnakeMode,
    #[serde(default)]
    pub obstacles: Vec<u16>,
    pub score: u16,
    #[serde(default = "default_target_score")]
    pub win_score: u16,
    pub moves: u16,
    pub status: SnakeStatus,
    pub seed: u64,
    #[serde(default)]
    pub paused: bool,
    #[serde(skip)]
    pub undo: Option<Snapshot>,
    #[serde(skip)]
    pub elapsed: f32,
}

impl Default for Snake {
    fn default() -> Self {
        Self::new(0x5AAE_0001)
    }
}

impl Snake {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_and_target(
            seed,
            SnakeMode::Classic,
            content.balance.arcade.snake_target_score,
        )
    }

    pub fn new_with_mode(seed: u64, mode: SnakeMode) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_mode_and_target(seed, mode, content.balance.arcade.snake_target_score)
    }

    pub fn new_with_mode_and_target(seed: u64, mode: SnakeMode, win_score: u16) -> Self {
        let center = (HEIGHT / 2 * WIDTH + WIDTH / 2) as u16;
        let mut game = Self {
            body: vec![center, center - 1, center - 2],
            direction: SnakeDirection::Right,
            food: 0,
            food_kind: FoodKind::Berry,
            mode,
            obstacles: Vec::new(),
            score: 0,
            win_score,
            moves: 0,
            status: SnakeStatus::Playing,
            seed,
            paused: false,
            undo: None,
            elapsed: 0.,
        };
        game.place_obstacles();
        game.food = game.next_food();
        game
    }

    pub fn set_direction(&mut self, direction: SnakeDirection) -> bool {
        if self.status != SnakeStatus::Playing || direction.opposite(self.direction) {
            return false;
        }
        self.direction = direction;
        true
    }

    pub fn step(&mut self, direction: SnakeDirection) -> bool {
        if !self.set_direction(direction) {
            return false;
        }
        self.advance_one()
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != SnakeStatus::Playing || self.paused {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= self.move_interval() && self.status == SnakeStatus::Playing {
            self.elapsed -= self.move_interval();
            advanced |= self.advance_one();
        }
        advanced
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.status != SnakeStatus::Playing {
            return false;
        }
        self.paused = !self.paused;
        if self.paused {
            self.elapsed = 0.;
        }
        true
    }

    pub fn advance_one(&mut self) -> bool {
        self.undo = Some((
            self.body.clone(),
            self.direction,
            self.food,
            self.food_kind,
            self.score,
            self.moves,
            self.status,
            self.seed,
        ));
        let head = self.body[0] as i32;
        let row = head / WIDTH;
        let column = head % WIDTH;
        let (row_step, column_step) = match self.direction {
            SnakeDirection::Up => (-1, 0),
            SnakeDirection::Right => (0, 1),
            SnakeDirection::Down => (1, 0),
            SnakeDirection::Left => (0, -1),
        };
        let mut next_row = row + row_step;
        let mut next_column = column + column_step;
        self.moves = self.moves.saturating_add(1);
        if self.mode == SnakeMode::Wrap {
            next_row = next_row.rem_euclid(HEIGHT);
            next_column = next_column.rem_euclid(WIDTH);
        } else if !(0..HEIGHT).contains(&next_row) || !(0..WIDTH).contains(&next_column) {
            self.status = SnakeStatus::Lost;
            return true;
        }
        let next = (next_row * WIDTH + next_column) as u16;
        if self.obstacles.contains(&next) {
            self.status = SnakeStatus::Lost;
            return true;
        }
        let eating = next == self.food;
        if self.body.contains(&next) && (eating || self.body[..self.body.len() - 1].contains(&next))
        {
            self.status = SnakeStatus::Lost;
            return true;
        }
        self.body.insert(0, next);
        if eating {
            self.score = self.score.saturating_add(self.food_kind.value());
            if self.score >= self.win_score {
                self.status = SnakeStatus::Won;
            } else {
                self.food = self.next_food();
                self.food_kind = if (self.score + 1).is_multiple_of(5) {
                    FoodKind::Gold
                } else {
                    FoodKind::Berry
                };
            }
        } else {
            self.body.pop();
        }
        true
    }

    pub fn hint_direction(&self) -> Option<SnakeDirection> {
        if self.status != SnakeStatus::Playing {
            return None;
        }
        [
            SnakeDirection::Up,
            SnakeDirection::Right,
            SnakeDirection::Down,
            SnakeDirection::Left,
        ]
        .into_iter()
        .filter(|&direction| !direction.opposite(self.direction))
        .filter_map(|direction| {
            let next = self.next_cell(direction)?;
            self.is_safe(next)
                .then_some((direction, self.food_distance(next)))
        })
        .min_by_key(|(_, distance)| *distance)
        .map(|(direction, _)| direction)
    }

    pub fn undo(&mut self) -> bool {
        if let Some((body, direction, food, food_kind, score, moves, status, seed)) =
            self.undo.take()
        {
            self.body = body;
            self.direction = direction;
            self.food = food;
            self.food_kind = food_kind;
            self.score = score;
            self.moves = moves;
            self.status = status;
            self.seed = seed;
            self.elapsed = 0.;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_mode_and_target(seed, self.mode, self.win_score);
    }

    pub fn next_food(&mut self) -> u16 {
        for _ in 0..(WIDTH * HEIGHT) {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let candidate = (self.seed % (WIDTH * HEIGHT) as u64) as u16;
            if !self.body.contains(&candidate) && !self.obstacles.contains(&candidate) {
                return candidate;
            }
        }
        0
    }

    pub fn next_cell(&self, direction: SnakeDirection) -> Option<u16> {
        let head = self.body.first().copied()? as i32;
        let row = head / WIDTH;
        let column = head % WIDTH;
        let (row_step, column_step) = match direction {
            SnakeDirection::Up => (-1, 0),
            SnakeDirection::Right => (0, 1),
            SnakeDirection::Down => (1, 0),
            SnakeDirection::Left => (0, -1),
        };
        let mut next_row = row + row_step;
        let mut next_column = column + column_step;
        if self.mode == SnakeMode::Wrap {
            next_row = next_row.rem_euclid(HEIGHT);
            next_column = next_column.rem_euclid(WIDTH);
        } else if !(0..HEIGHT).contains(&next_row) || !(0..WIDTH).contains(&next_column) {
            return None;
        }
        Some((next_row * WIDTH + next_column) as u16)
    }

    pub fn is_safe(&self, next: u16) -> bool {
        let eating = next == self.food;
        !self.obstacles.contains(&next)
            && (!self.body.contains(&next)
                || (!eating && !self.body[..self.body.len() - 1].contains(&next)))
    }

    pub fn food_distance(&self, next: u16) -> i32 {
        let row = i32::from(next) / WIDTH;
        let column = i32::from(next) % WIDTH;
        let food_row = i32::from(self.food) / WIDTH;
        let food_column = i32::from(self.food) % WIDTH;
        let row_distance = (row - food_row).abs();
        let column_distance = (column - food_column).abs();
        if self.mode == SnakeMode::Wrap {
            row_distance.min(HEIGHT - row_distance) + column_distance.min(WIDTH - column_distance)
        } else {
            row_distance + column_distance
        }
    }

    pub const fn target_score() -> u16 {
        TARGET_SCORE
    }

    pub fn speed_stage(&self) -> u16 {
        self.score / 5 + 1
    }

    pub fn move_interval(&self) -> f32 {
        let base = match self.mode {
            SnakeMode::Classic => MOVE_INTERVAL,
            SnakeMode::Wrap => MOVE_INTERVAL - 0.02,
            SnakeMode::Garden => MOVE_INTERVAL + 0.02,
        };
        (base - f32::from(self.score / 5) * 0.015).max(0.10)
    }

    pub fn place_obstacles(&mut self) {
        if self.mode != SnakeMode::Garden {
            return;
        }
        let center_row = HEIGHT / 2;
        for _ in 0..WIDTH * HEIGHT * 2 {
            if self.obstacles.len() >= GARDEN_ROCKS {
                break;
            }
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let candidate = (self.seed % (WIDTH * HEIGHT) as u64) as u16;
            let row = i32::from(candidate) / WIDTH;
            let column = i32::from(candidate) % WIDTH;
            let safe_opening =
                row == center_row && (WIDTH / 2 - 3..=WIDTH / 2 + 3).contains(&column);
            if !safe_opening
                && !self.body.contains(&candidate)
                && !self.obstacles.contains(&candidate)
            {
                self.obstacles.push(candidate);
            }
        }
    }
}
