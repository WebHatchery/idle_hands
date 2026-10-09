//! The seeded first-reveal generator preserves a safe starting cell.

use idle_hands::testing::modules::minesweeper::*;

#[test]
fn first_reveal_is_safe_and_seeded() {
    let mut left = Minesweeper::beginner(7);
    let mut right = Minesweeper::beginner(7);
    assert!(left.reveal(40));
    assert!(right.reveal(40));
    assert_eq!(left.cells, right.cells);
    assert!(!matches!(left.cells[40], Cell::Revealed(9)));
}
