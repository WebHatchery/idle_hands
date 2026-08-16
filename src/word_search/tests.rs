use super::*;

#[test]
fn seeded_grids_repeat_and_contain_the_catalog_words() {
    let first = WordSearch::new(42);
    let second = WordSearch::new(42);
    assert_eq!(first.cells, second.cells);
    for &(row, column, row_step, column_step) in &PLACEMENTS {
        let word_index = PLACEMENTS
            .iter()
            .position(|placement| *placement == (row, column, row_step, column_step))
            .unwrap();
        for (offset, letter) in WORDS[word_index].bytes().enumerate() {
            let target_row = (row as isize + row_step * offset as isize) as usize;
            let target_column = (column as isize + column_step * offset as isize) as usize;
            assert_eq!(
                first.cells[target_row * SIZE + target_column],
                letter - b'A'
            );
        }
    }
}

#[test]
fn endpoint_selection_finds_forward_and_reverse_words() {
    let mut game = WordSearch::new(42);
    assert!(game.select(0));
    assert!(game.select(4));
    assert!(game.found[0]);
    assert!(game.select(69));
    assert!(game.select(29));
    assert!(game.found[1]);
    assert_eq!(game.moves, 2);
}

#[test]
fn invalid_path_clears_selection_without_progress() {
    let mut game = WordSearch::new(42);
    assert!(game.select(0));
    assert!(!game.select(11));
    assert_eq!(game.selected_start, None);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_word_identifies_the_first_unfound_endpoints_without_mutating() {
    let game = WordSearch::new(42);
    let before = game.selected_start;
    let (word, start, end) = game.hint_word().unwrap();
    assert_eq!(word, 0);
    assert_eq!((start, end), (0, 4));
    assert_eq!(game.selected_start, before);
}
