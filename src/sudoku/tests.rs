use super::*;

#[test]
fn puzzle_has_a_fixed_solution_shape_and_given_cells_cannot_change() {
    let mut game = Sudoku::new();
    assert_eq!(game.puzzle.len(), 81);
    let given = game.puzzle.iter().position(|&value| value != 0).unwrap();
    assert!(!game.place(given, 9));
}

#[test]
fn every_catalog_difficulty_has_one_solution() {
    let mut puzzles = Vec::new();
    for difficulty in SudokuDifficulty::ALL {
        let sudoku = Sudoku::with_difficulty(difficulty);
        assert_eq!(count_solutions(&sudoku.puzzle, 2), 1);
        puzzles.push(sudoku.puzzle);
    }
    assert_ne!(puzzles[0], puzzles[1]);
    assert_ne!(puzzles[1], puzzles[2]);
}

#[test]
fn invalid_peer_value_is_rejected_and_notes_toggle() {
    let mut game = Sudoku::new();
    assert!(game.place(2, 4));
    assert!(!game.place(3, 4));
    assert!(game.toggle_note(3, 6));
    assert_eq!(game.notes[3], 1 << 6);
    assert!(game.toggle_note(3, 6));
    assert_eq!(game.notes[3], 0);
}

#[test]
fn erase_clears_player_entry_but_not_given() {
    let mut game = Sudoku::new();
    assert!(game.place(2, 4));
    assert!(game.erase(2));
    assert_eq!(game.values[2], 0);
    let given = game.puzzle.iter().position(|&value| value != 0).unwrap();
    assert!(!game.erase(given));
}

#[test]
fn difficulty_and_undo_restore_the_previous_entry() {
    let mut game = Sudoku::with_difficulty(SudokuDifficulty::Easy);
    assert_eq!(game.difficulty, SudokuDifficulty::Easy);
    assert!(game.place(69, 6));
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert_eq!(game.values[69], 0);
    assert_eq!(game.moves, 0);
}
