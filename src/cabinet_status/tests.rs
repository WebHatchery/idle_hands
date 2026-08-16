use super::*;

#[test]
fn cabinet_filters_keep_new_drawers_open_and_completed_drawers_done() {
    let mut state = AppState::default();
    assert!(matches_filter(&state, GameId::Game2048, 0));
    assert!(matches_filter(&state, GameId::Game2048, 1));
    assert!(!matches_filter(&state, GameId::Game2048, 2));

    state.records.best_2048 = 2048;
    assert!(matches_filter(&state, GameId::Game2048, 0));
    assert!(!matches_filter(&state, GameId::Game2048, 1));
    assert!(matches_filter(&state, GameId::Game2048, 2));
}

#[test]
fn cabinet_filter_clamps_unknown_values_to_done() {
    let state = AppState::default();
    assert!(!matches_filter(&state, GameId::Solitaire, 99));
}

#[test]
fn filtered_drawers_keep_their_canonical_collection_numbers() {
    assert_eq!(drawer_number(GameId::Solitaire), 1);
    assert_eq!(drawer_number(GameId::FreeCell), 2);
    assert_eq!(drawer_number(GameId::WordLadder), GameId::ALL.len());
}
