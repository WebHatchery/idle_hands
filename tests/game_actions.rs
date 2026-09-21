//! Shell-action classification regressions.

use idle_hands::testing::{modules::game_actions::is_shell, Direction, GameId, UiAction};

#[test]
fn shell_routing_is_explicit_at_the_boundary() {
    for action in [
        UiAction::Cabinet,
        UiAction::Inspect(GameId::Solitaire.index()),
        UiAction::Profile,
        UiAction::CabinetSort,
        UiAction::RulesFilter(4),
        UiAction::Save,
        UiAction::CycleSoundVolume,
        UiAction::ResumeLifecycle,
    ] {
        assert!(is_shell(action), "{action:?} should be shell-owned");
    }
    for action in [UiAction::Game2048Hint, UiAction::Move(Direction::Left)] {
        assert!(!is_shell(action), "{action:?} should be game-owned");
    }
}
