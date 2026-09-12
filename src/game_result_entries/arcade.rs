//! Terminal result copy for arcade and action drawers.

use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    match game {
        GameId::Snake => {
            let game = &state.games.snake;
            let kind = match game.status {
                crate::snake::SnakeStatus::Won => ResultKind::Won,
                crate::snake::SnakeStatus::Lost => ResultKind::Lost,
                crate::snake::SnakeStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The snake reached the run target."
                } else {
                    "The snake hit an obstacle or its own trail."
                },
                format!(
                    "SCORE {} / {}  ·  LENGTH {}",
                    game.score,
                    game.win_score,
                    game.body.len()
                ),
                UiAction::SnakeNew,
                "RESTART RUN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Breakout => {
            let game = &state.games.breakout;
            let kind = match game.status {
                crate::breakout::BreakoutStatus::Won => ResultKind::Won,
                crate::breakout::BreakoutStatus::Lost => ResultKind::Lost,
                crate::breakout::BreakoutStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "All three walls are clear."
                } else {
                    "The paddle ran out of lives before the walls were clear."
                },
                format!(
                    "LEVEL {} / {}  ·  SCORE {}  ·  LIVES {}",
                    game.level, game.target_level, game.score, game.lives
                ),
                UiAction::BreakoutNew,
                "NEW RUN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::HigherLower => {
            let game = &state.games.higher_lower;
            let kind = match game.status {
                crate::higher_lower::HigherLowerStatus::Won => ResultKind::Won,
                crate::higher_lower::HigherLowerStatus::Lost => ResultKind::Lost,
                crate::higher_lower::HigherLowerStatus::Playing => return None,
            };
            let explanation = if kind == ResultKind::Won {
                if game.cashed_out {
                    "You banked the run safely."
                } else {
                    "You called every card correctly."
                }
            } else {
                "The next card did not match your call."
            };
            result_entry!(
                state,
                game,
                kind,
                explanation,
                format!(
                    "RUN {} / 10  ·  POT {}  ·  BANKED {}",
                    game.score, game.pot, game.banked
                ),
                UiAction::HigherLowerNew,
                "NEW RUN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::TinyTowerDefence => {
            let game = &state.games.tiny_tower_defence;
            let kind = match game.phase {
                crate::tiny_tower_defence::TowerPhase::Won => ResultKind::Won,
                crate::tiny_tower_defence::TowerPhase::Lost => ResultKind::Lost,
                _ => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "All eight waves were held at the gate."
                } else {
                    "The gate fell before the final wave was stopped."
                },
                format!(
                    "WAVE {} / {}  ·  LIVES {}  ·  SCORE {}",
                    game.wave,
                    crate::tiny_tower_defence::TinyTowerDefence::target_wave(),
                    game.lives,
                    game.score
                ),
                UiAction::TowerNew,
                "NEW DEFENCE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::OneRoomRoguelike => {
            let game = &state.games.one_room_roguelike;
            let kind = match game.phase {
                crate::one_room_roguelike::RoomPhase::Won => ResultKind::Won,
                crate::one_room_roguelike::RoomPhase::Lost => ResultKind::Lost,
                _ => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The room is clear and the treasure is secured."
                } else {
                    "Your health reached zero before the room was clear."
                },
                format!(
                    "ROOM {} / {}  ·  SCORE {}  ·  TURNS {}",
                    game.room,
                    crate::one_room_roguelike::OneRoomRoguelike::target_room(),
                    game.score,
                    game.turns
                ),
                UiAction::RogueNew,
                "NEW RUN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::DailyDungeon => {
            let game = &state.games.daily_dungeon;
            let kind = match game.phase {
                crate::daily_dungeon::DailyPhase::Won => ResultKind::Won,
                crate::daily_dungeon::DailyPhase::Lost => ResultKind::Lost,
                crate::daily_dungeon::DailyPhase::Exploring => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The daily exit is reached with the route objectives complete."
                } else {
                    "The dungeon took all your hearts before the exit."
                },
                format!(
                    "{}  ·  RUNES {} / 3  ·  HEARTS {}  ·  SCORE {}  ·  BEST {}",
                    crate::daily_challenge::label(game.day_key, game.challenge),
                    game.runes_found,
                    game.hearts,
                    game.score,
                    state
                        .records
                        .daily_score(game.day_key)
                        .map_or_else(|| "—".to_owned(), |score| score.to_string())
                ),
                UiAction::DailyNew,
                "REPLAY DAY",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::DotsBoxes => {
            let game = &state.games.dots_boxes;
            let kind = match game.phase {
                crate::dots_boxes::DotsPhase::Won => ResultKind::Won,
                crate::dots_boxes::DotsPhase::Lost => ResultKind::Lost,
                crate::dots_boxes::DotsPhase::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Your side owns more completed boxes."
                } else {
                    "The cabinet owns more completed boxes."
                },
                format!(
                    "YOU {}  ·  CABINET {}  ·  MOVES {}",
                    game.scores[0], game.scores[1], game.moves
                ),
                UiAction::DotsNew,
                "NEW GRID",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::SpaceInvaders => {
            let game = &state.games.space_invaders;
            let kind = match game.status {
                crate::space_invaders::SpaceInvadersStatus::Won => ResultKind::Won,
                crate::space_invaders::SpaceInvadersStatus::Lost => ResultKind::Lost,
                crate::space_invaders::SpaceInvadersStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every invader wave is cleared."
                } else {
                    "The invaders reached the ship or used all lives."
                },
                format!(
                    "WAVE {} / {}  ·  SCORE {}  ·  LIVES {}",
                    game.wave, game.target_wave, game.score, game.lives
                ),
                UiAction::SpaceInvadersNew,
                "NEW WAVE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Asteroids => {
            let game = &state.games.asteroids;
            let kind = match game.status {
                crate::asteroids::AsteroidsStatus::Won => ResultKind::Won,
                crate::asteroids::AsteroidsStatus::Lost => ResultKind::Lost,
                crate::asteroids::AsteroidsStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The asteroid score target is reached."
                } else {
                    "The ship used all of its lives."
                },
                format!(
                    "SCORE {} / {}  ·  LIVES {}  ·  MOVES {}",
                    game.score, game.target_score, game.lives, game.moves
                ),
                UiAction::AsteroidsNew,
                "NEW RUN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Frogger => {
            let game = &state.games.frogger;
            let kind = match game.status {
                crate::frogger::FroggerStatus::Won => ResultKind::Won,
                crate::frogger::FroggerStatus::Lost => ResultKind::Lost,
                crate::frogger::FroggerStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every crossing is complete."
                } else {
                    "The traffic used all of the frog's lives."
                },
                format!(
                    "CROSSINGS {} / {}  ·  SCORE {}  ·  LIVES {}",
                    game.crossings, game.target_crossings, game.score, game.lives
                ),
                UiAction::FroggerNew,
                "NEW CROSSING",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::MunchMaze => {
            let game = &state.games.munch_maze;
            let kind = match game.status {
                crate::munch_maze::MunchStatus::Won => ResultKind::Won,
                crate::munch_maze::MunchStatus::Lost => ResultKind::Lost,
                crate::munch_maze::MunchStatus::Playing => return None,
            };
            let pellets_left = game.pellets.iter().filter(|pellet| **pellet).count();
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every glowing pellet is collected."
                } else {
                    "The patrol caught the player before the maze was clear."
                },
                format!(
                    "SCORE {}  ·  PELLETS LEFT {}  ·  LIVES {}",
                    game.score, pellets_left, game.lives
                ),
                UiAction::MunchNew,
                "NEW MAZE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::BlockStack => {
            let game = &state.games.block_stack;
            let kind = match game.status {
                crate::block_stack::BlockStatus::Won => ResultKind::Won,
                crate::block_stack::BlockStatus::Lost => ResultKind::Lost,
                crate::block_stack::BlockStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The line target is reached."
                } else {
                    "The stack reached the top before the line target."
                },
                format!(
                    "LINES {} / {}  ·  SCORE {}  ·  LEVEL {}",
                    game.lines, game.target_lines, game.score, game.level
                ),
                UiAction::BlockNew,
                "NEW STACK",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::TerrainCannon => {
            let game = &state.games.terrain_cannon;
            let kind = match game.status {
                crate::terrain_cannon::CannonStatus::Won => ResultKind::Won,
                crate::terrain_cannon::CannonStatus::Lost => ResultKind::Lost,
                crate::terrain_cannon::CannonStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "The target is silenced after three clean impacts."
                } else {
                    "The shot cycle ended before the target was silenced."
                },
                format!(
                    "SCORE {}  ·  MOVES {}  ·  TARGET HEALTH {}",
                    game.score, game.moves, game.target_health
                ),
                UiAction::CannonNew,
                "NEW HILLSIDE",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::PaddleDuel => {
            let game = &state.games.paddle_duel;
            let kind = match game.status {
                crate::paddle_duel::PaddleStatus::Won => ResultKind::Won,
                crate::paddle_duel::PaddleStatus::Lost => ResultKind::Lost,
                crate::paddle_duel::PaddleStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "You reached the target first."
                } else {
                    "The cabinet reached the target first."
                },
                format!(
                    "YOU {} / {}  ·  CABINET {} / {}  ·  RALLIES {}",
                    game.player_score, game.win_score, game.cpu_score, game.win_score, game.moves
                ),
                UiAction::PaddleNew,
                "NEW RALLY",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        _ => None,
    }
}
