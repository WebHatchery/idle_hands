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

#[test]
fn reset_and_mode_changes_keep_one_pair_per_lane() {
    let mut game = Frogger::new(10);
    game.reset(11);
    assert_eq!(game.cars.len(), 10);

    game.cycle_mode();
    assert_eq!(game.cars.len(), 10);
}

#[test]
fn left_bound_traffic_wraps_to_the_far_edge() {
    let mut game = Frogger::new(12);
    let car = game
        .cars
        .iter_mut()
        .find(|car| car.direction < 0)
        .expect("the opening has a left-moving lane");
    car.x = 0;

    assert!(game.tick(STEP_INTERVAL));
    assert_eq!(
        game.cars
            .iter()
            .find(|car| car.direction < 0 && car.row == 4)
            .map(|car| car.x),
        Some(WIDTH - 1)
    );
}
