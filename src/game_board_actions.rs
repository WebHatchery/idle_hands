//! Small action handlers for the touch-first board-game cabinets.

use super::Game;
use crate::{sound::SoundCue, ui::UiAction};

impl Game {
    pub(super) fn apply_board_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::HangmanGuess(letter) => {
                self.state.hangman.guess(*letter);
            }
            UiAction::HangmanNew => {
                let seed = self.state.hangman.seed.wrapping_add(1);
                self.state.hangman.reset(seed);
            }
            UiAction::ConnectFourDrop(column) => self.apply_connect_four_drop(*column),
            UiAction::ConnectFourUndo => self.apply_connect_four_undo(),
            UiAction::ConnectFourNew => self.apply_connect_four_new(),
            UiAction::CheckersTap(square) => self.apply_checkers_tap(*square),
            UiAction::CheckersUndo => self.apply_checkers_undo(),
            UiAction::CheckersNew => self.apply_checkers_new(),
            UiAction::PegSolitaireTap(square) => {
                self.state.peg_solitaire.tap(*square);
            }
            UiAction::PegSolitaireUndo => {
                self.state.peg_solitaire.undo();
            }
            UiAction::PegSolitaireNew => {
                let seed = self.state.peg_solitaire.seed.wrapping_add(1);
                self.state.peg_solitaire.reset(seed);
            }
            UiAction::MahjongSolitaireTap(index) => {
                self.state.mahjong_solitaire.tap(*index);
            }
            UiAction::MahjongSolitaireUndo => {
                self.state.mahjong_solitaire.undo();
            }
            UiAction::MahjongSolitaireNew => {
                let seed = self.state.mahjong_solitaire.seed.wrapping_add(1);
                self.state.mahjong_solitaire.reset(seed);
            }
            UiAction::SnakeStep(direction) => {
                self.state.snake.step(*direction);
            }
            UiAction::SnakeUndo => {
                self.state.snake.undo();
            }
            UiAction::SnakeNew => {
                let seed = self.state.snake.seed.wrapping_add(1);
                self.state.snake.reset(seed);
            }
            UiAction::BreakoutStep(movement) => {
                self.state.breakout.step(*movement);
            }
            UiAction::BreakoutUndo => {
                self.state.breakout.undo();
            }
            UiAction::BreakoutNew => {
                let seed = self.state.breakout.seed.wrapping_add(1);
                self.state.breakout.reset(seed);
            }
            _ => return false,
        }
        true
    }

    pub(super) fn finish_action(
        &mut self,
        previous_screen: crate::state::Screen,
        action: UiAction,
    ) {
        if self.state.screen != previous_screen {
            self.transition = if self.state.reduced_motion { 0. } else { 1. };
        }
        self.update_records();
        self.save_autosave();
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
        self.state.connect_four.drop(column);
    }

    pub(super) fn apply_connect_four_undo(&mut self) {
        self.state.connect_four.undo();
    }

    pub(super) fn apply_connect_four_new(&mut self) {
        let seed = self.state.connect_four.seed.wrapping_add(1);
        self.state.connect_four.reset(seed);
    }

    pub(super) fn apply_checkers_tap(&mut self, square: usize) {
        self.state.checkers.tap(square);
    }

    pub(super) fn apply_checkers_undo(&mut self) {
        self.state.checkers.undo();
    }

    pub(super) fn apply_checkers_new(&mut self) {
        let seed = self.state.checkers.seed.wrapping_add(1);
        self.state.checkers.reset(seed);
    }
}
