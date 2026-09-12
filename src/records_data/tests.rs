//! Regression coverage for the tests module.

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

#[test]
fn page_labels_are_stable_at_empty_and_end_boundaries() {
    assert_eq!(page_label(0, 0, 11), "PAGE 1 / 1");
    assert_eq!(page_label(11, 23, 11), "PAGE 2 / 3");
    assert_eq!(page_label(99, 23, 11), "PAGE 3 / 3");
}

#[test]
fn record_rows_cover_the_catalog_and_partition_by_category() {
    let state = AppState::default();
    let all = rows(&state, 0);
    let mut unique_games = Vec::new();

    for row in &all {
        if !unique_games.contains(&row.game) {
            unique_games.push(row.game);
        }
    }
    assert_eq!(unique_games.len(), GameId::ALL.len());
    for game in GameId::ALL {
        assert!(all.iter().any(|row| row.game == game));
    }
    for filter in FILTERS.into_iter().skip(1) {
        let filtered = rows(&state, filter);
        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|row| row.category == filter));
    }
}
