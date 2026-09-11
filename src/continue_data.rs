//! Shared selection rules for the cabinet's Continue drawer.

use crate::state::{AppState, GameId};

pub fn preferred_game(state: &AppState) -> GameId {
    preferred_game_for_build(state, crate::game_descriptor::is_demo_build())
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
    match crate::cabinet_status::status(state, preferred_game(state)) {
        "PLAY NOW" => "START PLAYING",
        "COMPLETE" => "PLAY AGAIN",
        _ => "CONTINUE PLAYING",
    }
}

pub fn action_label(state: &AppState) -> &'static str {
    match title(state) {
        "START PLAYING" => "START  >",
        "PLAY AGAIN" => "REPLAY  >",
        _ => "CONTINUE  >",
    }
}

pub fn compact_action_label(state: &AppState) -> &'static str {
    match title(state) {
        "START PLAYING" => "START",
        "PLAY AGAIN" => "REPLAY",
        _ => "CONTINUE",
    }
}

fn selected_if_playable(selected: usize, demo_build: bool) -> Option<GameId> {
    GameId::ALL.get(selected).copied().filter(|game| {
        crate::storefront_data::availability_for_build(*game, demo_build).is_playable()
    })
}

#[cfg(test)]
mod tests;
