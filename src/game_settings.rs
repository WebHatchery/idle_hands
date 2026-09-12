//! Settings action mutation and feedback handling.

use super::Game;
use crate::{audio_settings, cosmetics::CosmeticKind, settings_data, ui::UiAction};

impl Game {
    pub(super) fn apply_settings_action(&mut self, action: UiAction) {
        let message = match action {
            UiAction::ToggleSound => {
                self.state.sound = !self.state.sound;
                format!(
                    "Sound {}",
                    audio_settings::label(self.state.sound, self.state.sound_level)
                )
            }
            UiAction::CycleSoundVolume => {
                self.state.sound_level = audio_settings::next_level(self.state.sound_level);
                format!(
                    "Sound volume {}",
                    audio_settings::label(self.state.sound, self.state.sound_level)
                )
            }
            UiAction::ToggleMotion => {
                self.state.reduced_motion = !self.state.reduced_motion;
                format!(
                    "Motion {}",
                    if self.state.reduced_motion {
                        "reduced"
                    } else {
                        "full"
                    }
                )
            }
            UiAction::ToggleHighContrast => {
                self.state.high_contrast = !self.state.high_contrast;
                format!(
                    "High contrast {}",
                    if self.state.high_contrast {
                        "on"
                    } else {
                        "off"
                    }
                )
            }
            UiAction::ToggleLargeText => {
                self.state.large_text = !self.state.large_text;
                format!(
                    "Large text {}",
                    if self.state.large_text { "on" } else { "off" }
                )
            }
            UiAction::CycleCardBack => self.cycle_cosmetic(CosmeticKind::CardBack),
            UiAction::CycleBoardTheme => self.cycle_cosmetic(CosmeticKind::BoardTheme),
            UiAction::CycleSoundSet => self.cycle_cosmetic(CosmeticKind::SoundSet),
            UiAction::CycleCabinetDecoration => {
                self.cycle_cosmetic(CosmeticKind::CabinetDecoration)
            }
            UiAction::SetProfileName(index) => self.set_profile_name(index),
            _ => return,
        };
        self.notifications.info(message);
    }

    fn cycle_cosmetic(&mut self, kind: CosmeticKind) -> String {
        let value = match kind {
            CosmeticKind::CardBack => {
                self.state.card_back =
                    crate::cosmetics::next_card_back(self.state.card_back, self.state.stamps);
                self.state.card_back
            }
            CosmeticKind::BoardTheme => {
                self.state.board_theme =
                    crate::cosmetics::next_board_theme(self.state.board_theme, self.state.stamps);
                self.state.board_theme
            }
            CosmeticKind::SoundSet => {
                self.state.sound_set =
                    crate::cosmetics::next_sound_set(self.state.sound_set, self.state.stamps);
                self.state.sound_set
            }
            CosmeticKind::CabinetDecoration => {
                self.state.cabinet_decoration = crate::cosmetics::next_cabinet_decoration(
                    self.state.cabinet_decoration,
                    self.state.stamps,
                );
                self.state.cabinet_decoration
            }
        };
        let Some(row) = settings_data::cosmetic_rows(&self.state)
            .into_iter()
            .find(|row| row.kind == kind)
        else {
            return format!("{}: unavailable", kind.label());
        };
        format!("{}: {} ({value})", kind.label(), row.option.name)
    }

    fn set_profile_name(&mut self, index: u8) -> String {
        let name = crate::profile_data::name(index).to_owned();
        self.state.profile_name = name.clone();
        self.request_autosave();
        format!("Profile name: {name}")
    }
}
