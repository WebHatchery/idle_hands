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
