use super::*;
use crate::domain::Direction;

#[test]
fn maze_starts_with_pellets_and_ghosts() {
    let game = MunchMaze::new(7);
    assert_eq!(game.ghosts.len(), 4);
    assert!(game.pellets.iter().any(|pellet| *pellet));
    assert!(!game.is_wall(1, 1));
}

#[test]
fn touch_direction_advances_and_state_round_trips() {
    let mut game = MunchMaze::new(8);
    game.set_direction(Direction::Right);
    assert!(game.tick(0.2));
    assert!(game.moves > 0);
    let restored: MunchMaze = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.ghosts.len(), 4);
    assert_eq!(restored.status, MunchStatus::Playing);
}
