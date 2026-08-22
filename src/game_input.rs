//! Shared drag routing for the two classic card tables.

use super::Game;
use crate::domain::Direction;
use crate::{
    freecell_ui, responsive_cards, responsive_landscape_cards, solitaire_ui, spider_solitaire_ui,
    state::{AppState, GameId, Screen},
    ui,
};
use macroquad::prelude::{is_mouse_button_down, touches, MouseButton, Vec2};

impl Game {
    pub(super) fn update_card_peek(&mut self) {
        if self.capture_solitaire_peek {
            return;
        }
        let touch_active = !touches().is_empty();
        let pointer_active = touch_active || is_mouse_button_down(MouseButton::Left);
        if self.state.screen != Screen::Game(GameId::Solitaire) {
            self.state.games.solitaire_peek = None;
        } else if touch_active || !self.touch_was_active {
            self.state.games.solitaire_peek =
                solitaire_ui::tableau_card_at(&self.state.games.solitaire, crate::ui::mouse())
                    .map(|(column, depth)| crate::solitaire::CardSource::Tableau(column, depth));
        } else {
            self.state.games.solitaire_peek = None;
        }
        if self.state.screen == Screen::Game(GameId::SpiderSolitaire) && pointer_active {
            self.state.games.spider_solitaire_peek = spider_solitaire_ui::tableau_card_at(
                &self.state.games.spider_solitaire,
                crate::ui::mouse(),
            );
        } else {
            self.state.games.spider_solitaire_peek = None;
        }
        self.touch_was_active = touch_active;
    }
}

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

pub fn swipe_direction(delta: Vec2) -> Direction {
    if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if delta.y > 0.0 {
        Direction::Down
    } else {
        Direction::Up
    }
}
