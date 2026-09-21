//! Deterministic touch-first Nim with a bounded local opponent.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NimStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NimRule {
    #[default]
    Normal,
    Misere,
}

impl NimRule {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Misere => "MISERE",
        }
    }
}

pub type Snapshot = ([u8; 3], u16, NimStatus);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nim {
    pub seed: u64,
    pub heaps: [u8; 3],
    pub selected_heap: Option<usize>,
    pub moves: u16,
    #[serde(default)]
    pub rule: NimRule,
    #[serde(default)]
    pub last_player_take: u8,
    #[serde(default)]
    pub last_ai_take: u8,
    #[serde(default)]
    pub last_player_heap: Option<usize>,
    #[serde(default)]
    pub last_ai_heap: Option<usize>,
    pub status: NimStatus,
    #[serde(skip)]
    pub history: Vec<Snapshot>,
}

impl Default for Nim {
    fn default() -> Self {
        Self::new(0x004E_494D)
    }
}

impl Nim {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            seed,
            heaps: [0; 3],
            selected_heap: None,
            moves: 0,
            rule: NimRule::Normal,
            last_player_take: 0,
            last_ai_take: 0,
            last_player_heap: None,
            last_ai_heap: None,
            status: NimStatus::Playing,
            history: Vec::new(),
        };
        game.reset(seed);
        game
    }

    pub fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.heaps = [
            3 + (seed as u8 % 3),
            4 + ((seed >> 8) as u8 % 3),
            5 + ((seed >> 16) as u8 % 3),
        ];
        self.selected_heap = None;
        self.moves = 0;
        self.last_player_take = 0;
        self.last_ai_take = 0;
        self.last_player_heap = None;
        self.last_ai_heap = None;
        self.status = NimStatus::Playing;
        self.history.clear();
    }

    pub fn set_rule(&mut self, rule: NimRule, seed: u64) {
        self.rule = rule;
        self.reset(seed);
    }

    pub fn select_heap(&mut self, heap: usize) {
        if self.status == NimStatus::Playing && heap < self.heaps.len() && self.heaps[heap] > 0 {
            self.selected_heap = Some(heap);
            self.last_player_take = 0;
            self.last_ai_take = 0;
            self.last_player_heap = None;
            self.last_ai_heap = None;
        }
    }

    pub fn take(&mut self, amount: u8) -> bool {
        let Some(heap) = self.selected_heap else {
            return false;
        };
        if self.status != NimStatus::Playing
            || amount == 0
            || amount > 3
            || amount > self.heaps[heap]
        {
            return false;
        }
        self.history.push((self.heaps, self.moves, self.status));
        self.heaps[heap] -= amount;
        self.last_player_take = amount;
        self.last_ai_take = 0;
        self.last_player_heap = Some(heap);
        self.last_ai_heap = None;
        self.selected_heap = None;
        self.moves = self.moves.saturating_add(1);
        if self.heaps == [0; 3] {
            self.status = match self.rule {
                NimRule::Normal => NimStatus::Won,
                NimRule::Misere => NimStatus::Lost,
            };
            return true;
        }
        self.ai_move();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((heaps, moves, status)) = self.history.pop() {
            self.heaps = heaps;
            self.moves = moves;
            self.status = status;
            self.selected_heap = None;
            self.last_player_take = 0;
            self.last_ai_take = 0;
            self.last_player_heap = None;
            self.last_ai_heap = None;
            true
        } else {
            false
        }
    }

    pub fn won(&self) -> bool {
        self.status == NimStatus::Won
    }

    pub fn hint_move(&self) -> Option<(usize, u8)> {
        if self.status != NimStatus::Playing {
            return None;
        }
        legal_moves(self.heaps)
            .find(|&(heap, amount)| self.move_is_winning(heap, amount) == Some(true))
            .or_else(|| legal_moves(self.heaps).next())
    }

    pub fn move_is_winning(&self, heap: usize, amount: u8) -> Option<bool> {
        if heap >= self.heaps.len() || amount == 0 || amount > 3 || amount > self.heaps[heap] {
            return None;
        }
        let mut next = self.heaps;
        next[heap] -= amount;
        if next == [0; 3] {
            return Some(self.rule == NimRule::Normal);
        }
        Some(!winning_position(next, self.rule))
    }

    pub fn ai_move(&mut self) {
        let (heap, amount) = legal_moves(self.heaps)
            .find(|&(heap, amount)| {
                let mut next = self.heaps;
                next[heap] -= amount;
                !winning_position(next, self.rule)
            })
            .or_else(|| legal_moves(self.heaps).next())
            .unwrap_or((0, 0));
        if amount > 0 {
            self.heaps[heap] -= amount;
            self.last_ai_take = amount;
            self.last_ai_heap = Some(heap);
        }
        if self.heaps == [0; 3] {
            self.status = match self.rule {
                NimRule::Normal => NimStatus::Lost,
                NimRule::Misere => NimStatus::Won,
            };
        }
    }
}

pub fn legal_moves(heaps: [u8; 3]) -> impl Iterator<Item = (usize, u8)> {
    (0..3).flat_map(move |heap| (1..=heaps[heap].min(3)).map(move |amount| (heap, amount)))
}

pub fn winning_position(heaps: [u8; 3], rule: NimRule) -> bool {
    winning_position_cached(heaps, rule, &mut HashMap::new())
}

pub fn winning_position_cached(
    heaps: [u8; 3],
    rule: NimRule,
    memo: &mut HashMap<[u8; 3], bool>,
) -> bool {
    if heaps == [0; 3] {
        return rule == NimRule::Misere;
    }
    if let Some(&result) = memo.get(&heaps) {
        return result;
    }
    let result = legal_moves(heaps).any(|(heap, amount)| {
        let mut next = heaps;
        next[heap] -= amount;
        !winning_position_cached(next, rule, memo)
    });
    memo.insert(heaps, result);
    result
}
