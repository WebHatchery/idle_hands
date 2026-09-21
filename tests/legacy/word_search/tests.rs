//! Regression coverage for the tests module.

use idle_hands::testing::modules::word_search::*;

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

#[test]
fn alternate_word_rooms_keep_the_grid_contract_but_change_the_catalog() {
    let nature = WordSearch::new_with_theme(52, WordSearchTheme::Nature);
    let workshop = WordSearch::new_with_theme(52, WordSearchTheme::Workshop);
    assert_ne!(nature.words(), workshop.words());
    assert_eq!(nature.cells.len(), workshop.cells.len());
    let (_, start, end) = nature.hint_word().unwrap();
    assert!(nature.cells[start] <= 25);
    assert!(end < nature.cells.len());
}
