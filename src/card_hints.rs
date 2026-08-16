//! Deterministic, non-playing hints for the two card games.

use crate::{freecell::FreeSource, state::AppState};

pub fn solitaire(state: &AppState) -> String {
    let game = &state.solitaire;
    if game.status == crate::solitaire::SolitaireStatus::Won {
        return "The table is already cleared.".into();
    }
    if let Some(column) = game.tableau.iter().enumerate().find_map(|(column, cards)| {
        cards.last().and_then(|card| {
            (card.rank == game.foundations[card.suit as usize] + 1).then_some(column)
        })
    }) {
        return format!(
            "Try the top card in tableau {} on its foundation.",
            column + 1
        );
    }
    if let Some(card) = game.waste.last().copied() {
        if card.rank == game.foundations[card.suit as usize] + 1 {
            return "Try the waste card on its foundation.".into();
        }
        if game.tableau.iter().any(|cards| {
            cards.last().is_none_or(|top| {
                top.face_up && top.rank == card.rank + 1 && top.red() != card.red()
            })
        }) {
            return "Try moving the waste card onto a tableau column.".into();
        }
    }
    for (source_column, cards) in game.tableau.iter().enumerate() {
        let Some(card) = cards.last().copied() else {
            continue;
        };
        if game
            .tableau
            .iter()
            .enumerate()
            .any(|(destination, target)| {
                destination != source_column
                    && target.last().is_none_or(|top| {
                        top.face_up && top.rank == card.rank + 1 && top.red() != card.red()
                    })
            })
        {
            return format!(
                "Try moving the top card from tableau {}.",
                source_column + 1
            );
        }
    }
    if !game.stock.is_empty() || !game.waste.is_empty() {
        "Try STOCK to reveal another card.".into()
    } else {
        "No obvious move — try UNDO or inspect the foundations.".into()
    }
}

pub fn freecell(state: &AppState) -> String {
    let game = &state.freecell;
    if game.status == crate::freecell::FreeCellStatus::Won {
        return "Every foundation is complete.".into();
    }
    let mut sources = game
        .cells
        .iter()
        .enumerate()
        .filter_map(|(cell, card)| card.map(|card| (FreeSource::Cell(cell), card)))
        .chain(
            game.cascades
                .iter()
                .enumerate()
                .filter_map(|(cascade, cards)| {
                    cards
                        .last()
                        .copied()
                        .map(|card| (FreeSource::Cascade(cascade, cards.len() - 1), card))
                }),
        );
    if let Some((source, _card)) =
        sources.find(|(_, card)| card.rank == game.foundations[card.suit as usize] + 1)
    {
        return match source {
            FreeSource::Cell(cell) => format!("Try CELL {} on its foundation.", cell + 1),
            FreeSource::Cascade(cascade, _) => {
                format!(
                    "Try the top card in cascade {} on its foundation.",
                    cascade + 1
                )
            }
        };
    }
    if game.cascades.iter().any(Vec::is_empty) {
        return "Try moving a visible card into an empty cascade.".into();
    }
    "No obvious move — try UNDO and keep every card visible.".into()
}

pub fn pyramid(state: &AppState) -> String {
    let game = &state.pyramid;
    if game.status == crate::pyramid::PyramidStatus::Won {
        return "The pyramid is already clear.".into();
    }
    let exposed = (0..28).filter(|&index| game.available(index));
    if let Some(index) = exposed
        .clone()
        .find(|&index| game.pyramid[index].is_some_and(|card| card.rank == 13))
    {
        return format!("Tap the king in pyramid position {}.", index + 1);
    }
    let cards = exposed
        .filter_map(|index| game.pyramid[index].map(|card| (index, card)))
        .collect::<Vec<_>>();
    if let Some(waste) = game.waste.last() {
        if let Some((index, _)) = cards
            .iter()
            .find(|(_, card)| card.rank.saturating_add(waste.rank) == 13)
        {
            return format!("Tap WASTE, then exposed card {}.", index + 1);
        }
    }
    if let Some((left, right)) = cards.iter().enumerate().find_map(|(index, (_, card))| {
        cards
            .iter()
            .skip(index + 1)
            .find(|(_, other)| card.rank.saturating_add(other.rank) == 13)
            .map(|(right, _)| (cards[index].0, *right))
    }) {
        return format!("Pair exposed cards {} and {}.", left + 1, right + 1);
    }
    if !game.stock.is_empty() {
        "Tap STOCK to reveal another card.".into()
    } else {
        "No obvious move — try UNDO or inspect the waste.".into()
    }
}

