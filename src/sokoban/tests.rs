use super::*;

#[test]
fn walks_and_pushes_a_crate() {
    let mut game = Sokoban::new(1);
    assert!(game.move_in(Direction::Up));
    assert!(game.move_in(Direction::Left));
    assert!(game.move_in(Direction::Up));
    assert!(game.move_in(Direction::Up));
    assert_eq!(game.tiles[WIDTH + 2], 4);
    assert_eq!(game.player, 2 * WIDTH + 2);
}

#[test]
fn cannot_push_a_crate_into_a_wall() {
    let mut game = Sokoban::new(2);
    assert!(game.move_in(Direction::Left));
    assert!(game.move_in(Direction::Left));
    game.player = 5 * WIDTH + 1;
    assert!(!game.move_in(Direction::Left));
}

#[test]
fn undo_restores_player_and_crate_positions() {
    let mut game = Sokoban::new(3);
    let before = game.clone_without_undo();
    assert!(game.move_in(Direction::Up));
    assert!(game.undo());
    assert_eq!(game.player, before.player);
    assert_eq!(game.tiles, before.tiles);
    assert_eq!(game.moves, before.moves);
}

#[test]
fn placing_a_crate_on_a_target_wins() {
    let mut game = Sokoban::new(4);
    game.tiles = vec![1; CELLS];
    game.player = 3 * WIDTH + 2;
    game.crates = 1;
    game.tiles[3 * WIDTH + 4] = 2;
    game.tiles[3 * WIDTH + 3] = 3;
    assert!(game.move_in(Direction::Right));
    assert!(game.won());
}

#[test]
fn default_room_has_a_complete_touch_solution() {
    let mut game = Sokoban::new(9);
    for direction in [
        Direction::Up,
        Direction::Left,
        Direction::Up,
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Right,
        Direction::Right,
    ] {
        assert!(game.move_in(direction));
    }
    assert!(game.won());
}

#[test]
fn reset_starts_a_fresh_room_with_the_new_seed() {
    let mut game = Sokoban::new(5);
    game.move_in(Direction::Up);
    game.reset(6);
    assert_eq!(game.seed, 6);
    assert_eq!(game.moves, 0);
    assert_eq!(game.phase, SokobanPhase::Playing);
    assert_eq!(game.player, 5 * WIDTH + 3);
}

#[test]
fn hint_returns_the_first_shortest_solution_move_without_mutating_the_room() {
    let game = Sokoban::new(10);
    let before = game.clone_without_undo();

    assert_eq!(game.hint_direction(), Some(Direction::Up));
    assert_eq!(game.player, before.player);
    assert_eq!(game.tiles, before.tiles);
    assert_eq!(game.moves, before.moves);
}

#[test]
fn following_hints_solves_the_default_room() {
    let mut game = Sokoban::new(11);
    for _ in 0..32 {
        let Some(direction) = game.hint_direction() else {
            break;
        };
        assert!(game.move_in(direction));
        if game.won() {
            break;
        }
    }
    assert!(game.won());
    assert_eq!(game.hint_direction(), None);
}

#[test]
fn every_authored_room_has_a_bounded_hint_solution() {
    for level in 0..LEVEL_COUNT {
        let mut game = Sokoban::new_with_level(99, level);
        for _ in 0..96 {
            let Some(direction) = game.hint_direction() else {
                break;
            };
            assert!(game.move_in(direction));
            if game.won() {
                break;
            }
        }
        assert!(game.won(), "authored level {level} should be solvable");
        assert!(
            game.moves <= game.par_moves(),
            "level {level} shortest route {} exceeds par {}",
            game.moves,
            game.par_moves()
        );
    }
}

#[test]
fn multiple_undos_walk_back_more_than_one_step() {
    let mut game = Sokoban::new(1);
    let start = game.player;
    assert!(game.move_in(Direction::Up));
    let after_one = game.player;
    assert!(game.move_in(Direction::Left));

    assert!(game.undo());
    assert_eq!(game.player, after_one);
    assert!(game.undo());
    assert_eq!(game.player, start);
    assert!(!game.undo());
}

#[test]
fn pushing_an_unsolved_crate_into_a_corner_marks_the_room_stuck() {
    let mut game = Sokoban::new(1);
    game.tiles = vec![1; CELLS];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if row == 0 || col == 0 || row + 1 == HEIGHT || col + 1 == WIDTH {
                game.tiles[row * WIDTH + col] = 0;
            }
        }
    }
    game.player = WIDTH + 3;
    game.tiles[WIDTH + 2] = 3;
    game.tiles[2 * WIDTH + 2] = 2;
    game.crates = 1;

    assert!(game.move_in(Direction::Left));
    assert_eq!(game.phase, SokobanPhase::Stuck);
    assert!(game.is_deadlocked_crate(WIDTH + 1));
    assert_eq!(game.hint_direction(), None);
    assert!(game.undo());
    assert_eq!(game.phase, SokobanPhase::Playing);
}

#[test]
fn target_crates_never_count_as_corner_deadlocks() {
    let mut game = Sokoban::new(1);
    game.tiles[WIDTH + 1] = 4;
    assert!(!game.is_deadlocked_crate(WIDTH + 1));
}

#[test]
fn pushes_and_clear_rank_track_the_solve() {
    let mut game = Sokoban::new_with_level(1, 3);
    assert!(game.move_in(Direction::Up));

    assert!(game.won());
    assert_eq!(game.pushes, 1);
    assert_eq!(game.moves, 1);
    assert_eq!(game.clear_rank(), "GOLD");
}

#[test]
fn legacy_saves_default_to_zero_pushes() {
    let mut value = serde_json::to_value(Sokoban::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("pushes");
    let loaded: Sokoban = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.pushes, 0);
}
