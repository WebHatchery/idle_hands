//! Regression coverage for the tests module.

use idle_hands::testing::modules::stats_data::*;
use idle_hands::testing::state::GameId;

#[test]
fn summary_derives_collection_and_daily_counts() {
    let mut state = AppState::default();
    state.favorites[GameId::Solitaire.index()] = true;
    state.recent_games = vec![GameId::Solitaire, GameId::Game2048];
    state.records.elapsed_seconds = vec![42; GameId::ALL.len()];
    state.records.best_time_seconds = vec![None; GameId::ALL.len()];
    state.records.best_time_seconds[GameId::Solitaire.index()] = Some(19);
    state.records.record_daily_result(12, 80, true);
    state.records.record_daily_result(13, 25, false);

    let summary = from_state(&state);

    assert_eq!(
        summary.total_playtime_seconds,
        42 * GameId::ALL.len() as u64
    );
    assert_eq!(summary.played_games, GameId::ALL.len());
    assert_eq!(summary.favorite_games, 1);
    assert_eq!(summary.recent_games, 2);
    assert_eq!(summary.daily_clears, 1);
    assert_eq!(summary.daily_attempts, 2);
    assert_eq!(summary.fastest_clear.unwrap().game, GameId::Solitaire);
}

#[test]
fn top_playtime_is_descending_and_bounded() {
    let mut state = AppState::default();
    state.records.elapsed_seconds = vec![0; GameId::ALL.len()];
    state.records.elapsed_seconds[GameId::Solitaire.index()] = 10;
    state.records.elapsed_seconds[GameId::Game2048.index()] = 90;
    state.records.elapsed_seconds[GameId::Sudoku.index()] = 40;

    let rows = top_playtime(&state, 2);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].game, GameId::Game2048);
    assert_eq!(rows[1].game, GameId::Sudoku);
}

#[test]
fn completion_percent_handles_empty_catalogs_without_panicking() {
    let summary = StatisticsSummary {
        total_playtime_seconds: 0,
        played_games: 0,
        completed_games: 0,
        total_games: 0,
        favorite_games: 0,
        recent_games: 0,
        daily_clears: 0,
        daily_attempts: 0,
        fastest_clear: None,
        longest_session: None,
    };

    assert_eq!(completion_percent(summary), 0);
}