pub fn tri_peaks(state: &AppState) -> String {
    let game = &state.tri_peaks;
    if game.status == crate::tri_peaks::TriPeaksStatus::Won {
        return "The three peaks are already clear.".into();
    }
    if let Some(index) = (0..28).find(|&index| game.can_play(index)) {
        return format!("Try the playable peak card at position {}.", index + 1);
    }
    if !game.stock.is_empty() {
        "Tap STOCK to reveal another card.".into()
    } else {
        "No peak card can play — try UNDO or a new deal.".into()
    }
}

pub fn klondike_golf(state: &AppState) -> String {
    let game = &state.klondike_golf;
    if game.status == crate::klondike_golf::GolfStatus::Won {
        return "The golf columns are already clear.".into();
    }
    let Some(waste) = game.waste.last() else {
        return "Tap STOCK to reveal the first waste card.".into();
    };
    if let Some(column) = game.tableau.iter().enumerate().find_map(|(column, stack)| {
        stack
            .last()
            .is_some_and(|card| card.rank.abs_diff(waste.rank) == 1)
            .then_some(column)
    }) {
        return format!("Tap the playable card in column {}.", column + 1);
    }
    if !game.stock.is_empty() {
        "Tap STOCK to reveal another waste card.".into()
    } else {
        "No golf card can play — try UNDO or a new board.".into()
    }
}

pub fn spider_solitaire(state: &AppState) -> String {
    let game = &state.spider_solitaire;
    if game.status == crate::spider_solitaire::SpiderSolitaireStatus::Won {
        return "All eight spider runs are already clear.".into();
    }
    for (source, stack) in game.tableau.iter().enumerate() {
        for depth in 0..stack.len() {
            if !crate::spider_solitaire::is_run(&stack[depth..]) {
                continue;
            }
            let card = stack[depth];
            if let Some(destination) =
                game.tableau
                    .iter()
                    .enumerate()
                    .find_map(|(column, target)| {
                        if column == source {
                            return None;
                        }
                        target
                            .last()
                            .is_none_or(|top| top.face_up && top.rank == card.rank + 1)
                            .then_some(column)
                    })
            {
                return format!(
                    "Move the run from column {} to column {}.",
                    source + 1,
                    destination + 1
                );
            }
        }
    }
    if !game.stock.is_empty() {
        "Tap STOCK to deal one card to each column.".into()
    } else {
        "No obvious run move — try UNDO or a new deal.".into()
    }
}

pub fn nim(state: &AppState) -> String {
    let game = &state.nim;
    match game.status {
        crate::nim::NimStatus::Won => return "The final stone is already yours.".into(),
        crate::nim::NimStatus::Lost => return "Tap NEW BOARD to begin another heap set.".into(),
        crate::nim::NimStatus::Playing => {}
    }
    let xor = game.heaps.iter().fold(0, |total, heap| total ^ heap);
    let best = game
        .heaps
        .iter()
        .enumerate()
        .find_map(|(heap, &stones)| {
            let target = stones ^ xor;
            (target < stones && stones - target <= 3).then_some((heap, stones - target))
        })
        .or_else(|| {
            game.heaps
                .iter()
                .enumerate()
                .find(|(_, stones)| **stones > 0)
                .map(|(heap, stones)| (heap, (*stones).min(3)))
        });
    best.map_or_else(
        || "No stones remain — tap NEW BOARD to begin again.".into(),
        |(heap, amount)| format!("Select heap {} and tap TAKE {}.", heap + 1, amount),
    )
}

