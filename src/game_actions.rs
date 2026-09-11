//! Routing boundary between cabinet-shell actions and game actions.

use crate::ui::UiAction;

/// Returns whether an action mutates collection-shell state. Game rules are
/// handled by the game action modules and never need to know this list.
pub fn is_shell(action: UiAction) -> bool {
    matches!(
        action,
        UiAction::Open(_)
            | UiAction::ContinueGame
            | UiAction::ToggleFavorite(_)
            | UiAction::ClearRecent
            | UiAction::Cabinet
            | UiAction::Help
            | UiAction::Records
            | UiAction::Statistics
            | UiAction::Tutorials
            | UiAction::OpenTutorial(_)
            | UiAction::Favorites
            | UiAction::Recent
            | UiAction::DailyArchive
            | UiAction::DailyArchiveScroll(_)
            | UiAction::DailyArchiveOpen(_)
            | UiAction::RecordsFilter(_)
            | UiAction::RulesFilter(_)
            | UiAction::CabinetFilter(_)
            | UiAction::CabinetSort
            | UiAction::CabinetScroll(_)
            | UiAction::LibraryScroll(_)
            | UiAction::Achievements
            | UiAction::AchievementFilter(_)
            | UiAction::Rules
            | UiAction::Credits
            | UiAction::ResetData
            | UiAction::ConfirmResetData
            | UiAction::CancelResetData
            | UiAction::TutorialContinue
            | UiAction::ReplayTutorial
            | UiAction::Settings
            | UiAction::Save
            | UiAction::Load
            | UiAction::Restart
            | UiAction::ConfirmRestart
            | UiAction::Cancel
            | UiAction::ToggleSound
            | UiAction::CycleSoundVolume
            | UiAction::ResumeLifecycle
            | UiAction::DismissSaveRecovery
            | UiAction::ToggleNoticeLog
            | UiAction::ToggleMotion
            | UiAction::ToggleHighContrast
            | UiAction::ToggleLargeText
            | UiAction::CycleCardBack
            | UiAction::CycleBoardTheme
            | UiAction::CycleSoundSet
            | UiAction::CycleCabinetDecoration
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_routing_is_explicit_at_the_boundary() {
        assert!(is_shell(UiAction::Cabinet));
        assert!(is_shell(UiAction::CabinetSort));
        assert!(is_shell(UiAction::RulesFilter(4)));
        assert!(is_shell(UiAction::Save));
        assert!(is_shell(UiAction::CycleSoundVolume));
        assert!(is_shell(UiAction::ResumeLifecycle));
        assert!(!is_shell(UiAction::Game2048Hint));
        assert!(!is_shell(UiAction::Move(crate::domain::Direction::Left)));
    }
}
