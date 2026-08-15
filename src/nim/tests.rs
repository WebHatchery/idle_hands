use super::*;

#[test]
fn seeded_deals_repeat() {
    assert_eq!(Nim::new(7).heaps, Nim::new(7).heaps);
    assert_ne!(Nim::new(7).heaps, Nim::new(8).heaps);
}

#[test]
fn player_turn_requires_selected_heap_and_valid_amount() {
    let mut game = Nim::new(7);
    assert!(!game.take(1));
    game.select_heap(0);
    assert!(!game.take(0));
    assert!(!game.take(4));
    assert!(game.take(1));
    assert_eq!(game.selected_heap, None);
}

#[test]
fn undo_restores_the_position_before_the_player_turn() {
    let mut game = Nim::new(7);
    let heaps = game.heaps;
    game.select_heap(0);
    assert!(game.take(1));
    game.undo();
    assert_eq!(game.heaps, heaps);
    assert_eq!(game.moves, 0);
    assert_eq!(game.status, NimStatus::Playing);
}

#[test]
fn taking_the_last_heap_wins_before_the_opponent_moves() {
    let mut game = Nim::new(7);
    game.heaps = [0, 0, 1];
    game.select_heap(2);
    assert!(game.take(1));
    assert_eq!(game.status, NimStatus::Won);
}
