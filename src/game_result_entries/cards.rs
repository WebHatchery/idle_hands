use crate::state::{AppState, GameId};
use crate::ui::UiAction;

use super::super::{ResultInfo, ResultKind};
use super::result;

pub(super) fn info(state: &AppState, game: GameId) -> Option<ResultInfo> {
    match game {
        GameId::Solitaire => {
            let game = &state.games.solitaire;
            if game.status != crate::solitaire::SolitaireStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "All four foundations are complete.",
                format!("MOVES {}  ·  FOUNDATIONS 52 / 52", game.moves),
                UiAction::SolitaireNew,
                "NEW DEAL",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::FreeCell => {
            let game = &state.games.freecell;
            if game.status != crate::freecell::FreeCellStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "All cards have reached the four foundations.",
                format!("MOVES {}  ·  FOUNDATIONS 52 / 52", game.moves),
                UiAction::FreeCellNew,
                "NEW DEAL",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Yahtzee => {
            let game = &state.games.fivefold;
            if game.status != crate::fivefold::FivefoldStatus::Complete {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "Every scorecard category has been chosen.",
                format!("TOTAL {}  ·  CATEGORIES 13 / 13", game.total()),
                UiAction::FivefoldNew,
                "NEW SCORECARD",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Spider => {
            let game = &state.games.spider;
            if game.status != crate::spider::SpiderStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "All eight runs have been completed.",
                format!("RUNS {} / 8  ·  MOVES {}", game.completed, game.moves),
                UiAction::SpiderNew,
                "NEW DEAL",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::KlondikeGolf => {
            let game = &state.games.klondike_golf;
            let kind = match game.status {
                crate::klondike_golf::GolfStatus::Won => ResultKind::Won,
                crate::klondike_golf::GolfStatus::Stuck => ResultKind::Stuck,
                crate::klondike_golf::GolfStatus::Playing => return None,
            };
            let remaining = game.tableau.iter().map(Vec::len).sum::<usize>() + game.stock.len();
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every card has left the golf columns."
                } else {
                    "No legal golf move remains with cards still in play."
                },
                format!("CARDS LEFT {}  ·  MOVES {}", remaining, game.moves),
                UiAction::KlondikeGolfNew,
                "NEW DEAL",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Blackjack => {
            let game = &state.games.blackjack;
            let kind = match game.status {
                crate::blackjack::BlackjackStatus::Won => ResultKind::Won,
                crate::blackjack::BlackjackStatus::Lost => ResultKind::Lost,
                crate::blackjack::BlackjackStatus::Push => ResultKind::Push,
                crate::blackjack::BlackjackStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                match kind {
                    ResultKind::Won => "Your hand beats the dealer.",
                    ResultKind::Lost => "The dealer wins this hand.",
                    ResultKind::Push => "Both hands finish on the same total.",
                    _ => "The hand is complete.",
                },
                format!(
                    "YOU {}  ·  DEALER {}  ·  WINS {}",
                    game.player_total(),
                    game.dealer_total(),
                    game.wins
                ),
                UiAction::BlackjackNew,
                "DEAL AGAIN",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::SpiderSolitaire => {
            let game = &state.games.spider_solitaire;
            if game.status != crate::spider_solitaire::SpiderSolitaireStatus::Won {
                return None;
            }
            result(
                state,
                game,
                ResultKind::Won,
                "All eight suited runs have been removed.",
                format!("RUNS {} / 8  ·  MOVES {}", game.completed, game.moves),
                UiAction::SpiderSolitaireNew,
                "NEW DEAL",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::Pyramid => {
            let game = &state.games.pyramid;
            let kind = match game.status {
                crate::pyramid::PyramidStatus::Won => ResultKind::Won,
                crate::pyramid::PyramidStatus::Stuck => ResultKind::Stuck,
                crate::pyramid::PyramidStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "Every card has left the pyramid."
                } else {
                    "No legal pair remains and the stock cannot help."
                },
                format!(
                    "POINTS {}  ·  MOVES {}  ·  REDEALS {}",
                    game.points, game.moves, game.redeals_remaining
                ),
                UiAction::PyramidNew,
                "NEW PYRAMID",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        GameId::TriPeaks => {
            let game = &state.games.tri_peaks;
            let kind = match game.status {
                crate::tri_peaks::TriPeaksStatus::Won => ResultKind::Won,
                crate::tri_peaks::TriPeaksStatus::Stuck => ResultKind::Stuck,
                crate::tri_peaks::TriPeaksStatus::Playing => return None,
            };
            result(
                state,
                game,
                kind,
                if kind == ResultKind::Won {
                    "All three peaks are clear."
                } else {
                    "No exposed peak card can be played."
                },
                format!(
                    "POINTS {}  ·  MOVES {}  ·  RUN {}",
                    game.points, game.moves, game.run
                ),
                UiAction::TriPeaksNew,
                "NEW PEAKS",
                UiAction::Cabinet,
                "CABINET",
            )
        }
        _ => None,
    }
}
