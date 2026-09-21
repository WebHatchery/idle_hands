//! Regression coverage for the tests module.

use idle_hands::testing::modules::maze_walk::*;

#[test]
fn blocked_moves_do_not_change_position_or_count() {
    let mut game = MazeWalk::new(8);
    assert!(!game.step(Direction::Up));
    assert_eq!(game.player, 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn beacons_lock_the_exit_until_both_are_collected() {
    let mut game = MazeWalk::new(4);
    game.player = game.goal - 1;
    assert!(game.step(Direction::Right));
    assert_eq!(game.player, game.goal);
    assert!(!game.won());
    game.collected = game.beacons.clone();
    game.player = game.goal - 1;
    assert!(game.step(Direction::Right));
    assert!(game.won());
}

#[test]
fn fog_reveals_the_trail_and_adjacent_cells_only() {
    let mut game = MazeWalk::new_with_mode(5, MazeMode::Fog);
    assert!(game.is_visible(0));
    assert!(game.is_visible(1));
    assert!(game.is_visible(SIDE));
    assert!(!game.is_visible(SIDE + 1));
    assert!(game.step(Direction::Right));
    assert!(game.is_visible(0));
    assert!(game.is_visible(2));
}

#[test]
fn repeated_undo_rewinds_position_beacons_and_trail() {
    let mut game = MazeWalk::new(0);
    assert!(game.step(Direction::Right));
    assert!(game.step(Direction::Right));
    assert!(game.undo());
    assert_eq!(game.player, 1);
    assert!(game.undo());
    assert_eq!(game.player, 0);
    assert!(!game.visited[1]);
    assert!(!game.undo());
}
