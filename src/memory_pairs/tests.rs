use super::*;

#[test]
fn seeded_deals_repeat_and_contain_each_pair_twice() {
    let first = MemoryPairs::new(42);
    assert_eq!(first.cards, MemoryPairs::new(42).cards);
    for pair in 0..PAIRS as u8 {
        assert_eq!(
            first.cards.iter().filter(|card| card.pair == pair).count(),
            2
        );
    }
}

#[test]
fn matching_two_cards_marks_the_pair_and_counts_one_move() {
    let mut game = MemoryPairs::new(42);
    let pair = game.cards[0].pair;
    let second = (1..CELLS)
        .find(|&index| game.cards[index].pair == pair)
        .unwrap();
    assert!(game.select(0));
    assert!(game.select(second));
    assert!(game.cards[0].matched);
    assert!(game.cards[second].matched);
    assert_eq!(game.matched_pairs, 1);
    assert_eq!(game.moves, 1);
}

#[test]
fn a_mismatch_stays_visible_until_the_next_selection_and_undo_restores_it() {
    let mut game = MemoryPairs::new(7);
    let first_pair = game.cards[0].pair;
    let mismatch = game
        .cards
        .iter()
        .position(|card| card.pair != first_pair)
        .unwrap();
    assert!(game.select(0));
    assert!(game.select(mismatch));
    assert!(game.mismatch_waiting);
    assert!(game.cards[0].face_up && game.cards[mismatch].face_up);
    assert!(game.undo());
    assert!(game.cards[0].face_up);
    assert!(!game.cards[mismatch].face_up);
    assert_eq!(game.selected, [Some(0), None]);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_pair_finds_the_first_unmatched_pair_without_mutating_the_board() {
    let game = MemoryPairs::new(42);
    let before = game.cards;
    let (first, second) = game.hint_pair().unwrap();
    assert_eq!(game.cards[first].pair, game.cards[second].pair);
    assert_eq!(game.cards, before);
}
