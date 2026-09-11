//! Canonical filtered rows and paging for the Achievements shelf.

use crate::{
    progression::{AchievementId, AchievementProgress},
    state::AppState,
};

pub const FILTERS: [u8; 3] = [0, 1, 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AchievementRow {
    pub achievement: AchievementId,
    pub earned: bool,
    pub progress: AchievementProgress,
}

pub fn normalize_filter(filter: u8) -> u8 {
    if FILTERS.contains(&filter) {
        filter
    } else {
        0
    }
}

pub fn filter_label(filter: u8) -> &'static str {
    match normalize_filter(filter) {
        1 => "EARNED",
        2 => "LOCKED",
        _ => "ALL",
    }
}

pub fn empty_label(filter: u8) -> &'static str {
    match normalize_filter(filter) {
        1 => "No achievements earned yet.",
        2 => "Every achievement is earned.",
        _ => "No achievements match this shelf.",
    }
}

pub fn rows(state: &AppState, filter: u8) -> Vec<AchievementRow> {
    AchievementId::ALL
        .into_iter()
        .filter(|achievement| matches_filter(state, *achievement, filter))
        .map(|achievement| AchievementRow {
            achievement,
            earned: is_earned(state, achievement),
            progress: achievement.progress(&state.records),
        })
        .collect()
}

pub fn page_rows(
    state: &AppState,
    filter: u8,
    start: usize,
    capacity: usize,
) -> Vec<AchievementRow> {
    let rows = rows(state, filter);
    let start = page_start(rows.len(), start, capacity);
    rows.into_iter().skip(start).take(capacity).collect()
}

pub fn page_start(total: usize, start: usize, capacity: usize) -> usize {
    start.min(total.saturating_sub(capacity.max(1)))
}

pub fn scroll_limit(state: &AppState, filter: u8, capacity: usize) -> usize {
    filter_count(state, filter).saturating_sub(capacity.max(1))
}

pub fn filter_count(state: &AppState, filter: u8) -> usize {
    rows(state, filter).len()
}

pub fn earned_count(state: &AppState) -> usize {
    filter_count(state, 1)
}

pub fn is_earned(state: &AppState, achievement: AchievementId) -> bool {
    state
        .achievements
        .get(achievement.index())
        .copied()
        .unwrap_or(false)
}

fn matches_filter(state: &AppState, achievement: AchievementId, filter: u8) -> bool {
    match normalize_filter(filter) {
        1 => is_earned(state, achievement),
        2 => !is_earned(state, achievement),
        _ => true,
    }
}

#[cfg(test)]
mod tests;
