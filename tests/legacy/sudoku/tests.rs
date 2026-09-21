//! Regression coverage for the tests module.

use idle_hands::testing::modules::sudoku::*;

#[test]
fn puzzle_has_a_fixed_solution_shape_and_given_cells_cannot_change() {
    let mut game = Sudoku::new();
    assert_eq!(game.puzzle.len(), 81);
    let given = game.puzzle.iter().position(|&value| value != 0).unwrap();
    assert!(!game.place(given, 9));
}

#[test]
fn seeded_generation_changes_the_board_while_preserving_uniqueness() {
    let first = randomized_puzzle(PUZZLE, &mut SeededRng::new(1));
    let second = randomized_puzzle(PUZZLE, &mut SeededRng::new(2));
    assert_ne!(first, second);
    assert_eq!(count_solutions(&first, 2), 1);
    assert_eq!(count_solutions(&second, 2), 1);
}

#[test]
fn invalid_peer_value_is_rejected_and_notes_toggle() {
    let mut game = Sudoku::new();
    let (index, value) = game.hint_move().unwrap();
    assert!(game.place(index, value));
    let peer = (0..81)
        .find(|&other| game.values[other] == 0 && Sudoku::peers(index, other))
        .unwrap();
    assert!(!game.place(peer, value));
    assert!(game.toggle_note(peer, 6));
    assert_eq!(game.notes[peer], 1 << 6);
    assert!(game.toggle_note(peer, 6));
    assert_eq!(game.notes[peer], 0);
}

#[test]
fn difficulty_and_undo_restore_the_previous_entry() {
    let mut game = Sudoku::with_difficulty(SudokuDifficulty::Easy);
    assert_eq!(game.difficulty, SudokuDifficulty::Easy);
    let (index, value) = game.hint_move().unwrap();
    assert!(game.place(index, value));
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.values[index], 0);
    assert_eq!(game.moves, 0);
}
