use super::Game;
use crate::{cosmetics::CosmeticKind, settings_data, ui::UiAction};

impl Game {
    pub(super) fn apply_settings_action(&mut self, action: UiAction) {
        let message = match action {
            UiAction::ToggleSound => {
                self.state.sound = !self.state.sound;
                format!("Sound {}", if self.state.sound { "on" } else { "off" })
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
        let row = settings_data::cosmetic_rows(&self.state)
            .into_iter()
            .find(|row| row.kind == kind)
            .expect("every cosmetic kind has one settings row");
        format!("{}: {} ({value})", kind.label(), row.option.name)
    }
}
