//! Deterministic one-room exploration with short, turn-based combat.

use crate::state::Direction;
use serde::{Deserialize, Serialize};

const SIZE: usize = 7;
const CELLS: usize = SIZE * SIZE;
const START_HEALTH: u8 = 10;
const START_POTIONS: u8 = 2;
const EMPTY: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomEnemy {
    pub position: usize,
    pub health: u8,
    pub damage: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomPhase {
    Exploring,
    Won,
    Lost,
}

type Snapshot = (
    usize,
    Vec<RoomEnemy>,
    usize,
    u8,
    u8,
    u32,
    u16,
    u64,
    RoomPhase,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneRoomRoguelike {
    pub player: usize,
    pub enemies: Vec<RoomEnemy>,
    pub treasure: usize,
    pub exit: usize,
    pub health: u8,
    pub potions: u8,
    pub score: u32,
    pub turns: u16,
    pub seed: u64,
    pub phase: RoomPhase,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for OneRoomRoguelike {
    fn default() -> Self {
        Self::new(0x0E_700001)
    }
}

impl OneRoomRoguelike {
    pub fn new(seed: u64) -> Self {
        let mut room = Self {
            player: Self::center(),
            enemies: Vec::new(),
            treasure: EMPTY,
            exit: CELLS - SIZE,
            health: START_HEALTH,
            potions: START_POTIONS,
            score: 0,
            turns: 0,
            seed,
            phase: RoomPhase::Exploring,
            history: Vec::new(),
        };
        room.place_room();
        room
    }

    pub const fn size() -> usize {
        SIZE
    }

    pub fn move_in(&mut self, direction: Direction) -> bool {
        if self.phase != RoomPhase::Exploring {
            return false;
        }
        let Some(destination) = self.destination(direction) else {
            return false;
        };
        self.snapshot();
        if let Some(enemy) = self.enemy_at(destination) {
            self.resolve_strike(enemy);
        } else {
            self.player = destination;
            self.collect_treasure();
            self.finish_if_at_exit();
            if self.phase == RoomPhase::Exploring {
                self.enemy_turn();
            }
        }
        true
    }

    pub fn strike(&mut self) -> bool {
        if self.phase != RoomPhase::Exploring {
            return false;
        }
        let Some(enemy) = self.adjacent_enemy() else {
            return false;
        };
        self.snapshot();
        self.resolve_strike(enemy);
        true
    }

    pub fn drink_potion(&mut self) -> bool {
        if self.phase != RoomPhase::Exploring || self.potions == 0 || self.health >= START_HEALTH {
            return false;
        }
        self.snapshot();
        self.potions -= 1;
        self.health = self.health.saturating_add(4).min(START_HEALTH);
        self.turns = self.turns.saturating_add(1);
        self.enemy_turn();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((player, enemies, treasure, health, potions, score, turns, seed, phase)) =
            self.history.pop()
        {
            self.player = player;
            self.enemies = enemies;
            self.treasure = treasure;
            self.health = health;
            self.potions = potions;
            self.score = score;
            self.turns = turns;
            self.seed = seed;
            self.phase = phase;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn won(&self) -> bool {
        self.phase == RoomPhase::Won
    }

    fn resolve_strike(&mut self, enemy: usize) {
        self.turns = self.turns.saturating_add(1);
        if self.enemies[enemy].health <= 2 {
            self.enemies.remove(enemy);
            self.score = self.score.saturating_add(10);
        } else {
            self.enemies[enemy].health -= 2;
        }
        self.finish_if_at_exit();
        if self.phase == RoomPhase::Exploring {
            self.enemy_turn();
        }
    }

    fn enemy_turn(&mut self) {
        let damage: u8 = self
            .enemies
            .iter()
            .filter(|enemy| Self::distance(enemy.position, self.player) == 1)
            .map(|enemy| enemy.damage)
            .sum();
        if damage > 0 {
            self.health = self.health.saturating_sub(damage);
            if self.health == 0 {
                self.phase = RoomPhase::Lost;
            }
        }
    }

    fn finish_if_at_exit(&mut self) {
        if self.player == self.exit && self.enemies.is_empty() {
            self.phase = RoomPhase::Won;
            self.score = self.score.saturating_add(50);
        }
    }

    fn collect_treasure(&mut self) {
        if self.player == self.treasure {
            self.treasure = EMPTY;
            self.potions = self.potions.saturating_add(1);
            self.score = self.score.saturating_add(5);
        }
    }

    fn destination(&self, direction: Direction) -> Option<usize> {
        let row = self.player / SIZE;
        let column = self.player % SIZE;
        match direction {
            Direction::Up if row > 0 => Some(self.player - SIZE),
            Direction::Right if column + 1 < SIZE => Some(self.player + 1),
            Direction::Down if row + 1 < SIZE => Some(self.player + SIZE),
            Direction::Left if column > 0 => Some(self.player - 1),
            _ => None,
        }
    }

    fn adjacent_enemy(&self) -> Option<usize> {
        self.enemies
            .iter()
            .position(|enemy| Self::distance(enemy.position, self.player) == 1)
    }

    fn enemy_at(&self, position: usize) -> Option<usize> {
        self.enemies
            .iter()
            .position(|enemy| enemy.position == position)
    }

    fn place_room(&mut self) {
        let mut occupied = vec![self.player, self.exit];
        for health in [3, 4, 4, 5] {
            let position = self.open_position(&occupied);
            occupied.push(position);
            self.enemies.push(RoomEnemy {
                position,
                health,
                damage: 1,
            });
        }
        self.treasure = self.open_position(&occupied);
    }

    fn open_position(&mut self, occupied: &[usize]) -> usize {
        loop {
            let position = (self.next_random() as usize) % CELLS;
            if !occupied.contains(&position) && Self::distance(position, self.player) > 1 {
                return position;
            }
        }
    }

    fn center() -> usize {
        (SIZE / 2) * SIZE + SIZE / 2
    }

    fn distance(first: usize, second: usize) -> usize {
        let first_row = first / SIZE;
        let first_column = first % SIZE;
        let second_row = second / SIZE;
        let second_column = second % SIZE;
        first_row.abs_diff(second_row) + first_column.abs_diff(second_column)
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
            self.player,
            self.enemies.clone(),
            self.treasure,
            self.health,
            self.potions,
            self.score,
            self.turns,
            self.seed,
            self.phase,
        ));
    }
}

#[cfg(test)]
mod tests;
