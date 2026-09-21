//! Regression coverage for the tests module.

use idle_hands::testing::modules::terrain_cannon::*;

#[test]
fn starts_with_adjustable_shot_and_terrain() {
    let game = TerrainCannon::new(7);
    assert_eq!(game.terrain.len(), usize::from(WIDTH));
    assert_eq!(game.target_health, 3);
    assert!(game.angle >= 15 && game.angle <= 75);
}

#[test]
fn firing_advances_physics_and_state_round_trips() {
    let mut game = TerrainCannon::new(8);
    let before = game.terrain.clone();
    assert!(game.fire());
    assert!(game.tick(1.0));
    assert!(game.moves > 0);
    assert!(game.shot.is_some() || game.terrain != before || game.status != CannonStatus::Playing);
    let restored: TerrainCannon =
        serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.terrain.len(), usize::from(WIDTH));
}
