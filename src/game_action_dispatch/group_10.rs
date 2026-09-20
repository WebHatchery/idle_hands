use super::*;

impl Game {
    pub(super) fn apply_shell_group_10(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::ToggleGameSetup => {
                if self.state.screen.is_game() {
                    self.state.game_setup_open = !self.state.game_setup_open;
                }
            }
            ui::UiAction::ConfirmRestart => {
                if let Some(restart) = self.state.pending_restart.take() {
                    self.state.confirm_restart = false;
                    self.confirmation_bypass = true;
                    self.apply(restart);
                    self.confirmation_bypass = false;
                    return ShellActionResult::Stop;
                }
                let board_size = self.state.games.game.board_size;
                self.state.games.game = crate::state::Game2048::new_with_size(
                    self.state.games.game.seed.wrapping_add(1),
                    board_size,
                );
                self.state.confirm_restart = false;
            }
            ui::UiAction::Cancel => {
                self.state.confirm_restart = false;
                self.state.pending_restart = None;
            }
            ui::UiAction::ResumeLifecycle => self.resume_lifecycle(),
            ui::UiAction::DismissSaveRecovery => self.dismiss_save_recovery(),
            ui::UiAction::ToggleNoticeLog => self.toggle_notice_log(),
            ui::UiAction::ToggleSound
            | ui::UiAction::ToggleMotion
            | ui::UiAction::ToggleHighContrast
            | ui::UiAction::ToggleLargeText
            | ui::UiAction::CycleCardBack
            | ui::UiAction::CycleBoardTheme
            | ui::UiAction::CycleSoundSet
            | ui::UiAction::CycleCabinetDecoration
            | ui::UiAction::SetProfileName(_) => self.apply_settings_action(action),
            ui::UiAction::ResetData => self.state.confirm_reset = true,
            ui::UiAction::ConfirmResetData => {
                self.state = AppState::new(&self.data);
            }
            ui::UiAction::CancelResetData => self.state.confirm_reset = false,
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
