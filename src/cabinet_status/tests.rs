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
fn cabinet_filter_unknown_values_show_all_games() {
    let state = AppState::default();
    assert!(matches_filter(&state, GameId::Solitaire, 99));
}

#[test]
fn category_filters_partition_the_whole_collection() {
    let state = AppState::default();
    let total: usize = CATEGORY_FILTERS
        .iter()
        .map(|filter| filter_count(&state, *filter))
        .sum();
    assert_eq!(total, GameId::ALL.len());
    assert!(matches_filter(&state, GameId::Solitaire, 3));
    assert!(matches_filter(&state, GameId::Sudoku, 4));
    assert!(matches_filter(&state, GameId::WordSearch, 6));
    assert_eq!(filter_count(&state, 3), 12);
    assert_eq!(filter_count(&state, 4), 9);
    assert_eq!(filter_count(&state, 5), 8);
    assert_eq!(filter_count(&state, 6), 7);
    assert_eq!(filter_count(&state, 7), 6);
    assert_eq!(filter_count(&state, 8), 5);
}
