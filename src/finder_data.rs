//! Alphabetical drawer-finder rows shared by every cabinet layout.

use crate::state::{AppState, GameId};

pub const FILTERS: [u8; 5] = [0, 1, 2, 3, 4];
const LABELS: [&str; 5] = ["ALL", "#–F", "G–M", "N–S", "T–Z"];

pub fn normalize_filter(filter: u8) -> u8 {
    if FILTERS.contains(&filter) {
        filter
    } else {
        0
    }
}

pub fn filter_label(filter: u8) -> &'static str {
    LABELS[normalize_filter(filter) as usize]
}

pub fn rows(filter: u8) -> Vec<GameId> {
    let mut games = GameId::ALL
        .into_iter()
        .filter(|game| matches_filter(*game, filter))
        .collect::<Vec<_>>();
    games.sort_by(|left, right| left.title().cmp(right.title()));
    games
}

pub fn matches_filter(game: GameId, filter: u8) -> bool {
    match normalize_filter(filter) {
        0 => true,
        1 => first_letter(game) <= 'F',
        2 => ('G'..='M').contains(&first_letter(game)),
        3 => ('N'..='S').contains(&first_letter(game)),
        4 => ('T'..='Z').contains(&first_letter(game)),
        _ => true,
    }
}

pub fn visible_count() -> usize {
    if crate::ui::is_portrait() || crate::ui::is_compact_landscape() {
        8
    } else {
        12
    }
}

pub fn page(state: &AppState) -> Vec<GameId> {
    let games = rows(state.cabinet_filter);
    let start = state.library_scroll.min(scroll_limit(state));
    games
        .into_iter()
        .skip(start)
        .take(visible_count())
        .collect()
}

pub fn scroll_limit(state: &AppState) -> usize {
    rows(state.cabinet_filter)
        .len()
        .saturating_sub(visible_count())
}

pub fn page_label(state: &AppState) -> String {
    let total = rows(state.cabinet_filter).len();
    if total == 0 {
        return "NO DRAWERS".to_owned();
    }
    let start = state.library_scroll.min(scroll_limit(state));
    let end = (start + visible_count()).min(total);
    format!("{}–{} OF {}", start + 1, end, total)
}

pub fn status_label(state: &AppState, game: GameId, compact: bool) -> &'static str {
    match crate::storefront::availability(game) {
        crate::storefront::GameAvailability::Playable => crate::cabinet_status::status(state, game),
        availability => availability.cabinet_label(compact),
    }
}

fn first_letter(game: GameId) -> char {
    game.title()
        .chars()
        .next()
        .unwrap_or('Z')
        .to_ascii_uppercase()
}

#[cfg(test)]
mod tests;
