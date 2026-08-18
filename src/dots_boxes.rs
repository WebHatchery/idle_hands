//! Deterministic single-player Dots and Boxes.

use serde::{Deserialize, Serialize};

pub const SIDE: usize = 4;
const DOTS: usize = SIDE + 1;
const HORIZONTAL: usize = DOTS * SIDE;
const VERTICAL: usize = DOTS * SIDE;
const BOX_COUNT: usize = SIDE * SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DotsDifficulty {
    Standard,
    Hard,
    Expert,
}

impl Default for DotsDifficulty {
    fn default() -> Self {
        Self::Standard
    }
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

    pub fn side(self) -> usize {
        match self {
            Self::Standard => 4,
            Self::Hard => 5,
            Self::Expert => 6,
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
    pub boxes: Vec<u8>,
    pub scores: [u8; 2],
    pub current_player: u8,
    pub moves: u16,
    pub seed: u64,
    #[serde(default)]
    pub difficulty: DotsDifficulty,
    pub phase: DotsPhase,
    #[serde(skip)]
    undo: Option<Box<Self>>,
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
        let side = difficulty.side();
        let dots = side + 1;
        Self {
            horizontal: vec![false; dots * side],
            vertical: vec![false; dots * side],
            boxes: vec![0; side * side],
            scores: [0; 2],
            current_player: 0,
            moves: 0,
            seed,
            difficulty,
            phase: DotsPhase::Playing,
            undo: None,
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new_with_difficulty(seed, self.difficulty);
    }

    pub fn play(&mut self, edge: Edge) -> bool {
        if self.phase != DotsPhase::Playing || !self.edge_available(edge) {
            return false;
        }
        let previous = self.clone_without_undo();
        self.undo = Some(Box::new(previous));
        self.claim_edge(edge);
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
            .or_else(|| self.available_edges().next())
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

    fn clone_without_undo(&self) -> Self {
        let mut copy = self.clone();
        copy.undo = None;
        copy
    }

    fn claim_edge(&mut self, edge: Edge) -> bool {
        let (horizontal, vertical) = self.edge_counts();
        match edge {
            Edge::Horizontal(index) if index < horizontal && !self.horizontal[index] => {
                self.horizontal[index] = true;
                true
            }
            Edge::Vertical(index) if index < vertical && !self.vertical[index] => {
                self.vertical[index] = true;
                true
            }
            _ => false,
        }
    }

    fn edge_available(&self, edge: Edge) -> bool {
        let (horizontal, vertical) = self.edge_counts();
        match edge {
            Edge::Horizontal(index) if index < horizontal => !self.horizontal[index],
            Edge::Vertical(index) if index < vertical => !self.vertical[index],
            _ => false,
        }
    }

    fn claim_completed(&mut self, owner: u8) -> usize {
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

    fn box_complete(&self, row: usize, col: usize) -> bool {
        let side = self.side();
        let dots = side + 1;
        self.horizontal[row * side + col]
            && self.horizontal[(row + 1) * side + col]
            && self.vertical[row * dots + col]
            && self.vertical[row * dots + col + 1]
    }

    fn available_edges(&self) -> impl Iterator<Item = Edge> + '_ {
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

    fn cpu_turn(&mut self) {
        while self.phase == DotsPhase::Playing && self.current_player == 1 {
            let edge = self
                .available_edges()
                .find(|edge| self.would_complete(*edge))
                .or_else(|| {
                    let available: Vec<_> = self.available_edges().collect();
                    if available.is_empty() {
                        None
                    } else {
                        Some(available[(self.seed as usize) % available.len()])
                    }
                });
            let Some(edge) = edge else {
                self.finish();
                break;
            };
            self.claim_edge(edge);
            self.moves = self.moves.saturating_add(1);
            let scored = self.claim_completed(2);
            if self.finished() {
                self.finish();
            } else if scored == 0 {
                self.current_player = 0;
            }
        }
    }

    fn would_complete(&self, edge: Edge) -> bool {
        let mut copy = self.clone_without_undo();
        if !copy.claim_edge(edge) {
            return false;
        }
        copy.claim_completed(2) > 0
    }

    fn finished(&self) -> bool {
        self.boxes.iter().all(|owner| *owner != 0)
    }

    fn side(&self) -> usize {
        self.difficulty.side()
    }

    fn edge_counts(&self) -> (usize, usize) {
        let side = self.side();
        (side * (side + 1), side * (side + 1))
    }

    fn finish(&mut self) {
        self.phase = if self.scores[0] > self.scores[1] {
            DotsPhase::Won
        } else {
            DotsPhase::Lost
        };
    }
}

#[cfg(test)]
mod tests;
