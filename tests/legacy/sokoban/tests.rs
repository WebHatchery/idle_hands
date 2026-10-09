//! The bounded Sokoban pathfinder must solve every authored room.

use idle_hands::testing::modules::sokoban::*;

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
