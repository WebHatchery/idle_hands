//! Record, feedback, and movement helpers for the game host.

use super::Game;
use crate::domain::Direction;
use crate::{
    progression,
    sound::SoundCue,
    state::{GameId, Screen},
};

impl Game {
    pub(super) fn tick_elapsed(&mut self, dt: f32) {
        self.state.records.ensure_time_slots();
        let Screen::Game(game) = self.state.screen else {
            return;
        };
        if !is_timed_game(game)
            || self.state.tutorial.is_some()
            || self.state.confirm_restart
            || self.state.confirm_reset
            || round_is_complete(&self.state, game)
        {
            return;
        }
        self.state.records.elapsed_remainder += dt.max(0.0);
        let whole_seconds = self.state.records.elapsed_remainder.floor() as u32;
        if whole_seconds > 0 {
            self.state.records.elapsed_remainder -= whole_seconds as f32;
            self.state.records.elapsed_seconds[game.index()] =
                self.state.records.elapsed_seconds[game.index()].saturating_add(whole_seconds);
        }
    }

    pub(super) fn reset_elapsed(&mut self) {
        if let Screen::Game(game) = self.state.screen {
            self.state.records.reset_time(game.index());
            self.state.records.elapsed_remainder = 0.0;
        }
    }

    pub(super) fn play_feedback(&self, cue: SoundCue) {
        if self.state.sound {
            self.sounds.play(self.state.sound_set, cue);
        }
    }

