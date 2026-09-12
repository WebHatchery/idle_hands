//! Deterministic touch-first TriPeaks Solitaire.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriPeaksStatus {
    Playing,
    Won,
    Stuck,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriPeaksRule {
    #[default]
    Strict,
    Wrap,
}

impl TriPeaksRule {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Strict => "STRICT",
            Self::Wrap => "A↔K WRAP",
        }
    }
}

type Snapshot = (
    Vec<Option<Card>>,
    Vec<Card>,
    Vec<Card>,
    u16,
    TriPeaksStatus,
    u32,
    u16,
    u16,
    u8,
    bool,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriPeaks {
    pub tableau: Vec<Option<Card>>,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub moves: u16,
    pub status: TriPeaksStatus,
    pub seed: u64,
    #[serde(default)]
    pub rule: TriPeaksRule,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub run: u16,
    #[serde(default)]
    pub best_run: u16,
    #[serde(default = "default_bridges")]
    pub bridges: u8,
    #[serde(default)]
    pub bridge_armed: bool,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for TriPeaks {
    fn default() -> Self {
        Self::new(0x7A1F_0001)
    }
}

impl TriPeaks {
    pub fn new(seed: u64) -> Self {
        let (deck, rng) = shuffled_deck(seed, true);
        let tableau = deck.iter().take(28).copied().map(Some).collect();
        let mut stock = deck[28..].to_vec();
        stock.reverse();
        let waste = stock.pop().into_iter().collect();
        let mut game = Self {
            tableau,
            stock,
            waste,
            moves: 0,
            status: TriPeaksStatus::Playing,
            seed: rng,
            rule: TriPeaksRule::Strict,
            points: 0,
            run: 0,
            best_run: 0,
            bridges: default_bridges(),
            bridge_armed: false,
            history: Vec::new(),
        };
        game.resolve();
        game
    }

    pub fn tap(&mut self, index: usize) -> bool {
        if self.status != TriPeaksStatus::Playing || !self.can_play(index) {
            return false;
        }
        self.snapshot();
        // The playable-card guard above identifies an occupied tableau slot.
        let card = self.tableau[index].take().expect("playable card");
        self.waste.push(card);
        if self.bridge_armed {
            self.bridges = self.bridges.saturating_sub(1);
            self.bridge_armed = false;
        }
        self.run = self.run.saturating_add(1);
        self.best_run = self.best_run.max(self.run);
        self.points = self
            .points
            .saturating_add(10_u32.saturating_mul(u32::from(self.run)))
            .saturating_add(if index < 3 { 50 } else { 0 });
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn draw_stock(&mut self) -> bool {
        if self.status != TriPeaksStatus::Playing || self.stock.is_empty() {
            self.resolve();
            return false;
        }
        self.snapshot();
        // A non-empty stock is required by the branch above.
        self.waste
            .push(self.stock.pop().expect("stock checked above"));
        self.run = 0;
        self.bridge_armed = false;
        self.moves = self.moves.saturating_add(1);
        self.resolve();
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some((
            tableau,
            stock,
            waste,
            moves,
            status,
            points,
            run,
            best_run,
            bridges,
            bridge_armed,
        )) = self.history.pop()
        {
            self.tableau = tableau;
            self.stock = stock;
            self.waste = waste;
            self.moves = moves;
            self.status = status;
            self.points = points;
            self.run = run;
            self.best_run = best_run;
            self.bridges = bridges;
            self.bridge_armed = bridge_armed;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        let rule = self.rule;
        *self = Self::new(seed);
        self.rule = rule;
    }

    pub fn set_rule(&mut self, rule: TriPeaksRule, seed: u64) {
        *self = Self::new(seed);
        self.rule = rule;
    }

    pub fn toggle_bridge(&mut self) -> bool {
        if self.status != TriPeaksStatus::Playing || self.bridges == 0 {
            return false;
        }
        self.bridge_armed = !self.bridge_armed;
        true
    }

    pub fn exposed(&self, index: usize) -> bool {
        index < self.tableau.len()
            && self.tableau[index].is_some()
            && children(index)
                .iter()
                .all(|&child| self.tableau[child].is_none())
    }

    pub fn can_play(&self, index: usize) -> bool {
        self.exposed(index)
            && (self.bridge_armed
                || self
                    .waste
                    .last()
                    .zip(self.tableau[index])
                    .is_some_and(|(waste, card)| adjacent(card.rank, waste.rank, self.rule)))
    }

    pub fn playable_count(&self) -> usize {
        (0..28).filter(|&index| self.can_play(index)).count()
    }

    fn resolve(&mut self) {
        if self.tableau.iter().all(Option::is_none) {
            self.status = TriPeaksStatus::Won;
        } else if self.stock.is_empty()
            && self.bridges == 0
            && !(0..28).any(|index| self.can_play(index))
        {
            self.status = TriPeaksStatus::Stuck;
        } else {
            self.status = TriPeaksStatus::Playing;
        }
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.tableau.clone(),
            self.stock.clone(),
            self.waste.clone(),
            self.moves,
            self.status,
            self.points,
            self.run,
            self.best_run,
            self.bridges,
            self.bridge_armed,
        ));
    }
}

fn adjacent(first: u8, second: u8, rule: TriPeaksRule) -> bool {
    first.abs_diff(second) == 1
        || (rule == TriPeaksRule::Wrap
            && ((first == 1 && second == 13) || (first == 13 && second == 1)))
}

const fn default_bridges() -> u8 {
    1
}

fn children(index: usize) -> &'static [usize] {
    match index {
        0 => &[3, 4],
        1 => &[5, 6],
        2 => &[7, 8],
        3 => &[9, 10],
        4 => &[10, 11],
        5 => &[12, 13],
        6 => &[13, 14],
        7 => &[15, 16],
        8 => &[16, 17],
        9 => &[18, 19],
        10 => &[19, 20],
        11 => &[20, 21],
        12 => &[21, 22],
        13 => &[22, 23],
        14 => &[23, 24],
        15 => &[24, 25],
        16 => &[25, 26],
        17 => &[26, 27],
        _ => &[],
    }
}

#[cfg(test)]
mod tests;
