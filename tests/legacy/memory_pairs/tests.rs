//! Regression coverage for the tests module.

use idle_hands::testing::modules::memory_pairs::*;

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
fn hints_do_not_reveal_unknown_pairs() {
    let game = MemoryPairs::new(43);
    assert_eq!(game.hint_pair(), None);
    assert_eq!(game.hint_choice(), Some(0));
}

#[test]
fn matching_streaks_score_and_mismatches_break_the_chain() {
    let mut game = MemoryPairs::new(44);
    let first_pair = game.cards[0].pair;
    let mate = (1..CELLS)
        .find(|&index| game.cards[index].pair == first_pair)
        .unwrap();
    game.select(0);
    game.select(mate);
    assert_eq!((game.score, game.combo, game.best_combo), (20, 1, 1));
    let first = (0..CELLS)
        .find(|&index| !game.cards[index].matched)
        .unwrap();
    let mismatch = (0..CELLS)
        .find(|&index| {
            !game.cards[index].matched && game.cards[index].pair != game.cards[first].pair
        })
        .unwrap();
    game.select(first);
    game.select(mismatch);
    assert_eq!((game.score, game.combo, game.mistakes), (15, 0, 1));
}

#[test]
fn repeated_undo_restores_score_memory_and_peek_charge() {
    let mut game = MemoryPairs::new(46);
    game.peek();
    let next = (0..CELLS)
        .find(|index| !game.peeked.contains(&Some(*index)))
        .unwrap();
    game.select(next);
    assert!(game.undo());
    assert!(game.peek_waiting);
    assert!(game.undo());
    assert_eq!((game.peeks, game.seen_count()), (1, 0));
}
