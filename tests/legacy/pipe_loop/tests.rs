//! Regression coverage for the tests module.

use super::*;

#[test]
fn symmetric_junctions_do_not_spend_rotations() {
    let mut game = PipeLoop::new_with_pattern(7, PipePattern::Trunk);
    game.pipes = game.solution.clone();
    game.phase = PipePhase::Playing;
    let center = (SIDE / 2) * SIDE + SIDE / 2;
    assert_eq!(game.pipes[center], 0b1111);
    assert!(!game.rotate(center));
    assert_eq!(game.moves, 0);
}

#[test]
fn network_rules_count_powered_tiles_and_open_leaks() {
    let mut game = PipeLoop::new(2);
    game.pipes = game.solution.clone();
    assert_eq!(game.connected_count(), SIDE * SIDE);
    assert_eq!(game.leak_count(), 0);
    assert!(game.is_complete_network());
    game.pipes[0] = rotate_mask(game.pipes[0]);
    assert!(game.connected_count() < SIDE * SIDE || game.leak_count() > 0);
    assert!(!game.is_complete_network());
}

#[test]
fn repeated_undo_rewinds_multiple_rotations() {
    let mut game = PipeLoop::new(6);
    let opening = game.pipes.clone();
    assert!(game.rotate(0));
    assert!(game.rotate(1));
    assert!(game.undo());
    assert!(game.undo());
    assert_eq!(game.pipes, opening);
    assert!(!game.undo());
}

#[test]
fn legacy_saves_default_to_serpent_without_breaking_par_display() {
    let original = PipeLoop::new(12);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    object.remove("pattern");
    object.remove("par");
    let restored: PipeLoop = serde_json::from_value(value).unwrap();
    assert_eq!(restored.pattern, PipePattern::Serpent);
    assert_eq!(restored.par, 0);
}
