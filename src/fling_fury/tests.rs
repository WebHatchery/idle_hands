use super::*;

#[test]
fn starts_with_targets_blocks_and_birds() {
    let game = FlingFury::new(7);
    assert_eq!(game.targets.len(), 2);
    assert!(!game.blocks.is_empty());
    assert_eq!(game.birds, 5);
}

#[test]
fn firing_consumes_a_bird_and_state_round_trips() {
    let mut game = FlingFury::new(8);
    assert!(game.fire());
    assert_eq!(game.birds, 4);
    assert!(game.tick(0.5));
    let restored: FlingFury = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.targets.len(), 2);
}
