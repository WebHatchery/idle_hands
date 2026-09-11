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
