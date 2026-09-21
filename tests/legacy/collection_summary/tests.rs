//! Regression coverage for the tests module.

use idle_hands::testing::modules::collection_summary::*;

#[test]
fn summary_collects_progression_and_time_state_in_one_snapshot() {
    let mut state = AppState {
        stamps: 7,
        ..AppState::default()
    };
    state.achievements[0] = true;
    state.records.solitaire_best_moves = Some(42);
    state.records.best_time_seconds = vec![Some(95)];

    let summary = from_state(&state);

    assert_eq!(summary.completed_games, 1);
    assert_eq!(summary.earned_achievements, 1);
    assert_eq!(summary.stamps, 7);
    assert_eq!(summary.total_playtime_seconds, 0);
    assert_eq!(summary.active_games, 0);
    assert_eq!(summary.completion_percent(), 1);
    assert_eq!(summary.achievement_percent(), 1);
    assert_eq!(
        summary.progress_label(),
        "Drawers 1/60  ·  Achievements 1/62  ·  Stamps 7"
    );
    assert_eq!(summary.fastest_label(), "Fastest 1m 35s");
}
