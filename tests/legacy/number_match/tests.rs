//! Regression coverage for the tests module.

use super::*;

#[test]
fn rejects_non_adjacent_pairs_and_allows_reselection() {
    let mut game = NumberMatch::new(2);
    game.cells = vec![0; CELLS];
    game.cells[0] = 4;
    game.cells[3] = 4;
    assert!(game.tap(0));
    assert!(game.tap(3));
    assert_eq!(game.selected, Some(3));
    assert_eq!(game.moves, 0);
}

#[test]
fn line_and_diagonal_rules_bridge_cleared_spaces() {
    let mut game = NumberMatch::new_with_rule(9, LinkRule::Lines);
    game.cells = vec![0; CELLS];
    game.cells[0] = 4;
    game.cells[5] = 6;
    game.cells[2 * SIDE] = 7;
    game.cells[5 * SIDE + 3] = 3;

    assert!(game.can_pair(0, 5));
    assert!(!game.can_pair(2 * SIDE, 5 * SIDE + 3));
    game.rule = LinkRule::Diagonals;
    assert!(game.can_pair(2 * SIDE, 5 * SIDE + 3));
}

#[test]
fn a_bad_alternate_pair_can_enter_stuck_and_undo_recovers() {
    let mut game = NumberMatch::new(10);
    game.cells = vec![0; CELLS];
    game.cells[0] = 4;
    game.cells[1] = 4;
    game.cells[2] = 2;
    game.cells[3] = 3;

    assert!(game.tap(0));
    assert!(game.tap(1));
    assert_eq!(game.phase, NumberMatchPhase::Stuck);
    assert!(game.undo());
    assert_eq!(game.phase, NumberMatchPhase::Playing);
}

#[test]
fn remix_spends_a_charge_and_restores_a_playable_grid() {
    let mut game = NumberMatch::new(11);
    game.cells = vec![0; CELLS];
    game.cells[0] = 2;
    game.cells[1] = 3;
    game.phase = NumberMatchPhase::Stuck;

    assert!(game.remix());
    assert_eq!(game.remixes_left, 1);
    assert_eq!(game.phase, NumberMatchPhase::Playing);
    assert!(game.hint_pair().is_some());
    assert!(game.undo());
    assert_eq!(game.phase, NumberMatchPhase::Stuck);
    assert_eq!(game.remixes_left, 2);
}
