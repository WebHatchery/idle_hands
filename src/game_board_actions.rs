//! Small action handlers for the touch-first board-game cabinets.

use super::Game;
use crate::{sound::SoundCue, ui::UiAction};
use macroquad::prelude::get_time;

pub(super) fn fresh_seed(previous: u64) -> u64 {
    get_time().to_bits() ^ previous.rotate_left(17)
}

impl Game {
    pub(super) fn active_misc_game_mut(&mut self) -> Option<&mut crate::misc_games::MiscGame> {
        match self.state.screen {
            crate::state::Screen::Game(crate::state::GameId::RiddleRoom) => {
                Some(&mut self.state.games.riddle_room)
            }
            crate::state::Screen::Game(crate::state::GameId::PatternVault) => {
                Some(&mut self.state.games.pattern_vault)
            }
            crate::state::Screen::Game(crate::state::GameId::SumCircuit) => {
                Some(&mut self.state.games.sum_circuit)
            }
            crate::state::Screen::Game(crate::state::GameId::OrbitOrder) => {
                Some(&mut self.state.games.orbit_order)
            }
            crate::state::Screen::Game(crate::state::GameId::WordForge) => {
                Some(&mut self.state.games.word_forge)
            }
            _ => None,
        }
    }

    pub(super) fn finish_action(
        &mut self,
        previous_screen: crate::state::Screen,
        action: UiAction,
    ) {
        if crate::card_hints::is_hint(action) {
            if let crate::state::Screen::Game(game) = self.state.screen {
                if super::game_progression::round_is_complete(&self.state, game) {
                    self.state.card_hint =
                        Some(crate::card_hints::authored_copy(&self.state, game, true));
                }
            }
        }
        if action.starts_new_round() {
            self.reset_elapsed();
            self.state.game_setup_open = false;
        }
        if self.state.screen != previous_screen {
            self.transition = if self.state.reduced_motion { 0. } else { 1. };
        }
        if self.state.screen != crate::state::Screen::Settings {
            self.state.notice_log_view = false;
        }
        self.update_records();
        self.request_autosave();
        self.play_feedback(
            if matches!(
                action,
                UiAction::ConfirmRestart | UiAction::ConfirmResetData
            ) {
                SoundCue::Success
            } else {
                SoundCue::Tap
            },
        );
    }

    pub(super) fn apply_connect_four_drop(&mut self, column: usize) {
        self.state.games.connect_four.drop(column);
    }

    pub(super) fn apply_connect_four_undo(&mut self) {
        self.state.games.connect_four.undo();
    }

    pub(super) fn apply_connect_four_new(&mut self) {
        let seed = self.state.games.connect_four.seed.wrapping_add(1);
        self.state.games.connect_four.reset(seed);
    }

    pub(super) fn apply_checkers_tap(&mut self, square: usize) {
        self.state.games.checkers.tap(square);
    }

    pub(super) fn apply_checkers_undo(&mut self) {
        self.state.games.checkers.undo();
    }

    pub(super) fn apply_checkers_new(&mut self) {
        let seed = self.state.games.checkers.seed.wrapping_add(1);
        self.state.games.checkers.reset(seed);
    }
}
