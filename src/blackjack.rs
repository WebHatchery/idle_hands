//! Deterministic touch-first Blackjack.

use crate::cards::{shuffled_deck, Card};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlackjackStatus {
    Playing,
    Won,
    Lost,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlackjackHint {
    Hit,
    Stand,
}

type Snapshot = (
    Vec<Card>,
    Vec<Card>,
    Vec<Card>,
    u16,
    u16,
    BlackjackStatus,
    u64,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blackjack {
    pub player: Vec<Card>,
    pub dealer: Vec<Card>,
    pub deck: Vec<Card>,
    pub rounds: u16,
    pub wins: u16,
    pub status: BlackjackStatus,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<Snapshot>,
}

impl Default for Blackjack {
    fn default() -> Self {
        Self::new(0xB1AC_0001)
    }
}

impl Blackjack {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            player: Vec::new(),
            dealer: Vec::new(),
            deck: Vec::new(),
            rounds: 0,
            wins: 0,
            status: BlackjackStatus::Playing,
            seed,
            undo: None,
        };
        game.start_round(seed);
        game
    }

    pub fn hit(&mut self) -> bool {
        if self.status != BlackjackStatus::Playing || self.deck.is_empty() {
            return false;
        }
        self.snapshot();
        self.player
            .push(self.deck.pop().expect("deck checked above"));
        if self.total(&self.player) > 21 {
            self.status = BlackjackStatus::Lost;
        }
        true
    }

    pub fn stand(&mut self) -> bool {
        if self.status != BlackjackStatus::Playing {
            return false;
        }
        self.snapshot();
        while self.total(&self.dealer) < 17 && !self.deck.is_empty() {
            self.dealer
                .push(self.deck.pop().expect("deck checked above"));
        }
        let player = self.total(&self.player);
        let dealer = self.total(&self.dealer);
        self.status = if dealer > 21 || player > dealer {
            BlackjackStatus::Won
        } else if player < dealer {
            BlackjackStatus::Lost
        } else {
            BlackjackStatus::Push
        };
        if self.status == BlackjackStatus::Won {
            self.wins = self.wins.saturating_add(1);
        }
        true
    }

    pub fn hint_action(&self) -> Option<BlackjackHint> {
        if self.status != BlackjackStatus::Playing {
            return None;
        }
        let player = self.player_total();
        let dealer_upcard = self.dealer.get(1)?.rank.min(10);
        let should_stand = player >= 17 || (player > 11 && dealer_upcard <= 6);
        Some(if should_stand {
            BlackjackHint::Stand
        } else {
            BlackjackHint::Hit
        })
    }

    pub fn undo(&mut self) -> bool {
        if let Some((player, dealer, deck, rounds, wins, status, seed)) = self.undo.take() {
            self.player = player;
            self.dealer = dealer;
            self.deck = deck;
            self.rounds = rounds;
            self.wins = wins;
            self.status = status;
            self.seed = seed;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, seed: u64) {
        let rounds = self.rounds.saturating_add(1);
        let wins = self.wins;
        self.start_round(seed);
        self.rounds = rounds;
        self.wins = wins;
        if self.player_total() == 21 {
            self.status = BlackjackStatus::Won;
            self.wins = self.wins.saturating_add(1);
        }
    }

    pub fn player_total(&self) -> u8 {
        self.total(&self.player)
    }

    pub fn dealer_total(&self) -> u8 {
        self.total(&self.dealer)
    }

    fn start_round(&mut self, seed: u64) {
        let (mut deck, shuffled_seed) = shuffled_deck(seed, true);
        deck.reverse();
        self.player = vec![
            deck.pop().expect("fresh deck"),
            deck.pop().expect("fresh deck"),
        ];
        self.dealer = vec![
            deck.pop().expect("fresh deck"),
            deck.pop().expect("fresh deck"),
        ];
        self.deck = deck;
        self.rounds = 1;
        self.wins = 0;
        self.status = BlackjackStatus::Playing;
        self.seed = shuffled_seed;
        self.undo = None;
        if self.player_total() == 21 {
            self.status = BlackjackStatus::Won;
            self.wins = 1;
        }
    }

    fn snapshot(&mut self) {
        self.undo = Some((
            self.player.clone(),
            self.dealer.clone(),
            self.deck.clone(),
            self.rounds,
            self.wins,
            self.status,
            self.seed,
        ));
    }

    fn total(&self, hand: &[Card]) -> u8 {
        let mut total = 0u8;
        let mut aces = 0u8;
        for card in hand {
            total = total.saturating_add(card.rank.min(10));
            if card.rank == 1 {
                aces = aces.saturating_add(1);
            }
        }
        while aces > 0 && total.saturating_add(10) <= 21 {
            total = total.saturating_add(10);
            aces -= 1;
        }
        total
    }
}

#[cfg(test)]
mod tests;
