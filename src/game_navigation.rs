//! Cabinet-to-drawer navigation and recent-history ownership.

use super::Game;
use crate::state::{GameId, Screen};
use macroquad::prelude::{mouse_wheel, Vec2};

impl Game {
    pub(super) fn apply_navigation_drag(&mut self, start: Vec2, end: Vec2) {
        if (end.y - start.y).abs() <= 24.0 {
            return;
        }
        let delta = if end.y < start.y { 1 } else { -1 };
        if self.state.screen == Screen::Cabinet {
            self.apply(crate::ui::UiAction::CabinetScroll(delta));
        } else if self.library_scroll_is_active() {
            self.apply(crate::ui::UiAction::LibraryScroll(delta));
        }
    }

    pub(super) fn update_navigation_scroll(&mut self) {
        let (_, wheel_y) = mouse_wheel();
        if wheel_y.abs() <= f32::EPSILON {
            return;
        }
        let delta = if wheel_y < 0. { 1 } else { -1 };
        if self.state.screen == Screen::Cabinet {
            self.apply(crate::ui::UiAction::CabinetScroll(delta));
        } else if self.library_scroll_is_active() {
            self.apply(crate::ui::UiAction::LibraryScroll(delta));
        }
    }

    fn library_scroll_is_active(&self) -> bool {
        matches!(self.state.screen, Screen::Records | Screen::Rules)
    }

    pub(super) fn open_game(&mut self, index: usize) {
        let Some(id) = GameId::ALL.get(index).copied() else {
            return;
        };
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.achievements_view = false;
        self.state.selected = index;
        if crate::cabinet_status::is_active(id) {
            self.state.recent_games.retain(|recent| *recent != id);
            self.state.recent_games.insert(0, id);
            self.state.recent_games.truncate(5);
            self.state.screen = Screen::Game(id);
            self.state.tutorial = (!self.state.tutorial_seen[id.index()]).then_some(id);
        } else {
            self.notifications
                .info(format!("{} is coming soon", id.title()));
        }
    }
}
