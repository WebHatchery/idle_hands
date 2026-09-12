//! Terminal result copy for the miscellaneous word-and-number drawers.

use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    let game = match game {
        GameId::RiddleRoom => &state.games.riddle_room,
        GameId::PatternVault => &state.games.pattern_vault,
        GameId::SumCircuit => &state.games.sum_circuit,
        GameId::OrbitOrder => &state.games.orbit_order,
        GameId::WordForge => &state.games.word_forge,
        _ => return None,
    };
    if game.phase != crate::misc_games::MiscPhase::Won {
        return None;
    }
    result_entry!(
        state,
        game,
        ResultKind::Won,
        "The cabinet challenge is solved.",
        format!(
            "ROUND {}  ·  SCORE {}  ·  MOVES {}  ·  MISTAKES {}",
            game.round, game.score, game.moves, game.mistakes
        ),
        UiAction::MiscNew,
        "NEW ROUND",
        UiAction::Cabinet,
        "CABINET",
    )
}
