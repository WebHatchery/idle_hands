//! Deterministic touch-first Hangman rules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HangmanCategory {
    #[default]
    Cabinet,
    Nature,
    Voyage,
}

impl HangmanCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cabinet => "CABINET",
            Self::Nature => "NATURE",
            Self::Voyage => "VOYAGE",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Cabinet => Self::Nature,
            Self::Nature => Self::Voyage,
            Self::Voyage => Self::Cabinet,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Cabinet => "cabinet",
            Self::Nature => "nature",
            Self::Voyage => "voyage",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HangmanRule {
    #[default]
    Classic,
    Rapid,
}

impl HangmanRule {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC 6",
            Self::Rapid => "RAPID 4 ×2",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HangmanStatus {
    Playing,
    Won,
    Lost,
}

type Snapshot = (
    [bool; 26],
    [bool; 26],
    u8,
    u16,
    HangmanStatus,
    u32,
    u8,
    u8,
    u8,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hangman {
    pub word: String,
    pub guessed: [bool; 26],
    pub wrong: [bool; 26],
    pub wrong_count: u8,
    pub moves: u16,
    pub status: HangmanStatus,
    pub seed: u64,
    #[serde(default)]
    pub category: HangmanCategory,
    #[serde(default)]
    pub rule: HangmanRule,
    #[serde(default)]
    pub score: u32,
    #[serde(default)]
    pub combo: u8,
    #[serde(default)]
    pub best_combo: u8,
    #[serde(default = "default_classic_wrong")]
    pub max_wrong: u8,
    #[serde(default = "default_score_multiplier")]
    pub score_multiplier: u32,
    #[serde(default = "default_reveals")]
    pub reveals: u8,
    #[serde(default)]
    pub word_list: Vec<String>,
    #[serde(skip)]
    history: Vec<Snapshot>,
}

impl Default for Hangman {
    fn default() -> Self {
        Self::new(0x0BAD_5EED)
    }
}

impl Hangman {
    pub fn new(seed: u64) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_options_config_and_balance(
            seed,
            HangmanCategory::Cabinet,
            HangmanRule::Classic,
            &content.words.hangman,
            &content.balance.word_games,
        )
    }

    pub fn new_with_options(seed: u64, category: HangmanCategory, rule: HangmanRule) -> Self {
        let content = crate::data::GameData::default_content();
        Self::new_with_options_config_and_balance(
            seed,
            category,
            rule,
            &content.words.hangman,
            &content.balance.word_games,
        )
    }

    pub fn new_with_options_config_and_balance(
        seed: u64,
        category: HangmanCategory,
        rule: HangmanRule,
        word_lists: &std::collections::BTreeMap<String, Vec<String>>,
        balance: &crate::content::WordGameBalance,
    ) -> Self {
        let words = word_lists
            .get(category.key())
            .expect("validated content has every Hangman category");
        let word = words[(seed as usize) % words.len()].clone();
        Self {
            word,
            guessed: [false; 26],
            wrong: [false; 26],
            wrong_count: 0,
            moves: 0,
            status: HangmanStatus::Playing,
            seed,
            category,
            rule,
            score: 0,
            combo: 0,
            best_combo: 0,
            max_wrong: match rule {
                HangmanRule::Classic => balance.hangman_classic_wrong,
                HangmanRule::Rapid => balance.hangman_rapid_wrong,
            },
            score_multiplier: match rule {
                HangmanRule::Classic => 1,
                HangmanRule::Rapid => balance.hangman_rapid_multiplier,
            },
            reveals: default_reveals(),
            word_list: words.clone(),
            history: Vec::new(),
        }
    }

    pub fn guess(&mut self, letter: u8) -> bool {
        if letter >= 26 || self.status != HangmanStatus::Playing || self.guessed[letter as usize] {
            return false;
        }
        self.snapshot();
        self.apply_letter(letter, true);
        true
    }

    pub fn reveal(&mut self) -> bool {
        if self.status != HangmanStatus::Playing || self.reveals == 0 {
            return false;
        }
        let Some(letter) = self
            .word
            .bytes()
            .map(|letter| letter - b'A')
            .find(|&letter| !self.guessed[letter as usize])
        else {
            return false;
        };
        self.snapshot();
        self.reveals -= 1;
        self.combo = 0;
        self.apply_letter(letter, false);
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some((guessed, wrong, wrong_count, moves, status, score, combo, best_combo, reveals)) =
            self.history.pop()
        else {
            return false;
        };
        self.guessed = guessed;
        self.wrong = wrong;
        self.wrong_count = wrong_count;
        self.moves = moves;
        self.status = status;
        self.score = score;
        self.combo = combo;
        self.best_combo = best_combo;
        self.reveals = reveals;
        true
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::with_words(
            seed,
            self.category,
            self.rule,
            &self.word_list,
            self.max_wrong,
            self.score_multiplier,
        );
    }

    pub fn set_category(&mut self, category: HangmanCategory, seed: u64) {
        *self = Self::with_words(
            seed,
            category,
            self.rule,
            &self.word_list,
            self.max_wrong,
            self.score_multiplier,
        );
    }

    pub fn set_category_with_config_and_balance(
        &mut self,
        category: HangmanCategory,
        seed: u64,
        word_lists: &std::collections::BTreeMap<String, Vec<String>>,
        balance: &crate::content::WordGameBalance,
    ) {
        *self = Self::new_with_options_config_and_balance(
            seed, category, self.rule, word_lists, balance,
        );
    }

    pub fn set_rule_with_balance(
        &mut self,
        rule: HangmanRule,
        seed: u64,
        balance: &crate::content::WordGameBalance,
    ) {
        *self = Self::new_with_options_config_and_balance(
            seed,
            self.category,
            rule,
            &std::collections::BTreeMap::from([(
                self.category.key().to_owned(),
                self.word_list.clone(),
            )]),
            balance,
        );
    }

    pub fn hint_letter(&self) -> Option<u8> {
        if self.status != HangmanStatus::Playing {
            return None;
        }
        let mut frequency = [0usize; 26];
        for word in self
            .word_list
            .iter()
            .filter(|word| self.matches_candidate(word))
        {
            let mut counted = [false; 26];
            for letter in word.bytes() {
                let index = (letter - b'A') as usize;
                if !self.guessed[index] && !counted[index] {
                    frequency[index] += 1;
                    counted[index] = true;
                }
            }
        }
        frequency
            .iter()
            .enumerate()
            .max_by_key(|(letter, count)| (**count, std::cmp::Reverse(*letter)))
            .filter(|(_, count)| **count > 0)
            .map(|(letter, _)| letter as u8)
    }

    pub fn candidate_count(&self) -> usize {
        self.word_list
            .iter()
            .filter(|word| self.matches_candidate(word))
            .count()
    }

    pub fn is_revealed(&self, character: u8) -> bool {
        character < 26 && self.guessed[character as usize]
    }

    fn apply_letter(&mut self, letter: u8, scores: bool) {
        self.guessed[letter as usize] = true;
        self.moves = self.moves.saturating_add(1);
        let character = b'A' + letter;
        let matches = self
            .word
            .bytes()
            .filter(|&candidate| candidate == character)
            .count() as u32;
        if matches > 0 {
            if scores {
                self.combo = self.combo.saturating_add(1);
                self.best_combo = self.best_combo.max(self.combo);
                self.score = self
                    .score
                    .saturating_add(matches * 10 * u32::from(self.combo) * self.score_multiplier);
            }
            if self
                .word
                .bytes()
                .all(|candidate| self.guessed[(candidate - b'A') as usize])
            {
                self.status = HangmanStatus::Won;
                self.score = self.score.saturating_add(
                    u32::from(self.max_wrong - self.wrong_count) * 25 * self.score_multiplier,
                );
            }
        } else {
            self.combo = 0;
            self.wrong[letter as usize] = true;
            self.wrong_count = self.wrong_count.saturating_add(1);
            if self.wrong_count >= self.max_wrong {
                self.status = HangmanStatus::Lost;
            }
        }
    }

    fn matches_candidate(&self, candidate: &str) -> bool {
        candidate.len() == self.word.len()
            && self
                .word
                .bytes()
                .zip(candidate.bytes())
                .all(|(answer, proposed)| {
                    let answer_index = (answer - b'A') as usize;
                    let proposed_index = (proposed - b'A') as usize;
                    !self.wrong[proposed_index]
                        && (!self.guessed[answer_index] || proposed == answer)
                        && (!self.guessed[proposed_index] || proposed == answer)
                })
    }

    fn snapshot(&mut self) {
        self.history.push((
            self.guessed,
            self.wrong,
            self.wrong_count,
            self.moves,
            self.status,
            self.score,
            self.combo,
            self.best_combo,
            self.reveals,
        ));
    }

    fn with_words(
        seed: u64,
        category: HangmanCategory,
        rule: HangmanRule,
        words: &[String],
        max_wrong: u8,
        score_multiplier: u32,
    ) -> Self {
        let word = words[(seed as usize) % words.len()].clone();
        Self {
            word,
            guessed: [false; 26],
            wrong: [false; 26],
            wrong_count: 0,
            moves: 0,
            status: HangmanStatus::Playing,
            seed,
            category,
            rule,
            score: 0,
            combo: 0,
            best_combo: 0,
            max_wrong,
            score_multiplier,
            reveals: default_reveals(),
            word_list: words.to_vec(),
            history: Vec::new(),
        }
    }
}

const fn default_reveals() -> u8 {
    1
}

fn default_classic_wrong() -> u8 {
    6
}

fn default_score_multiplier() -> u32 {
    1
}

#[cfg(test)]
#[path = "../tests/legacy/hangman/tests.rs"]
mod tests;
