//! Collection, profile, and per-game autosave persistence.

use super::Game;
use crate::state::{CollectionSave, GameId, GameSnapshot, ProfileSave, Screen};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};
use serde::de::DeserializeOwned;

impl Game {
    pub(super) fn save_autosave(&mut self) {
        let save = CollectionSave::from_state(&self.state, &self.data.config.version);
        if let Err(error) = save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            self.notifications
                .warning(format!("Autosave failed: {}", error));
        }
        let profile_slot = format!("{}_profile", self.data.config.save_slot);
        if let Err(error) = save_to_slot_with_version(
            &self.data.config.game_name,
            &profile_slot,
            &ProfileSave::from_state(&self.state, &self.data.config.version),
            &self.data.config.version,
        ) {
            self.notifications
                .warning(format!("Profile save failed: {}", error));
        }
        if let Screen::Game(game) = self.state.screen {
            let game_slot = format!("{}_{}", self.data.config.save_slot, game.save_key());
            if let Err(error) = save_to_slot_with_version(
                &self.data.config.game_name,
                &game_slot,
                &GameSnapshot::from_state(&self.state, game),
                &self.data.config.version,
            ) {
                self.notifications
                    .warning(format!("{} save failed: {}", game.title(), error));
            }
        }
    }

    pub(super) fn load_autosave(&mut self) {
        let mut restored = false;
        let collection_slot = self.data.config.save_slot.clone();
        if slot_exists(&self.data.config.game_name, &collection_slot) {
            match self.load_slot::<CollectionSave>(&collection_slot) {
                Ok(save) => {
                    save.apply_to(&mut self.state);
                    restored = true;
                }
                Err(error) => self
                    .notifications
                    .warning(format!("Autosave could not be loaded: {}", error)),
            }
        }
        let profile_slot = format!("{}_profile", collection_slot);
        if slot_exists(&self.data.config.game_name, &profile_slot) {
            if let Ok(profile) = self.load_slot::<ProfileSave>(&profile_slot) {
                profile.apply_to(&mut self.state);
                restored = true;
            }
        }
        for game in GameId::ALL {
            let game_slot = format!("{}_{}", collection_slot, game.save_key());
            if slot_exists(&self.data.config.game_name, &game_slot) {
                if let Ok(snapshot) = self.load_slot::<GameSnapshot>(&game_slot) {
                    snapshot.apply_to(&mut self.state);
                    restored = true;
                }
            }
        }
        if restored {
            self.notifications.info("Restored the cabinet autosave");
        }
    }

    fn load_slot<T: DeserializeOwned>(&self, slot: &str) -> Result<T, String> {
        load_from_slot_with_migration(
            &self.data.config.game_name,
            slot,
            &self.data.config.version,
            |_, value| {
                let payload = value.get("data").cloned().unwrap_or(value);
                serde_json::from_value(payload)
                    .map_err(|error| format!("Unsupported save: {}", error))
            },
        )
    }
}
