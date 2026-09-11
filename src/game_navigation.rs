//! Cabinet-to-drawer navigation and recent-history ownership.

use super::Game;
use crate::state::{GameId, Screen};
use macroquad::prelude::{get_time, mouse_wheel, Vec2};
use macroquad_toolkit::persistence::slot_exists;

impl Game {
    pub(super) fn apply_browse_action(&mut self, action: crate::ui::UiAction) -> bool {
        match action {
            crate::ui::UiAction::Statistics => self.open_statistics(),
            crate::ui::UiAction::Tutorials => self.open_tutorial_library(),
            crate::ui::UiAction::OpenTutorial(index) => self.open_tutorial(index),
            crate::ui::UiAction::ToggleTutorialFilter => self.toggle_tutorial_filter(),
            crate::ui::UiAction::Favorites => self.open_records_view(true, false),
            crate::ui::UiAction::Recent => self.open_records_view(false, true),
            _ => return false,
        }
        true
    }

    fn open_records_view(&mut self, favorites: bool, recent: bool) {
        self.state.screen = Screen::Records;
        self.state.library_scroll = 0;
        self.state.favorites_view = favorites;
        self.state.recent_view = recent;
        self.state.daily_archive_view = false;
        self.state.achievements_view = false;
    }

    pub(super) fn open_statistics(&mut self) {
        self.state.screen = Screen::Statistics;
        self.state.library_scroll = 0;
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.daily_archive_view = false;
        self.state.achievements_view = false;
    }

    pub(super) fn open_tutorial_library(&mut self) {
        self.state.screen = Screen::Tutorials;
        self.state.library_scroll = 0;
        self.state.tutorial_filter = false;
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.daily_archive_view = false;
        self.state.achievements_view = false;
    }

    pub(super) fn open_tutorial(&mut self, index: usize) {
        let Some(game) = GameId::ALL.get(index).copied() else {
            return;
        };
        let before = self.state.screen;
        self.open_game(index);
        if self.state.screen != before && self.state.screen == Screen::Game(game) {
            self.state.tutorial = Some(game);
        }
    }

    fn toggle_tutorial_filter(&mut self) {
        self.state.tutorial_filter = !self.state.tutorial_filter;
        self.state.library_scroll = 0;
    }

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
        matches!(
            self.state.screen,
            Screen::Records | Screen::Rules | Screen::Tutorials
        ) && !self.state.daily_archive_view
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
        if self.state.screen == Screen::Records && self.state.achievements_view {
            let capacity = if crate::ui::is_portrait() {
                8
            } else if crate::ui::is_compact_landscape() {
                10
            } else {
                crate::achievements_data::filter_count(&self.state, self.state.achievement_filter)
            };
            return crate::achievements_data::scroll_limit(
                &self.state,
                self.state.achievement_filter,
                capacity,
            );
        }
        if self.state.screen == Screen::Tutorials {
            return crate::tutorial_library_data::scroll_limit(&self.state);
        }
        GameId::ALL.len().saturating_sub(1)
    }

    pub(super) fn toggle_favorite(&mut self, index: usize) {
        let Some(game) = GameId::ALL.get(index).copied() else {
            return;
        };
        let Some(favorite) = self.state.favorites.get_mut(index) else {
            return;
        };
        *favorite = !*favorite;
        let notice = if *favorite {
            format!("{} added to favorites", game.title())
        } else {
            format!("{} removed from favorites", game.title())
        };
        self.notifications.info(notice);
    }

    pub(super) fn set_achievement_filter(&mut self, filter: u8) {
        let filter = crate::achievements_data::normalize_filter(filter);
        self.state.achievement_filter = filter;
        self.state.library_scroll = 0;
        self.notifications.info(format!(
            "Achievement shelf: {}",
            crate::achievements_data::filter_label(filter)
        ));
    }

    pub(super) fn open_game(&mut self, index: usize) {
        let Some(id) = GameId::ALL.get(index).copied() else {
            return;
        };
        let from_rules = matches!(self.state.screen, Screen::Rules);
        match crate::storefront::availability(id) {
            crate::storefront::GameAvailability::Playable => {
                self.state.selected = index;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
                self.state.recent_games.retain(|recent| *recent != id);
                self.state.recent_games.insert(0, id);
                self.state.recent_games.truncate(5);
                self.state.screen = Screen::Game(id);
                if from_rules {
                    self.notifications.info(format!("Opening {}", id.title()));
                }
                self.state.tutorial = (!self.state.tutorial_seen[id.index()]).then_some(id);
            }
            crate::storefront::GameAvailability::DemoRestricted => {
                self.notifications
                    .info(crate::storefront::availability_message(
                        id.title(),
                        crate::storefront::GameAvailability::DemoRestricted,
                    ));
            }
            crate::storefront::GameAvailability::ComingSoon => {
                self.notifications
                    .info(crate::storefront::availability_message(
                        id.title(),
                        crate::storefront::GameAvailability::ComingSoon,
                    ));
            }
        }
    }

    pub(super) fn open_archived_daily_day(&mut self, day: u64) {
        let id = GameId::DailyDungeon;
        match crate::storefront::availability(id) {
            crate::storefront::GameAvailability::Playable => {}
            crate::storefront::GameAvailability::DemoRestricted => {
                self.notifications
                    .info(crate::storefront::availability_message(
                        id.title(),
                        crate::storefront::GameAvailability::DemoRestricted,
                    ));
                return;
            }
            crate::storefront::GameAvailability::ComingSoon => {
                self.notifications
                    .info(crate::storefront::availability_message(
                        id.title(),
                        crate::storefront::GameAvailability::ComingSoon,
                    ));
                return;
            }
        }
        self.state.games.daily_dungeon = crate::daily_dungeon::DailyDungeon::new_for_day(day);
        self.state.favorites_view = false;
        self.state.recent_view = false;
        self.state.daily_archive_view = false;
        self.state.achievements_view = false;
        self.state.selected = id.index();
        self.state.recent_games.retain(|recent| *recent != id);
        self.state.recent_games.insert(0, id);
        self.state.recent_games.truncate(5);
        self.state.screen = Screen::Game(id);
        self.state.tutorial = None;
        self.notifications.info(format!(
            "Opening {}",
            crate::daily_challenge::label(day, crate::daily_challenge::challenge_for_day(day))
        ));
    }
}
