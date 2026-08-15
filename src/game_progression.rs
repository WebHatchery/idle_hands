//! Record, feedback, and movement helpers for the game host.

use super::Game;
use crate::{progression, sound::SoundCue, state::Direction};

impl Game {
    pub(super) fn play_feedback(&self, cue: SoundCue) {
        if self.state.sound {
            self.sounds.play(self.state.sound_set, cue);
        }
    }

    pub(super) fn update_records(&mut self) {
        let records = &mut self.state.records;
        records.best_2048 = records.best_2048.max(self.state.game.best);
        let sudoku_index = match self.state.sudoku.difficulty {
            crate::sudoku::SudokuDifficulty::Easy => 0,
            crate::sudoku::SudokuDifficulty::Medium => 1,
            crate::sudoku::SudokuDifficulty::Hard => 2,
        };
        if self.state.sudoku.status == crate::sudoku::SudokuStatus::Won {
            if let Some(moves) = self.state.sudoku.best_moves {
                records.sudoku[sudoku_index] =
                    Some(records.sudoku[sudoku_index].map_or(moves, |best| best.min(moves)));
            }
        }
        let nonogram_index = match self.state.nonogram.preset {
            crate::nonogram::NonogramPreset::Small => 0,
            crate::nonogram::NonogramPreset::Medium => 1,
            crate::nonogram::NonogramPreset::Large => 2,
        };
        if self.state.nonogram.status == crate::nonogram::NonogramStatus::Won {
            if let Some(moves) = self.state.nonogram.best_moves {
                records.nonogram[nonogram_index] =
                    Some(records.nonogram[nonogram_index].map_or(moves, |best| best.min(moves)));
            }
        }
        if self.state.solitaire.status == crate::solitaire::SolitaireStatus::Won {
            records.solitaire_best_moves = Some(
                records
                    .solitaire_best_moves
                    .map_or(self.state.solitaire.moves, |best| {
                        best.min(self.state.solitaire.moves)
                    }),
            );
        }
        if self.state.freecell.status == crate::freecell::FreeCellStatus::Won {
            records.freecell_best_moves = Some(
                records
                    .freecell_best_moves
                    .map_or(self.state.freecell.moves, |best| {
                        best.min(self.state.freecell.moves)
                    }),
            );
        }
        if self.state.fivefold.status == crate::fivefold::FivefoldStatus::Complete {
            records.fivefold_best_total =
                records.fivefold_best_total.max(self.state.fivefold.total());
        }
        if self.state.reversi.status == crate::reversi::ReversiStatus::Won {
            records.reversi_best_score = records
                .reversi_best_score
                .max(self.state.reversi.score(1) as u8);
        }
        if self.state.lights_out.status == crate::lights_out::LightsOutStatus::Won {
            records.lights_out_best_moves = Some(
                records
                    .lights_out_best_moves
                    .map_or(self.state.lights_out.moves, |best| {
                        best.min(self.state.lights_out.moves)
                    }),
            );
        }
        if self.state.tic_tac_toe.status
            == crate::tic_tac_toe::TicTacToeStatus::Won(crate::tic_tac_toe::Mark::X)
        {
            records.tic_tac_toe_best_moves = Some(
                records
                    .tic_tac_toe_best_moves
                    .map_or(self.state.tic_tac_toe.moves, |best| {
                        best.min(self.state.tic_tac_toe.moves)
                    }),
            );
        }
        if self.state.memory_pairs.status == crate::memory_pairs::MemoryStatus::Won {
            records.memory_pairs_best_moves = Some(
                records
                    .memory_pairs_best_moves
                    .map_or(self.state.memory_pairs.moves, |best| {
                        best.min(self.state.memory_pairs.moves)
                    }),
            );
        }
        if self.state.sliding_puzzle.status == crate::sliding_puzzle::SlidingStatus::Won {
            records.sliding_puzzle_best_moves = Some(
                records
                    .sliding_puzzle_best_moves
                    .map_or(self.state.sliding_puzzle.moves, |best| {
                        best.min(self.state.sliding_puzzle.moves)
                    }),
            );
        }
        if self.state.mastermind.status == crate::mastermind::MastermindStatus::Won {
            records.mastermind_best_rows = Some(
                records
                    .mastermind_best_rows
                    .map_or(self.state.mastermind.row, |best| {
                        best.min(self.state.mastermind.row)
                    }),
            );
        }
        if self.state.spider.status == crate::spider::SpiderStatus::Won {
            records.spider_best_moves = Some(
                records
                    .spider_best_moves
                    .map_or(self.state.spider.moves, |best| {
                        best.min(self.state.spider.moves)
                    }),
            );
        }
        let previous_stamps = self.state.stamps;
        let newly_earned = progression::sync(
            &mut self.state.achievements,
            &mut self.state.stamps,
            records,
        );
        if self.state.stamps > previous_stamps {
            if let Some(achievement) = newly_earned.first() {
                self.notifications.success(format!(
                    "{} — {} stamps",
                    achievement.title(),
                    self.state.stamps
                ));
            }
        }
    }

    pub(super) fn try_move(&mut self, direction: Direction) {
        if self.state.game.move_in(direction) && self.state.game.won() {
            self.notifications
                .success("2048 reached — keep playing or start a fresh board");
        }
        self.save_autosave();
    }
}
