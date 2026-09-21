//! Regression coverage for the tests module.

use idle_hands::testing::modules::sliding_puzzle::*;

#[test]
fn only_adjacent_tiles_move_and_undo_restores_the_board() {
    let mut game = SlidingPuzzle::new(42);
    let blank = game.blank();
    let invalid = (0..CELLS)
        .find(|&index| index != blank && !neighbors(blank).contains(&index))
        .unwrap();
    assert!(!game.move_tile(invalid));
    let tile = neighbors(blank)[0];
    let before = game.cells;
    assert!(game.move_tile(tile));
    assert_ne!(game.cells, before);
    assert!(game.undo());
    assert_eq!(game.cells, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn the_final_tile_completes_the_puzzle() {
    let mut game = SlidingPuzzle::new(42);
    game.cells = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];
    assert!(game.move_tile(15));
    assert_eq!(game.status, SlidingStatus::Won);
    assert!(!game.move_tile(14));
}

#[test]
fn hint_move_is_legal_and_does_not_mutate_the_board() {
    let game = SlidingPuzzle::new(42);
    let before = game.cells;
    let hint = game.hint_move().unwrap();
    assert!(neighbors(game.blank()).contains(&hint));
    assert_eq!(game.cells, before);
}

#[test]
fn marathon_variant_is_a_deeper_but_repeatable_scramble() {
    let wanderer = SlidingPuzzle::new_with_variant(60, SlidingVariant::Wanderer);
    let marathon = SlidingPuzzle::new_with_variant(60, SlidingVariant::Marathon);
    assert_eq!(
        marathon.cells,
        SlidingPuzzle::new_with_variant(60, SlidingVariant::Marathon).cells
    );
    assert_ne!(wanderer.cells, marathon.cells);
}
