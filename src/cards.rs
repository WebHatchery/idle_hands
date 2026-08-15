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
        self.suit >= 2
    }
}

pub fn shuffled_deck(seed: u64, face_up: bool) -> (Vec<Card>, u64) {
    let mut deck = (0..4)
        .flat_map(|suit| {
            (1..=13).map(move |rank| Card {
                rank,
                suit,
                face_up,
            })
        })
        .collect::<Vec<_>>();
    let mut rng = seed;
    for index in (1..deck.len()).rev() {
        rng = next_seed(rng);
        deck.swap(index, (rng as usize) % (index + 1));
    }
    (deck, rng)
}

fn next_seed(seed: u64) -> u64 {
    seed.wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

#[cfg(test)]
mod tests;
