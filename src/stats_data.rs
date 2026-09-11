//! Derived collection statistics for the player-facing statistics shelf.

use crate::{progression, state::AppState, state::GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaytimeRow {
    pub game: GameId,
    pub seconds: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatisticsSummary {
    pub total_playtime_seconds: u64,
    pub played_games: usize,
    pub completed_games: usize,
    pub total_games: usize,
    pub favorite_games: usize,
    pub recent_games: usize,
    pub daily_clears: usize,
    pub daily_attempts: usize,
    pub fastest_clear: Option<PlaytimeRow>,
    pub longest_session: Option<PlaytimeRow>,
}

pub fn from_state(state: &AppState) -> StatisticsSummary {
    let time = state.records.time_summary();
    StatisticsSummary {
        total_playtime_seconds: time.total_seconds,
        played_games: time.active_games,
        completed_games: progression::completed_games(&state.records),
        total_games: GameId::ALL.len(),
        favorite_games: state.favorites.iter().filter(|favorite| **favorite).count(),
        recent_games: state.recent_games.len(),
        daily_clears: state.records.daily_clear_count(),
        daily_attempts: state.records.daily_results.len(),
        fastest_clear: fastest_clear(state),
        longest_session: longest_session(state),
    }
}

pub fn top_playtime(state: &AppState, limit: usize) -> Vec<PlaytimeRow> {
    let mut rows: Vec<_> = GameId::ALL
        .into_iter()
        .filter_map(|game| {
            let seconds = state.records.current_time(game.index());
            (seconds > 0).then_some(PlaytimeRow { game, seconds })
        })
        .collect();
    rows.sort_by_key(|row| (std::cmp::Reverse(row.seconds), row.game.index()));
    rows.truncate(limit);
    rows
}

fn fastest_clear(state: &AppState) -> Option<PlaytimeRow> {
    GameId::ALL
        .into_iter()
        .filter_map(|game| {
            state
                .records
                .best_time(game.index())
                .map(|seconds| PlaytimeRow { game, seconds })
        })
        .min_by_key(|row| (row.seconds, row.game.index()))
}

fn longest_session(state: &AppState) -> Option<PlaytimeRow> {
    GameId::ALL
        .into_iter()
        .filter_map(|game| {
            let seconds = state.records.current_time(game.index());
            (seconds > 0).then_some(PlaytimeRow { game, seconds })
        })
        .max_by_key(|row| (row.seconds, std::cmp::Reverse(row.game.index())))
}

pub fn completion_percent(summary: StatisticsSummary) -> usize {
    summary
        .completed_games
        .saturating_mul(100)
        .checked_div(summary.total_games)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
