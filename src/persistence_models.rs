//! Small, authoritative save records for the collection shell.

use crate::state::{AppState, GameId, Screen};
use serde::{Deserialize, Serialize};

/// The collection index owns shell navigation, not game rules or profile data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionIndex {
    pub version: String,
    pub selected: usize,
    #[serde(default)]
    pub active_game: Option<GameId>,
}

impl CollectionIndex {
    pub fn from_state(state: &AppState, version: &str) -> Self {
        Self {
            version: version.to_owned(),
            selected: state.selected,
            active_game: match state.screen {
                Screen::Game(game) => Some(game),
                _ => None,
            },
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version.trim().is_empty() {
            return Err("collection index has no version".into());
        }
        if self.selected >= GameId::ALL.len() {
            return Err(format!(
                "collection index selected game {} is out of range",
                self.selected
            ));
        }
        if let Some(game) = self.active_game {
            if !GameId::ALL.contains(&game) {
                return Err("collection index references an unknown game".into());
            }
        }
        Ok(())
    }

    pub fn apply_to(self, state: &mut AppState) {
        state.selected = self.selected.min(GameId::ALL.len().saturating_sub(1));
        state.screen = self.active_game.map_or(Screen::Cabinet, Screen::Game);
    }
}

#[cfg(test)]
mod tests;
