//! A deterministic real-time tower defence board for short cabinet sessions.

use serde::{Deserialize, Serialize};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 5;
pub const TARGET_WAVE: u8 = 8;
pub const STARTING_GOLD: u16 = 12;
pub const STARTING_LIVES: u8 = 3;
pub const WAVE_INTERVAL: f32 = 0.4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enemy {
    pub row: u8,
    pub column: u8,
    pub health: u8,
    #[serde(default)]
    pub kind: EnemyKind,
    #[serde(default)]
    pub slow_ticks: u8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyKind {
    #[default]
    Grunt,
    Swift,
    Armored,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TowerKind {
    #[default]
    Bolt,
    Frost,
    Burst,
}

impl TowerKind {
    pub const ALL: [Self; 3] = [Self::Bolt, Self::Frost, Self::Burst];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Bolt => "BOLT",
            Self::Frost => "FROST",
            Self::Burst => "BURST",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Bolt => "DIRECT DAMAGE",
            Self::Frost => "SLOWS A LANE",
            Self::Burst => "SPLASH DAMAGE",
        }
    }

    pub const fn cost_at_level(self, level: u8) -> Option<u16> {
        if level >= 3 {
            return None;
        }
        let base = match self {
            Self::Bolt => 3,
            Self::Frost => 4,
            Self::Burst => 5,
        };
        Some(base + level.saturating_sub(1) as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TowerPhase {
    Build,
    Wave,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerHint {
    Build(usize, TowerKind),
    WaveControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerCellAvailability {
    Blocked,
    Build {
        kind: TowerKind,
        cost: u16,
    },
    Upgrade {
        kind: TowerKind,
        level: u8,
        cost: u16,
    },
    MaxLevel {
        kind: TowerKind,
        level: u8,
    },
}

pub type Snapshot = (
    Vec<u8>,
    Vec<TowerKind>,
    Vec<Enemy>,
    u16,
    u8,
    u8,
    u32,
    u64,
    TowerPhase,
    u16,
    bool,
    TowerKind,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TinyTowerDefence {
    pub towers: Vec<u8>,
    #[serde(default)]
    pub tower_kinds: Vec<TowerKind>,
    pub enemies: Vec<Enemy>,
    pub gold: u16,
    pub lives: u8,
    pub wave: u8,
    pub score: u32,
    pub seed: u64,
    pub phase: TowerPhase,
    pub tick: u16,
    #[serde(default)]
    pub paused: bool,
    #[serde(default)]
    pub selected_kind: TowerKind,
    #[serde(skip)]
    pub history: Vec<Snapshot>,
    #[serde(skip)]
    pub elapsed: f32,
}

impl Default for TinyTowerDefence {
    fn default() -> Self {
        Self::new(0x71A7_0001)
    }
}

impl TinyTowerDefence {
    pub fn new(seed: u64) -> Self {
        Self {
            towers: vec![0; WIDTH * HEIGHT],
            tower_kinds: vec![TowerKind::Bolt; WIDTH * HEIGHT],
            enemies: Vec::new(),
            gold: STARTING_GOLD,
            lives: STARTING_LIVES,
            wave: 1,
            score: 0,
            seed,
            phase: TowerPhase::Build,
            tick: 0,
            paused: false,
            selected_kind: TowerKind::Bolt,
            history: Vec::new(),
            elapsed: 0.,
        }
    }

    pub const fn width() -> usize {
        WIDTH
    }

    pub const fn height() -> usize {
        HEIGHT
    }

    pub const fn target_wave() -> u8 {
        TARGET_WAVE
    }

    pub fn tower_cost(&self, index: usize) -> Option<u16> {
        let level = *self.towers.get(index)?;
        let kind = if level == 0 {
            self.selected_kind
        } else {
            self.tower_kind(index)
        };
        tower_cost(kind, level)
    }

    pub fn cell_availability(&self, index: usize) -> TowerCellAvailability {
        if index >= self.towers.len() || index.is_multiple_of(WIDTH) || index % WIDTH == WIDTH - 1 {
            return TowerCellAvailability::Blocked;
        }
        let level = self.towers[index];
        let kind = if level == 0 {
            self.selected_kind
        } else {
            self.tower_kind(index)
        };
        match kind.cost_at_level(level) {
            Some(cost) if level == 0 => TowerCellAvailability::Build { kind, cost },
            Some(cost) => TowerCellAvailability::Upgrade { kind, level, cost },
            None => TowerCellAvailability::MaxLevel { kind, level },
        }
    }

    pub fn tower_kind(&self, index: usize) -> TowerKind {
        self.tower_kinds.get(index).copied().unwrap_or_default()
    }

    pub fn select_kind(&mut self, kind: TowerKind) -> bool {
        if self.phase != TowerPhase::Build || self.selected_kind == kind {
            return false;
        }
        self.selected_kind = kind;
        true
    }

    pub fn hint_action(&self) -> Option<TowerHint> {
        match self.phase {
            TowerPhase::Build => {
                let center = (HEIGHT / 2, WIDTH / 2);
                let recommended = self.recommended_kind();
                (0..self.towers.len())
                    .filter(|&index| self.valid_build_cell(index))
                    .filter(|&index| {
                        let kind = if self.towers[index] == 0 {
                            recommended
                        } else {
                            self.tower_kind(index)
                        };
                        tower_cost(kind, self.towers[index]).is_some_and(|cost| cost <= self.gold)
                    })
                    .min_by_key(|&index| {
                        let row = index / WIDTH;
                        let column = index % WIDTH;
                        row.abs_diff(center.0) + column.abs_diff(center.1)
                    })
                    .map(|index| {
                        let kind = if self.towers[index] == 0 {
                            recommended
                        } else {
                            self.tower_kind(index)
                        };
                        TowerHint::Build(index, kind)
                    })
            }
            TowerPhase::Wave if !self.enemies.is_empty() => Some(TowerHint::WaveControl),
            TowerPhase::Wave | TowerPhase::Won | TowerPhase::Lost => None,
        }
    }

    pub fn build_or_upgrade(&mut self, index: usize) -> bool {
        if self.phase != TowerPhase::Build || !self.valid_build_cell(index) {
            return false;
        }
        let Some(cost) = self.tower_cost(index) else {
            return false;
        };
        if self.gold < cost {
            return false;
        }
        self.snapshot();
        self.ensure_tower_kinds();
        self.gold -= cost;
        if self.towers[index] == 0 {
            self.tower_kinds[index] = self.selected_kind;
        }
        self.towers[index] = self.towers[index].saturating_add(1);
        true
    }

    /// Advances a deterministic session without changing the visible pause control.
    ///
    /// The crate-level capture harness uses this seam to exercise both build and
    /// wave phases while keeping the production action surface unchanged.
    pub fn start_or_advance(&mut self) -> bool {
        match self.phase {
            TowerPhase::Build => self.start_wave(),
            TowerPhase::Wave => self.advance_wave(true),
            TowerPhase::Won | TowerPhase::Lost => false,
        }
    }

    pub fn start_or_toggle_pause(&mut self) -> bool {
        match self.phase {
            TowerPhase::Build => self.start_wave(),
            TowerPhase::Wave => {
                self.paused = !self.paused;
                if self.paused {
                    self.elapsed = 0.;
                }
                true
            }
            TowerPhase::Won | TowerPhase::Lost => false,
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if self.phase != TowerPhase::Wave || self.paused || self.enemies.is_empty() {
            return false;
        }
        self.elapsed += dt.max(0.);
        let mut advanced = false;
        while self.elapsed >= WAVE_INTERVAL && self.phase == TowerPhase::Wave {
            self.elapsed -= WAVE_INTERVAL;
            self.advance_wave_state();
            advanced = true;
        }
        advanced
    }

    pub fn undo(&mut self) -> bool {
        if let Some((
            towers,
            tower_kinds,
            enemies,
            gold,
            lives,
            wave,
            score,
            seed,
            phase,
            tick,
            paused,
            selected_kind,
        )) = self.history.pop()
        {
            self.towers = towers;
            self.tower_kinds = tower_kinds;
            self.enemies = enemies;
            self.gold = gold;
            self.lives = lives;
            self.wave = wave;
            self.score = score;
            self.seed = seed;
            self.phase = phase;
            self.tick = tick;
            self.paused = paused;
            self.selected_kind = selected_kind;
            self.elapsed = 0.;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == TowerPhase::Won
    }

    pub fn start_wave(&mut self) -> bool {
        self.snapshot();
        self.enemies.clear();
        for enemy_index in 0..self.wave.saturating_add(1) {
            let row = (self.next_random() as usize % HEIGHT) as u8;
            let kind = if self.wave >= 5 && enemy_index % 3 == 2 {
                EnemyKind::Armored
            } else if self.wave >= 3 && enemy_index % 3 == 1 {
                EnemyKind::Swift
            } else {
                EnemyKind::Grunt
            };
            self.enemies.push(Enemy {
                row,
                column: 0,
                health: 1 + self.wave / 3 + u8::from(kind == EnemyKind::Armored) * 2,
                kind,
                slow_ticks: 0,
            });
        }
        self.phase = TowerPhase::Wave;
        self.paused = false;
        self.elapsed = 0.;
        true
    }

    pub fn advance_wave(&mut self, record_undo: bool) -> bool {
        if self.enemies.is_empty() {
            return false;
        }
        if record_undo {
            self.snapshot();
        }
        self.advance_wave_state();
        true
    }

    pub fn advance_wave_state(&mut self) {
        self.tick = self.tick.saturating_add(1);
        self.fire_towers();
        let mut remaining = Vec::with_capacity(self.enemies.len());
        for mut enemy in self.enemies.drain(..) {
            if enemy.health == 0 {
                self.score = self.score.saturating_add(10);
                self.gold = self.gold.saturating_add(2);
            } else {
                if enemy.slow_ticks > 0 {
                    enemy.slow_ticks = enemy.slow_ticks.saturating_sub(1);
                } else {
                    let steps = if enemy.kind == EnemyKind::Swift { 2 } else { 1 };
                    enemy.column = enemy.column.saturating_add(steps);
                }
                if usize::from(enemy.column) >= WIDTH - 1 {
                    self.lives = self.lives.saturating_sub(1);
                } else {
                    remaining.push(enemy);
                }
            }
        }
        self.enemies = remaining;
        if self.lives == 0 {
            self.phase = TowerPhase::Lost;
        } else if self.enemies.is_empty() {
            if self.wave >= TARGET_WAVE {
                self.phase = TowerPhase::Won;
            } else {
                self.wave = self.wave.saturating_add(1);
                self.phase = TowerPhase::Build;
            }
        }
        if self.phase != TowerPhase::Wave {
            self.elapsed = 0.;
        }
    }

    pub fn fire_towers(&mut self) {
        self.ensure_tower_kinds();
        for index in 0..self.towers.len() {
            let level = self.towers[index];
            if level == 0 {
                continue;
            }
            let row = index / WIDTH;
            let column = index % WIDTH;
            let kind = self.tower_kinds[index];
            let range = match kind {
                TowerKind::Bolt => 3,
                TowerKind::Frost => 2 + usize::from(level),
                TowerKind::Burst => 2,
            };
            let target = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(_, enemy)| {
                    let row_distance = row.abs_diff(usize::from(enemy.row));
                    row_distance <= usize::from(kind == TowerKind::Burst)
                        && enemy.health > 0
                        && column.abs_diff(usize::from(enemy.column)) <= range
                })
                .min_by_key(|(_, enemy)| {
                    (
                        column.abs_diff(usize::from(enemy.column)),
                        row.abs_diff(usize::from(enemy.row)),
                    )
                })
                .map(|(enemy, _)| enemy);
            if let Some(target) = target {
                match kind {
                    TowerKind::Bolt => {
                        self.enemies[target].health =
                            self.enemies[target].health.saturating_sub(level);
                    }
                    TowerKind::Frost => {
                        self.enemies[target].health = self.enemies[target].health.saturating_sub(1);
                        self.enemies[target].slow_ticks = self.enemies[target]
                            .slow_ticks
                            .max(1 + level.saturating_sub(1) / 2);
                    }
                    TowerKind::Burst => {
                        let target_row = self.enemies[target].row;
                        let target_column = self.enemies[target].column;
                        let damage = 1 + level.saturating_sub(1) / 2;
                        for enemy in &mut self.enemies {
                            if enemy.health > 0
                                && enemy.row.abs_diff(target_row) <= 1
                                && enemy.column.abs_diff(target_column) <= 1
                            {
                                enemy.health = enemy.health.saturating_sub(damage);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn valid_build_cell(&self, index: usize) -> bool {
        index < self.towers.len()
            && !index.is_multiple_of(WIDTH)
            && index % WIDTH != WIDTH - 1
            && self.towers[index] < 3
    }

    pub fn next_random(&mut self) -> u64 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.seed
    }

    pub fn snapshot(&mut self) {
        self.history.push((
            self.towers.clone(),
            self.tower_kinds.clone(),
            self.enemies.clone(),
            self.gold,
            self.lives,
            self.wave,
            self.score,
            self.seed,
            self.phase,
            self.tick,
            self.paused,
            self.selected_kind,
        ));
    }

    pub fn ensure_tower_kinds(&mut self) {
        if self.tower_kinds.len() != self.towers.len() {
            self.tower_kinds = vec![TowerKind::Bolt; self.towers.len()];
        }
    }

    pub fn recommended_kind(&self) -> TowerKind {
        match self.wave {
            1..=2 => TowerKind::Bolt,
            3..=4 => TowerKind::Frost,
            _ => TowerKind::Burst,
        }
    }
}

pub fn tower_cost(kind: TowerKind, level: u8) -> Option<u16> {
    kind.cost_at_level(level)
}