    pub(super) fn update_records(&mut self) {
        if let Screen::Game(game) = self.state.screen {
            if round_is_complete(&self.state, game) {
                self.state.records.record_time(game.index());
            }
        }
        let records = &mut self.state.records;
        records.best_2048 = records.best_2048.max(self.state.games.game.best);
        let sudoku_index = match self.state.games.sudoku.difficulty {
            crate::sudoku::SudokuDifficulty::Easy => 0,
            crate::sudoku::SudokuDifficulty::Medium => 1,
            crate::sudoku::SudokuDifficulty::Hard => 2,
        };
        if self.state.games.sudoku.status == crate::sudoku::SudokuStatus::Won {
            if let Some(moves) = self.state.games.sudoku.best_moves {
                records.sudoku[sudoku_index] =
                    Some(records.sudoku[sudoku_index].map_or(moves, |best| best.min(moves)));
            }
        }
        let nonogram_index = match self.state.games.nonogram.preset {
            crate::nonogram::NonogramPreset::Small => 0,
            crate::nonogram::NonogramPreset::Medium => 1,
            crate::nonogram::NonogramPreset::Large => 2,
        };
        if self.state.games.nonogram.status == crate::nonogram::NonogramStatus::Won {
            if let Some(moves) = self.state.games.nonogram.best_moves {
                records.nonogram[nonogram_index] =
                    Some(records.nonogram[nonogram_index].map_or(moves, |best| best.min(moves)));
            }
        }
        if self.state.games.solitaire.status == crate::solitaire::SolitaireStatus::Won {
            records.solitaire_best_moves = Some(
                records
                    .solitaire_best_moves
                    .map_or(self.state.games.solitaire.moves, |best| {
                        best.min(self.state.games.solitaire.moves)
                    }),
            );
        }
        if self.state.games.freecell.status == crate::freecell::FreeCellStatus::Won {
            records.freecell_best_moves = Some(
                records
                    .freecell_best_moves
                    .map_or(self.state.games.freecell.moves, |best| {
                        best.min(self.state.games.freecell.moves)
                    }),
            );
        }
        if self.state.games.fivefold.status == crate::fivefold::FivefoldStatus::Complete {
            records.fivefold_best_total = records
                .fivefold_best_total
                .max(self.state.games.fivefold.total());
        }
        if self.state.games.reversi.status == crate::reversi::ReversiStatus::Won
            && self.state.games.reversi.winner == Some(1)
        {
            records.reversi_best_score = records
                .reversi_best_score
                .max(self.state.games.reversi.score(1) as u8);
        }
        if self.state.games.lights_out.status == crate::lights_out::LightsOutStatus::Won {
            records.lights_out_best_moves = Some(
                records
                    .lights_out_best_moves
                    .map_or(self.state.games.lights_out.moves, |best| {
                        best.min(self.state.games.lights_out.moves)
                    }),
            );
        }
        if self.state.games.tic_tac_toe.status
            == crate::tic_tac_toe::TicTacToeStatus::Won(crate::tic_tac_toe::Mark::X)
        {
            records.tic_tac_toe_best_moves = Some(
                records
                    .tic_tac_toe_best_moves
                    .map_or(self.state.games.tic_tac_toe.moves, |best| {
                        best.min(self.state.games.tic_tac_toe.moves)
                    }),
            );
        }
        if self.state.games.memory_pairs.status == crate::memory_pairs::MemoryStatus::Won {
            records.memory_pairs_best_moves = Some(
                records
                    .memory_pairs_best_moves
                    .map_or(self.state.games.memory_pairs.moves, |best| {
                        best.min(self.state.games.memory_pairs.moves)
                    }),
            );
        }
        if self.state.games.sliding_puzzle.status == crate::sliding_puzzle::SlidingStatus::Won {
            records.sliding_puzzle_best_moves = Some(
                records
                    .sliding_puzzle_best_moves
                    .map_or(self.state.games.sliding_puzzle.moves, |best| {
                        best.min(self.state.games.sliding_puzzle.moves)
                    }),
            );
        }
        if self.state.games.mastermind.status == crate::mastermind::MastermindStatus::Won {
            records.mastermind_best_rows = Some(
                records
                    .mastermind_best_rows
                    .map_or(self.state.games.mastermind.row, |best| {
                        best.min(self.state.games.mastermind.row)
                    }),
            );
        }
        if self.state.games.spider.status == crate::spider::SpiderStatus::Won {
            records.spider_best_moves = Some(
                records
                    .spider_best_moves
                    .map_or(self.state.games.spider.moves, |best| {
                        best.min(self.state.games.spider.moves)
                    }),
            );
        }
        if self.state.games.word_search.status == crate::word_search::WordSearchStatus::Won {
            records.word_search_best_moves = Some(
                records
                    .word_search_best_moves
                    .map_or(self.state.games.word_search.moves, |best| {
                        best.min(self.state.games.word_search.moves)
                    }),
            );
        }
        if self.state.games.hangman.status == crate::hangman::HangmanStatus::Won {
            records.hangman_best_moves = Some(
                records
                    .hangman_best_moves
                    .map_or(self.state.games.hangman.moves, |best| {
                        best.min(self.state.games.hangman.moves)
                    }),
            );
        }
        if self.state.games.connect_four.status
            == crate::connect_four::ConnectFourStatus::Won(crate::connect_four::Disc::Red)
        {
            records.connect_four_best_moves = Some(
                records
                    .connect_four_best_moves
                    .map_or(self.state.games.connect_four.moves, |best| {
                        best.min(self.state.games.connect_four.moves)
                    }),
            );
        }
        if self.state.games.checkers.status
            == crate::checkers::CheckersStatus::Won(crate::checkers::Side::Red)
        {
            records.checkers_best_moves = Some(
                records
                    .checkers_best_moves
                    .map_or(self.state.games.checkers.moves, |best| {
                        best.min(self.state.games.checkers.moves)
                    }),
            );
        }
        if self.state.games.peg_solitaire.status == crate::peg_solitaire::PegSolitaireStatus::Won {
            records.peg_solitaire_best_moves = Some(
                records
                    .peg_solitaire_best_moves
                    .map_or(self.state.games.peg_solitaire.moves, |best| {
                        best.min(self.state.games.peg_solitaire.moves)
                    }),
            );
        }
        if self.state.games.mahjong_solitaire.status == crate::mahjong_solitaire::MahjongStatus::Won
        {
            records.mahjong_solitaire_best_moves = Some(
                records
                    .mahjong_solitaire_best_moves
                    .map_or(self.state.games.mahjong_solitaire.moves, |best| {
                        best.min(self.state.games.mahjong_solitaire.moves)
                    }),
            );
        }
        if self.state.games.snake.status == crate::snake::SnakeStatus::Won {
            records.snake_best_score = Some(
                records
                    .snake_best_score
                    .map_or(self.state.games.snake.score, |best| {
                        best.max(self.state.games.snake.score)
                    }),
            );
        }
        if self.state.games.breakout.status == crate::breakout::BreakoutStatus::Won {
            records.breakout_best_score = Some(
                records
                    .breakout_best_score
                    .map_or(self.state.games.breakout.score, |best| {
                        best.max(self.state.games.breakout.score)
                    }),
            );
        }
        if self.state.games.higher_lower.status == crate::higher_lower::HigherLowerStatus::Won {
            records.higher_lower_best_score = Some(
                records
                    .higher_lower_best_score
                    .map_or(self.state.games.higher_lower.score, |best| {
                        best.max(self.state.games.higher_lower.score)
                    }),
            );
        }
        if self.state.games.klondike_golf.status == crate::klondike_golf::GolfStatus::Won {
            records.klondike_golf_best_moves = Some(
                records
                    .klondike_golf_best_moves
                    .map_or(self.state.games.klondike_golf.moves, |best| {
                        best.min(self.state.games.klondike_golf.moves)
                    }),
            );
        }
        if self.state.games.blackjack.status == crate::blackjack::BlackjackStatus::Won {
            records.blackjack_best_wins = Some(
                records
                    .blackjack_best_wins
                    .map_or(self.state.games.blackjack.wins, |best| {
                        best.max(self.state.games.blackjack.wins)
                    }),
            );
        }
        if self.state.games.spider_solitaire.status
            == crate::spider_solitaire::SpiderSolitaireStatus::Won
        {
            records.spider_solitaire_best_moves = Some(
                records
                    .spider_solitaire_best_moves
                    .map_or(self.state.games.spider_solitaire.moves, |best| {
                        best.min(self.state.games.spider_solitaire.moves)
                    }),
            );
        }
        if self.state.games.dungeon_sweeper.status == crate::dungeon_sweeper::DungeonStatus::Won {
            records.dungeon_sweeper_best_moves = Some(
                records
                    .dungeon_sweeper_best_moves
                    .map_or(self.state.games.dungeon_sweeper.moves, |best| {
                        best.min(self.state.games.dungeon_sweeper.moves)
                    }),
            );
        }
        if self.state.games.potion_2048.won() {
            records.potion_2048_best_score = Some(
                records
                    .potion_2048_best_score
                    .map_or(self.state.games.potion_2048.best, |best| {
                        best.max(self.state.games.potion_2048.best)
                    }),
            );
        }
        if self.state.games.tiny_tower_defence.won() {
            records.tiny_tower_defence_best_wave = Some(
                records
                    .tiny_tower_defence_best_wave
                    .map_or(self.state.games.tiny_tower_defence.wave, |best| {
                        best.max(self.state.games.tiny_tower_defence.wave)
                    }),
            );
        }
        if self.state.games.one_room_roguelike.won() {
            records.one_room_roguelike_best_score = Some(
                records
                    .one_room_roguelike_best_score
                    .map_or(self.state.games.one_room_roguelike.score, |best| {
                        best.max(self.state.games.one_room_roguelike.score)
                    }),
            );
        }
        if self.state.games.daily_dungeon.won() {
            records.daily_dungeon_best_score = Some(
                records
                    .daily_dungeon_best_score
                    .map_or(self.state.games.daily_dungeon.score, |best| {
                        best.max(self.state.games.daily_dungeon.score)
                    }),
            );
        }
        if self.state.games.dots_boxes.won() {
            records.dots_boxes_best_score = Some(
                records
                    .dots_boxes_best_score
                    .map_or(self.state.games.dots_boxes.scores[0], |best| {
                        best.max(self.state.games.dots_boxes.scores[0])
                    }),
            );
        }
        if self.state.games.sokoban.won() {
            records.sokoban_best_moves = Some(
                records
                    .sokoban_best_moves
                    .map_or(self.state.games.sokoban.moves, |best| {
                        best.min(self.state.games.sokoban.moves)
                    }),
            );
        }
        if self.state.games.mancala.won() {
            records.mancala_best_score = Some(
                records
                    .mancala_best_score
                    .map_or(self.state.games.mancala.pits[6], |best| {
                        best.max(self.state.games.mancala.pits[6])
                    }),
            );
        }
        if self.state.games.hanoi.won() {
            records.hanoi_best_moves = Some(
                records
                    .hanoi_best_moves
                    .map_or(self.state.games.hanoi.moves, |best| {
                        best.min(self.state.games.hanoi.moves)
                    }),
            );
        }
        if self.state.games.number_match.won() {
            records.number_match_best_moves = Some(
                records
                    .number_match_best_moves
                    .map_or(self.state.games.number_match.moves, |best| {
                        best.min(self.state.games.number_match.moves)
                    }),
            );
        }
        if self.state.games.flood_it.won() {
            records.flood_it_best_moves = Some(
                records
                    .flood_it_best_moves
                    .map_or(self.state.games.flood_it.moves, |best| {
                        best.min(self.state.games.flood_it.moves)
                    }),
            );
        }
        if self.state.games.color_sort.won() {
            records.color_sort_best_moves = Some(
                records
                    .color_sort_best_moves
                    .map_or(self.state.games.color_sort.moves, |best| {
                        best.min(self.state.games.color_sort.moves)
                    }),
            );
        }
        if self.state.games.battleship.won() {
            records.battleship_best_moves = Some(
                records
                    .battleship_best_moves
                    .map_or(self.state.games.battleship.moves, |best| {
                        best.min(self.state.games.battleship.moves)
                    }),
            );
        }
        if self.state.games.word_grid.won() {
            records.word_grid_best_moves = Some(records.word_grid_best_moves.map_or(
                self.state.games.word_grid.moves.min(u8::MAX as u16) as u8,
                |best| best.min(self.state.games.word_grid.moves.min(u8::MAX as u16) as u8),
            ));
        }
        if self.state.games.pipe_loop.won() {
            records.pipe_loop_best_moves = Some(
                records
                    .pipe_loop_best_moves
                    .map_or(self.state.games.pipe_loop.moves, |best| {
                        best.min(self.state.games.pipe_loop.moves)
                    }),
            );
        }
        if self.state.games.maze_walk.won() {
            records.maze_walk_best_moves = Some(
                records
                    .maze_walk_best_moves
                    .map_or(self.state.games.maze_walk.moves, |best| {
                        best.min(self.state.games.maze_walk.moves)
                    }),
            );
        }
        if self.state.games.match_three.won() {
            records.match_three_best_score = Some(
                records
                    .match_three_best_score
                    .map_or(self.state.games.match_three.score, |best| {
                        best.max(self.state.games.match_three.score)
                    }),
            );
        }
        if self.state.games.pyramid.status == crate::pyramid::PyramidStatus::Won {
            records.pyramid_best_moves = Some(
                records
                    .pyramid_best_moves
                    .map_or(self.state.games.pyramid.moves, |best| {
                        best.min(self.state.games.pyramid.moves)
                    }),
            );
        }
        if self.state.games.tri_peaks.status == crate::tri_peaks::TriPeaksStatus::Won {
            records.tri_peaks_best_moves = Some(
                records
                    .tri_peaks_best_moves
                    .map_or(self.state.games.tri_peaks.moves, |best| {
                        best.min(self.state.games.tri_peaks.moves)
                    }),
            );
        }
        if self.state.games.nim.won() {
            records.nim_best_moves = Some(
                records
                    .nim_best_moves
                    .map_or(self.state.games.nim.moves, |best| {
                        best.min(self.state.games.nim.moves)
                    }),
            );
        }
        if self.state.games.word_ladder.phase == crate::word_ladder::WordLadderPhase::Won {
            records.word_ladder_best_moves = Some(
                records
                    .word_ladder_best_moves
                    .map_or(self.state.games.word_ladder.moves, |best| {
                        best.min(self.state.games.word_ladder.moves)
                    }),
            );
        }
        if self.state.games.space_invaders.status == crate::space_invaders::SpaceInvadersStatus::Won
        {
            records.space_invaders_best_score = Some(
                records
                    .space_invaders_best_score
                    .map_or(self.state.games.space_invaders.score, |best| {
                        best.max(self.state.games.space_invaders.score)
                    }),
            );
        }
        if self.state.games.asteroids.status == crate::asteroids::AsteroidsStatus::Won {
            records.asteroids_best_score = Some(
                records
                    .asteroids_best_score
                    .map_or(self.state.games.asteroids.score, |best| {
                        best.max(self.state.games.asteroids.score)
                    }),
            );
        }
        if self.state.games.frogger.status == crate::frogger::FroggerStatus::Won {
            records.frogger_best_score = Some(
                records
                    .frogger_best_score
                    .map_or(self.state.games.frogger.score, |best| {
                        best.max(self.state.games.frogger.score)
                    }),
            );
        }
        if self.state.games.munch_maze.status == crate::munch_maze::MunchStatus::Won {
            records.munch_maze_best_score = Some(
                records
                    .munch_maze_best_score
                    .map_or(u32::from(self.state.games.munch_maze.score), |best| {
                        best.max(u32::from(self.state.games.munch_maze.score))
                    }),
            );
        }
        if self.state.games.block_stack.status == crate::block_stack::BlockStatus::Won {
            records.block_stack_best_score = Some(
                records
                    .block_stack_best_score
                    .map_or(self.state.games.block_stack.score, |best| {
                        best.max(self.state.games.block_stack.score)
                    }),
            );
        }
        if self.state.games.terrain_cannon.status == crate::terrain_cannon::CannonStatus::Won {
            records.terrain_cannon_best_score = Some(
                records
                    .terrain_cannon_best_score
                    .map_or(u32::from(self.state.games.terrain_cannon.score), |best| {
                        best.max(u32::from(self.state.games.terrain_cannon.score))
                    }),
            );
        }
        if self.state.games.fling_fury.status == crate::fling_fury::FlingStatus::Won {
            records.fling_fury_best_score = Some(
                records
                    .fling_fury_best_score
                    .map_or(u32::from(self.state.games.fling_fury.score), |best| {
                        best.max(u32::from(self.state.games.fling_fury.score))
                    }),
            );
        }
        if self.state.games.paddle_duel.status == crate::paddle_duel::PaddleStatus::Won {
            records.paddle_duel_best_score = Some(records.paddle_duel_best_score.map_or(
                u32::from(self.state.games.paddle_duel.player_score),
                |best| best.max(u32::from(self.state.games.paddle_duel.player_score)),
            ));
        }
        for (index, complete) in [
            self.state.games.riddle_room.won(),
            self.state.games.pattern_vault.won(),
            self.state.games.sum_circuit.won(),
            self.state.games.orbit_order.won(),
            self.state.games.word_forge.won(),
        ]
        .into_iter()
        .enumerate()
        {
            if complete {
                records.misc_best_moves[index] = Some(records.misc_best_moves[index].map_or(
                    [
                        self.state.games.riddle_room.moves,
                        self.state.games.pattern_vault.moves,
                        self.state.games.sum_circuit.moves,
                        self.state.games.orbit_order.moves,
                        self.state.games.word_forge.moves,
                    ][index],
                    |best| {
                        best.min(
                            [
                                self.state.games.riddle_room.moves,
                                self.state.games.pattern_vault.moves,
                                self.state.games.sum_circuit.moves,
                                self.state.games.orbit_order.moves,
                                self.state.games.word_forge.moves,
                            ][index],
                        )
                    },
                ));
            }
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
        if self.state.games.game.move_in(direction) && self.state.games.game.won() {
            self.notifications
                .success("2048 reached — keep playing or start a fresh board");
        }
        self.update_records();
        self.request_autosave();
    }
}

pub(super) fn is_timed_game(game: GameId) -> bool {
    !matches!(
        game,
        GameId::Snake
            | GameId::Breakout
            | GameId::TinyTowerDefence
            | GameId::SpaceInvaders
            | GameId::Asteroids
            | GameId::Frogger
            | GameId::MunchMaze
            | GameId::BlockStack
            | GameId::TerrainCannon
            | GameId::FlingFury
            | GameId::PaddleDuel
    )
}

pub(crate) fn round_is_complete(state: &crate::state::AppState, game: GameId) -> bool {
    match game {
        GameId::Game2048 => state.games.game.won(),
        GameId::Minesweeper => {
            state.games.minesweeper.status == crate::minesweeper::MineStatus::Won
        }
        GameId::Sudoku => state.games.sudoku.status == crate::sudoku::SudokuStatus::Won,
        GameId::Nonogram => state.games.nonogram.status == crate::nonogram::NonogramStatus::Won,
        GameId::Solitaire => state.games.solitaire.status == crate::solitaire::SolitaireStatus::Won,
        GameId::FreeCell => state.games.freecell.status == crate::freecell::FreeCellStatus::Won,
        GameId::Yahtzee => state.games.fivefold.status == crate::fivefold::FivefoldStatus::Complete,
        GameId::Reversi => state.games.reversi.status == crate::reversi::ReversiStatus::Won,
        GameId::LightsOut => {
            state.games.lights_out.status == crate::lights_out::LightsOutStatus::Won
        }
        GameId::TicTacToe => {
            state.games.tic_tac_toe.status
                == crate::tic_tac_toe::TicTacToeStatus::Won(crate::tic_tac_toe::Mark::X)
        }
        GameId::MemoryPairs => {
            state.games.memory_pairs.status == crate::memory_pairs::MemoryStatus::Won
        }
        GameId::SlidingPuzzle => {
            state.games.sliding_puzzle.status == crate::sliding_puzzle::SlidingStatus::Won
        }
        GameId::Mastermind => {
            state.games.mastermind.status == crate::mastermind::MastermindStatus::Won
        }
        GameId::Spider => state.games.spider.status == crate::spider::SpiderStatus::Won,
        GameId::WordSearch => {
            state.games.word_search.status == crate::word_search::WordSearchStatus::Won
        }
        GameId::Hangman => state.games.hangman.status == crate::hangman::HangmanStatus::Won,
        GameId::ConnectFour => {
            state.games.connect_four.status
                == crate::connect_four::ConnectFourStatus::Won(crate::connect_four::Disc::Red)
        }
        GameId::Checkers => {
            state.games.checkers.status
                == crate::checkers::CheckersStatus::Won(crate::checkers::Side::Red)
        }
        GameId::PegSolitaire => {
            state.games.peg_solitaire.status == crate::peg_solitaire::PegSolitaireStatus::Won
        }
        GameId::MahjongSolitaire => {
            state.games.mahjong_solitaire.status == crate::mahjong_solitaire::MahjongStatus::Won
        }
        GameId::HigherLower => {
            state.games.higher_lower.status == crate::higher_lower::HigherLowerStatus::Won
        }
        GameId::KlondikeGolf => {
            state.games.klondike_golf.status == crate::klondike_golf::GolfStatus::Won
        }
        GameId::Blackjack => state.games.blackjack.status == crate::blackjack::BlackjackStatus::Won,
        GameId::SpiderSolitaire => {
            state.games.spider_solitaire.status
                == crate::spider_solitaire::SpiderSolitaireStatus::Won
        }
        GameId::DungeonSweeper => {
            state.games.dungeon_sweeper.status == crate::dungeon_sweeper::DungeonStatus::Won
        }
        GameId::Potion2048 => state.games.potion_2048.won(),
        GameId::OneRoomRoguelike => state.games.one_room_roguelike.won(),
        GameId::DailyDungeon => state.games.daily_dungeon.won(),
        GameId::DotsBoxes => state.games.dots_boxes.won(),
        GameId::Sokoban => state.games.sokoban.won(),
        GameId::Mancala => state.games.mancala.won(),
        GameId::Hanoi => state.games.hanoi.won(),
        GameId::NumberMatch => state.games.number_match.won(),
        GameId::FloodIt => state.games.flood_it.won(),
        GameId::ColorSort => state.games.color_sort.won(),
        GameId::Battleship => state.games.battleship.won(),
        GameId::WordGrid => state.games.word_grid.won(),
        GameId::PipeLoop => state.games.pipe_loop.won(),
        GameId::MazeWalk => state.games.maze_walk.won(),
        GameId::MatchThree => state.games.match_three.won(),
        GameId::Pyramid => state.games.pyramid.status == crate::pyramid::PyramidStatus::Won,
        GameId::TriPeaks => state.games.tri_peaks.status == crate::tri_peaks::TriPeaksStatus::Won,
        GameId::Nim => state.games.nim.won(),
        GameId::WordLadder => {
            state.games.word_ladder.phase == crate::word_ladder::WordLadderPhase::Won
        }
        GameId::RiddleRoom => state.games.riddle_room.won(),
        GameId::PatternVault => state.games.pattern_vault.won(),
        GameId::SumCircuit => state.games.sum_circuit.won(),
        GameId::OrbitOrder => state.games.orbit_order.won(),
        GameId::WordForge => state.games.word_forge.won(),
        GameId::Snake
        | GameId::Breakout
        | GameId::TinyTowerDefence
        | GameId::SpaceInvaders
        | GameId::Asteroids
        | GameId::Frogger => false,
        GameId::MunchMaze
        | GameId::BlockStack
        | GameId::TerrainCannon
        | GameId::FlingFury
        | GameId::PaddleDuel => false,
    }
}
