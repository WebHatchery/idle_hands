use super::*;

#[test]
fn seeded_puzzle_is_repeatable_and_scrambled() {
    let first = PipeLoop::new(3);
    assert_eq!(first.pipes, PipeLoop::new(3).pipes);
    assert_eq!(first.solution.len(), SIDE * SIDE);
    assert_ne!(first.pipes, first.solution);
}

#[test]
fn rotating_a_tile_changes_its_mask_and_counts_a_move() {
    let mut game = PipeLoop::new(5);
    let before = game.pipes[0];
    assert!(game.rotate(0));
    assert_eq!(game.pipes[0], rotate_mask(before));
    assert_eq!(game.moves, 1);
}

#[test]
fn invalid_and_finished_rotations_are_rejected() {
    let mut game = PipeLoop::new(7);
    assert!(!game.rotate(CELLS));
    game.pipes = game.solution.clone();
    game.phase = PipePhase::Won;
    assert!(!game.rotate(0));
}

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
fn undo_restores_the_previous_pipe_and_reset_starts_again() {
    let mut game = PipeLoop::new(9);
    let before = game.pipes.clone();
    assert!(game.rotate(4));
    assert!(game.undo());
    assert_eq!(game.pipes, before);
    game.reset(10);
    assert_ne!(game.seed, 9);
    assert_eq!(game.moves, 0);
}

#[test]
fn matching_every_solution_tile_wins() {
    let mut game = PipeLoop::new(11);
    for index in 0..CELLS {
        while game.pipes[index] != game.solution[index] {
            assert!(game.rotate(index));
        }
    }
    assert!(game.won());
}

#[test]
fn hint_returns_the_first_unsolved_tile_without_mutating_the_loop() {
    let game = PipeLoop::new(13);
    let before = game.pipes.clone();

    let Some((index, count)) = game.hint_rotation() else {
        panic!("seeded Pipe Loop should have an unsolved tile");
    };
    assert!(index < CELLS);
    assert!((1..=3).contains(&count));
    assert_eq!(game.pipes, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn following_rotation_hints_solves_the_loop() {
    let mut game = PipeLoop::new(14);
    while let Some((index, count)) = game.hint_rotation() {
        for _ in 0..count {
            assert!(game.rotate(index));
        }
    }
    assert!(game.won());
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
fn trunk_pattern_is_a_distinct_connected_solution() {
    let serpent = PipeLoop::new_with_pattern(4, PipePattern::Serpent);
    let mut trunk = PipeLoop::new_with_pattern(4, PipePattern::Trunk);
    assert_ne!(serpent.solution, trunk.solution);
    trunk.pipes = trunk.solution.clone();
    assert!(trunk.is_complete_network());
    assert_eq!(trunk.pattern, PipePattern::Trunk);
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
fn pattern_switch_and_reset_preserve_the_selected_network() {
    let mut game = PipeLoop::new(8);
    game.set_pattern(PipePattern::Trunk, 9);
    assert_eq!(game.pattern, PipePattern::Trunk);
    game.reset(10);
    assert_eq!(game.pattern, PipePattern::Trunk);
    assert!(game.par > 0);
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
