//! Shared rule rows used by each responsive Rules shelf.

use crate::state::GameId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleRow {
    pub category: u8,
    pub game: GameId,
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub const FILTERS: [u8; 7] = [0, 3, 4, 5, 6, 7, 8];

pub fn normalize_filter(filter: u8) -> u8 {
    if FILTERS.contains(&filter) {
        filter
    } else {
        0
    }
}

pub fn filter_label(filter: u8) -> &'static str {
    match filter {
        0 => "ALL",
        3..=8 => crate::cabinet_status::category_name(filter),
        _ => "ALL",
    }
}

pub fn next_filter(filter: u8) -> u8 {
    let current = FILTERS
        .iter()
        .position(|candidate| *candidate == filter)
        .unwrap_or(0);
    FILTERS[(current + 1) % FILTERS.len()]
}

pub fn summary_label(filter: u8) -> String {
    format!("{} DRAWERS", rows(filter).len())
}

pub fn action_label(game: GameId) -> &'static str {
    if crate::cabinet_status::is_available(game) {
        "OPEN"
    } else {
        "FULL"
    }
}

pub fn page_label(start: usize, total: usize, page_size: usize) -> String {
    let page_size = page_size.max(1);
    let pages = total.max(1).div_ceil(page_size);
    let page = (start / page_size + 1).min(pages);
    format!("PAGE {} / {}", page, pages)
}

pub fn rows(filter: u8) -> Vec<RuleRow> {
    let filter = normalize_filter(filter);
    GameId::ALL
        .into_iter()
        .map(|game| RuleRow {
            category: crate::cabinet_status::category_filter(game),
            game,
            title: game.title(),
            subtitle: game.subtitle(),
        })
        .filter(|row| filter == 0 || row.category == filter)
        .collect()
}

pub fn page_rows(filter: u8, start: usize, capacity: usize) -> Vec<RuleRow> {
    let rows = rows(filter);
    let start = start.min(rows.len());
    rows.into_iter().skip(start).take(capacity).collect()
}

#[cfg(test)]
mod tests;
