//! Typed authored content shared by the cabinet shell and game builders.

use crate::state::GameId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const CONTENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameContent {
    pub schema_version: u32,
    pub games: Vec<GameEntry>,
    pub tutorials: BTreeMap<String, [String; 3]>,
    pub labels: Labels,
    pub variants: BTreeMap<String, Vec<VariantEntry>>,
    pub words: WordLists,
    pub achievements: Vec<AchievementEntry>,
    pub hints: BTreeMap<String, HintCopy>,
    pub balance: Balance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEntry {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub save_key: String,
    pub category: String,
    pub active: bool,
    pub completion: String,
    pub has_variants: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Labels {
    pub help_paragraphs: Vec<String>,
    pub help_navigation: Vec<String>,
    pub credits_title: String,
    pub credits_paragraphs: Vec<String>,
    pub profile_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantEntry {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordLists {
    pub hangman: BTreeMap<String, Vec<String>>,
    pub word_grid: Vec<String>,
    pub word_ladder: WordLadderWords,
    pub word_search: Vec<WordSearchTheme>,
    pub misc: Vec<WordClue>,
    pub riddles: Vec<RiddleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordLadderWords {
    pub dictionary: Vec<String>,
    pub puzzles: Vec<WordLadderPuzzle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordLadderPuzzle {
    pub start: String,
    pub target: String,
    pub waypoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordSearchTheme {
    pub id: String,
    pub label: String,
    pub words: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordClue {
    pub word: String,
    pub clue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiddleEntry {
    pub question: String,
    pub clue: String,
    pub answers: [String; 4],
    pub answer: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub stamp_value: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HintCopy {
    pub fallback: String,
    pub complete: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub arcade: ArcadeBalance,
    pub word_games: WordGameBalance,
    pub misc_games: MiscGameBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArcadeBalance {
    pub snake_target_score: u16,
    pub breakout_target_level: u8,
    pub space_invaders_target_wave: u8,
    pub asteroids_target_score: u16,
    pub frogger_target_crossings: u8,
    pub block_stack_target_lines: u16,
    pub paddle_duel_win_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WordGameBalance {
    pub word_length: usize,
    pub word_grid_max_guesses: usize,
    pub hangman_classic_wrong: u8,
    pub hangman_rapid_wrong: u8,
    pub hangman_rapid_multiplier: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiscGameBalance {
    pub riddle_rounds: u32,
    pub pattern_rounds: u32,
    pub sum_rounds: u32,
    pub orbit_size: usize,
    pub word_forge_rounds: u32,
}

impl GameContent {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != CONTENT_SCHEMA_VERSION {
            return Err(format!(
                "content_config.schema_version must be {}",
                CONTENT_SCHEMA_VERSION
            ));
        }
        validate_games(&self.games)?;
        validate_tutorials(&self.tutorials, &self.games)?;
        validate_labels(&self.labels)?;
        validate_variants(&self.variants, &self.games)?;
        validate_words(&self.words)?;
        validate_achievements(&self.achievements)?;
        validate_hints(&self.hints, &self.games)?;
        validate_balance(&self.balance)
    }

    pub fn game(&self, game: GameId) -> Option<&GameEntry> {
        self.games.iter().find(|entry| entry.id == game.key())
    }

    pub fn tutorial(&self, game: GameId) -> Option<&[String; 3]> {
        self.tutorials.get(game.key())
    }

    pub fn variants_for(&self, game: GameId) -> &[VariantEntry] {
        self.variants
            .get(game.key())
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

pub fn validate_games(games: &[GameEntry]) -> Result<(), String> {
    if games.len() != GameId::ALL.len() {
        return Err(format!(
            "content_config.games must contain {} entries",
            GameId::ALL.len()
        ));
    }
    let mut ids = Vec::new();
    let mut save_keys = Vec::new();
    for entry in games {
        if !GameId::ALL.iter().any(|game| game.key() == entry.id) {
            return Err(format!("content_config.games has unknown id {}", entry.id));
        }
        if !ids.iter().any(|id| id == &entry.id) {
            ids.push(entry.id.clone());
        } else {
            return Err(format!("content_config.games duplicates {}", entry.id));
        }
        if entry.title.trim().is_empty() || entry.subtitle.trim().is_empty() {
            return Err(format!("content_config.games {} has empty copy", entry.id));
        }
        if entry.save_key.trim().is_empty() || save_keys.contains(&entry.save_key) {
            return Err(format!(
                "content_config.games {} has invalid save_key",
                entry.id
            ));
        }
        save_keys.push(entry.save_key.clone());
        if !matches!(
            entry.category.as_str(),
            "cards" | "logic" | "board" | "word" | "arcade" | "misc"
        ) {
            return Err(format!(
                "content_config.games {} has invalid category",
                entry.id
            ));
        }
        if !matches!(entry.completion.as_str(), "record" | "runtime_only") {
            return Err(format!(
                "content_config.games {} has invalid completion",
                entry.id
            ));
        }
    }
    for game in GameId::ALL {
        if !ids.iter().any(|id| id == game.key()) {
            return Err(format!("content_config.games is missing {}", game.key()));
        }
    }
    Ok(())
}

pub fn validate_tutorials(
    tutorials: &BTreeMap<String, [String; 3]>,
    games: &[GameEntry],
) -> Result<(), String> {
    if tutorials.len() != games.len() {
        return Err("content_config.tutorials must cover every game".into());
    }
    for game in games {
        let Some(instructions) = tutorials.get(&game.id) else {
            return Err(format!("content_config.tutorials is missing {}", game.id));
        };
        if instructions
            .iter()
            .any(|instruction| instruction.trim().is_empty())
        {
            return Err(format!(
                "content_config.tutorials {} has empty copy",
                game.id
            ));
        }
    }
    Ok(())
}

pub fn validate_labels(labels: &Labels) -> Result<(), String> {
    if labels.help_paragraphs.len() != 4 || labels.help_navigation.len() != 4 {
        return Err("content_config.labels has an invalid help shape".into());
    }
    if labels.credits_title.trim().is_empty()
        || labels.credits_paragraphs.is_empty()
        || labels.profile_names.len() != 8
        || labels
            .help_paragraphs
            .iter()
            .chain(labels.help_navigation.iter())
            .chain(labels.credits_paragraphs.iter())
            .any(|text| text.trim().is_empty())
    {
        return Err("content_config.labels has empty copy".into());
    }
    Ok(())
}

pub fn validate_variants(
    variants: &BTreeMap<String, Vec<VariantEntry>>,
    games: &[GameEntry],
) -> Result<(), String> {
    if variants.len() != games.len() {
        return Err("content_config.variants must cover every game".into());
    }
    for game in games {
        let Some(entries) = variants.get(&game.id) else {
            return Err(format!("content_config.variants is missing {}", game.id));
        };
        if game.has_variants && entries.len() < 2 {
            return Err(format!(
                "content_config.variants {} needs two entries",
                game.id
            ));
        }
        if entries.is_empty()
            || entries
                .iter()
                .any(|entry| entry.id.trim().is_empty() || entry.label.trim().is_empty())
            || entries
                .iter()
                .enumerate()
                .any(|(index, entry)| entries[..index].iter().any(|other| other.id == entry.id))
        {
            return Err(format!(
                "content_config.variants {} has invalid entries",
                game.id
            ));
        }
    }
    Ok(())
}

pub fn validate_words(words: &WordLists) -> Result<(), String> {
    for (category, list) in &words.hangman {
        if list.len() < 3 || list.iter().any(|word| !is_upper_word(word, 4, 10)) {
            return Err(format!(
                "content_config.words.hangman.{category} is invalid"
            ));
        }
    }
    if words.hangman.len() != 3
        || words.word_grid.len() < 8
        || words
            .word_grid
            .iter()
            .any(|word| !is_upper_word(word, 5, 5))
        || words.word_ladder.dictionary.len() < 8
        || words
            .word_ladder
            .dictionary
            .iter()
            .any(|word| !is_upper_word(word, 5, 5))
        || words.word_ladder.puzzles.len() != 3
        || words.misc.len() < 5
        || words
            .misc
            .iter()
            .any(|entry| !is_upper_word(&entry.word, 4, 10) || entry.clue.trim().is_empty())
        || words.riddles.len() != 5
        || words.riddles.iter().any(|riddle| {
            riddle.question.trim().is_empty()
                || riddle.clue.trim().is_empty()
                || riddle.answer >= riddle.answers.len()
                || riddle.answers.iter().any(|answer| answer.trim().is_empty())
        })
    {
        return Err("content_config.words has invalid word data".into());
    }
    for puzzle in &words.word_ladder.puzzles {
        if [&puzzle.start, &puzzle.target, &puzzle.waypoint]
            .iter()
            .any(|word| !words.word_ladder.dictionary.contains(word) || word.chars().count() != 5)
        {
            return Err("content_config.words.word_ladder has an unknown puzzle word".into());
        }
    }
    if words.word_search.len() != 3
        || words.word_search.iter().any(|theme| {
            theme.words.len() != 6
                || theme.id.trim().is_empty()
                || theme.label.trim().is_empty()
                || theme.words.iter().any(|word| !is_upper_word(word, 3, 7))
        })
    {
        return Err("content_config.words.word_search has invalid themes".into());
    }
    Ok(())
}

pub fn validate_achievements(achievements: &[AchievementEntry]) -> Result<(), String> {
    if achievements.len() != GameId::ALL.len() + 2 {
        return Err("content_config.achievements must contain 62 entries".into());
    }
    if achievements.iter().any(|entry| {
        entry.id.trim().is_empty()
            || entry.title.trim().is_empty()
            || entry.description.trim().is_empty()
            || entry.stamp_value == 0
    }) {
        return Err("content_config.achievements has invalid copy".into());
    }
    let expected_ids = std::iter::once("first_finish".to_owned())
        .chain(
            GameId::ALL
                .into_iter()
                .map(|game| format!("game:{}", game.key())),
        )
        .chain(std::iter::once("full_cabinet".to_owned()))
        .collect::<Vec<_>>();
    if achievements
        .iter()
        .zip(expected_ids.iter())
        .any(|(entry, expected)| entry.id != *expected)
    {
        return Err("content_config.achievements must follow the cabinet order".into());
    }
    Ok(())
}

pub fn validate_hints(
    hints: &BTreeMap<String, HintCopy>,
    games: &[GameEntry],
) -> Result<(), String> {
    if hints.len() != games.len()
        || games.iter().any(|game| {
            hints.get(&game.id).is_none_or(|hint| {
                hint.fallback.trim().is_empty() || hint.complete.trim().is_empty()
            })
        })
    {
        return Err("content_config.hints must cover every game with copy".into());
    }
    Ok(())
}

pub fn validate_balance(balance: &Balance) -> Result<(), String> {
    let positive = [
        u32::from(balance.arcade.snake_target_score),
        u32::from(balance.arcade.breakout_target_level),
        u32::from(balance.arcade.space_invaders_target_wave),
        u32::from(balance.arcade.asteroids_target_score),
        u32::from(balance.arcade.frogger_target_crossings),
        u32::from(balance.arcade.block_stack_target_lines),
        u32::from(balance.arcade.paddle_duel_win_score),
        balance.word_games.word_length as u32,
        balance.word_games.word_grid_max_guesses as u32,
        u32::from(balance.word_games.hangman_classic_wrong),
        u32::from(balance.word_games.hangman_rapid_wrong),
        balance.word_games.hangman_rapid_multiplier,
        balance.misc_games.riddle_rounds,
        balance.misc_games.pattern_rounds,
        balance.misc_games.sum_rounds,
        balance.misc_games.orbit_size as u32,
        balance.misc_games.word_forge_rounds,
    ];
    if positive.contains(&0) {
        return Err("content_config.balance values must be positive".into());
    }
    if balance.word_games.hangman_rapid_wrong >= balance.word_games.hangman_classic_wrong
        || balance.word_games.word_length != 5
        || balance.misc_games.orbit_size != 8
    {
        return Err("content_config.balance violates game invariants".into());
    }
    Ok(())
}

pub fn is_upper_word(word: &str, min: usize, max: usize) -> bool {
    let length = word.chars().count();
    length >= min && length <= max && word.chars().all(|character| character.is_ascii_uppercase())
}
