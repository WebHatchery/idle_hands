//! Profile-only persistence kept separate from the active-game collection save.

use super::{AppState, CollectionRecords, GameId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSave {
    pub version: String,
    pub profile_name: String,
    pub sound: bool,
    pub reduced_motion: bool,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub large_text: bool,
    pub mine_flag_mode: bool,
    pub mine_records: [Option<u32>; 4],
    pub sudoku_note_mode: bool,
    pub records: CollectionRecords,
    #[serde(default)]
    pub achievements: Vec<bool>,
    #[serde(default)]
    pub stamps: u16,
    #[serde(default)]
    pub card_back: u8,
    #[serde(default)]
    pub board_theme: u8,
    #[serde(default)]
    pub sound_set: u8,
    #[serde(default)]
    pub cabinet_decoration: u8,
    #[serde(default)]
    pub tutorial_seen: Vec<bool>,
    #[serde(default)]
    pub favorites: Vec<bool>,
    #[serde(default)]
    pub recent_games: Vec<GameId>,
}

pub(super) fn normalize_tutorial_seen(mut tutorial_seen: Vec<bool>) -> Vec<bool> {
    tutorial_seen.resize(GameId::ALL.len(), false);
    tutorial_seen.truncate(GameId::ALL.len());
    tutorial_seen
}

pub(super) fn normalize_favorites(mut favorites: Vec<bool>) -> Vec<bool> {
    favorites.resize(GameId::ALL.len(), false);
    favorites.truncate(GameId::ALL.len());
    favorites
}

pub(super) fn normalize_recent_games(recent_games: Vec<GameId>) -> Vec<GameId> {
    let mut normalized = Vec::new();
    for game in recent_games {
        if GameId::ALL.contains(&game) && !normalized.contains(&game) {
            normalized.push(game);
        }
        if normalized.len() == 5 {
            break;
        }
    }
    normalized
}

pub(super) fn normalize_achievements(mut achievements: Vec<bool>) -> Vec<bool> {
    achievements.resize(crate::progression::AchievementId::ALL.len(), false);
    achievements.truncate(crate::progression::AchievementId::ALL.len());
    achievements
}

impl ProfileSave {
    pub fn from_state(state: &AppState, version: &str) -> Self {
        Self {
            version: version.to_owned(),
            profile_name: state.profile_name.clone(),
            sound: state.sound,
            reduced_motion: state.reduced_motion,
            high_contrast: state.high_contrast,
            large_text: state.large_text,
            mine_flag_mode: state.mine_flag_mode,
            mine_records: state.mine_records,
            sudoku_note_mode: state.sudoku_note_mode,
            records: state.records.clone(),
            achievements: state.achievements.clone(),
            stamps: state.stamps,
            card_back: state.card_back,
            board_theme: state.board_theme,
            sound_set: state.sound_set,
            cabinet_decoration: state.cabinet_decoration,
            tutorial_seen: state.tutorial_seen.clone(),
            favorites: state.favorites.clone(),
            recent_games: state.recent_games.clone(),
        }
    }

    pub fn apply_to(self, state: &mut AppState) {
        state.profile_name = self.profile_name;
        state.sound = self.sound;
        state.reduced_motion = self.reduced_motion;
        state.high_contrast = self.high_contrast;
        state.large_text = self.large_text;
        state.mine_flag_mode = self.mine_flag_mode;
        state.mine_records = self.mine_records;
        state.sudoku_note_mode = self.sudoku_note_mode;
        state.records = self.records;
        state.achievements = normalize_achievements(self.achievements);
        state.stamps = self.stamps;
        state.card_back =
            crate::cosmetics::CosmeticKind::CardBack.normalize(self.card_back, state.stamps);
        state.board_theme =
            crate::cosmetics::CosmeticKind::BoardTheme.normalize(self.board_theme, state.stamps);
        state.sound_set =
            crate::cosmetics::CosmeticKind::SoundSet.normalize(self.sound_set, state.stamps);
        state.cabinet_decoration = crate::cosmetics::CosmeticKind::CabinetDecoration
            .normalize(self.cabinet_decoration, state.stamps);
        state.tutorial_seen = normalize_tutorial_seen(self.tutorial_seen);
        state.favorites = normalize_favorites(self.favorites);
        state.recent_games = normalize_recent_games(self.recent_games);
    }
}
