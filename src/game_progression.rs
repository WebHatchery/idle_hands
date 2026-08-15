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
        if self.state.word_search.status == crate::word_search::WordSearchStatus::Won {
            records.word_search_best_moves = Some(
                records
                    .word_search_best_moves
                    .map_or(self.state.word_search.moves, |best| {
                        best.min(self.state.word_search.moves)
                    }),
            );
        }
        if self.state.hangman.status == crate::hangman::HangmanStatus::Won {
            records.hangman_best_moves = Some(
                records
                    .hangman_best_moves
                    .map_or(self.state.hangman.moves, |best| {
                        best.min(self.state.hangman.moves)
                    }),
            );
        }
        if self.state.connect_four.status
            == crate::connect_four::ConnectFourStatus::Won(crate::connect_four::Disc::Red)
        {
            records.connect_four_best_moves = Some(
                records
                    .connect_four_best_moves
                    .map_or(self.state.connect_four.moves, |best| {
                        best.min(self.state.connect_four.moves)
                    }),
            );
        }
        if self.state.checkers.status
            == crate::checkers::CheckersStatus::Won(crate::checkers::Side::Red)
        {
            records.checkers_best_moves = Some(
                records
                    .checkers_best_moves
                    .map_or(self.state.checkers.moves, |best| {
                        best.min(self.state.checkers.moves)
                    }),
            );
        }
        if self.state.peg_solitaire.status == crate::peg_solitaire::PegSolitaireStatus::Won {
            records.peg_solitaire_best_moves = Some(
                records
                    .peg_solitaire_best_moves
                    .map_or(self.state.peg_solitaire.moves, |best| {
                        best.min(self.state.peg_solitaire.moves)
                    }),
            );
        }
        if self.state.mahjong_solitaire.status == crate::mahjong_solitaire::MahjongStatus::Won {
            records.mahjong_solitaire_best_moves = Some(
                records
                    .mahjong_solitaire_best_moves
                    .map_or(self.state.mahjong_solitaire.moves, |best| {
                        best.min(self.state.mahjong_solitaire.moves)
                    }),
            );
        }
        if self.state.snake.status == crate::snake::SnakeStatus::Won {
            records.snake_best_score = Some(
                records
                    .snake_best_score
                    .map_or(self.state.snake.score, |best| {
                        best.max(self.state.snake.score)
                    }),
            );
        }
        if self.state.breakout.status == crate::breakout::BreakoutStatus::Won {
            records.breakout_best_score = Some(
                records
                    .breakout_best_score
                    .map_or(self.state.breakout.score, |best| {
                        best.max(self.state.breakout.score)
                    }),
            );
        }
        if self.state.higher_lower.status == crate::higher_lower::HigherLowerStatus::Won {
            records.higher_lower_best_score = Some(
                records
                    .higher_lower_best_score
                    .map_or(self.state.higher_lower.score, |best| {
                        best.max(self.state.higher_lower.score)
                    }),
            );
        }
        if self.state.klondike_golf.status == crate::klondike_golf::GolfStatus::Won {
            records.klondike_golf_best_moves = Some(
                records
                    .klondike_golf_best_moves
                    .map_or(self.state.klondike_golf.moves, |best| {
                        best.min(self.state.klondike_golf.moves)
                    }),
            );
        }
        if self.state.blackjack.status == crate::blackjack::BlackjackStatus::Won {
            records.blackjack_best_wins = Some(
                records
                    .blackjack_best_wins
                    .map_or(self.state.blackjack.wins, |best| {
                        best.max(self.state.blackjack.wins)
                    }),
            );
        }
        if self.state.spider_solitaire.status == crate::spider_solitaire::SpiderSolitaireStatus::Won
        {
            records.spider_solitaire_best_moves = Some(
                records
                    .spider_solitaire_best_moves
                    .map_or(self.state.spider_solitaire.moves, |best| {
                        best.min(self.state.spider_solitaire.moves)
                    }),
            );
        }
        if self.state.dungeon_sweeper.status == crate::dungeon_sweeper::DungeonStatus::Won {
            records.dungeon_sweeper_best_moves = Some(
                records
                    .dungeon_sweeper_best_moves
                    .map_or(self.state.dungeon_sweeper.moves, |best| {
                        best.min(self.state.dungeon_sweeper.moves)
                    }),
            );
        }
        if self.state.potion_2048.won() {
            records.potion_2048_best_score = Some(
                records
                    .potion_2048_best_score
                    .map_or(self.state.potion_2048.best, |best| {
                        best.max(self.state.potion_2048.best)
                    }),
            );
        }
        if self.state.tiny_tower_defence.won() {
            records.tiny_tower_defence_best_wave = Some(
                records
                    .tiny_tower_defence_best_wave
                    .map_or(self.state.tiny_tower_defence.wave, |best| {
                        best.max(self.state.tiny_tower_defence.wave)
                    }),
            );
        }
        if self.state.one_room_roguelike.won() {
            records.one_room_roguelike_best_score = Some(
                records
                    .one_room_roguelike_best_score
                    .map_or(self.state.one_room_roguelike.score, |best| {
                        best.max(self.state.one_room_roguelike.score)
                    }),
            );
        }
        if self.state.daily_dungeon.won() {
            records.daily_dungeon_best_score = Some(
                records
                    .daily_dungeon_best_score
                    .map_or(self.state.daily_dungeon.score, |best| {
                        best.max(self.state.daily_dungeon.score)
                    }),
            );
        }
        if self.state.dots_boxes.won() {
            records.dots_boxes_best_score = Some(
                records
                    .dots_boxes_best_score
                    .map_or(self.state.dots_boxes.scores[0], |best| {
                        best.max(self.state.dots_boxes.scores[0])
                    }),
            );
        }
        if self.state.sokoban.won() {
            records.sokoban_best_moves = Some(
                records
                    .sokoban_best_moves
                    .map_or(self.state.sokoban.moves, |best| {
                        best.min(self.state.sokoban.moves)
                    }),
            );
        }
        if self.state.mancala.won() {
            records.mancala_best_score = Some(
                records
                    .mancala_best_score
                    .map_or(self.state.mancala.pits[6], |best| {
                        best.max(self.state.mancala.pits[6])
                    }),
            );
        }
        if self.state.hanoi.won() {
            records.hanoi_best_moves = Some(
                records
                    .hanoi_best_moves
                    .map_or(self.state.hanoi.moves, |best| {
                        best.min(self.state.hanoi.moves)
                    }),
            );
        }
        if self.state.number_match.won() {
            records.number_match_best_moves = Some(
                records
                    .number_match_best_moves
                    .map_or(self.state.number_match.moves, |best| {
                        best.min(self.state.number_match.moves)
                    }),
            );
        }
        if self.state.flood_it.won() {
            records.flood_it_best_moves = Some(
                records
                    .flood_it_best_moves
                    .map_or(self.state.flood_it.moves, |best| {
                        best.min(self.state.flood_it.moves)
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
