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
    let mut game = MemoryPairs::new(42);
    let pair = game.cards[0].pair;
    let second = (1..CELLS)
        .find(|&index| game.cards[index].pair == pair)
        .unwrap();
    game.seen[0] = true;
    game.seen[second] = true;
    let before = game.cards;
    let (first, second) = game.hint_pair().unwrap();
    assert_eq!(game.cards[first].pair, game.cards[second].pair);
    assert_eq!(game.cards, before);
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
fn peek_reveals_two_unknown_cards_until_the_next_selection() {
    let mut game = MemoryPairs::new(45);
    assert!(game.peek());
    assert_eq!(
        (game.peeks, game.seen_count(), game.peek_waiting),
        (0, 2, true)
    );
    let peeked = game.peeked;
    let next = (0..CELLS)
        .find(|index| !peeked.contains(&Some(*index)))
        .unwrap();
    game.select(next);
    for index in peeked.into_iter().flatten() {
        assert!(!game.cards[index].face_up);
    }
    assert!(!game.peek_waiting);
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

#[test]
fn legacy_saves_receive_depth_defaults() {
    let original = MemoryPairs::new(47);
    let mut value = serde_json::to_value(&original).unwrap();
    for field in [
        "seen",
        "score",
        "combo",
        "best_combo",
        "mistakes",
        "peeks",
        "peeked",
        "peek_waiting",
    ] {
        value.as_object_mut().unwrap().remove(field);
    }
    let restored: MemoryPairs = serde_json::from_value(value).unwrap();
    assert_eq!(restored.seen_count(), 0);
    assert_eq!(
        (restored.score, restored.combo, restored.mistakes),
        (0, 0, 0)
    );
    assert_eq!(restored.peeks, 1);
}
