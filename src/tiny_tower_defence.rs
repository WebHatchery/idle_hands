//! A deterministic, turn-based tower defence board for quiet cabinet sessions.

use serde::{Deserialize, Serialize};

const WIDTH: usize = 7;
const HEIGHT: usize = 5;
const TARGET_WAVE: u8 = 8;
const STARTING_GOLD: u16 = 12;
const STARTING_LIVES: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enemy {
    pub row: u8,
    pub column: u8,
    pub health: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TowerPhase {
    Build,
    Wave,
    Won,
    Lost,
}

type Snapshot = (Vec<u8>, Vec<Enemy>, u16, u8, u8, u32, u64, TowerPhase, u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TinyTowerDefence {
    pub towers: Vec<u8>,
    pub enemies: Vec<Enemy>,
    pub gold: u16,
    pub lives: u8,
    pub wave: u8,
    pub score: u32,
    pub seed: u64,
    pub phase: TowerPhase,
    pub tick: u16,
    #[serde(skip)]
    history: Vec<Snapshot>,
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
            enemies: Vec::new(),
            gold: STARTING_GOLD,
            lives: STARTING_LIVES,
            wave: 1,
            score: 0,
            seed,
            phase: TowerPhase::Build,
            tick: 0,
            history: Vec::new(),
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
        if level == 0 {
            Some(3)
        } else if level < 3 {
            Some(2 + u16::from(level))
        } else {
            None
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
        self.gold -= cost;
        self.towers[index] = self.towers[index].saturating_add(1);
        true
    }

    pub fn start_or_advance(&mut self) -> bool {
        match self.phase {
            TowerPhase::Build => self.start_wave(),
            TowerPhase::Wave => self.advance_wave(),
            TowerPhase::Won | TowerPhase::Lost => false,
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some((towers, enemies, gold, lives, wave, score, seed, phase, tick)) =
            self.history.pop()
        {
            self.towers = towers;
            self.enemies = enemies;
            self.gold = gold;
            self.lives = lives;
            self.wave = wave;
            self.score = score;
            self.seed = seed;
            self.phase = phase;
            self.tick = tick;
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

    fn start_wave(&mut self) -> bool {
        self.snapshot();
        self.enemies.clear();
        for _ in 0..self.wave.saturating_add(1) {
            let row = (self.next_random() as usize % HEIGHT) as u8;
            self.enemies.push(Enemy {
                row,
                column: 0,
                health: 1 + self.wave / 3,
            });
        }
        self.phase = TowerPhase::Wave;
        true
    }

    fn advance_wave(&mut self) -> bool {
        if self.enemies.is_empty() {
            return false;
        }
        self.snapshot();
        self.tick = self.tick.saturating_add(1);
        self.fire_towers();
        let mut remaining = Vec::with_capacity(self.enemies.len());
        for mut enemy in self.enemies.drain(..) {
            if enemy.health == 0 {
                self.score = self.score.saturating_add(10);
                self.gold = self.gold.saturating_add(2);
            } else {
                enemy.column = enemy.column.saturating_add(1);
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
        true
    }

    fn fire_towers(&mut self) {
        for index in 0..self.towers.len() {
            let level = self.towers[index];
            if level == 0 {
                continue;
            }
            let row = index / WIDTH;
            let column = index % WIDTH;
            let target = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(_, enemy)| {
                    usize::from(enemy.row) == row
                        && enemy.health > 0
                        && column.abs_diff(usize::from(enemy.column)) <= 3
                })
                .min_by_key(|(_, enemy)| column.abs_diff(usize::from(enemy.column)))
                .map(|(enemy, _)| enemy);
            if let Some(target) = target {
                self.enemies[target].health = self.enemies[target].health.saturating_sub(level);
            }
        }
    }

    fn valid_build_cell(&self, index: usize) -> bool {
        index < self.towers.len()
            && !index.is_multiple_of(WIDTH)
            && index % WIDTH != WIDTH - 1
            && self.towers[index] < 3
    }

    fn next_random(&mut self) -> u64 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.seed
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.towers.clone(),
            self.enemies.clone(),
            self.gold,
            self.lives,
            self.wave,
            self.score,
            self.seed,
            self.phase,
            self.tick,
        ));
    }
}

#[cfg(test)]
mod tests;
