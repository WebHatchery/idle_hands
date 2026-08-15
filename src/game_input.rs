//! Shared drag routing for the two classic card tables.

use crate::{
    freecell_ui, responsive_cards, responsive_landscape_cards, solitaire_ui,
    state::{AppState, GameId, Screen},
    ui,
};
use macroquad::prelude::Vec2;

pub fn card_drag_actions(
    state: &AppState,
    start: Vec2,
    end: Vec2,
    portrait: bool,
    compact_landscape: bool,
) -> Vec<ui::UiAction> {
    if !matches!(
        state.screen,
        Screen::Game(GameId::Solitaire | GameId::FreeCell)
    ) {
        return Vec::new();
    }
    let actions_at = |point| {
        if portrait {
            match state.screen {
                Screen::Game(GameId::Solitaire) => responsive_cards::solitaire_clicks(state, point),
                Screen::Game(GameId::FreeCell) => responsive_cards::freecell_clicks(state, point),
                _ => Vec::new(),
            }
        } else if compact_landscape {
            match state.screen {
                Screen::Game(GameId::Solitaire) => {
                    responsive_landscape_cards::solitaire_clicks(state, point)
                }
                Screen::Game(GameId::FreeCell) => {
                    responsive_landscape_cards::freecell_clicks(state, point)
                }
                _ => Vec::new(),
            }
        } else {
            match state.screen {
                Screen::Game(GameId::Solitaire) => solitaire_ui::solitaire_clicks(state, point),
                Screen::Game(GameId::FreeCell) => freecell_ui::freecell_clicks(state, point),
                _ => Vec::new(),
            }
        }
    };
    actions_at(start)
        .into_iter()
        .chain(actions_at(end))
        .collect()
}
