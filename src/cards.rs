//! Shared card identity and deterministic deck construction for the card games.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub rank: u8,
    pub suit: u8,
    pub face_up: bool,
}

impl Card {
    pub fn red(self) -> bool {
        matches!(self.suit, 1 | 2)
    }
}

pub fn shuffled_deck(seed: u64, face_up: bool) -> (Vec<Card>, u64) {
    let deck = (0..4)
        .flat_map(|suit| {
            (1..=13).map(move |rank| Card {
                rank,
                suit,
                face_up,
            })
        })
        .collect::<Vec<_>>();
    shuffle_cards(deck, seed)
}

pub fn shuffle_cards(mut cards: Vec<Card>, seed: u64) -> (Vec<Card>, u64) {
    let mut rng = seed;
    for index in (1..cards.len()).rev() {
        rng = next_seed(rng);
        cards.swap(index, (rng as usize) % (index + 1));
    }
    (cards, rng)
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
#[path = "../tests/legacy/cards/tests.rs"]
mod tests;
