use super::*;

#[test]
fn seeded_board_has_thirty_two_pegs_and_repeatable_layout() {
    let first = PegSolitaire::new(7);
    let second = PegSolitaire::new(7);
    assert_eq!(first.cells, second.cells);
    assert_eq!(
        first
            .cells
            .iter()
            .filter(|hole| **hole == Hole::Peg)
            .count(),
        32
    );
    assert_eq!(first.targets(3 * SIZE + 1).len(), 1);
}

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
