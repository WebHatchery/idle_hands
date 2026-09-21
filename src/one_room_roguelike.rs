//! Deterministic room-to-room exploration with short, turn-based combat.

use crate::domain::Direction;
use serde::{Deserialize, Serialize};

pub const SIZE: usize = 7;
pub const CELLS: usize = SIZE * SIZE;
pub const START_HEALTH: u8 = 10;
pub const START_POTIONS: u8 = 2;
pub const START_ROOM: u16 = 1;
pub const TARGET_ROOM: u16 = 5;
pub const START_ENEMY_HEALTH: [u8; 4] = [3, 4, 4, 5];
pub const EMPTY: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomEnemy {
    pub position: usize,
    pub health: u8,
    pub damage: u8,
    #[serde(default)]
    pub kind: EnemyKind,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyKind {
    #[default]
    Guard,
    Stalker,
    Brute,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroClass {
    #[default]
    Blade,
    Warden,
    Alchemist,
}

impl HeroClass {
    pub const ALL: [Self; 3] = [Self::Blade, Self::Warden, Self::Alchemist];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Blade => "BLADE",
            Self::Warden => "WARDEN",
            Self::Alchemist => "ALCHEMIST",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomPhase {
    Exploring,
    Stairs,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RogueHint {
    Strike,
    Potion,
    Move(Direction),
}

pub type Snapshot = (
    usize,
    Vec<RoomEnemy>,
    usize,
    u8,
    u8,
    u32,
    u16,
    u64,
    RoomPhase,
    u16,
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
    #[serde(default)]
    pub hero_class: HeroClass,
    #[serde(default = "starting_room")]
    pub room: u16,
    #[serde(skip)]
    pub history: Vec<Snapshot>,
}

impl Default for OneRoomRoguelike {
    fn default() -> Self {
        Self::new(0x0E_700001)
    }
}

impl OneRoomRoguelike {
    pub fn new(seed: u64) -> Self {
        Self::new_with_class(seed, HeroClass::Blade)
    }

    pub fn new_with_class(seed: u64, hero_class: HeroClass) -> Self {
        let mut room = Self {
            player: Self::center(),
            enemies: Vec::new(),
            treasure: EMPTY,
            exit: CELLS - SIZE,
            health: Self::class_max_health(hero_class),
            potions: Self::class_starting_potions(hero_class),
            score: 0,
            turns: 0,
            seed,
            phase: RoomPhase::Exploring,
            hero_class,
            room: START_ROOM,
            history: Vec::new(),
        };
        room.place_room();
        room
    }

    pub const fn size() -> usize {
        SIZE
    }

    pub const fn target_room() -> u16 {
        TARGET_ROOM
    }

    pub fn max_health(&self) -> u8 {
        Self::class_max_health(self.hero_class)
    }

    pub fn attack_damage(&self) -> u8 {
        match self.hero_class {
            HeroClass::Blade => 3,
            HeroClass::Warden | HeroClass::Alchemist => 2,
        }
    }

    pub fn move_in(&mut self, direction: Direction) -> bool {
        if !matches!(self.phase, RoomPhase::Exploring | RoomPhase::Stairs) {
            return false;
        }
        let Some(destination) = self.destination(direction) else {
            return false;
        };
        self.snapshot();
        if let Some(enemy) = self.enemy_at(destination) {
            if self.phase != RoomPhase::Exploring {
                self.history.pop();
                return false;
            }
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

    pub fn hint_action(&self) -> Option<RogueHint> {
        if !matches!(self.phase, RoomPhase::Exploring | RoomPhase::Stairs) {
            return None;
        }
        if self.phase == RoomPhase::Exploring {
            if let Some(enemy) = self.adjacent_enemy() {
                if self.health <= 4 && self.potions > 0 && self.enemies[enemy].damage > 0 {
                    return Some(RogueHint::Potion);
                }
                return Some(RogueHint::Strike);
            }
        }
        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|direction| {
            self.destination(direction)
                .map(|destination| (direction, Self::distance(destination, self.exit)))
        })
        .min_by_key(|(_, distance)| *distance)
        .map(|(direction, _)| RogueHint::Move(direction))
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
        if self.phase != RoomPhase::Exploring
            || self.potions == 0
            || self.health >= self.max_health()
        {
            return false;
        }
        self.snapshot();
        self.potions -= 1;
        let healing = if self.hero_class == HeroClass::Alchemist {
            6
        } else {
            4
        };
        self.health = self.health.saturating_add(healing).min(self.max_health());
        self.turns = self.turns.saturating_add(1);
        self.enemy_turn();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((player, enemies, treasure, health, potions, score, turns, seed, phase, room)) =
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
            self.room = room;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_class(seed, self.hero_class);
    }

    pub fn won(&self) -> bool {
        self.phase == RoomPhase::Won
    }

    pub fn resolve_strike(&mut self, enemy: usize) {
        self.turns = self.turns.saturating_add(1);
        let damage = self.attack_damage();
        if self.enemies[enemy].health <= damage {
            let kind = self.enemies[enemy].kind;
            self.enemies.remove(enemy);
            self.score = self.score.saturating_add(self.enemy_score(kind));
        } else {
            self.enemies[enemy].health -= damage;
        }
        self.finish_if_at_exit();
        if self.phase == RoomPhase::Exploring {
            self.enemy_turn();
        }
    }

    pub fn enemy_turn(&mut self) {
        self.move_stalkers();
        let damage: u8 = self
            .enemies
            .iter()
            .filter(|enemy| Self::distance(enemy.position, self.player) == 1)
            .map(|enemy| {
                if self.hero_class == HeroClass::Warden {
                    enemy.damage.saturating_sub(1)
                } else {
                    enemy.damage
                }
            })
            .sum();
        if damage > 0 {
            self.health = self.health.saturating_sub(damage);
            if self.health == 0 {
                self.phase = RoomPhase::Lost;
            }
        }
    }

    pub fn finish_if_at_exit(&mut self) {
        if self.phase == RoomPhase::Exploring && self.enemies.is_empty() {
            self.phase = RoomPhase::Stairs;
        }
        if self.phase == RoomPhase::Stairs && self.player == self.exit {
            self.score = self.score.saturating_add(self.room_clear_score());
            if self.room >= TARGET_ROOM {
                self.phase = RoomPhase::Won;
            } else {
                self.enter_next_room();
            }
        }
    }

    pub fn collect_treasure(&mut self) {
        if self.player == self.treasure {
            self.treasure = EMPTY;
            self.potions = self.potions.saturating_add(1);
            self.score = self.score.saturating_add(self.treasure_score());
        }
    }

    pub fn destination(&self, direction: Direction) -> Option<usize> {
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

    pub fn adjacent_enemy(&self) -> Option<usize> {
        self.enemies
            .iter()
            .position(|enemy| Self::distance(enemy.position, self.player) == 1)
    }

    pub fn enemy_at(&self, position: usize) -> Option<usize> {
        self.enemies
            .iter()
            .position(|enemy| enemy.position == position)
    }

    pub fn place_room(&mut self) {
        self.enemies.clear();
        let mut occupied = vec![self.player, self.exit];
        for index in 0..self.enemy_count() {
            let position = self.open_position(&occupied);
            occupied.push(position);
            let kind = self.enemy_kind(index);
            self.enemies.push(RoomEnemy {
                position,
                health: self.enemy_health(index, kind),
                damage: self.enemy_damage(kind),
                kind,
            });
        }
        self.treasure = self.open_position(&occupied);
    }

    pub fn enter_next_room(&mut self) {
        self.room = self.room.saturating_add(1);
        self.player = Self::center();
        self.exit = CELLS - SIZE;
        self.treasure = EMPTY;
        self.phase = RoomPhase::Exploring;
        self.place_room();
    }

    pub fn enemy_count(&self) -> usize {
        4 + self.room.saturating_sub(1).min(4) as usize
    }

    pub fn enemy_kind(&self, index: usize) -> EnemyKind {
        if self.room == START_ROOM {
            EnemyKind::Guard
        } else if self.room == TARGET_ROOM && index == 0
            || self.room >= 3 && index.is_multiple_of(4)
        {
            EnemyKind::Brute
        } else if self.room >= 2 && index % 3 == 1 {
            EnemyKind::Stalker
        } else {
            EnemyKind::Guard
        }
    }

    pub fn enemy_health(&self, index: usize, kind: EnemyKind) -> u8 {
        if self.room == START_ROOM && index < START_ENEMY_HEALTH.len() {
            START_ENEMY_HEALTH[index]
        } else {
            match kind {
                EnemyKind::Guard => 3 + ((index + self.room as usize) % 3) as u8,
                EnemyKind::Stalker => 3 + (self.room / 4) as u8,
                EnemyKind::Brute => 6 + (self.room / 2) as u8,
            }
        }
    }

    pub fn enemy_damage(&self, kind: EnemyKind) -> u8 {
        match kind {
            EnemyKind::Guard => 1 + (self.room.saturating_sub(1) / 3).min(2) as u8,
            EnemyKind::Stalker => 1 + (self.room / 4).min(1) as u8,
            EnemyKind::Brute => 2 + (self.room / 4).min(1) as u8,
        }
    }

    pub fn enemy_score(&self, kind: EnemyKind) -> u32 {
        let base = 10 + u32::from(self.room.saturating_sub(1)) * 2;
        base + match kind {
            EnemyKind::Guard => 0,
            EnemyKind::Stalker => 4,
            EnemyKind::Brute => 10,
        }
    }

    pub fn treasure_score(&self) -> u32 {
        5 + u32::from(self.room.saturating_sub(1))
    }

    pub fn room_clear_score(&self) -> u32 {
        50 + u32::from(self.room.saturating_sub(1)) * 10
    }

    pub fn move_stalkers(&mut self) {
        for index in 0..self.enemies.len() {
            if self.enemies[index].kind != EnemyKind::Stalker
                || Self::distance(self.enemies[index].position, self.player) <= 1
            {
                continue;
            }
            let position = self.enemies[index].position;
            let row = position / SIZE;
            let column = position % SIZE;
            let player_row = self.player / SIZE;
            let player_column = self.player % SIZE;
            let horizontal = if column < player_column {
                Some(position + 1)
            } else if column > player_column {
                Some(position - 1)
            } else {
                None
            };
            let vertical = if row < player_row {
                Some(position + SIZE)
            } else if row > player_row {
                Some(position - SIZE)
            } else {
                None
            };
            let candidates = if (index + self.room as usize).is_multiple_of(2) {
                [horizontal, vertical]
            } else {
                [vertical, horizontal]
            };
            if let Some(destination) = candidates.into_iter().flatten().find(|candidate| {
                *candidate != self.player
                    && *candidate != self.exit
                    && *candidate != self.treasure
                    && !self
                        .enemies
                        .iter()
                        .enumerate()
                        .any(|(other, enemy)| other != index && enemy.position == *candidate)
            }) {
                self.enemies[index].position = destination;
            }
        }
    }

    pub const fn class_max_health(hero_class: HeroClass) -> u8 {
        match hero_class {
            HeroClass::Blade | HeroClass::Alchemist => START_HEALTH,
            HeroClass::Warden => 14,
        }
    }

    pub const fn class_starting_potions(hero_class: HeroClass) -> u8 {
        match hero_class {
            HeroClass::Blade | HeroClass::Warden => START_POTIONS,
            HeroClass::Alchemist => 3,
        }
    }

    pub fn open_position(&mut self, occupied: &[usize]) -> usize {
        loop {
            let position = (self.next_random() as usize) % CELLS;
            if !occupied.contains(&position) && Self::distance(position, self.player) > 1 {
                return position;
            }
        }
    }

    pub fn center() -> usize {
        (SIZE / 2) * SIZE + SIZE / 2
    }

    pub fn distance(first: usize, second: usize) -> usize {
        let first_row = first / SIZE;
        let first_column = first % SIZE;
        let second_row = second / SIZE;
        let second_column = second % SIZE;
        first_row.abs_diff(second_row) + first_column.abs_diff(second_column)
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
            self.player,
            self.enemies.clone(),
            self.treasure,
            self.health,
            self.potions,
            self.score,
            self.turns,
            self.seed,
            self.phase,
            self.room,
        ));
    }
}

pub fn starting_room() -> u16 {
    START_ROOM
}
