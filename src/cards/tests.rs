use super::*;
use std::collections::HashSet;

#[test]
fn seeded_decks_are_repeatable_and_contain_each_card_once() {
    let (left, left_seed) = shuffled_deck(42, true);
    let (right, right_seed) = shuffled_deck(42, true);
    assert_eq!(left, right);
    assert_eq!(left_seed, right_seed);
    assert_eq!(left.len(), 52);
    let identities = left
        .iter()
        .map(|card| (card.rank, card.suit))
        .collect::<HashSet<_>>();
    assert_eq!(identities.len(), 52);
}

#[test]
fn deck_face_state_is_set_by_the_caller() {
    assert!(shuffled_deck(1, true).0.iter().all(|card| card.face_up));
    assert!(shuffled_deck(1, false).0.iter().all(|card| !card.face_up));
}
