//! Collection index, profile, and per-game autosave persistence.

use super::Game;
use crate::persistence_models::CollectionIndex;
use crate::state::{CollectionSave, GameId, GameSnapshot, ProfileSave, Screen};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, quarantine_slot, save_to_slot_with_version, slot_exists,
};
use serde::de::DeserializeOwned;

const AUTOSAVE_DELAY_SECONDS: f32 = 0.75;

impl Game {
    pub(super) fn dismiss_save_recovery(&mut self) {
        if self.state.save_recovery.take().is_some() {
            self.request_autosave();
        }
    }

    pub(super) fn toggle_notice_log(&mut self) {
        self.state.notice_log_view = !self.state.notice_log_view;
    }

    /// Mark the authoritative records dirty. The next few actions are written
    /// together instead of rewriting every save slot after every tap.
    pub(super) fn request_autosave(&mut self) {
        self.save_dirty = true;
        self.save_timer = AUTOSAVE_DELAY_SECONDS;
    }

    pub(super) fn tick_autosave(&mut self, dt: f32) {
        if !self.save_dirty {
            return;
        }
        self.save_timer -= dt.max(0.0);
        if self.save_timer <= 0.0 {
            self.flush_autosave();
        }
    }

    /// Flush the collection index, profile, and currently active game record.
    /// Each record has one owner, so a failed game save cannot silently erase
    /// an otherwise valid profile or collection index.
    pub(super) fn flush_autosave(&mut self) {
        let version = self.data.config.version.clone();
        let game_name = self.data.config.game_name.clone();
        let collection_slot = self.data.config.save_slot.clone();
        let mut succeeded = true;

        let index = CollectionIndex::from_state(&self.state, &version);
        if let Err(error) =
            save_to_slot_with_version(&game_name, &collection_slot, &index, &version)
        {
            succeeded = false;
            self.notifications
                .warning(format!("Collection index save failed: {}", error));
        }

        let profile_slot = format!("{}_profile", collection_slot);
        if let Err(error) = save_to_slot_with_version(
            &game_name,
            &profile_slot,
            &ProfileSave::from_state(&self.state, &version),
            &version,
        ) {
            succeeded = false;
            self.notifications
                .warning(format!("Profile save failed: {}", error));
        }

        if let Screen::Game(game) = self.state.screen {
            let game_slot = format!("{}_{}", collection_slot, self.state.game_save_key(game));
            if let Err(error) = save_to_slot_with_version(
                &game_name,
                &game_slot,
                &GameSnapshot::from_state(&self.state, game),
                &version,
            ) {
                succeeded = false;
                self.notifications.warning(format!(
                    "{} save failed: {}",
                    self.state.game_title(game),
                    error
                ));
            }
        }

        if succeeded {
            self.save_dirty = false;
            self.save_timer = 0.0;
        } else {
            self.save_dirty = true;
            self.save_timer = AUTOSAVE_DELAY_SECONDS * 2.0;
        }
    }

    pub(super) fn load_autosave(&mut self) {
        self.save_dirty = false;
        self.save_timer = 0.0;
        self.state.save_recovery = None;
        self.state.notice_log_view = false;
        let mut restored = false;
        let collection_slot = self.data.config.save_slot.clone();
        if slot_exists(&self.data.config.game_name, &collection_slot) {
            match self.load_slot::<CollectionIndex>(&collection_slot) {
                Ok(index) => match index.validate() {
                    Ok(()) => {
                        index.apply_to(&mut self.state);
                        restored = true;
                    }
                    Err(error) => self.handle_bad_slot("collection index", &collection_slot, error),
                },
                Err(index_error) => match self.load_slot::<CollectionSave>(&collection_slot) {
                    Ok(legacy) => {
                        legacy.apply_to(&mut self.state);
                        restored = true;
                        self.notifications.info(
                            "Migrated the legacy collection save; future saves use separate records",
                        );
                    }
                    Err(legacy_error) => self.handle_bad_slot(
                        "collection index",
                        &collection_slot,
                        format!("{} (legacy save: {})", index_error, legacy_error),
                    ),
                },
            }
        }

        let profile_slot = format!("{}_profile", collection_slot);
        if slot_exists(&self.data.config.game_name, &profile_slot) {
            match self.load_slot::<ProfileSave>(&profile_slot) {
                Ok(profile) => {
                    profile.apply_to(&mut self.state);
                    restored = true;
                }
                Err(error) => self.handle_bad_slot("profile", &profile_slot, error),
            }
        }

        for game in GameId::ALL {
            let game_slot = format!("{}_{}", collection_slot, self.state.game_save_key(game));
            if !slot_exists(&self.data.config.game_name, &game_slot) {
                continue;
            }
            match self.load_slot::<GameSnapshot>(&game_slot) {
                Ok(snapshot) => {
                    snapshot.apply_to(&mut self.state);
                    restored = true;
                }
                Err(error) => {
                    let game_title = self.state.game_title(game).to_owned();
                    self.handle_bad_slot(&game_title, &game_slot, error)
                }
            }
        }
        if crate::continue_data::repair_selected(&mut self.state) {
            self.request_autosave();
        }
        if restored {
            self.notifications.info("Restored the cabinet autosave");
        }
    }

    fn handle_bad_slot(&mut self, label: &str, slot: &str, error: String) {
        self.notifications
            .warning(format!("{} save could not be loaded: {}", label, error));
        let quarantined = match quarantine_slot(&self.data.config.game_name, slot) {
            Ok(quarantine) => {
                self.notifications.warning(format!(
                    "Preserved the damaged {} save as {}",
                    label, quarantine
                ));
                true
            }
            Err(quarantine_error) => {
                self.notifications.warning(format!(
                    "Could not quarantine the damaged {} save: {}",
                    label, quarantine_error
                ));
                false
            }
        };
        self.state
            .save_recovery
            .get_or_insert_with(crate::save_recovery::SaveRecoveryNotice::new)
            .record(quarantined);
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
