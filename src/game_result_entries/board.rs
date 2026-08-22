use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};
use super::result;

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    match game {
        GameId::Reversi => {
            let game = &state.games.reversi;
            if game.status != crate::reversi::ReversiStatus::Won {
                return None;
            }
            let explanation = match game.winner {
                Some(1) => "The board is full and your dark discs lead.",
                Some(2) => "The board is full and the light discs lead.",
                _ => "The board is full and the final score is recorded.",
            };
            result(
                state,
                game,
                ResultKind::Won,
                explanation,
                format!(
                    "DARK {}  ·  LIGHT {}  ·  MOVES {}",
                    game.score(1),
                    game.score(2),
                    game.moves
                ),
                UiAction::ReversiNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::TicTacToe => {
            let game = &state.games.tic_tac_toe;
            let (kind, explanation) = match game.status {
                crate::tic_tac_toe::TicTacToeStatus::Won(crate::tic_tac_toe::Mark::X) => {
                    (ResultKind::Won, "You made three in a row.")
                }
                crate::tic_tac_toe::TicTacToeStatus::Won(_) => {
                    (ResultKind::Lost, "The cabinet made three in a row.")
                }
                crate::tic_tac_toe::TicTacToeStatus::Draw => (
                    ResultKind::Draw,
                    "Every square is filled and neither side won.",
                ),
                crate::tic_tac_toe::TicTacToeStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                explanation,
                format!("MOVES {}  ·  LEVEL {:?}", game.moves, game.ai_level),
                UiAction::TicTacToeNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::ConnectFour => {
            let game = &state.games.connect_four;
            let (kind, explanation) = match game.status {
                crate::connect_four::ConnectFourStatus::Won(crate::connect_four::Disc::Red) => {
                    (ResultKind::Won, "Your red discs connect four.")
                }
                crate::connect_four::ConnectFourStatus::Won(_) => {
                    (ResultKind::Lost, "The cabinet connects four first.")
                }
                crate::connect_four::ConnectFourStatus::Draw => (
                    ResultKind::Draw,
                    "The grid is full and neither side connects four.",
                ),
                crate::connect_four::ConnectFourStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                explanation,
                format!("MOVES {}  ·  LEVEL {:?}", game.moves, game.ai_level),
                UiAction::ConnectFourNew,
                "NEW GRID",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Checkers => {
            let game = &state.games.checkers;
            let (kind, explanation) = match game.status {
                crate::checkers::CheckersStatus::Won(crate::checkers::Side::Red) => {
                    (ResultKind::Won, "Your red pieces control the board.")
                }
                crate::checkers::CheckersStatus::Won(_) => {
                    (ResultKind::Lost, "The cabinet controls the board.")
                }
                crate::checkers::CheckersStatus::Draw => {
                    (ResultKind::Draw, "Neither side can force a win.")
                }
                crate::checkers::CheckersStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                explanation,
                format!("MOVES {}  ·  LEVEL {:?}", game.moves, game.ai_level),
                UiAction::CheckersNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::PegSolitaire => {
            let game = &state.games.peg_solitaire;
            let kind = match game.status {
                crate::peg_solitaire::PegSolitaireStatus::Won => ResultKind::Won,
                crate::peg_solitaire::PegSolitaireStatus::Stuck => ResultKind::Stuck,
                crate::peg_solitaire::PegSolitaireStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "One peg remains in the finishing hole."
                } else {
                    "No legal jump remains before the finishing hole."
                },
                format!(
                    "PEGS {}  ·  MOVES {}",
                    game.cells
                        .iter()
                        .filter(|cell| **cell == crate::peg_solitaire::Hole::Peg)
                        .count(),
                    game.moves
                ),
                UiAction::PegSolitaireNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::MahjongSolitaire => {
            let game = &state.games.mahjong_solitaire;
            let kind = match game.status {
                crate::mahjong_solitaire::MahjongStatus::Won => ResultKind::Won,
                crate::mahjong_solitaire::MahjongStatus::Stuck => ResultKind::Stuck,
                crate::mahjong_solitaire::MahjongStatus::Playing => return None,
            };
            let remaining = game.tiles.iter().filter(|tile| !tile.removed).count();
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every tile has been paired and removed."
                } else {
                    "No free matching pair remains."
                },
                format!("TILES LEFT {}  ·  MOVES {}", remaining, game.moves),
                UiAction::MahjongSolitaireNew,
                "NEW LAYOUT",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Sokoban => {
            let game = &state.games.sokoban;
            let kind = match game.phase {
                crate::sokoban::SokobanPhase::Won => ResultKind::Won,
                crate::sokoban::SokobanPhase::Stuck => ResultKind::Stuck,
                crate::sokoban::SokobanPhase::Playing => return None,
            };
            let (primary_action, primary_label, secondary_action, secondary_label) =
                if kind == ResultKind::Won {
                    (
                        UiAction::SokobanNew,
                        "NEXT ROOM",
                        UiAction::SokobanRestart,
                        "REPLAY ROOM",
                    )
                } else {
                    (
                        UiAction::SokobanRestart,
                        "RESTART ROOM",
                        UiAction::SokobanNew,
                        "SKIP ROOM",
                    )
                };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every crate is on a marked square."
                } else {
                    "A crate is cornered and no useful push remains."
                },
                format!(
                    "ROOM {}  ·  MOVES {}  ·  PUSHES {}",
                    game.level + 1,
                    game.moves,
                    game.pushes
                ),
                primary_action,
                primary_label,
                secondary_action,
                secondary_label,
            )
        }
        GameId::Mancala => {
            let game = &state.games.mancala;
            let kind = match game.phase {
                crate::mancala::MancalaPhase::Won => ResultKind::Won,
                crate::mancala::MancalaPhase::Lost => ResultKind::Lost,
                crate::mancala::MancalaPhase::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Your store holds more stones at the end."
                } else {
                    "The cabinet store holds more stones at the end."
                },
                format!(
                    "YOU {}  ·  CABINET {}  ·  MOVES {}",
                    game.pits[6], game.pits[13], game.moves
                ),
                UiAction::MancalaNew,
                "NEW BOARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Hanoi => {
            let game = &state.games.hanoi;
            if game.phase != crate::hanoi::HanoiPhase::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every disk is stacked on the far peg.",
                format!(
                    "DISKS {}  ·  MOVES {}  ·  PAR {}",
                    game.disks,
                    game.moves,
                    game.optimal_moves()
                ),
                UiAction::HanoiDisks(game.disks),
                "NEW TOWER",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Nim => {
            let game = &state.games.nim;
            let kind = match game.status {
                crate::nim::NimStatus::Won => ResultKind::Won,
                crate::nim::NimStatus::Lost => ResultKind::Lost,
                crate::nim::NimStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The final stone fell in your favor."
                } else {
                    "The final stone fell in the cabinet's favor."
                },
                format!(
                    "HEAPS {}  ·  MOVES {}  ·  {}",
                    game.heaps.iter().sum::<u8>(),
                    game.moves,
                    game.rule.label()
                ),
                UiAction::NimNew,
                "NEW HEAPS",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Battleship => {
            let game = &state.games.battleship;
            if game.phase != crate::battleship::BattleshipPhase::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every ship in the fleet has been sunk.",
                format!(
                    "HITS {} / {}  ·  SHIPS {} / {}  ·  SCORE {}",
                    game.hits(),
                    game.ship_cells(),
                    game.sunk_ships(),
                    game.ship_count(),
                    game.score
                ),
                UiAction::BattleshipNew,
                "NEW FLEET",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        _ => None,
    }
}
