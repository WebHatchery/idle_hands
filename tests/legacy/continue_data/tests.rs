//! Regression coverage for the tests module.

use idle_hands::testing::modules::continue_data::*;

#[test]
fn continue_falls_back_to_recent_playable_drawer_when_selection_is_locked() {
    let state = AppState {
        selected: GameId::Blackjack.index(),
        recent_games: vec![GameId::Solitaire, GameId::FreeCell],
        ..AppState::default()
    };

    assert_eq!(preferred_game_for_build(&state, true), GameId::Solitaire);
}

#[test]
fn continue_falls_back_to_the_first_demo_drawer_without_history() {
    let state = AppState {
        selected: GameId::Blackjack.index(),
        ..AppState::default()
    };

    assert_eq!(preferred_game_for_build(&state, true), GameId::Solitaire);
}

#[test]
fn invalid_selection_still_resolves_to_a_real_drawer() {
    let state = AppState {
        selected: usize::MAX,
        ..AppState::default()
    };

    assert!(GameId::ALL.contains(&preferred_game_for_build(&state, false)));
}

#[test]
fn repair_selected_replaces_a_locked_saved_target_but_leaves_playable_targets_alone() {
    let mut locked = AppState {
        selected: GameId::Blackjack.index(),
        recent_games: vec![GameId::Solitaire],
        ..AppState::default()
    };
    assert!(repair_selected_for_build(&mut locked, true));
    assert_eq!(locked.selected, GameId::Solitaire.index());

    let mut playable = AppState {
        selected: GameId::FreeCell.index(),
        ..AppState::default()
    };
    assert!(!repair_selected_for_build(&mut playable, true));
    assert_eq!(playable.selected, GameId::FreeCell.index());
}
