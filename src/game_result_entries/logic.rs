use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};
use super::result;

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    match game {
        GameId::Game2048 => {
            let game = &state.games.game;
            let kind = if game.won() {
                ResultKind::Won
            } else if !game.can_move() {
                ResultKind::Stuck
            } else {
                return None;
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "You reached the 2048 tile."
                } else {
                    "No moves remain on this board."
                },
                format!(
                    "SCORE {}  ·  BEST {}  ·  BOARD {}",
                    game.score,
                    game.best,
                    game.board_size.label()
                ),
                UiAction::Restart,
                "RESTART",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Minesweeper => {
            let game = &state.games.minesweeper;
            let kind = match game.status {
                crate::minesweeper::MineStatus::Won => ResultKind::Won,
                crate::minesweeper::MineStatus::Lost => ResultKind::Lost,
                _ => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every safe square is clear."
                } else {
                    "A mine was triggered before the field was clear."
                },
                format!(
                    "MINES {}  ·  TIME {}s  ·  FLAGGED {}",
                    game.mines,
                    game.elapsed_whole_seconds(),
                    game.flagged_count()
                ),
                UiAction::MineRestart,
                "RESTART",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Sudoku => {
            let game = &state.games.sudoku;
            if game.status != crate::sudoku::SudokuStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every square is filled correctly.",
                format!("MOVES {}  ·  {}", game.moves, game.difficulty.label()),
                UiAction::SudokuDifficulty(game.difficulty),
                "NEW PUZZLE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Nonogram => {
            let game = &state.games.nonogram;
            if game.status != crate::nonogram::NonogramStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "The hidden picture matches every clue.",
                format!(
                    "MOVES {}  ·  SIZE {} × {}",
                    game.moves, game.size, game.size
                ),
                UiAction::NonogramPreset(game.preset),
                "NEW PUZZLE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::LightsOut => {
            let game = &state.games.lights_out;
            if game.status != crate::lights_out::LightsOutStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every light is off.",
                format!(
                    "MOVES {}  ·  PAR {}  ·  {}",
                    game.moves,
                    game.par,
                    game.difficulty.label()
                ),
                UiAction::LightsOutNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::MemoryPairs => {
            let game = &state.games.memory_pairs;
            if game.status != crate::memory_pairs::MemoryStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every matching pair has been found.",
                format!(
                    "PAIRS {} / 8  ·  MOVES {}  ·  SCORE {}",
                    game.matched_pairs, game.moves, game.score
                ),
                UiAction::MemoryPairsNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::SlidingPuzzle => {
            let game = &state.games.sliding_puzzle;
            if game.status != crate::sliding_puzzle::SlidingStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every tile is back in order.",
                format!("MOVES {}  ·  {}", game.moves, game.variant.label()),
                UiAction::SlidingPuzzleNew,
                "NEW PUZZLE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Mastermind => {
            let game = &state.games.mastermind;
            let kind = match game.status {
                crate::mastermind::MastermindStatus::Won => ResultKind::Won,
                crate::mastermind::MastermindStatus::Lost => ResultKind::Lost,
                crate::mastermind::MastermindStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The hidden color code is solved."
                } else {
                    "All guesses were used before the code was solved."
                },
                format!("GUESSES {} / 10  ·  {}", game.row, game.variant.label()),
                UiAction::MastermindNew,
                "NEW CODE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::DungeonSweeper => {
            let game = &state.games.dungeon_sweeper;
            let kind = match game.status {
                crate::dungeon_sweeper::DungeonStatus::Won => ResultKind::Won,
                crate::dungeon_sweeper::DungeonStatus::Lost => ResultKind::Lost,
                _ => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The exit is open and every required relic is recovered."
                } else {
                    "The dungeon took the last heart before the route was clear."
                },
                format!(
                    "RELICS {} / {}  ·  HEARTS {}  ·  MOVES {}",
                    game.relics_found(),
                    game.relic_total(),
                    game.hearts,
                    game.moves
                ),
                UiAction::DungeonNew,
                "NEW DUNGEON",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Potion2048 => {
            let game = &state.games.potion_2048;
            let kind = if game.won() {
                ResultKind::Won
            } else if game.hint_direction().is_none() {
                ResultKind::Stuck
            } else {
                return None;
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The target potion has been brewed."
                } else {
                    "No merge remains and the board is full."
                },
                format!(
                    "SCORE {}  ·  GOAL {}  ·  CATALYSTS {}",
                    game.score,
                    game.target(),
                    game.catalysts_brewed
                ),
                UiAction::PotionNew,
                "NEW BREW",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::NumberMatch => {
            let game = &state.games.number_match;
            let kind = match game.phase {
                crate::number_match::NumberMatchPhase::Won => ResultKind::Won,
                crate::number_match::NumberMatchPhase::Stuck => ResultKind::Stuck,
                crate::number_match::NumberMatchPhase::Playing => return None,
            };
            let primary = if kind == ResultKind::Stuck && game.remixes_left > 0 {
                UiAction::NumberMatchRemix
            } else {
                UiAction::NumberMatchNew
            };
            let label = if kind == ResultKind::Stuck && game.remixes_left > 0 {
                "REMIX BOARD"
            } else {
                "NEW BOARD"
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every number has found its matching link."
                } else {
                    "No legal link remains on this board."
                },
                format!(
                    "SCORE {}  ·  MOVES {}  ·  REMIXES {}",
                    game.points, game.moves, game.remixes_left
                ),
                primary,
                label,
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::FloodIt => {
            let game = &state.games.flood_it;
            let kind = match game.phase {
                crate::flood_it::FloodPhase::Won => ResultKind::Won,
                crate::flood_it::FloodPhase::Lost => ResultKind::Lost,
                crate::flood_it::FloodPhase::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The whole field is one color within the move limit."
                } else {
                    "The move limit ended before the field was flooded."
                },
                format!(
                    "MOVES {} / {}  ·  SCORE {}  ·  REGION {}",
                    game.moves,
                    game.move_limit(),
                    game.points,
                    game.region_size()
                ),
                UiAction::FloodNew,
                "NEW FIELD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::ColorSort => {
            let game = &state.games.color_sort;
            if game.phase != crate::color_sort::ColorSortPhase::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every color is sorted into a complete tube.",
                format!(
                    "TUBES {} / {}  ·  MOVES {}  ·  SCORE {}",
                    game.completed_tubes(),
                    game.tubes.len(),
                    game.moves,
                    game.points
                ),
                UiAction::ColorSortNew,
                "NEW SORT",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::PipeLoop => {
            let game = &state.games.pipe_loop;
            if game.phase != crate::pipe_loop::PipePhase::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "The network is powered with every leak closed.",
                format!(
                    "CONNECTED {} / {}  ·  LEAKS {}  ·  MOVES {}",
                    game.connected_count(),
                    game.pipes.len(),
                    game.leak_count(),
                    game.moves
                ),
                UiAction::PipeNew,
                "NEW NETWORK",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::MazeWalk => {
            let game = &state.games.maze_walk;
            if game.phase != crate::maze_walk::MazePhase::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "The exit is reached after collecting every beacon.",
                format!(
                    "BEACONS {} / {}  ·  MOVES {}  ·  PAR {}",
                    game.collected.len(),
                    game.beacons.len(),
                    game.moves,
                    game.par
                ),
                UiAction::MazeNew,
                "NEW MAZE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::MatchThree => {
            let game = &state.games.match_three;
            let kind = match game.phase {
                crate::match_three::MatchThreePhase::Won => ResultKind::Won,
                crate::match_three::MatchThreePhase::Lost => ResultKind::Lost,
                crate::match_three::MatchThreePhase::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The score target is reached."
                } else {
                    "The move limit ended before the score target."
                },
                format!(
                    "SCORE {} / {}  ·  MOVES LEFT {}  ·  BEST CASCADE {}",
                    game.score,
                    game.target_score(),
                    game.moves_left(),
                    game.best_cascade
                ),
                UiAction::MatchThreeNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        _ => None,
    }
}
