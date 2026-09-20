//! Frame update orchestration for the cabinet runtime.

use super::Game;
use crate::domain::Direction;
use crate::game_input::{card_drag_actions, swipe_direction};
use crate::input::{Gesture, PointerScope};
use crate::state::{GameId, Screen};
use crate::{minesweeper_ui, nonogram_ui, responsive_landscape_games, responsive_puzzles, ui};
use macroquad::prelude::*;

impl Game {
    pub fn update(&mut self, dt: f32) {
        self.pointer
            .sync_scope(PointerScope::from_state(&self.state));
        if crate::input::should_cancel_for_touch_count(touches().len()) {
            self.pointer.cancel();
        }
        if is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Left)
            || !get_keys_pressed().is_empty()
        {
            self.analytics.note_input();
        }
        self.notifications.update(dt);
        self.pointer.tick(dt);
        self.update_card_peek();
        if self.state.reduced_motion {
            self.transition = 0.;
        } else {
            self.transition = (self.transition - dt * 3.5).max(0.);
        }
        if !self.state.lifecycle_paused {
            self.state.games.minesweeper.tick(dt);
        }
        if !self.state.lifecycle_paused
            && self.state.games.minesweeper.status == crate::minesweeper::MineStatus::Won
        {
            let slot = self.state.games.minesweeper.preset.index();
            let time = self.state.games.minesweeper.elapsed_whole_seconds();
            self.state.mine_records[slot] =
                Some(self.state.mine_records[slot].map_or(time, |best| best.min(time)));
            self.state.records.minesweeper[slot] = self.state.mine_records[slot];
        }
        self.tick_realtime(dt);
        self.tick_elapsed(dt);
        self.tick_autosave(dt);
        if is_mouse_button_pressed(MouseButton::Left) {
            let viewport = ui::viewport();
            self.pointer.press(
                viewport.screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1)),
                PointerScope::from_state(&self.state),
            );
        }
        if is_mouse_button_released(MouseButton::Left) {
            let viewport = ui::viewport();
            let position =
                viewport.screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1));
            if let Some(gesture) = self
                .pointer
                .release(position, PointerScope::from_state(&self.state))
            {
                match gesture {
                    Gesture::Drag { start, end }
                        if self.state.screen == Screen::Game(GameId::Game2048)
                            && (end - start).length() > 32.0 =>
                    {
                        self.try_move(swipe_direction(end - start))
                    }
                    Gesture::Drag { start, end }
                        if self.state.screen == Screen::Game(GameId::Nonogram) =>
                    {
                        let actions = if ui::is_compact_landscape() {
                            responsive_landscape_games::nonogram_drag_actions(
                                &self.state,
                                start,
                                end,
                            )
                        } else if ui::is_portrait() {
                            responsive_puzzles::nonogram_drag_actions(&self.state, start, end)
                        } else {
                            nonogram_ui::drag_actions(&self.state, start, end)
                        };
                        for action in actions {
                            self.apply(action);
                        }
                    }
                    Gesture::Drag { start, end }
                        if matches!(
                            self.state.screen,
                            Screen::Game(GameId::Solitaire | GameId::FreeCell)
                        ) =>
                    {
                        for action in card_drag_actions(
                            &self.state,
                            start,
                            end,
                            ui::is_portrait(),
                            ui::is_compact_landscape(),
                        ) {
                            self.apply(action);
                        }
                    }
                    Gesture::Tap(position) => {
                        for action in ui::clicks_at(&self.state, position) {
                            self.apply(action);
                        }
                    }
                    Gesture::LongPress(position)
                        if self.state.screen == Screen::Game(GameId::Minesweeper) =>
                    {
                        let actions = if ui::is_compact_landscape() {
                            responsive_landscape_games::minesweeper_long_press(
                                &self.state,
                                position,
                            )
                        } else if ui::is_portrait() {
                            responsive_puzzles::minesweeper_long_press(&self.state, position)
                        } else {
                            minesweeper_ui::long_press(&self.state, position)
                        };
                        for action in actions {
                            self.apply(action);
                        }
                    }
                    Gesture::Drag { start, end } => self.apply_navigation_drag(start, end),
                    Gesture::LongPress(_) => {}
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) && !self.state.lifecycle_paused {
            self.pointer.cancel();
            self.state.screen = Screen::Cabinet;
            self.state.favorites_view = false;
            self.state.recent_view = false;
            self.state.daily_archive_view = false;
            self.state.achievements_view = false;
            self.state.confirm_restart = false;
            self.state.pending_restart = None;
            self.state.game_setup_open = false;
            self.state.sudoku_focus_open = false;
            self.state.nonogram_focus_open = false;
            self.state.confirm_reset = false;
            self.state.tutorial = None;
            if !self.state.reduced_motion {
                self.transition = 1.;
            }
        }
        if self.state.screen.game() == Some(GameId::Game2048) {
            for (key, direction) in [
                (KeyCode::Up, Direction::Up),
                (KeyCode::Right, Direction::Right),
                (KeyCode::Down, Direction::Down),
                (KeyCode::Left, Direction::Left),
            ] {
                if is_key_pressed(key) {
                    self.try_move(direction);
                }
            }
        }
        self.update_navigation_scroll();
        self.analytics.update(dt, &self.state);
    }
}
