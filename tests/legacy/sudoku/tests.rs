//! Stable Sudoku generation must always produce a unique puzzle.

use idle_hands::testing::modules::sudoku::*;

#[test]
fn seeded_generation_preserves_solution_uniqueness() {
    let first = randomized_puzzle(PUZZLE, &mut SeededRng::new(1));
    let second = randomized_puzzle(PUZZLE, &mut SeededRng::new(2));
    assert_eq!(count_solutions(&first, 2), 1);
    assert_eq!(count_solutions(&second, 2), 1);
}
