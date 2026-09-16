//! Regression coverage for the tests module.

use super::*;

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