pub fn game_2048(state: &AppState) -> String {
    if state.game.won() {
        return "2048 is already on the board — keep exploring or start over.".into();
    }
    state.game.hint_direction().map_or_else(
        || "No legal slide remains — tap NEW GAME to begin again.".into(),
        |direction| format!("Try the {} arrow.", direction_name(direction)),
    )
}

fn direction_name(direction: crate::state::Direction) -> &'static str {
    match direction {
        crate::state::Direction::Up => "UP",
        crate::state::Direction::Right => "RIGHT",
        crate::state::Direction::Down => "DOWN",
        crate::state::Direction::Left => "LEFT",
    }
}

pub fn tic_tac_toe(state: &AppState) -> String {
    let game = &state.tic_tac_toe;
    if game.status != crate::tic_tac_toe::TicTacToeStatus::Playing {
        return "The board is settled — tap NEW BOARD for another round.".into();
    }
    game.hint_move().map_or_else(
        || "No empty square remains — tap NEW BOARD for another round.".into(),
        |index| format!("Try square {}.", index + 1),
    )
}

pub fn lights_out(state: &AppState) -> String {
    let game = &state.lights_out;
    if game.status == crate::lights_out::LightsOutStatus::Won {
        return "Every light is already quiet — tap NEW BOARD to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No press is available — tap NEW BOARD to begin again.".into(),
        |index| format!("Try square {} to quiet the field.", index + 1),
    )
}

pub fn memory_pairs(state: &AppState) -> String {
    let game = &state.memory_pairs;
    if game.status == crate::memory_pairs::MemoryStatus::Won {
        return "Every pair is already resting — tap NEW BOARD to play again.".into();
    }
    game.hint_pair().map_or_else(
        || "No unmatched pair remains — tap NEW BOARD to begin again.".into(),
        |(first, second)| format!("Pair cards {} and {}.", first + 1, second + 1),
    )
}

pub fn sliding_puzzle(state: &AppState) -> String {
    let game = &state.sliding_puzzle;
    if game.status == crate::sliding_puzzle::SlidingStatus::Won {
        return "The tiles are already in order — tap NEW BOARD to play again.".into();
    }
    game.hint_move().map_or_else(
        || "No tile can move — tap NEW BOARD to begin again.".into(),
        |index| format!("Tap tile {} beside the empty space.", game.cells[index]),
    )
}

pub fn mastermind(state: &AppState) -> String {
    let game = &state.mastermind;
    match game.status {
        crate::mastermind::MastermindStatus::Won => {
            return "The code is already open — tap NEW BOARD to play again.".into();
        }
        crate::mastermind::MastermindStatus::Lost => {
            return "The code stayed quiet — tap NEW BOARD to begin again.".into();
        }
        crate::mastermind::MastermindStatus::Playing => {}
    }
    game.hint_pick().map_or_else(
        || "Try a color in the next open slot.".into(),
        |(slot, color)| format!("Try the {} peg in slot {}.", color_name(color), slot + 1),
    )
}

pub fn sudoku(state: &AppState) -> String {
    let game = &state.sudoku;
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
    let game = &state.minesweeper;
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

fn color_name(color: u8) -> &'static str {
    ["red", "amber", "green", "blue", "violet", "gold"][color as usize % 6]
}

pub fn is_hint(action: crate::ui::UiAction) -> bool {
    matches!(
        action,
        crate::ui::UiAction::Game2048Hint
            | crate::ui::UiAction::TicTacToeHint
            | crate::ui::UiAction::LightsOutHint
            | crate::ui::UiAction::SolitaireHint
            | crate::ui::UiAction::FreeCellHint
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
    )
}

#[cfg(test)]
mod tests;
