//! Regression coverage for the tests module.

use super::*;

#[test]
fn misere_rule_makes_the_final_take_lose() {
    let mut game = Nim::new(7);
    game.set_rule(NimRule::Misere, 7);
    game.heaps = [0, 0, 1];
    game.select_heap(2);
    assert!(game.take(1));
    assert_eq!(game.status, NimStatus::Lost);
}

#[test]
fn exact_solver_changes_the_safe_move_between_end_rules() {
    let mut normal = Nim::new(1);
    normal.heaps = [0, 1, 2];
    normal.rule = NimRule::Normal;
    let normal_hint = normal.hint_move().unwrap();
    assert_eq!(
        normal.move_is_winning(normal_hint.0, normal_hint.1),
        Some(true)
    );

    let mut misere = normal.clone();
    misere.rule = NimRule::Misere;
    let misere_hint = misere.hint_move().unwrap();
    assert_eq!(
        misere.move_is_winning(misere_hint.0, misere_hint.1),
        Some(true)
    );
    assert_ne!(normal_hint, misere_hint);
}

#[test]
fn repeated_undo_rewinds_multiple_player_and_opponent_turns() {
    let mut game = Nim::new(3);
    let opening = game.heaps;
    game.select_heap(0);
    assert!(game.take(1));
    let after_first = game.heaps;
    let heap = game.heaps.iter().position(|&stones| stones > 0).unwrap();
    game.select_heap(heap);
    assert!(game.take(1));
    assert!(game.undo());
    assert_eq!(game.heaps, after_first);
    assert!(game.undo());
    assert_eq!(game.heaps, opening);
    assert!(!game.undo());
}

#[test]
fn taking_the_last_heap_wins_before_the_opponent_moves() {
    let mut game = Nim::new(7);
    game.heaps = [0, 0, 1];
    game.select_heap(2);
    assert!(game.take(1));
    assert_eq!(game.status, NimStatus::Won);
}
