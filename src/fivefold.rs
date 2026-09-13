//! Seeded five-dice scorecard rules for the Idle Hands cabinet.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeKind,
    FourKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    FiveOfKind,
    Chance,
}

impl Category {
    pub const ALL: [Self; 13] = [
        Self::Ones,
        Self::Twos,
        Self::Threes,
        Self::Fours,
        Self::Fives,
        Self::Sixes,
        Self::ThreeKind,
        Self::FourKind,
        Self::FullHouse,
        Self::SmallStraight,
        Self::LargeStraight,
        Self::FiveOfKind,
        Self::Chance,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Ones => "Ones",
            Self::Twos => "Twos",
            Self::Threes => "Threes",
            Self::Fours => "Fours",
            Self::Fives => "Fives",
            Self::Sixes => "Sixes",
            Self::ThreeKind => "Three of a kind",
            Self::FourKind => "Four of a kind",
            Self::FullHouse => "Full house",
            Self::SmallStraight => "Small straight",
            Self::LargeStraight => "Large straight",
            Self::FiveOfKind => "Fivefold",
            Self::Chance => "Chance",
        }
    }
    pub fn index(self) -> usize {
        // Every enum variant is listed exactly once in the canonical order.
        Self::ALL
            .iter()
            .position(|category| *category == self)
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FivefoldStatus {
    Ready,
    Rolling,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FivefoldVariant {
    #[default]
    Classic,
    Quick,
    Wild,
}

impl FivefoldVariant {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Quick, Self::Wild];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC · 13 CALLS",
            Self::Quick => "QUICK · 9 CALLS",
            Self::Wild => "WILD · 5-FOLD 75",
        }
    }

    pub const fn category_limit(self) -> usize {
        match self {
            Self::Classic | Self::Wild => Category::ALL.len(),
            Self::Quick => 9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fivefold {
    pub dice: [u8; 5],
    pub held: [bool; 5],
    pub roll_number: u8,
    pub scores: [Option<u16>; 13],
    pub status: FivefoldStatus,
    pub seed: u64,
    #[serde(default)]
    pub variant: FivefoldVariant,
    #[serde(skip)]
    pub selected_category: Option<Category>,
}

impl Default for Fivefold {
    fn default() -> Self {
        Self::new(0xF1_5E_01)
    }
}

impl Fivefold {
    pub fn new(seed: u64) -> Self {
        Self::new_with_variant(seed, FivefoldVariant::default())
    }

    pub fn new_with_variant(seed: u64, variant: FivefoldVariant) -> Self {
        Self {
            dice: [0; 5],
            held: [false; 5],
            roll_number: 0,
            scores: [None; 13],
            status: FivefoldStatus::Ready,
            seed,
            variant,
            selected_category: None,
        }
    }
    pub fn roll(&mut self) -> bool {
        if self.roll_number >= 3 || self.status == FivefoldStatus::Complete {
            return false;
        }
        for index in 0..5 {
            if !self.held[index] {
                self.seed = next_seed(self.seed);
                self.dice[index] = (self.seed % 6 + 1) as u8;
            }
        }
        self.roll_number += 1;
        self.status = FivefoldStatus::Rolling;
        true
    }
    pub fn toggle_hold(&mut self, index: usize) -> bool {
        if self.roll_number == 0
            || self.roll_number >= 3
            || self.dice.get(index).is_none_or(|value| *value == 0)
        {
            return false;
        }
        self.held[index] = !self.held[index];
        true
    }
    pub fn choose_category(&mut self, category: Category) -> bool {
        if self.roll_number == 0
            || category.index() >= self.variant.category_limit()
            || self.scores[category.index()].is_some()
        {
            return false;
        }
        self.scores[category.index()] = Some(self.score_for(category));
        self.dice = [0; 5];
        self.held = [false; 5];
        self.roll_number = 0;
        self.selected_category = None;
        self.status = if self.scores[..self.variant.category_limit()]
            .iter()
            .all(Option::is_some)
        {
            FivefoldStatus::Complete
        } else {
            FivefoldStatus::Ready
        };
        true
    }
    pub fn score_for(&self, category: Category) -> u16 {
        let counts = self.counts();
        let total: u16 = self.dice.iter().map(|value| *value as u16).sum();
        match category {
            Category::Ones
            | Category::Twos
            | Category::Threes
            | Category::Fours
            | Category::Fives
            | Category::Sixes => {
                let face = category.index() as u8 + 1;
                self.dice
                    .iter()
                    .filter(|value| **value == face)
                    .map(|value| *value as u16)
                    .sum()
            }
            Category::ThreeKind => {
                if counts.iter().any(|count| *count >= 3) {
                    total
                } else {
                    0
                }
            }
            Category::FourKind => {
                if counts.iter().any(|count| *count >= 4) {
                    total
                } else {
                    0
                }
            }
            Category::FullHouse => {
                if counts.contains(&3) && counts.contains(&2) {
                    25
                } else {
                    0
                }
            }
            Category::SmallStraight => {
                if has_straight(&counts, 4) {
                    30
                } else {
                    0
                }
            }
            Category::LargeStraight => {
                if has_straight(&counts, 5) {
                    40
                } else {
                    0
                }
            }
            Category::FiveOfKind => {
                if counts.contains(&5) {
                    if self.variant == FivefoldVariant::Wild {
                        75
                    } else {
                        50
                    }
                } else {
                    0
                }
            }
            Category::Chance => total,
        }
    }
    pub fn upper_total(&self) -> u16 {
        self.scores[..6].iter().flatten().sum()
    }
    pub fn bonus(&self) -> u16 {
        let target = if self.variant == FivefoldVariant::Quick {
            45
        } else {
            63
        };
        if self.upper_total() >= target {
            if self.variant == FivefoldVariant::Quick {
                25
            } else {
                35
            }
        } else {
            0
        }
    }
    pub fn total(&self) -> u16 {
        self.scores.iter().flatten().sum::<u16>() + self.bonus()
    }

    pub fn hint_category(&self) -> Option<Category> {
        if self.roll_number == 0 || self.status == FivefoldStatus::Complete {
            return None;
        }
        let mut best = None;
        let mut best_score = 0;
        for category in Category::ALL {
            if category.index() >= self.variant.category_limit()
                || self.scores[category.index()].is_some()
            {
                continue;
            }
            let score = self.score_for(category);
            if best.is_none() || score > best_score {
                best = Some(category);
                best_score = score;
            }
        }
        best
    }

    fn counts(&self) -> [u8; 6] {
        let mut counts = [0; 6];
        for value in self.dice {
            if (1..=6).contains(&value) {
                counts[value as usize - 1] += 1;
            }
        }
        counts
    }
}

fn has_straight(counts: &[u8; 6], length: usize) -> bool {
    (0..=6 - length).any(|start| (start..start + length).all(|index| counts[index] > 0))
}
fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
#[path = "../tests/legacy/fivefold/tests.rs"]
mod tests;
