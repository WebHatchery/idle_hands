use super::{format_duration, CollectionRecords};
use crate::state::GameId;

#[test]
fn elapsed_time_slots_follow_the_catalog() {
    let mut records = CollectionRecords::default();
    records.ensure_time_slots();

    assert_eq!(records.elapsed_seconds.len(), GameId::ALL.len());
    assert_eq!(records.best_time_seconds.len(), GameId::ALL.len());
    assert_eq!(records.current_time(GameId::Solitaire.index()), 0);
}

#[test]
fn best_time_keeps_the_fastest_completed_round() {
    let mut records = CollectionRecords::default();
    let game_index = GameId::Hanoi.index();
    records.ensure_time_slots();
    records.elapsed_seconds[game_index] = 42;
    records.record_time(game_index);
    records.elapsed_seconds[game_index] = 18;
    records.record_time(game_index);

    assert_eq!(records.best_time(game_index), Some(18));
}

#[test]
fn time_summary_collects_playtime_and_completion_depth() {
    let records = CollectionRecords {
        elapsed_seconds: vec![61, 3_601, 0],
        best_time_seconds: vec![Some(70), None, Some(22)],
        ..Default::default()
    };
    let summary = records.time_summary();
    assert_eq!(summary.total_seconds, 3_662);
    assert_eq!(summary.active_games, 2);
    assert_eq!(summary.completed_games, 2);
    assert_eq!(summary.fastest_seconds, Some(22));
    assert_eq!(format_duration(61), "1m 01s");
    assert_eq!(format_duration(3_661), "1h 01m");
}

#[test]
fn daily_results_merge_replays_without_creating_streak_pressure() {
    let mut records = CollectionRecords::default();
    records.record_daily_result(20_042, 70, false);
    records.record_daily_result(20_042, 92, true);
    records.record_daily_result(20_043, 55, false);
    records.record_daily_result(0, 999, true);

    assert_eq!(records.daily_results.len(), 2);
    assert_eq!(records.daily_score(20_042), Some(92));
    assert_eq!(records.daily_clear_count(), 1);
    assert_eq!(records.daily_best_score(), Some(92));
}

#[test]
fn daily_history_keeps_the_most_recent_ninety_days() {
    let mut records = CollectionRecords::default();
    for day in 1..=91 {
        records.record_daily_result(day, day as u32, true);
    }

    assert_eq!(records.daily_results.len(), 90);
    assert_eq!(
        records.daily_results.first().map(|result| result.day),
        Some(2)
    );
    assert_eq!(
        records.daily_results.last().map(|result| result.day),
        Some(91)
    );
}
