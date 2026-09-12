//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_deals_repeat() {
    assert_eq!(Nim::new(7).heaps, Nim::new(7).heaps);
    assert_ne!(Nim::new(7).heaps, Nim::new(8).heaps);
}

#[test]
fn player_turn_requires_selected_heap_and_valid_amount() {
    let mut game = Nim::new(7);
    assert!(!game.take(1));
    game.select_heap(0);
    assert!(!game.take(0));
    assert!(!game.take(4));
    assert!(game.take(1));
    assert_eq!(game.selected_heap, None);
}

#[test]
fn undo_restores_the_position_before_the_player_turn() {
    let mut game = Nim::new(7);
    let heaps = game.heaps;
    game.select_heap(0);
    assert!(game.take(1));
    game.undo();
    assert_eq!(game.heaps, heaps);
    assert_eq!(game.moves, 0);
    assert_eq!(game.status, NimStatus::Playing);
}

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
fn opponent_uses_a_forced_win_when_one_exists() {
    let mut game = Nim::new(2);
    game.heaps = [0, 0, 3];
    game.select_heap(2);
    assert!(game.take(1));
    assert_eq!(game.status, NimStatus::Lost);
    assert_eq!(game.heaps, [0, 0, 0]);
    assert_eq!(game.last_ai_take, 2);
}

#[test]
fn legacy_saves_default_to_normal_rules_and_empty_turn_notes() {
    let original = Nim::new(5);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in ["rule", "last_player_take", "last_ai_take"] {
        object.remove(field);
    }
    let restored: Nim = serde_json::from_value(value).unwrap();
    assert_eq!(restored.rule, NimRule::Normal);
    assert_eq!((restored.last_player_take, restored.last_ai_take), (0, 0));
}

#[test]
fn taking_the_last_heap_wins_before_the_opponent_moves() {
    let mut game = Nim::new(7);
    game.heaps = [0, 0, 1];
    game.select_heap(2);
    assert!(game.take(1));
    assert_eq!(game.status, NimStatus::Won);
}
