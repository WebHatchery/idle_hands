//! Embedded game data and asset manifests.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::content::GameContent;

pub const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
pub const PUZZLE_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/puzzle_config.json");
pub const CONTENT_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/content_config.json");
pub const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/texture_manifest.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleConfig {
    pub color_sort: ColorSortConfig,
    pub dots_boxes: DotsBoxesConfig,
    pub flood_it: FloodItConfig,
    pub match_three: MatchThreeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSortConfig {
    pub capacity: usize,
    pub difficulties: Vec<ColorSortDifficultyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSortDifficultyConfig {
    pub id: String,
    pub colors: u8,
    pub tubes: usize,
    pub scramble_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotsBoxesConfig {
    pub difficulties: Vec<DotsBoxesDifficultyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotsBoxesDifficultyConfig {
    pub id: String,
    pub side: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodItConfig {
    pub difficulties: Vec<FloodItDifficultyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodItDifficultyConfig {
    pub id: String,
    pub side: usize,
    pub colors: u8,
    pub move_limit: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchThreeConfig {
    pub difficulties: Vec<MatchThreeDifficultyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchThreeDifficultyConfig {
    pub id: String,
    pub side: usize,
    pub colors: u8,
    pub target_score: u16,
    pub move_limit: u16,
}

impl Default for ColorSortConfig {
    fn default() -> Self {
        Self {
            capacity: 4,
            difficulties: vec![
                ColorSortDifficultyConfig {
                    id: "standard".into(),
                    colors: 4,
                    tubes: 6,
                    scramble_steps: 52,
                },
                ColorSortDifficultyConfig {
                    id: "hard".into(),
                    colors: 5,
                    tubes: 7,
                    scramble_steps: 78,
                },
                ColorSortDifficultyConfig {
                    id: "expert".into(),
                    colors: 6,
                    tubes: 8,
                    scramble_steps: 108,
                },
            ],
        }
    }
}

impl Default for DotsBoxesConfig {
    fn default() -> Self {
        Self {
            difficulties: vec![
                DotsBoxesDifficultyConfig {
                    id: "standard".into(),
                    side: 4,
                },
                DotsBoxesDifficultyConfig {
                    id: "hard".into(),
                    side: 5,
                },
                DotsBoxesDifficultyConfig {
                    id: "expert".into(),
                    side: 6,
                },
            ],
        }
    }
}

impl Default for FloodItConfig {
    fn default() -> Self {
        Self {
            difficulties: vec![
                FloodItDifficultyConfig {
                    id: "standard".into(),
                    side: 8,
                    colors: 6,
                    move_limit: 24,
                },
                FloodItDifficultyConfig {
                    id: "hard".into(),
                    side: 10,
                    colors: 7,
                    move_limit: 32,
                },
                FloodItDifficultyConfig {
                    id: "expert".into(),
                    side: 12,
                    colors: 8,
                    move_limit: 40,
                },
            ],
        }
    }
}

impl Default for MatchThreeConfig {
    fn default() -> Self {
        Self {
            difficulties: vec![
                MatchThreeDifficultyConfig {
                    id: "standard".into(),
                    side: 7,
                    colors: 5,
                    target_score: 120,
                    move_limit: 18,
                },
                MatchThreeDifficultyConfig {
                    id: "hard".into(),
                    side: 8,
                    colors: 6,
                    target_score: 240,
                    move_limit: 24,
                },
                MatchThreeDifficultyConfig {
                    id: "expert".into(),
                    side: 9,
                    colors: 7,
                    target_score: 360,
                    move_limit: 30,
                },
            ],
        }
    }
}

impl PuzzleConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.color_sort.capacity == 0 {
            return Err("puzzle_config.color_sort.capacity must be positive".into());
        }
        validate_difficulties("color_sort", &self.color_sort.difficulties, |difficulty| {
            difficulty.colors > 0
                && difficulty.tubes >= difficulty.colors as usize
                && difficulty.scramble_steps > 0
        })?;
        validate_difficulties("dots_boxes", &self.dots_boxes.difficulties, |difficulty| {
            difficulty.side > 0
        })?;
        validate_difficulties("flood_it", &self.flood_it.difficulties, |difficulty| {
            difficulty.side > 0 && difficulty.colors > 0 && difficulty.move_limit > 0
        })?;
        validate_difficulties(
            "match_three",
            &self.match_three.difficulties,
            |difficulty| {
                difficulty.side > 0
                    && difficulty.colors > 0
                    && difficulty.target_score > 0
                    && difficulty.move_limit > 0
            },
        )?;
        Ok(())
    }
}

pub fn validate_difficulties<T>(
    name: &str,
    difficulties: &[T],
    valid: impl Fn(&T) -> bool,
) -> Result<(), String> {
    pub const EXPECTED: usize = 3;
    if difficulties.len() != EXPECTED {
        return Err(format!(
            "puzzle_config.{name}.difficulties must contain {EXPECTED} entries"
        ));
    }
    if difficulties.iter().any(|difficulty| !valid(difficulty)) {
        return Err(format!(
            "puzzle_config.{name}.difficulties contains invalid values"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub puzzles: PuzzleConfig,
    pub content: GameContent,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let puzzles: PuzzleConfig =
            load_embedded_json_labeled("puzzle_config", PUZZLE_CONFIG_JSON)?;
        puzzles.validate()?;
        let content: GameContent =
            load_embedded_json_labeled("content_config", CONTENT_CONFIG_JSON)?;
        content.validate()?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        Ok(Self {
            config,
            puzzles,
            content,
            texture_manifest,
        })
    }

    /// Keep the cabinet playable if an embedded data file is malformed in a
    /// development build. The validated defaults are intentionally limited to
    /// the configuration owned by this module; optional textures already have
    /// a visible placeholder in the asset manager.
    pub fn fallback() -> Self {
        Self {
            config: GameConfig {
                game_name: "idle_hands".into(),
                display_name: "Idle Hands".into(),
                save_slot: "autosave".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            puzzles: PuzzleConfig {
                color_sort: ColorSortConfig::default(),
                dots_boxes: DotsBoxesConfig::default(),
                flood_it: FloodItConfig::default(),
                match_three: MatchThreeConfig::default(),
            },
            content: load_embedded_json_labeled("content_config", CONTENT_CONFIG_JSON)
                .and_then(|content: GameContent| {
                    content.validate()?;
                    Ok(content)
                })
                .unwrap_or_else(|error| {
                    eprintln!("Idle Hands content fallback failed: {error}");
                    GameContent {
                        schema_version: crate::content::CONTENT_SCHEMA_VERSION,
                        games: Vec::new(),
                        tutorials: Default::default(),
                        labels: crate::content::Labels {
                            help_paragraphs: Vec::new(),
                            help_navigation: Vec::new(),
                            credits_title: String::new(),
                            credits_paragraphs: Vec::new(),
                            profile_names: Vec::new(),
                        },
                        variants: Default::default(),
                        words: crate::content::WordLists {
                            hangman: Default::default(),
                            word_grid: Vec::new(),
                            word_ladder: crate::content::WordLadderWords {
                                dictionary: Vec::new(),
                                puzzles: Vec::new(),
                            },
                            word_search: Vec::new(),
                            misc: Vec::new(),
                            riddles: Vec::new(),
                        },
                        achievements: Vec::new(),
                        hints: Default::default(),
                        balance: crate::content::Balance {
                            arcade: crate::content::ArcadeBalance {
                                snake_target_score: 1,
                                breakout_target_level: 1,
                                space_invaders_target_wave: 1,
                                asteroids_target_score: 1,
                                frogger_target_crossings: 1,
                                block_stack_target_lines: 1,
                                paddle_duel_win_score: 1,
                            },
                            word_games: crate::content::WordGameBalance {
                                word_length: 5,
                                word_grid_max_guesses: 1,
                                hangman_classic_wrong: 2,
                                hangman_rapid_wrong: 1,
                                hangman_rapid_multiplier: 1,
                            },
                            misc_games: crate::content::MiscGameBalance {
                                riddle_rounds: 1,
                                pattern_rounds: 1,
                                sum_rounds: 1,
                                orbit_size: 8,
                                word_forge_rounds: 1,
                            },
                        },
                    }
                }),
            texture_manifest: Vec::new(),
        }
    }

    pub fn default_content() -> Arc<GameContent> {
        Arc::new(Self::fallback().content)
    }
}
