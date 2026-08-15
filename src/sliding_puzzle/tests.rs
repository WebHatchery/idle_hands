use super::*;

#[test]
fn seeded_boards_repeat_and_are_not_already_solved() {
    let first = SlidingPuzzle::new(42);
    assert_eq!(first.cells, SlidingPuzzle::new(42).cells);
    assert_ne!(
        first.cells,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
    );
}

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
