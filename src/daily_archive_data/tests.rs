use super::*;
use crate::daily_dungeon::DailyRule;

#[test]
fn archive_rows_are_newest_first_and_keep_challenge_identity() {
    let mut state = AppState::default();
    state.records.record_daily_result(41, 90, false);
    state.records.record_daily_result(42, 120, true);

    let rows = rows(&state);

    assert_eq!(rows[0].day, 42);
    assert_eq!(rows[0].challenge, 42);
    assert_eq!(rows[0].rule, DailyRule::Wayfinder);
    assert_eq!(rows[0].score, 120);
    assert!(rows[0].won);
    assert_eq!(rows[1].day, 41);
}

#[test]
fn archive_rows_sort_historical_inserts_by_day_not_save_order() {
    let mut state = AppState::default();
    state.records.record_daily_result(42, 120, true);
    state.records.record_daily_result(40, 80, false);

    let rows = rows(&state);

    assert_eq!(rows[0].day, 42);
    assert_eq!(rows[1].day, 40);
}

#[test]
fn archive_page_rows_clamp_to_the_latest_window() {
    let mut state = AppState::default();
    for day in 1..=4 {
        state
            .records
            .record_daily_result(day, day as u32, day % 2 == 0);
    }

    let page = page_rows(&state, 99, 2);

    assert_eq!(page[0].day, 2);
    assert_eq!(page[1].day, 1);
}

#[test]
fn archive_window_labels_cover_empty_and_end_boundaries() {
    assert_eq!(window_label(0, 0, 7), "0-0 OF 0");
    assert_eq!(window_label(0, 11, 7), "1-7 OF 11");
    assert_eq!(window_label(99, 11, 7), "5-11 OF 11");
}

#[test]
fn archive_action_names_the_available_recovery_path() {
    assert_eq!(action_label(), "REPLAY");
}
