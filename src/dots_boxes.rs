//! Deterministic single-player Dots and Boxes.

use serde::{Deserialize, Serialize};

use crate::data::DotsBoxesConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DotsDifficulty {
    #[default]
    Standard,
    Hard,
    Expert,
}

impl DotsDifficulty {
    pub const ALL: [Self; 3] = [Self::Standard, Self::Hard, Self::Expert];

    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Hard => "HARD",
            Self::Expert => "EXPERT",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Standard => 0,
            Self::Hard => 1,
            Self::Expert => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DotsPhase {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edge {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotsBoxes {
    pub horizontal: Vec<bool>,
    pub vertical: Vec<bool>,
    #[serde(default)]
    pub horizontal_owners: Vec<u8>,
    #[serde(default)]
    pub vertical_owners: Vec<u8>,
    pub boxes: Vec<u8>,
    pub scores: [u8; 2],
    pub current_player: u8,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: DotsDifficulty,
    #[serde(default = "default_side")]
    pub side: usize,
    pub phase: DotsPhase,
    #[serde(skip)]
    pub undo: Option<Box<Self>>,
}

impl Default for DotsBoxes {
    fn default() -> Self {
        Self::new(0x00D0_7B0E)
    }
}

impl DotsBoxes {
    pub fn new(seed: u64) -> Self {
        Self::new_with_difficulty(seed, DotsDifficulty::Standard)
    }

    pub fn new_with_difficulty(seed: u64, difficulty: DotsDifficulty) -> Self {
        Self::new_with_config(seed, difficulty, &DotsBoxesConfig::default())
    }

    pub fn new_with_config(
        seed: u64,
        difficulty: DotsDifficulty,
        config: &DotsBoxesConfig,
    ) -> Self {
        // GameData validation guarantees one row for every difficulty variant.
        let side = config
            .difficulties
            .get(difficulty.index())
            .expect("validated Dots and Boxes difficulty configuration")
            .side;
        Self::new_with_settings(seed, difficulty, side)
    }

    pub fn new_with_settings(seed: u64, difficulty: DotsDifficulty, side: usize) -> Self {
        let dots = side + 1;
        Self {
            horizontal: vec![false; dots * side],
            vertical: vec![false; dots * side],
            horizontal_owners: vec![0; dots * side],
            vertical_owners: vec![0; dots * side],
            boxes: vec![0; side * side],
            scores: [0; 2],
            current_player: 0,
            moves: 0,
            seed,
            difficulty,
            side,
            phase: DotsPhase::Playing,
            undo: None,
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_settings(seed, self.difficulty, self.side);
    }

    pub fn play(&mut self, edge: Edge) -> bool {
        if self.phase != DotsPhase::Playing || !self.edge_available(edge) {
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        self.claim_edge(edge, self.current_player + 1);
        self.moves = self.moves.saturating_add(1);
        let scored = self.claim_completed(self.current_player + 1);
        if self.finished() {
            self.finish();
        } else if scored == 0 {
            self.current_player = 1;
            self.cpu_turn();
        }
        true
    }

    pub fn hint_edge(&self) -> Option<Edge> {
        if self.phase != DotsPhase::Playing {
            return None;
        }
        self.available_edges()
            .find(|&edge| self.would_complete(edge))
            .or_else(|| self.best_safe_edge())
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.take() else {
            return false;
        };
        *self = *previous;
        true
    }

    pub fn won(&self) -> bool {
        self.phase == DotsPhase::Won
    }

    pub fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    pub fn claim_edge(&mut self, edge: Edge, owner: u8) -> bool {
        self.ensure_owner_lengths();
        let (horizontal, vertical) = self.edge_counts();
        match edge {
            Edge::Horizontal(index) if index < horizontal && !self.horizontal[index] => {
                self.horizontal[index] = true;
                self.horizontal_owners[index] = owner;
                true
            }
            Edge::Vertical(index) if index < vertical && !self.vertical[index] => {
                self.vertical[index] = true;
                self.vertical_owners[index] = owner;
                true
            }
            _ => false,
        }
    }

    pub fn edge_available(&self, edge: Edge) -> bool {
        let (horizontal, vertical) = self.edge_counts();
        match edge {
            Edge::Horizontal(index) if index < horizontal => !self.horizontal[index],
            Edge::Vertical(index) if index < vertical => !self.vertical[index],
            _ => false,
        }
    }

    pub fn claim_completed(&mut self, owner: u8) -> usize {
        let mut scored = 0;
        let side = self.side();
        for row in 0..side {
            for col in 0..side {
                let box_index = row * side + col;
                if self.boxes[box_index] == 0 && self.box_complete(row, col) {
                    self.boxes[box_index] = owner;
                    self.scores[(owner - 1) as usize] += 1;
                    scored += 1;
                }
            }
        }
        scored
    }

    pub fn box_complete(&self, row: usize, col: usize) -> bool {
        let side = self.side();
        let dots = side + 1;
        self.horizontal[row * side + col]
            && self.horizontal[(row + 1) * side + col]
            && self.vertical[row * dots + col]
            && self.vertical[row * dots + col + 1]
    }

    pub fn available_edges(&self) -> impl Iterator<Item = Edge> + '_ {
        let (horizontal, vertical) = self.edge_counts();
        self.horizontal
            .iter()
            .enumerate()
            .take(horizontal)
            .filter(|(_, used)| !**used)
            .map(|(index, _)| Edge::Horizontal(index))
            .chain(
                self.vertical
                    .iter()
                    .enumerate()
                    .take(vertical)
                    .filter(|(_, used)| !**used)
                    .map(|(index, _)| Edge::Vertical(index)),
            )
    }

    pub fn cpu_turn(&mut self) {
        while self.phase == DotsPhase::Playing && self.current_player == 1 {
            let edge = self.choose_cpu_edge();
            let Some(edge) = edge else {
                self.finish();
                break;
            };
            self.claim_edge(edge, 2);
            self.moves = self.moves.saturating_add(1);
            let scored = self.claim_completed(2);
            if self.finished() {
                self.finish();
            } else if scored == 0 {
                self.current_player = 0;
            }
        }
    }

    pub fn would_complete(&self, edge: Edge) -> bool {
        let mut copy = self.clone_without_undo();
        if !copy.claim_edge(edge, 2) {
            return false;
        }
        copy.claim_completed(2) > 0
    }

    pub fn finished(&self) -> bool {
        self.boxes.iter().all(|owner| *owner != 0)
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn box_edge_count(&self, row: usize, col: usize) -> u8 {
        if row >= self.side || col >= self.side {
            return 0;
        }
        let dots = self.side + 1;
        [
            self.horizontal[row * self.side + col],
            self.horizontal[(row + 1) * self.side + col],
            self.vertical[row * dots + col],
            self.vertical[row * dots + col + 1],
        ]
        .into_iter()
        .filter(|drawn| *drawn)
        .count() as u8
    }

    pub fn edge_owner(&self, edge: Edge) -> u8 {
        match edge {
            Edge::Horizontal(index) => self.horizontal_owners.get(index).copied().unwrap_or(0),
            Edge::Vertical(index) => self.vertical_owners.get(index).copied().unwrap_or(0),
        }
    }

    pub fn choose_cpu_edge(&self) -> Option<Edge> {
        if let Some(edge) = self
            .available_edges()
            .find(|edge| self.would_complete(*edge))
        {
            return Some(edge);
        }
        let available: Vec<_> = self.available_edges().collect();
        if available.is_empty() {
            return None;
        }
        match self.difficulty {
            DotsDifficulty::Standard => Some(available[(self.seed as usize) % available.len()]),
            DotsDifficulty::Hard => {
                let safe: Vec<_> = available
                    .iter()
                    .copied()
                    .filter(|edge| self.edge_risk(*edge) == 0)
                    .collect();
                let choices = if safe.is_empty() { &available } else { &safe };
                Some(choices[(self.seed as usize) % choices.len()])
            }
            DotsDifficulty::Expert => available.into_iter().min_by_key(|edge| {
                (
                    self.edge_risk(*edge),
                    edge_order(*edge).wrapping_add(self.seed as usize) % 10_000,
                )
            }),
        }
    }

    pub fn best_safe_edge(&self) -> Option<Edge> {
        self.available_edges()
            .min_by_key(|edge| (self.edge_risk(*edge), edge_order(*edge)))
    }

    pub fn edge_risk(&self, edge: Edge) -> u8 {
        let before = self.danger_box_count();
        let mut copy = self.clone_without_undo();
        if !copy.claim_edge(edge, self.current_player + 1) {
            return u8::MAX;
        }
        copy.danger_box_count().saturating_sub(before)
    }

    pub fn danger_box_count(&self) -> u8 {
        let side = self.side();
        (0..side)
            .flat_map(|row| (0..side).map(move |col| (row, col)))
            .filter(|(row, col)| {
                self.boxes[row * side + col] == 0 && self.box_edge_count(*row, *col) == 3
            })
            .count()
            .min(u8::MAX as usize) as u8
    }

    pub fn ensure_owner_lengths(&mut self) {
        let (horizontal, vertical) = self.edge_counts();
        self.horizontal_owners.resize(horizontal, 0);
        self.vertical_owners.resize(vertical, 0);
    }

    pub fn edge_counts(&self) -> (usize, usize) {
        let side = self.side();
        (side * (side + 1), side * (side + 1))
    }

    pub fn finish(&mut self) {
        self.phase = if self.scores[0] > self.scores[1] {
            DotsPhase::Won
        } else {
            DotsPhase::Lost
        };
    }
}

pub fn edge_order(edge: Edge) -> usize {
    match edge {
        Edge::Horizontal(index) => index,
        Edge::Vertical(index) => 10_000 + index,
    }
}

pub fn default_side() -> usize {
    DotsBoxesConfig::default().difficulties[0].side
}
