//! Canonical rows and paging for the Daily Archive shelf.

use crate::{daily_dungeon::DailyRule, state::AppState, state_records::DailyResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveRow {
    pub day: u64,
    pub challenge: u32,
    pub rule: DailyRule,
    pub score: u32,
    pub won: bool,
}

pub fn rows(state: &AppState) -> Vec<ArchiveRow> {
    state
        .records
        .daily_results
        .iter()
        .rev()
        .map(ArchiveRow::from_result)
        .collect()
}

pub fn page_rows(state: &AppState, start: usize, capacity: usize) -> Vec<ArchiveRow> {
    let rows = rows(state);
    let start = page_start(rows.len(), start, capacity);
    rows.into_iter().skip(start).take(capacity).collect()
}

pub fn page_start(total: usize, start: usize, capacity: usize) -> usize {
    start.min(total.saturating_sub(capacity.max(1)))
}

pub fn window_label(start: usize, total: usize, capacity: usize) -> String {
    if total == 0 {
        return "0-0 OF 0".to_owned();
    }
    let start = page_start(total, start, capacity);
    let first = start + 1;
    let last = (start + capacity.max(1)).min(total);
    format!("{first}-{last} OF {total}")
}

impl ArchiveRow {
    fn from_result(result: &DailyResult) -> Self {
        Self {
            day: result.day,
            challenge: crate::daily_challenge::challenge_for_day(result.day),
            rule: crate::daily_challenge::rule_for_day(result.day),
            score: result.score,
            won: result.won,
        }
    }
}

#[cfg(test)]
mod tests;
