//! Canonical rows and summaries for the Favorites and Recent shelves.

use crate::state::{AppState, GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowseMode {
    Favorites,
    Recent,
}

impl BrowseMode {
    pub fn from_state(state: &AppState) -> Self {
        if state.recent_view {
            Self::Recent
        } else {
            Self::Favorites
        }
    }

    pub fn is_recent(self) -> bool {
        matches!(self, Self::Recent)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowseRow {
    pub game: GameId,
    pub source_index: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BrowseSummary {
    pub total: usize,
    pub open: usize,
    pub done: usize,
    pub locked: usize,
    pub coming_soon: usize,
}

impl BrowseSummary {
    pub fn restriction_label(self) -> String {
        if self.coming_soon == 0 {
            format!("{} locked", self.locked)
        } else {
            format!("{} locked · {} soon", self.locked, self.coming_soon)
        }
    }
}

pub fn games(state: &AppState, mode: BrowseMode) -> Vec<GameId> {
    match mode {
        BrowseMode::Recent => state.recent_games.clone(),
        BrowseMode::Favorites => GameId::ALL
            .iter()
            .copied()
            .filter(|game| state.favorites.get(game.index()).copied().unwrap_or(false))
            .collect(),
    }
}

pub fn page_rows(
    state: &AppState,
    mode: BrowseMode,
    start: usize,
    capacity: usize,
) -> Vec<BrowseRow> {
    let games = games(state, mode);
    let start = page_start(games.len(), start, capacity);
    games
        .into_iter()
        .enumerate()
        .skip(start)
        .take(capacity)
        .map(|(source_index, game)| BrowseRow { game, source_index })
        .collect()
}

pub fn page_start(total: usize, start: usize, capacity: usize) -> usize {
    start.min(total.saturating_sub(capacity.max(1)))
}

pub fn scroll_limit(state: &AppState, mode: BrowseMode, capacity: usize) -> usize {
    games(state, mode).len().saturating_sub(capacity.max(1))
}

pub fn summary(state: &AppState, mode: BrowseMode) -> BrowseSummary {
    let games = games(state, mode);
    let done = games
        .iter()
        .filter(|game| crate::cabinet_status::status(state, **game) == "COMPLETE")
        .count();
    let (locked, coming_soon) =
        games.iter().fold(
            (0, 0),
            |(locked, soon), game| match crate::cabinet_status::availability(*game) {
                crate::storefront::GameAvailability::Playable => (locked, soon),
                crate::storefront::GameAvailability::DemoRestricted => (locked + 1, soon),
                crate::storefront::GameAvailability::ComingSoon => (locked, soon + 1),
            },
        );
    BrowseSummary {
        total: games.len(),
        open: games.len().saturating_sub(done + locked + coming_soon),
        done,
        locked,
        coming_soon,
    }
}

#[cfg(test)]
mod tests;
