//! Regression coverage for achievement data.

use super::*;
use crate::state::GameId;

#[test]
fn achievement_filters_partition_the_catalog() {
    let mut state = AppState::default();
    state.achievements[AchievementId::FirstFinish.index()] = true;
    state.achievements[AchievementId::FullCabinet.index()] = true;

    assert_eq!(filter_count(&state, 0), AchievementId::ALL.len());
    assert_eq!(filter_count(&state, 1), 2);
    assert_eq!(filter_count(&state, 2), AchievementId::ALL.len() - 2);
    assert_eq!(
        crate::collection_summary::from_state(&state).earned_achievements,
        2
    );
}

#[test]
fn achievement_rows_keep_catalog_order_and_progress() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);

    let rows = rows(&state, 0);

    assert_eq!(rows[0].achievement, AchievementId::FirstFinish);
    assert_eq!(rows[1].achievement, AchievementId::Game(GameId::Solitaire));
    assert!(rows[1].progress.is_complete());
}

#[test]
fn achievement_pages_clamp_to_the_filtered_window() {
    let mut state = AppState::default();
    state.achievements[AchievementId::FirstFinish.index()] = true;
    state.achievements[AchievementId::Game(GameId::Solitaire).index()] = true;

    let page = page_rows(&state, 1, 99, 1);

    assert_eq!(page.len(), 1);
    assert_eq!(page[0].achievement, AchievementId::Game(GameId::Solitaire));
}

#[test]
fn achievement_scroll_limit_follows_filter_size() {
    let mut state = AppState::default();
    state.achievements[AchievementId::FirstFinish.index()] = true;

    assert_eq!(scroll_limit(&state, 1, 8), 0);
    assert_eq!(scroll_limit(&state, 0, 10), AchievementId::ALL.len() - 10);
}

#[test]
fn achievement_empty_copy_explains_each_filtered_state() {
    assert_eq!(empty_label(0), "No achievements match this shelf.");
    assert_eq!(empty_label(1), "No achievements earned yet.");
    assert_eq!(empty_label(2), "Every achievement is earned.");
}

#[test]
fn achievement_window_labels_clamp_to_the_filtered_end() {
    assert_eq!(window_label(0, 0, 8), "0-0 OF 0");
    assert_eq!(window_label(0, 17, 8), "1-8 OF 17");
    assert_eq!(window_label(99, 17, 8), "10-17 OF 17");
}
