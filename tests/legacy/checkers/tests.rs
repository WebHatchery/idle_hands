//! Regression coverage for the tests module.

use idle_hands::testing::modules::checkers::*;

#[test]
fn a_red_piece_can_step_and_promote() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[9] = Piece::RedMan;
    assert!(game.tap(9));
    assert!(game.tap(2));
    assert_eq!(game.cells[2], Piece::RedKing);
}

#[test]
fn captures_are_mandatory_and_remove_the_middle_piece() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[5 * SIZE] = Piece::RedMan;
    game.cells[4 * SIZE + 1] = Piece::YellowMan;
    assert!(game.capture_available_for(Side::Red));
    assert!(game.tap(5 * SIZE));
    assert!(game.tap(3 * SIZE + 2));
    assert_eq!(game.cells[4 * SIZE + 1], Piece::Empty);
}

#[test]
fn bounded_yellow_reply_and_undo_restore_the_red_turn() {
    let mut game = Checkers::new(42);
    assert!(game.tap(5 * SIZE));
    assert!(game.tap(4 * SIZE + 1));
    assert_eq!(game.turn, Side::Red);
    assert!(game.undo());
    assert_eq!(game.turn, Side::Red);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_move_finds_a_mandatory_capture_without_mutating() {
    let mut game = Checkers::new(1);
    game.cells = vec![Piece::Empty; CELLS];
    game.cells[5 * SIZE] = Piece::RedMan;
    game.cells[4 * SIZE + 1] = Piece::YellowMan;
    let before = game.cells.clone();
    assert_eq!(game.hint_move(), Some((5 * SIZE, 3 * SIZE + 2)));
    assert_eq!(game.cells, before);
}
