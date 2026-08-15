//! Seeded FreeCell rules with tap-selected cards and undo snapshots.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreeCellStatus {
    Playing,
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreeSource {
    Cascade(usize, usize),
    Cell(usize),
}

type FreeCellSnapshot = ([Option<Card>; 4], Vec<Vec<Card>>, [u8; 4], u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCell {
    pub cells: [Option<Card>; 4],
    pub cascades: Vec<Vec<Card>>,
    pub foundations: [u8; 4],
    pub selected: Option<FreeSource>,
    pub moves: u32,
    pub status: FreeCellStatus,
    pub seed: u64,
    #[serde(skip)]
    history: Vec<FreeCellSnapshot>,
}

impl Default for FreeCell {
    fn default() -> Self {
        Self::new(0xF4_EE_11)
    }
}

impl FreeCell {
    pub fn new(seed: u64) -> Self {
        let (deck, rng) = shuffled_deck(seed, true);
        let mut cascades = vec![Vec::new(); 8];
        for (index, card) in deck.into_iter().enumerate() {
            cascades[index % 8].push(card);
        }
        Self {
            cells: [None; 4],
            cascades,
            foundations: [0; 4],
            selected: None,
            moves: 0,
            status: FreeCellStatus::Playing,
            seed: rng,
            history: Vec::new(),
        }
    }

    pub fn select_cascade(&mut self, cascade: usize, depth: usize) -> bool {
        if self
            .cascades
            .get(cascade)
            .and_then(|cards| cards.get(depth))
            .is_some()
        {
            self.selected = Some(FreeSource::Cascade(cascade, depth));
            true
        } else {
            false
        }
    }
    pub fn select_cell(&mut self, cell: usize) -> bool {
        if self.cells.get(cell).is_some_and(Option::is_some) {
            self.selected = Some(FreeSource::Cell(cell));
            true
        } else {
            false
        }
    }
    pub fn move_selected_to_cascade(&mut self, destination: usize) -> bool {
        let Some(source) = self.selected.take() else {
            return false;
        };
        let Some(card) = self.source_card(source) else {
            self.selected = Some(source);
            return false;
        };
        if !self.valid_moving_stack(source)
            || !self.can_place(destination, card)
            || !self.capacity_allows(source, destination)
        {
            self.selected = Some(source);
            return false;
        }
        self.snapshot();
        match source {
            FreeSource::Cell(cell) => self.cells[cell] = None,
            FreeSource::Cascade(cascade, depth) => {
                let moving = self.cascades[cascade].split_off(depth);
                self.cascades[destination].extend(moving);
                self.moves += 1;
                return true;
            }
        }
        self.cascades[destination].push(card);
        self.moves += 1;
        true
    }
    pub fn move_selected_to_foundation(&mut self, suit: usize) -> bool {
        let Some(source) = self.selected.take() else {
            return false;
        };
        let Some(card) = self.source_card(source) else {
            self.selected = Some(source);
            return false;
        };
        if card.suit as usize != suit
            || card.rank != self.foundations[suit] + 1
            || matches!(source, FreeSource::Cascade(c, d) if self.cascades[c].len() != d + 1)
        {
            self.selected = Some(source);
            return false;
        }
        self.snapshot();
        match source {
            FreeSource::Cell(cell) => self.cells[cell] = None,
            FreeSource::Cascade(cascade, _) => {
                self.cascades[cascade].pop();
            }
        }
        self.foundations[suit] += 1;
        self.moves += 1;
        if self.foundations == [13; 4] {
            self.status = FreeCellStatus::Won;
        }
        true
    }
    pub fn undo(&mut self) -> bool {
        if let Some((cells, cascades, foundations, moves)) = self.history.pop() {
            self.cells = cells;
            self.cascades = cascades;
            self.foundations = foundations;
            self.moves = moves;
            self.status = FreeCellStatus::Playing;
            self.selected = None;
            true
        } else {
            false
        }
    }
    fn source_card(&self, source: FreeSource) -> Option<Card> {
        match source {
            FreeSource::Cell(cell) => self.cells.get(cell).copied().flatten(),
            FreeSource::Cascade(cascade, depth) => self
                .cascades
                .get(cascade)
                .and_then(|cards| cards.get(depth))
                .copied(),
        }
    }
    fn can_place(&self, destination: usize, card: Card) -> bool {
        match self
            .cascades
            .get(destination)
            .and_then(|cards| cards.last())
        {
            None => true,
            Some(top) => top.rank == card.rank + 1 && top.red() != card.red(),
        }
    }

    fn valid_moving_stack(&self, source: FreeSource) -> bool {
        let FreeSource::Cascade(cascade, depth) = source else {
            return true;
        };
        self.cascades[cascade][depth..]
            .windows(2)
            .all(|pair| pair[0].rank + 1 == pair[1].rank && pair[0].red() != pair[1].red())
    }
    fn capacity_allows(&self, source: FreeSource, destination: usize) -> bool {
        if !matches!(source, FreeSource::Cascade(_, _)) {
            return true;
        }
        let empty_cells = self.cells.iter().filter(|cell| cell.is_none()).count();
        let empty_cascades = self
            .cascades
            .iter()
            .enumerate()
            .filter(|(index, cards)| *index != destination && cards.is_empty())
            .count();
        let max_cards = (empty_cells + 1) * 2usize.pow(empty_cascades as u32);
        match source {
            FreeSource::Cascade(cascade, depth) => {
                self.cascades[cascade].len() - depth <= max_cards
            }
            FreeSource::Cell(_) => true,
        }
    }
    fn snapshot(&mut self) {
        self.history.push((
            self.cells,
            self.cascades.clone(),
            self.foundations,
            self.moves,
        ));
    }
}
#[cfg(test)]
mod tests;
