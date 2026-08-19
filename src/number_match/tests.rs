use super::*;

#[test]
fn adjacent_equal_or_sum_ten_pairs_clear() {
    let mut game = NumberMatch::new(1);
    let (first, second) = game.hint_pair().unwrap();
    assert!(game.tap(first));
    assert!(game.tap(second));
    assert_eq!(game.cells[first], 0);
    assert_eq!(game.cells[second], 0);
    assert_eq!(game.score, 1);
}

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
fn undo_restores_the_cleared_pair() {
    let mut game = NumberMatch::new(3);
    let (first, second) = game.hint_pair().unwrap();
    game.tap(first);
    game.tap(second);
    assert!(game.undo());
    assert_ne!(game.cells[first], 0);
    assert_ne!(game.cells[second], 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn clearing_every_seeded_pair_wins() {
    let mut game = NumberMatch::new(4);
    let mut pair = 0;
    for block_row in 0..(SIDE / 2) {
        for block_col in 0..(SIDE / 2) {
            let vertical = random_word(4, pair) & 1 == 1;
            for offset in 0..2 {
                let row = block_row * 2;
                let col = block_col * 2;
                let (first, second) = if vertical {
                    (row * SIDE + col + offset, (row + 1) * SIDE + col + offset)
                } else {
                    ((row + offset) * SIDE + col, (row + offset) * SIDE + col + 1)
                };
                assert!(game.tap(first));
                assert!(game.tap(second));
                pair += 1;
            }
        }
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

    let hint = game.hint_pair().unwrap();
    assert!(game.can_pair(hint.0, hint.1));
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

#[test]
fn consecutive_pairs_build_points_and_full_history() {
    let mut game = NumberMatch::new(12);
    game.cells = vec![0; CELLS];
    for (index, value) in [(0, 5), (1, 5), (2, 4), (3, 6), (4, 2), (5, 8)] {
        game.cells[index] = value;
    }
    for (first, second) in [(0, 1), (2, 3)] {
        game.tap(first);
        game.tap(second);
    }
    assert_eq!(game.combo, 2);
    assert_eq!(game.points, 30);
    assert!(game.undo());
    assert_eq!(game.combo, 1);
    assert!(game.undo());
    assert_eq!(game.points, 0);
}

#[test]
fn legacy_saves_default_to_neighbor_rules_and_two_remixes() {
    let mut value = serde_json::to_value(NumberMatch::new(13)).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in ["points", "combo", "best_combo", "rule", "remixes_left"] {
        object.remove(field);
    }
    let loaded: NumberMatch = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.rule, LinkRule::Neighbors);
    assert_eq!(loaded.remixes_left, 2);
    assert_eq!(loaded.points, 0);
}
