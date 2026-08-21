use super::*;

#[test]
fn default_field_is_ready_for_touch_play() {
    let game = Asteroids::new(7);
    assert_eq!(game.asteroids.len(), 8);
    assert_eq!(game.lives, 3);
}

#[test]
fn steering_wraps_and_fire_advances() {
    let mut game = Asteroids::new(8);
    game.set_control(ShipDirection::Left);
    let old_x = game.ship_x;
    assert!(game.fire());
    assert!(game.tick(0.13));
    assert_ne!(game.ship_x, old_x);
}

#[test]
fn state_round_trips() {
    let game = Asteroids::new(9);
    let restored: Asteroids = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.asteroids.len(), 8);
    assert_eq!(restored.status, AsteroidsStatus::Playing);
}
