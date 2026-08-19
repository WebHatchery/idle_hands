use super::*;

#[test]
fn seeded_maze_is_repeatable_and_keeps_the_edge_route_open() {
    let first = MazeWalk::new(4);
    assert_eq!(first.walls, MazeWalk::new(4).walls);
    let mut game = first;
    for _ in 0..6 {
        assert!(game.step(Direction::Right));
    }
    for _ in 0..6 {
        assert!(game.step(Direction::Down));
    }
    assert_eq!(game.player, game.goal);
    assert!(!game.won());
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
    assert_eq!(game.collected.len(), 2);
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
fn route_par_and_objective_distance_are_published() {
    let game = MazeWalk::new(6);
    assert_eq!(game.beacons.len(), 2);
    assert!(game.par >= 12);
    assert!(game.distance_to_objective() > 0);
    assert!(game.next_objective().is_some());
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

#[test]
fn mode_switch_and_reset_preserve_fog_rules() {
    let mut game = MazeWalk::new(7);
    game.set_mode(MazeMode::Fog, 8);
    assert_eq!(game.mode, MazeMode::Fog);
    game.reset(9);
    assert_eq!(game.mode, MazeMode::Fog);
}

#[test]
fn legacy_saves_default_to_an_open_explorer_route() {
    let original = MazeWalk::new(10);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in ["beacons", "collected", "visited", "mode", "par"] {
        object.remove(field);
    }
    let restored: MazeWalk = serde_json::from_value(value).unwrap();
    assert_eq!(restored.mode, MazeMode::Explorer);
    assert!(restored.beacons.is_empty());
    assert!(restored.is_visible(20));
}
