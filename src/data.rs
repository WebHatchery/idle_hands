//! Embedded game data and asset manifests.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{
    load_embedded_json, load_embedded_json_labeled, DataRegistry,
};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
const ACTIONS_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/actions.json");
const PUZZLE_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/puzzle_config.json");
const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/texture_manifest.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub starting_points: i64,
    pub starting_energy: f32,
    pub max_energy: f32,
    pub energy_per_second: f32,
    pub world_width: usize,
    pub world_height: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub energy_cost: f32,
    pub points_reward: i64,
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
                },
                MatchThreeDifficultyConfig {
                    id: "hard".into(),
                    side: 8,
                    colors: 6,
                    target_score: 240,
                },
                MatchThreeDifficultyConfig {
                    id: "expert".into(),
                    side: 9,
                    colors: 7,
                    target_score: 360,
                },
            ],
        }
    }
}

impl PuzzleConfig {
    fn validate(&self) -> Result<(), String> {
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
                difficulty.side > 0 && difficulty.colors > 0 && difficulty.target_score > 0
            },
        )?;
        Ok(())
    }
}

fn validate_difficulties<T>(
    name: &str,
    difficulties: &[T],
    valid: impl Fn(&T) -> bool,
) -> Result<(), String> {
    const EXPECTED: usize = 3;
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
    #[allow(dead_code)]
    pub actions: DataRegistry<ActionDef>,
    pub puzzles: PuzzleConfig,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let actions = DataRegistry::from_embedded_json(ACTIONS_JSON, "id")?;
        let puzzles: PuzzleConfig =
            load_embedded_json_labeled("puzzle_config", PUZZLE_CONFIG_JSON)?;
        puzzles.validate()?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        Ok(Self {
            config,
            actions,
            puzzles,
            texture_manifest,
        })
    }
}

#[cfg(test)]
mod tests;
