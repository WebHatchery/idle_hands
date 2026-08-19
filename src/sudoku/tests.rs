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
fn seeded_generation_changes_the_board_while_preserving_uniqueness() {
    let first = randomized_puzzle(PUZZLE, &mut SeededRng::new(1));
    let second = randomized_puzzle(PUZZLE, &mut SeededRng::new(2));
    assert_ne!(first, second);
    assert_eq!(count_solutions(&first, 2), 1);
    assert_eq!(count_solutions(&second, 2), 1);
}

#[test]
fn easy_starts_with_a_real_grid_to_solve() {
    let sudoku = Sudoku::with_difficulty(SudokuDifficulty::Easy);
    let givens = sudoku.puzzle.iter().filter(|&&value| value != 0).count();
    assert_eq!(givens, 42);
    for box_row in 0..3 {
        for box_column in 0..3 {
            let givens = (0..3)
                .flat_map(|row| {
                    (0..3).map(move |column| (box_row * 3 + row) * 9 + box_column * 3 + column)
                })
                .filter(|&index| sudoku.puzzle[index] != 0)
                .count();
            assert!(givens < 9, "Easy puzzle contains a complete 3×3 box");
        }
    }
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
fn erase_clears_player_entry_but_not_given() {
    let mut game = Sudoku::new();
    let (index, value) = game.hint_move().unwrap();
    assert!(game.place(index, value));
    assert!(game.erase(index));
    assert_eq!(game.values[index], 0);
    let given = game.puzzle.iter().position(|&value| value != 0).unwrap();
    assert!(!game.erase(given));
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

#[test]
fn hint_move_identifies_an_empty_cell_without_mutating_the_board() {
    let game = Sudoku::new();
    let before = game.values.clone();
    let (index, value) = game.hint_move().unwrap();
    assert_eq!(game.values[index], 0);
    assert!((1..=9).contains(&value));
    assert_eq!(game.values, before);
}
