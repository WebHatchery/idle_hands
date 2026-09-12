//! Terminal result copy for word drawers.

use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    match game {
        GameId::WordSearch => {
            let game = &state.games.word_search;
            if game.status != crate::word_search::WordSearchStatus::Won {
                return None;
            }
            result_entry!(
                state,
                game,
                ResultKind::Won,
                "Every hidden word has been found.",
                format!(
                    "WORDS {} / {}  ·  MOVES {}",
                    game.found.iter().filter(|found| **found).count(),
                    game.found.len(),
                    game.moves
                ),
                UiAction::WordSearchNew,
                "NEW SEARCH",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Hangman => {
            let game = &state.games.hangman;
            let kind = match game.status {
                crate::hangman::HangmanStatus::Won => ResultKind::Won,
                crate::hangman::HangmanStatus::Lost => ResultKind::Lost,
                crate::hangman::HangmanStatus::Playing => return None,
            };
            result_entry!(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every letter in the word was revealed."
                } else {
                    "The word was not revealed before the mistake limit."
                },
                format!("MISTAKES {}  ·  SCORE {}", game.wrong_count, game.score),
                UiAction::HangmanNew,
                "NEW WORD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::WordGrid => {
            let game = &state.games.word_grid;
            let kind = match game.phase {
                crate::word_grid::WordGridPhase::Won => ResultKind::Won,
                crate::word_grid::WordGridPhase::Lost => ResultKind::Lost,
                crate::word_grid::WordGridPhase::Playing => return None,
            };
            let explanation = if kind == ResultKind::Won {
                "The hidden word is solved.".to_owned()
            } else {
                format!("The word was {}.", game.target)
            };
            result_entry!(
                state,
                game,
                kind,
                explanation,
                format!("GUESSES {}  ·  MOVES {}", game.guesses.len(), game.moves),
                UiAction::WordGridNew,
                "NEW WORD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::WordLadder => {
            let game = &state.games.word_ladder;
            if game.phase != crate::word_ladder::WordLadderPhase::Won {
                return None;
            }
            result_entry!(
                state,
                game,
                ResultKind::Won,
                format!("{} is reached from {}.", game.target, game.start),
                format!(
                    "MOVES {}  ·  PAR {}  ·  STEPS LEFT {}",
                    game.moves,
                    game.par,
                    game.remaining_steps()
                ),
                UiAction::WordLadderNew,
                "NEW LADDER",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        _ => None,
    }
}
