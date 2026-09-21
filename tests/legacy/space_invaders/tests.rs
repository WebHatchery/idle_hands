//! Regression coverage for the tests module.

use idle_hands::testing::modules::space_invaders::*;

#[test]
fn default_wave_has_a_touch_playable_formation() {
    let game = SpaceInvaders::new(7);
    assert_eq!(game.invaders.len(), 32);
    assert_eq!(game.ship_x, WIDTH / 2);
}

#[test]
fn firing_and_ticking_advances_the_round() {
    let mut game = SpaceInvaders::new(8);
    assert!(game.fire());
    assert!(game.tick(0.11));
    assert!(game.moves > 0);
}

#[test]
fn state_round_trips_without_runtime_fields() {
    let game = SpaceInvaders::new(9);
    let restored: SpaceInvaders =
        serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.invaders.len(), 32);
    assert_eq!(restored.status, SpaceInvadersStatus::Playing);
}
