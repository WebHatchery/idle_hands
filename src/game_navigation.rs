//! Cabinet-to-drawer navigation and recent-history ownership.

use super::Game;
use crate::state::{GameId, Screen};
use macroquad::prelude::{get_time, mouse_wheel, Vec2};
use macroquad_toolkit::persistence::slot_exists;

impl Game {
    pub(super) fn refresh_daily_challenge(&mut self) {
        let day = crate::daily_challenge::current_day();
        if day == 0 || self.state.games.daily_dungeon.is_for_day(day) {
            return;
        }
        self.state.games.daily_dungeon = crate::daily_dungeon::DailyDungeon::new_for_day(day);
        self.notifications.info(format!(
            "A fresh {} route awaits",
            crate::daily_challenge::label(day, 0)
        ));
        self.request_autosave();
    }

    pub(super) fn initialize_launch_state(&mut self) {
        let collection_slot = &self.data.config.save_slot;
        let has_saved_state = slot_exists(&self.data.config.game_name, collection_slot)
            || slot_exists(
                &self.data.config.game_name,
                &format!("{}_profile", collection_slot),
            )
            || GameId::ALL.iter().any(|game| {
                slot_exists(
                    &self.data.config.game_name,
                    &format!("{}_{}", collection_slot, game.save_key()),
                )
            });
        if !has_saved_state {
            self.state = crate::state::AppState::new_random(&self.data, get_time().to_bits());
        }
    }

    pub(super) fn apply_navigation_drag(&mut self, start: Vec2, end: Vec2) {
        if (end.y - start.y).abs() <= 24.0 {
            return;
        }
        let delta = if end.y < start.y { 1 } else { -1 };
        if self.state.screen == Screen::Cabinet {
            self.apply(crate::ui::UiAction::CabinetScroll(delta));
        } else if self.state.daily_archive_view {
            self.apply(crate::ui::UiAction::DailyArchiveScroll(
                delta * crate::daily_archive_ui::page_size() as i8,
            ));
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
        } else if self.state.daily_archive_view {
            self.apply(crate::ui::UiAction::DailyArchiveScroll(
                delta * crate::daily_archive_ui::page_size() as i8,
            ));
        } else if self.library_scroll_is_active() {
            self.apply(crate::ui::UiAction::LibraryScroll(delta));
        }
    }

    fn library_scroll_is_active(&self) -> bool {
        matches!(self.state.screen, Screen::Records | Screen::Rules)
            && !self.state.daily_archive_view
    }

    pub(super) fn library_scroll_limit(&self) -> usize {
        if self.state.screen == Screen::Records
            && (self.state.favorites_view || self.state.recent_view)
        {
            let capacity = if crate::ui::is_portrait() {
                8
            } else if crate::ui::is_compact_landscape() {
                10
            } else {
                40
            };
            return crate::favorites_data::scroll_limit(
                &self.state,
                crate::favorites_data::BrowseMode::from_state(&self.state),
                capacity,
            );
        }
        GameId::ALL.len().saturating_sub(1)
    }

    pub(super) fn open_game(&mut self, index: usize) {
        let Some(id) = GameId::ALL.get(index).copied() else {
            return;
        };
        let from_rules = matches!(self.state.screen, Screen::Rules);
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.daily_archive_view = false;
        self.state.achievements_view = false;
        self.state.selected = index;
        if crate::cabinet_status::is_available(id) {
            self.state.recent_games.retain(|recent| *recent != id);
            self.state.recent_games.insert(0, id);
            self.state.recent_games.truncate(5);
            self.state.screen = Screen::Game(id);
            if from_rules {
                self.notifications.info(format!("Opening {}", id.title()));
            }
            self.state.tutorial = (!self.state.tutorial_seen[id.index()]).then_some(id);
        } else if crate::game_descriptor::is_demo_build() && crate::cabinet_status::is_active(id) {
            self.notifications
                .info(crate::storefront::purchase_message(id.title()));
        } else {
            self.notifications
                .info(format!("{} is coming soon", id.title()));
        }
    }
}
