//! Derived rows for the collection-wide tutorial shelf.

use crate::state::{AppState, GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TutorialRow {
    pub game: GameId,
    pub seen: bool,
}

pub fn rows(state: &AppState) -> Vec<TutorialRow> {
    GameId::ALL
        .into_iter()
        .map(|game| TutorialRow {
            game,
            seen: state
                .tutorial_seen
                .get(game.index())
                .copied()
                .unwrap_or(false),
        })
        .collect()
}

pub fn seen_count(state: &AppState) -> usize {
    rows(state).iter().filter(|row| row.seen).count()
}

pub fn visible_count() -> usize {
    if crate::ui::is_portrait() || crate::ui::is_compact_landscape() {
        8
    } else {
        12
    }
}

pub fn scroll_limit(state: &AppState) -> usize {
    rows(state).len().saturating_sub(visible_count())
}

pub fn page_label(start: usize, total: usize) -> String {
    if total == 0 {
        return "NO LESSONS".to_owned();
    }
    let end = (start + visible_count()).min(total);
    format!("{}–{} OF {}", start + 1, end, total)
}

#[cfg(test)]
mod tests;
