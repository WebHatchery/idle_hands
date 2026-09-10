use super::*;

#[test]
fn record_filters_cycle_through_the_six_cabinet_categories() {
    assert_eq!(FILTERS, [0, 3, 4, 5, 6, 7, 8]);
    assert_eq!(next_filter(0), 3);
    assert_eq!(next_filter(8), 0);
    assert_eq!(filter_label(5), "Board");
}

#[test]
fn filtered_rows_keep_their_category_and_record_values() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);

    let cards = rows(&state, 3);
    assert!(cards.iter().all(|row| row.category == 3));
    assert!(cards
        .iter()
        .any(|row| row.game == GameId::Solitaire && row.score == "42"));
    assert!(rows(&state, 4).iter().all(|row| row.category == 4));
}

#[test]
fn shelf_summary_counts_unique_games_and_variant_rows() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);

    let cards = summary(&state, 3);
    assert_eq!(cards.games, 11);
    assert_eq!(cards.rows, 11);
    assert_eq!(cards.completed, 1);
    assert_eq!(summary_label(&state, 3), "11G / 11R  ·  1 DONE");
}
