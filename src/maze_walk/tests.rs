use super::*;

#[test]
fn seeded_maze_is_repeatable_and_keeps_the_route_open() {
    let first = MazeWalk::new(4);
    assert_eq!(first.walls, MazeWalk::new(4).walls);
    let mut game = first;
    for _ in 0..6 {
        assert!(game.step(Direction::Right));
    }
    for _ in 0..6 {
        assert!(game.step(Direction::Down));
    }
    assert!(game.won());
}

#[test]
fn blocked_moves_do_not_change_position_or_count() {
    let mut game = MazeWalk::new(8);
    assert!(!game.step(Direction::Up));
    assert_eq!(game.player, 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn undo_restores_a_valid_step_and_finished_games_stop() {
    let mut game = MazeWalk::new(1);
    assert!(game.step(Direction::Right));
    assert!(game.undo());
    assert_eq!(game.player, 0);
    assert_eq!(game.moves, 0);
    game.player = game.goal;
    game.phase = MazePhase::Won;
    assert!(!game.step(Direction::Right));
}

#[test]
fn reset_rebuilds_the_seeded_maze() {
    let mut game = MazeWalk::new(2);
    let old = game.walls.clone();
    game.reset(3);
    assert_ne!(old, game.walls);
    assert_eq!(game.player, 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_returns_the_shortest_route_direction_without_mutating_the_maze() {
    let game = MazeWalk::new(17);
    let before = game.player;

    assert_eq!(game.hint_direction(), Some(Direction::Right));
    assert_eq!(game.player, before);
    assert_eq!(game.moves, 0);
}

#[test]
fn following_hints_reaches_the_exit() {
    let mut game = MazeWalk::new(18);
    for _ in 0..(SIDE * SIDE) {
        let Some(direction) = game.hint_direction() else {
            break;
        };
        assert!(game.step(direction));
        if game.won() {
            break;
        }
    }
    assert!(game.won());
}
