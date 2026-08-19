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
