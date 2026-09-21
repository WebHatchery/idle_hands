//! Regression coverage for the tests module.

use idle_hands::testing::modules::tutorial_library_data::*;
use idle_hands::testing::state::GameId;

#[test]
fn rows_follow_the_canonical_game_order_and_seen_flags() {
    let mut state = AppState::default();
    state.tutorial_seen[GameId::Snake.index()] = true;

    let rows = rows(&state);

    assert_eq!(rows.len(), GameId::ALL.len());
    assert_eq!(rows[0].game, GameId::Solitaire);
    assert!(rows[GameId::Snake.index()].seen);
    assert_eq!(seen_count(&state), 1);
}

#[test]
fn new_only_rows_exclude_seen_lessons_without_changing_the_total() {
    let mut state = AppState::default();
    state.tutorial_seen[GameId::Solitaire.index()] = true;
    state.tutorial_filter = true;

    let rows = rows(&state);

    assert_eq!(new_count(&state), GameId::ALL.len() - 1);
    assert_eq!(rows.len(), GameId::ALL.len() - 1);
    assert!(rows.iter().all(|row| !row.seen));
}
