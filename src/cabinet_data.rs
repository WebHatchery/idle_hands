//! Shared collection paging for every responsive cabinet shelf.

use crate::state::{AppState, GameId};

pub const DESKTOP_PAGE_SIZE: usize = 44;
pub const DESKTOP_PAGE_STEP: i8 = 11;
pub const PORTRAIT_PAGE_SIZE: usize = 12;
pub const COMPACT_LANDSCAPE_PAGE_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabinetPage {
    pub games: Vec<GameId>,
    pub start: usize,
    pub end: usize,
    pub total: usize,
}

impl CabinetPage {
    pub fn has_previous(&self) -> bool {
        self.start > 0
    }

    pub fn has_next(&self) -> bool {
        self.end < self.total
    }
}

pub fn visible_games(state: &AppState) -> Vec<GameId> {
    crate::cabinet_status::sorted_games(
        state,
        state.cabinet_filter,
        crate::cabinet_status::CabinetSort::from_index(state.cabinet_sort),
    )
}

pub fn favorite_count(state: &AppState) -> usize {
    state.favorites.iter().filter(|favorite| **favorite).count()
}

pub fn page(state: &AppState, page_size: usize) -> CabinetPage {
    let games = visible_games(state);
    let total = games.len();
    let start = page_start(total, state.cabinet_scroll, page_size);
    let end = (start + page_size).min(total);
    CabinetPage {
        games: games.into_iter().skip(start).take(page_size).collect(),
        start,
        end,
        total,
    }
}

pub fn page_start(total: usize, scroll: usize, page_size: usize) -> usize {
    scroll.min(total.saturating_sub(page_size))
}

pub fn scroll_limit(state: &AppState, page_size: usize) -> usize {
    page_start(visible_games(state).len(), usize::MAX, page_size)
}

pub fn range_label(page: &CabinetPage) -> String {
    if page.total == 0 {
        "0-0 OF 0".to_owned()
    } else {
        format!("{}-{} OF {}", page.start + 1, page.end, page.total)
    }
}

pub fn page_size_for_layout(portrait: bool, compact_landscape: bool) -> usize {
    if portrait {
        PORTRAIT_PAGE_SIZE
    } else if compact_landscape {
        COMPACT_LANDSCAPE_PAGE_SIZE
    } else {
        DESKTOP_PAGE_SIZE
    }
}

#[cfg(test)]
mod tests;
