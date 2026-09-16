//! Regression coverage for the tests module.

use super::*;

#[test]
fn cannot_push_a_crate_into_a_wall() {
    let mut game = Sokoban::new(2);
    assert!(game.move_in(Direction::Left));
    assert!(game.move_in(Direction::Left));
    game.player = 5 * WIDTH + 1;
    assert!(!game.move_in(Direction::Left));
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
