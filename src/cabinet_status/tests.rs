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

#[test]
fn cabinet_filter_labels_report_live_counts_and_empty_guidance() {
    let mut state = AppState::default();
    assert_eq!(filter_count(&state, 0), GameId::ALL.len());
    assert_eq!(filter_label(&state, 2), "DONE 0");
    assert!(empty_filter_message(2).contains("finish a game"));
    state.records.best_2048 = 2048;
    assert_eq!(filter_label(&state, 2), "DONE 1");
    assert_eq!(filter_count(&state, 1), GameId::ALL.len() - 1);
}
