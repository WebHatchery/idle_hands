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
