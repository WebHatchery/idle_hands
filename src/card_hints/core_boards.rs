use super::color_name;
use crate::state::AppState;

pub fn game_2048(state: &AppState) -> String {
    if state.games.game.won() {
        return "2048 is already on the board — keep exploring or start over.".into();
    }
    state.games.game.hint_direction().map_or_else(
        || "No legal slide remains — tap NEW GAME to begin again.".into(),
        |direction| format!("Try the {} arrow.", direction_name(direction)),
    )
}

fn direction_name(direction: crate::domain::Direction) -> &'static str {
    match direction {
        crate::domain::Direction::Up => "UP",
        crate::domain::Direction::Right => "RIGHT",
        crate::domain::Direction::Down => "DOWN",
        crate::domain::Direction::Left => "LEFT",
    }
}

pub fn tic_tac_toe(state: &AppState) -> String {
    let game = &state.games.tic_tac_toe;
    if game.status != crate::tic_tac_toe::TicTacToeStatus::Playing {
        return "The board is settled — tap NEW BOARD for another round.".into();
    }
    game.hint_move().map_or_else(
        || "No empty square remains — tap NEW BOARD for another round.".into(),
        |index| format!("Try square {}.", index + 1),
    )
}

pub fn lights_out(state: &AppState) -> String {
    let game = &state.games.lights_out;
    if game.status == crate::lights_out::LightsOutStatus::Won {
        return "Every light is already settled — tap NEW BOARD to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No press is available — tap NEW BOARD to begin again.".into(),
        |index| {
            format!(
                "Press row {}, column {}; it begins an exact {}-press route.",
                index / crate::lights_out::SIZE + 1,
                index % crate::lights_out::SIZE + 1,
                game.minimum_solution().len()
            )
        },
    )
}

pub fn memory_pairs(state: &AppState) -> String {
    let game = &state.games.memory_pairs;
    if game.status == crate::memory_pairs::MemoryStatus::Won {
        return "Every pair is already resting — tap NEW BOARD to play again.".into();
    }
    if let Some((first, second)) = game.hint_pair() {
        return format!(
            "You have seen a pair: cards {} and {}.",
            first + 1,
            second + 1
        );
    }
    game.hint_choice().map_or_else(
        || "No unmatched card remains — tap NEW BOARD to begin again.".into(),
        |index| format!("No known pair yet; inspect unseen card {}.", index + 1),
    )
}

pub fn sliding_puzzle(state: &AppState) -> String {
    let game = &state.games.sliding_puzzle;
    if game.status == crate::sliding_puzzle::SlidingStatus::Won {
        return "The tiles are already in order — tap NEW BOARD to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No tile can move — tap NEW BOARD to begin again.".into(),
        |index| format!("Tap tile {} beside the empty space.", game.cells[index]),
    )
}

pub fn mastermind(state: &AppState) -> String {
    let game = &state.games.mastermind;
    match game.status {
        crate::mastermind::MastermindStatus::Won => {
            return "The code is already open — tap NEW BOARD to play again.".into();
        }
        crate::mastermind::MastermindStatus::Lost => {
            return "The code held firm — tap NEW BOARD to begin again.".into();
        }
        crate::mastermind::MastermindStatus::Playing => {}
    }
    game.hint_pick().map_or_else(
        || "Try a color in the next open slot.".into(),
        |(slot, color)| format!("Try the {} peg in slot {}.", color_name(color), slot + 1),
    )
}

pub fn sudoku(state: &AppState) -> String {
    let game = &state.games.sudoku;
    if game.status == crate::sudoku::SudokuStatus::Won {
        return "The grid is already complete — choose a difficulty to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No safe entry is available — try UNDO or choose a new difficulty.".into(),
        |(index, value)| {
            format!(
                "Enter {} in row {}, column {}.",
                value,
                index / 9 + 1,
                index % 9 + 1
            )
        },
    )
}

pub fn minesweeper(state: &AppState) -> String {
    let game = &state.games.minesweeper;
    match game.status {
        crate::minesweeper::MineStatus::Won => {
            return "The field is already clear — tap RESTART to play again.".into();
        }
        crate::minesweeper::MineStatus::Lost => {
            return "A mine was found — tap RESTART to begin again.".into();
        }
        crate::minesweeper::MineStatus::Ready | crate::minesweeper::MineStatus::Playing => {}
    }
    game.hint_move().map_or_else(
        || "No certain square is visible — inspect another revealed number.".into(),
        |(index, safe)| {
            let row = index / game.width + 1;
            let column = index % game.width + 1;
            if safe {
                format!("Reveal row {}, column {}.", row, column)
            } else {
                format!("Flag row {}, column {}.", row, column)
            }
        },
    )
}

