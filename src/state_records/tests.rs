use super::CollectionRecords;
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
