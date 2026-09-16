//! Regression coverage for the tests module.

use super::*;

#[test]
fn continue_prefers_the_selected_drawer_when_it_is_playable() {
    let state = AppState {
        selected: GameId::FreeCell.index(),
        recent_games: vec![GameId::Solitaire],
        ..AppState::default()
    };

    assert_eq!(preferred_game_for_build(&state, true), GameId::FreeCell);
}

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
fn continue_accepts_the_selected_drawer_in_the_full_build() {
    let state = AppState {
        selected: GameId::Blackjack.index(),
        ..AppState::default()
    };

    assert_eq!(preferred_game_for_build(&state, false), GameId::Blackjack);
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
fn continue_copy_explains_fresh_active_and_completed_drawers() {
    let mut state = AppState::default();
    assert_eq!(intent(&state), ContinueIntent::Start);
    assert_eq!(title(&state), "START PLAYING");
    assert_eq!(action_label(&state), "START  >");
    assert_eq!(compact_action_label(&state), "START");

    state.games.game.score = 8;
    assert_eq!(intent(&state), ContinueIntent::Resume);
    assert_eq!(title(&state), "CONTINUE PLAYING");
    assert_eq!(action_label(&state), "CONTINUE  >");

    state.records.best_2048 = 2048;
    state.records.record_time(GameId::Game2048.index());
    assert_eq!(intent(&state), ContinueIntent::Replay);
    assert_eq!(title(&state), "PLAY AGAIN");
    assert_eq!(compact_action_label(&state), "REPLAY");
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
