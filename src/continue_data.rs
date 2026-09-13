//! Shared selection rules for the cabinet's Continue drawer.

use crate::state::{AppState, GameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinueIntent {
    Start,
    Resume,
    Replay,
}

impl ContinueIntent {
    pub const fn title(self) -> &'static str {
        match self {
            Self::Start => "START PLAYING",
            Self::Resume => "CONTINUE PLAYING",
            Self::Replay => "PLAY AGAIN",
        }
    }

    pub const fn action_label(self) -> &'static str {
        match self {
            Self::Start => "START  >",
            Self::Resume => "CONTINUE  >",
            Self::Replay => "REPLAY  >",
        }
    }

    pub const fn compact_action_label(self) -> &'static str {
        match self {
            Self::Start => "START",
            Self::Resume => "CONTINUE",
            Self::Replay => "REPLAY",
        }
    }
}

pub fn preferred_game(state: &AppState) -> GameId {
    preferred_game_for_build(state, crate::game_descriptor::is_demo_build())
}

pub fn repair_selected(state: &mut AppState) -> bool {
    repair_selected_for_build(state, crate::game_descriptor::is_demo_build())
}

pub fn repair_selected_for_build(state: &mut AppState, demo_build: bool) -> bool {
    let selected_is_playable = GameId::ALL.get(state.selected).is_some_and(|game| {
        crate::storefront_data::availability_for_build(*game, demo_build).is_playable()
    });
    if selected_is_playable {
        return false;
    }
    let repaired = preferred_game_for_build(state, demo_build);
    if state.selected == repaired.index() {
        return false;
    }
    state.selected = repaired.index();
    true
}

pub fn preferred_game_for_build(state: &AppState, demo_build: bool) -> GameId {
    selected_if_playable(state.selected, demo_build)
        .or_else(|| {
            state.recent_games.iter().copied().find(|game| {
                crate::storefront_data::availability_for_build(*game, demo_build).is_playable()
            })
        })
        .or_else(|| {
            GameId::ALL.iter().copied().find(|game| {
                crate::storefront_data::availability_for_build(*game, demo_build).is_playable()
            })
        })
        .unwrap_or(GameId::ALL[0])
}

pub fn title(state: &AppState) -> &'static str {
    intent(state).title()
}

pub fn action_label(state: &AppState) -> &'static str {
    intent(state).action_label()
}

pub fn compact_action_label(state: &AppState) -> &'static str {
    intent(state).compact_action_label()
}

pub fn intent(state: &AppState) -> ContinueIntent {
    match crate::cabinet_status::status(state, preferred_game(state)) {
        "PLAY NOW" => ContinueIntent::Start,
        "COMPLETE" => ContinueIntent::Replay,
        _ => ContinueIntent::Resume,
    }
}

fn selected_if_playable(selected: usize, demo_build: bool) -> Option<GameId> {
    GameId::ALL.get(selected).copied().filter(|game| {
        crate::storefront_data::availability_for_build(*game, demo_build).is_playable()
    })
}

#[cfg(test)]
#[path = "../tests/legacy/continue_data/tests.rs"]
mod tests;
