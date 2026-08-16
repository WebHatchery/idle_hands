use super::*;

#[test]
fn adjacent_equal_or_sum_ten_pairs_clear() {
    let mut game = NumberMatch::new(1);
    assert!(game.tap(0));
    assert!(game.tap(1));
    assert_eq!(game.cells[0], 0);
    assert_eq!(game.cells[1], 0);
    assert_eq!(game.score, 1);
}

#[test]
fn rejects_non_adjacent_pairs_and_allows_reselection() {
    let mut game = NumberMatch::new(2);
    assert!(game.tap(0));
    assert!(game.tap(3));
    assert_eq!(game.selected, Some(3));
    assert_eq!(game.moves, 0);
}

#[test]
fn undo_restores_the_cleared_pair() {
    let mut game = NumberMatch::new(3);
    game.tap(0);
    game.tap(1);
    assert!(game.undo());
    assert_ne!(game.cells[0], 0);
    assert_ne!(game.cells[1], 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn clearing_every_seeded_pair_wins() {
    let mut game = NumberMatch::new(4);
    for pair in 0..(CELLS / 2) {
        assert!(game.tap(pair * 2));
        assert!(game.tap(pair * 2 + 1));
    }
    assert!(game.won());
    assert_eq!(game.score, (CELLS / 2) as u16);
}

#[test]
fn reset_rebuilds_the_grid() {
    let mut game = NumberMatch::new(5);
    game.tap(0);
    game.tap(1);
    game.reset(6);
    assert_eq!(game.seed, 6);
    assert_eq!(game.moves, 0);
    assert_eq!(game.cells.len(), CELLS);
    assert_eq!(game.phase, NumberMatchPhase::Playing);
}

#[test]
fn hint_returns_the_first_valid_pair_without_mutating_the_grid() {
    let game = NumberMatch::new(7);
    let before = game.cells.clone();

    assert_eq!(game.hint_pair(), Some((0, 1)));
    assert_eq!(game.cells, before);
    assert_eq!(game.selected, None);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_is_empty_after_number_match_ends() {
    let mut game = NumberMatch::new(8);
    game.phase = NumberMatchPhase::Won;

    assert_eq!(game.hint_pair(), None);
}
