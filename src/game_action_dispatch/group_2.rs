use super::*;

impl Game {
    pub(super) fn apply_shell_group_2(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::RulesFilter(filter) => {
                self.state.rules_filter = crate::rules_data::normalize_filter(filter);
                self.state.library_scroll = 0;
            }
            ui::UiAction::Cabinet => {
                self.state.screen = Screen::Cabinet;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
                self.state.tutorial = None;
                self.state.confirm_reset = false;
                self.state.confirm_restart = false;
                self.state.pending_restart = None;
            }
            ui::UiAction::Help => {
                self.state.screen = Screen::Help;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Records => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.records_filter = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::DailyArchive => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.daily_archive_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = true;
                self.state.achievements_view = false;
            }
            ui::UiAction::Achievements => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = true;
                self.state.achievement_filter = 0;
            }
            ui::UiAction::AchievementFilter(filter) => {
                self.set_achievement_filter(filter);
            }
            ui::UiAction::Rules => {
                self.state.screen = Screen::Rules;
                self.state.library_scroll = 0;
                self.state.rules_filter = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Credits => {
                self.state.screen = Screen::Credits;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Settings => {
                self.state.screen = Screen::Settings;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::TutorialContinue => {
                if let Some(game) = self.state.tutorial {
                    self.state.tutorial_seen[game.index()] = true;
                    self.state.tutorial = None;
                }
            }
            ui::UiAction::ReplayTutorial => {
                if let Screen::Game(game) = self.state.screen {
                    self.state.tutorial = Some(game);
                }
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