pub fn nonogram(state: &AppState) -> String {
    let game = &state.games.nonogram;
    if game.status == crate::nonogram::NonogramStatus::Won {
        return "The hidden picture is complete — choose a size to play again.".into();
    }
    game.hint_cell().map_or_else(
        || "No empty square remains — inspect the marked picture.".into(),
        |(index, filled)| {
            format!(
                "{} row {}, column {}.",
                if filled { "Fill" } else { "Cross" },
                index / game.size + 1,
                index % game.size + 1
            )
        },
    )
}

pub fn word_search(state: &AppState) -> String {
    let game = &state.games.word_search;
    if game.status == crate::word_search::WordSearchStatus::Won {
        return "Every hidden word is found — tap NEW BOARD to search again.".into();
    }
    game.hint_word().map_or_else(
        || "No hidden word remains — tap NEW BOARD to search again.".into(),
        |(word, start, end)| {
            format!(
                "Try {} from row {}, column {} to row {}, column {}.",
                game.words()[word],
                start / 10 + 1,
                start % 10 + 1,
                end / 10 + 1,
                end % 10 + 1
            )
        },
    )
}

pub fn hangman(state: &AppState) -> String {
    let game = &state.games.hangman;
    match game.status {
        crate::hangman::HangmanStatus::Won => {
            return "The word is already yours — tap NEW WORD to play again.".into();
        }
        crate::hangman::HangmanStatus::Lost => {
            return "The word slipped away — tap NEW WORD to begin again.".into();
        }
        crate::hangman::HangmanStatus::Playing => {}
    }
    game.hint_letter().map_or_else(
        || "No unguessed candidate letter remains — tap NEW WORD to begin again.".into(),
        |letter| {
            format!(
                "Try {} across {} remaining {} words; REVEAL is still available ×{}.",
                char::from(b'A' + letter),
                game.candidate_count(),
                game.category.label(),
                game.reveals
            )
        },
    )
}

pub fn connect_four(state: &AppState) -> String {
    let game = &state.games.connect_four;
    match game.status {
        crate::connect_four::ConnectFourStatus::Won(_) => {
            return "The row is already settled — tap NEW BOARD to play again.".into();
        }
        crate::connect_four::ConnectFourStatus::Draw => {
            return "The board is full — tap NEW BOARD to begin again.".into();
        }
        crate::connect_four::ConnectFourStatus::Playing => {}
    }
    game.hint_column().map_or_else(
        || "No column remains — tap NEW BOARD to begin again.".into(),
        |column| format!("Drop a disc in column {}.", column + 1),
    )
}

pub fn checkers(state: &AppState) -> String {
    let game = &state.games.checkers;
    match game.status {
        crate::checkers::CheckersStatus::Won(_) => {
            return "The board is already settled — tap NEW BOARD to play again.".into();
        }
        crate::checkers::CheckersStatus::Draw => {
            return "The board rests in a draw — tap NEW BOARD to begin again.".into();
        }
        crate::checkers::CheckersStatus::Playing => {}
    }
    if game.turn != crate::checkers::Side::Red {
        return "Yellow is answering — wait for your next turn.".into();
    }
    game.hint_move().map_or_else(
        || "No legal red move remains — tap NEW BOARD to begin again.".into(),
        |(from, to)| format!("Tap square {}, then square {}.", from + 1, to + 1),
    )
}

pub fn reversi(state: &AppState) -> String {
    let game = &state.games.reversi;
    if game.status == crate::reversi::ReversiStatus::Won {
        return "The board is already settled — choose NEW BOARD to play again.".into();
    }
    if game.turn != 1 {
        return "The opponent is thinking — wait for your next turn.".into();
    }
    game.hint_move().map_or_else(
        || "No legal move remains — try PASS or NEW BOARD.".into(),
        |index| format!("Play row {}, column {}.", index / 8 + 1, index % 8 + 1),
    )
}

