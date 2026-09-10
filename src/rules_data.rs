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

pub fn rows(filter: u8) -> Vec<RuleRow> {
    let filter = if FILTERS.contains(&filter) { filter } else { 0 };
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

#[cfg(test)]
mod tests;
