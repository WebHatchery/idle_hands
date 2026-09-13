//! Deterministic touch-first Higher or Lower card guessing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Guess {
    Higher,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HigherLowerStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HigherLowerRule {
    #[default]
    Friendly,
    House,
}

impl HigherLowerRule {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Friendly => "FRIENDLY TIES",
            Self::House => "HOUSE ×2",
        }
    }

    const fn multiplier(self) -> u32 {
        match self {
            Self::Friendly => 1,
            Self::House => 2,
        }
    }
}

type Snapshot = (u8, u8, u16, u16, HigherLowerStatus, u64, u32, u32, bool);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HigherLower {
    pub current: u8,
    pub next: u8,
    pub score: u16,
    pub moves: u16,
    pub status: HigherLowerStatus,
    pub seed: u64,
    #[serde(default)]
    pub rule: HigherLowerRule,
    #[serde(default)]
    pub pot: u32,
    #[serde(default)]
    pub banked: u32,
    #[serde(default)]
    pub cashed_out: bool,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for HigherLower {
    fn default() -> Self {
        Self::new(0xC4AD_0001)
    }
}

impl HigherLower {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            current: ((seed % 13) + 1) as u8,
            next: 1,
            score: 0,
            moves: 0,
            status: HigherLowerStatus::Playing,
            seed,
            rule: HigherLowerRule::Friendly,
            pot: 0,
            banked: 0,
            cashed_out: false,
            history: Vec::new(),
        };
        game.next = game.draw();
        game
    }

    pub fn guess(&mut self, guess: Guess) -> bool {
        if self.status != HigherLowerStatus::Playing {
            return false;
        }
        self.snapshot();
        self.moves = self.moves.saturating_add(1);
        let correct = match (guess, self.rule) {
            (Guess::Higher, HigherLowerRule::Friendly) => self.next >= self.current,
            (Guess::Lower, HigherLowerRule::Friendly) => self.next <= self.current,
            (Guess::Higher, HigherLowerRule::House) => self.next > self.current,
            (Guess::Lower, HigherLowerRule::House) => self.next < self.current,
        };
        self.current = self.next;
        if correct {
            self.score = self.score.saturating_add(1);
            self.pot = self
                .pot
                .saturating_add(10 * u32::from(self.score) * self.rule.multiplier());
            if self.score >= 10 {
                self.banked = self.pot;
                self.status = HigherLowerStatus::Won;
            } else {
                self.next = self.draw();
            }
        } else {
            self.pot = 0;
            self.status = HigherLowerStatus::Lost;
        }
        true
    }

    pub fn cash_out(&mut self) -> bool {
        if self.status != HigherLowerStatus::Playing || self.score < 2 {
            return false;
        }
        self.snapshot();
        self.banked = self.pot;
        self.cashed_out = true;
        self.status = HigherLowerStatus::Won;
        true
    }

    pub fn chance(&self, guess: Guess) -> u8 {
        let winning_ranks = match (guess, self.rule) {
            (Guess::Higher, HigherLowerRule::Friendly) => 14 - self.current,
            (Guess::Lower, HigherLowerRule::Friendly) => self.current,
            (Guess::Higher, HigherLowerRule::House) => 13 - self.current,
            (Guess::Lower, HigherLowerRule::House) => self.current - 1,
        };
        ((u16::from(winning_ranks) * 100) / 13) as u8
    }

    pub fn set_rule(&mut self, rule: HigherLowerRule, seed: u64) {
        *self = Self::new(seed);
        self.rule = rule;
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.current,
            self.next,
            self.score,
            self.moves,
            self.status,
            self.seed,
            self.pot,
            self.banked,
            self.cashed_out,
        ));
    }

    pub fn hint_guess(&self) -> Option<Guess> {
        if self.status != HigherLowerStatus::Playing {
            return None;
        }
        let higher_or_equal = 14 - self.current;
        let lower_or_equal = self.current;
        Some(if higher_or_equal >= lower_or_equal {
            Guess::Higher
        } else {
            Guess::Lower
        })
    }

    pub fn undo(&mut self) -> bool {
        if let Some((current, next, score, moves, status, seed, pot, banked, cashed_out)) =
            self.history.pop()
        {
            self.current = current;
            self.next = next;
            self.score = score;
            self.moves = moves;
            self.status = status;
            self.seed = seed;
            self.pot = pot;
            self.banked = banked;
            self.cashed_out = cashed_out;
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
    fn draw(&mut self) -> u8 {
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.seed % 13) + 1) as u8
    }
}

#[cfg(test)]
#[path = "../tests/legacy/higher_lower/tests.rs"]
mod tests;
