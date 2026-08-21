use super::*;
use crate::domain::Direction;

#[test]
fn default_crossing_has_visible_lanes() {
    let game = Frogger::new(7);
    assert_eq!(game.cars.len(), 10);
    assert_eq!(game.player_row, HEIGHT - 1);
}

#[test]
fn touch_move_and_pause_are_available() {
    let mut game = Frogger::new(8);
    assert!(game.move_player(Direction::Up));
    assert_eq!(game.moves, 1);
    assert!(game.toggle_pause());
    assert!(!game.move_player(Direction::Up));
}

#[test]
fn state_round_trips() {
    let game = Frogger::new(9);
    let restored: Frogger = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.cars.len(), 10);
    assert_eq!(restored.status, FroggerStatus::Playing);
}