pub fn peg_solitaire(state: &AppState) -> String {
    let game = &state.games.peg_solitaire;
    match game.status {
        crate::peg_solitaire::PegSolitaireStatus::Won => {
            return "One peg remains — tap NEW BOARD to play again.".into();
        }
        crate::peg_solitaire::PegSolitaireStatus::Stuck => {
            return "No jump remains — tap NEW BOARD to begin again.".into();
        }
        crate::peg_solitaire::PegSolitaireStatus::Playing => {}
    }
    game.hint_move().map_or_else(
        || "No legal jump remains — tap NEW BOARD to begin again.".into(),
        |(from, to)| format!("Tap hole {}, then hole {}.", from + 1, to + 1),
    )
}

pub fn mahjong_solitaire(state: &AppState) -> String {
    let game = &state.games.mahjong_solitaire;
    match game.status {
        crate::mahjong_solitaire::MahjongStatus::Won => {
            return "Every tile is clear — tap NEW BOARD to play again.".into();
        }
        crate::mahjong_solitaire::MahjongStatus::Stuck => {
            return "No free pair remains — tap NEW BOARD to begin again.".into();
        }
        crate::mahjong_solitaire::MahjongStatus::Playing => {}
    }
    game.hint_pair().map_or_else(
        || "No removable pair remains — tap NEW BOARD to begin again.".into(),
        |(first, second)| format!("Pair tiles {} and {}.", first + 1, second + 1),
    )
}

pub fn is_hint(action: crate::ui::UiAction) -> bool {
    matches!(
        action,
        crate::ui::UiAction::Game2048Hint
            | crate::ui::UiAction::TicTacToeHint
            | crate::ui::UiAction::LightsOutHint
            | crate::ui::UiAction::SolitaireHint
            | crate::ui::UiAction::FreeCellHint
            | crate::ui::UiAction::FivefoldHint
            | crate::ui::UiAction::SpiderHint
            | crate::ui::UiAction::PyramidHint
            | crate::ui::UiAction::TriPeaksHint
            | crate::ui::UiAction::KlondikeGolfHint
            | crate::ui::UiAction::SpiderSolitaireHint
            | crate::ui::UiAction::NimHint
            | crate::ui::UiAction::MemoryPairsHint
            | crate::ui::UiAction::SlidingPuzzleHint
            | crate::ui::UiAction::MastermindHint
            | crate::ui::UiAction::SudokuHint
            | crate::ui::UiAction::MineHint
            | crate::ui::UiAction::NonogramHint
            | crate::ui::UiAction::WordSearchHint
            | crate::ui::UiAction::HangmanHint
            | crate::ui::UiAction::ConnectFourHint
            | crate::ui::UiAction::CheckersHint
            | crate::ui::UiAction::ReversiHint
            | crate::ui::UiAction::PegSolitaireHint
            | crate::ui::UiAction::MahjongSolitaireHint
            | crate::ui::UiAction::SnakeHint
            | crate::ui::UiAction::BreakoutHint
            | crate::ui::UiAction::HigherLowerHint
            | crate::ui::UiAction::BlackjackHint
            | crate::ui::UiAction::DungeonHint
            | crate::ui::UiAction::PotionHint
            | crate::ui::UiAction::TowerHint
            | crate::ui::UiAction::RogueHint
            | crate::ui::UiAction::DailyHint
            | crate::ui::UiAction::DotsHint
            | crate::ui::UiAction::SokobanHint
            | crate::ui::UiAction::MancalaHint
            | crate::ui::UiAction::HanoiHint
            | crate::ui::UiAction::NumberMatchHint
            | crate::ui::UiAction::FloodHint
            | crate::ui::UiAction::ColorSortHint
            | crate::ui::UiAction::BattleshipHint
            | crate::ui::UiAction::WordGridHint
            | crate::ui::UiAction::PipeHint
            | crate::ui::UiAction::MazeHint
            | crate::ui::UiAction::MatchThreeHint
            | crate::ui::UiAction::MiscHint
            | crate::ui::UiAction::WordLadderHint
    )
}

pub fn word_ladder(state: &AppState) -> String {
    let game = &state.games.word_ladder;
    if game.phase == crate::word_ladder::WordLadderPhase::Won {
        return "The ladder is complete — tap NEW LADDER to climb again.".into();
    }
    game.hint_word().map_or_else(
        || "No single-letter step is showing — tap UNDO or NEW LADDER.".into(),
        |word| {
            let objective =
                if game.mode == crate::word_ladder::LadderMode::Scenic && !game.waypoint_reached {
                    "scenic waypoint"
                } else {
                    "target"
                };
            format!(
                "Try {} next toward the {} ({} steps remain).",
                word,
                objective,
                game.remaining_steps()
            )
        },
    )
}
