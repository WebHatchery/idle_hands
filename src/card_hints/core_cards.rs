use crate::{freecell::FreeSource, state::AppState};

pub fn solitaire(state: &AppState) -> String {
    let game = &state.games.solitaire;
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
    let game = &state.games.freecell;
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
    if let Some((source, destination)) = game.hint_cascade_move() {
        return match source {
            FreeSource::Cell(cell) => {
                format!("Try CELL {} on cascade {}.", cell + 1, destination + 1)
            }
            FreeSource::Cascade(cascade, depth) if depth + 1 == game.cascades[cascade].len() => {
                format!(
                    "Try the top card in cascade {} on cascade {}.",
                    cascade + 1,
                    destination + 1
                )
            }
            FreeSource::Cascade(cascade, _) => {
                format!(
                    "Try the visible stack in cascade {} on cascade {}.",
                    cascade + 1,
                    destination + 1
                )
            }
        };
    }
    if game.cascades.iter().any(Vec::is_empty) {
        return "Try moving a visible card into an empty cascade.".into();
    }
    "No obvious move — try UNDO and keep every card visible.".into()
}

pub fn fivefold(state: &AppState) -> String {
    let game = &state.games.fivefold;
    if game.status == crate::fivefold::FivefoldStatus::Complete {
        return "The scorecard is complete — tap NEW SCORECARD to play again.".into();
    }
    if game.roll_number == 0 {
        return "Roll DICE before choosing a call.".into();
    }
    game.hint_category().map_or_else(
        || "No open call remains — tap NEW SCORECARD to begin again.".into(),
        |category| {
            format!(
                "Choose {} for {} points.",
                category.label(),
                game.score_for(category)
            )
        },
    )
}

pub fn spider(state: &AppState) -> String {
    let game = &state.games.spider;
    if game.status == crate::spider::SpiderStatus::Won {
        return "All eight spider runs are clear — tap NEW DEAL to play again.".into();
    }
    if let Some((source, depth, destination)) = game.hint_move() {
        return format!(
            "Move column {} card {} to column {}.",
            source + 1,
            depth + 1,
            destination + 1
        );
    }
    if !game.stock.is_empty() {
        "Deal STOCK for the next row.".into()
    } else {
        "No legal run remains — tap NEW DEAL to begin again.".into()
    }
}

pub fn pyramid(state: &AppState) -> String {
    let game = &state.games.pyramid;
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
    let game = &state.games.tri_peaks;
    if game.status == crate::tri_peaks::TriPeaksStatus::Won {
        return "The three peaks are already clear.".into();
    }
    if let Some(index) = (0..28).find(|&index| game.can_play(index)) {
        return format!(
            "Play outlined card {} to extend the run to {} for {} points.",
            index + 1,
            game.run + 1,
            10 * u32::from(game.run + 1) + if index < 3 { 50 } else { 0 }
        );
    }
    if game.bridges > 0 {
        return "Tap BRIDGE, then any exposed card, to rescue this route.".into();
    }
    if !game.stock.is_empty() {
        "Tap STOCK to reveal another card.".into()
    } else {
        "No peak card can play — try UNDO or a new deal.".into()
    }
}

pub fn klondike_golf(state: &AppState) -> String {
    let game = &state.games.klondike_golf;
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
    let game = &state.games.spider_solitaire;
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
    let game = &state.games.nim;
    match game.status {
        crate::nim::NimStatus::Won => return "The final stone is already yours.".into(),
        crate::nim::NimStatus::Lost => return "Tap NEW BOARD to begin another heap set.".into(),
        crate::nim::NimStatus::Playing => {}
    }
    game.hint_move().map_or_else(
        || "No stones remain — tap NEW BOARD to begin again.".into(),
        |(heap, amount)| {
            format!(
                "Select heap {} and tap TAKE {} for a forced-win route under {} rules.",
                heap + 1,
                amount,
                game.rule.label()
            )
        },
    )
}
