//! Regression coverage for the tests module.

use idle_hands::testing::modules::peg_solitaire::*;

#[test]
fn first_jump_removes_the_middle_peg_and_can_be_undone() {
    let mut game = PegSolitaire::new(1);
    assert!(game.tap(3 * SIZE + 1));
    assert!(game.tap(3 * SIZE + 3));
    assert_eq!(game.cells[3 * SIZE + 1], Hole::Empty);
    assert_eq!(game.cells[3 * SIZE + 2], Hole::Empty);
    assert_eq!(game.cells[3 * SIZE + 3], Hole::Peg);
    assert!(game.undo());
    assert_eq!(game.cells[3 * SIZE + 1], Hole::Peg);
    assert_eq!(game.moves, 0);
}

#[test]
fn a_centered_final_peg_wins() {
    let mut game = PegSolitaire::new(1);
    game.cells = vec![Hole::Empty; CELLS];
    game.cells[CENTER] = Hole::Peg;
    game.resolve();
    assert_eq!(game.status, PegSolitaireStatus::Won);
}

#[test]
fn hint_move_finds_the_first_legal_jump_without_mutating() {
    let game = PegSolitaire::new(1);
    let before = game.cells.clone();
    let (from, to) = game.hint_move().unwrap();
    assert_eq!((from, to), (10, 24));
    assert_eq!(game.cells, before);
}

#[test]
fn corner_variant_requires_the_offset_finishing_hole() {
    let mut game = PegSolitaire::new_with_variant(53, PegVariant::Corner);
    game.cells = vec![Hole::Empty; CELLS];
    game.cells[2 * SIZE + 2] = Hole::Peg;
    game.resolve();
    assert_eq!(game.status, PegSolitaireStatus::Won);
    assert_ne!(game.variant.winning_hole(), CENTER);
}
