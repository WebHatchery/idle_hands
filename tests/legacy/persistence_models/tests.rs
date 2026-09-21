//! Regression coverage for the tests module.

use idle_hands::testing::modules::persistence_models::*;

#[test]
fn index_round_trips_shell_state_without_game_payloads() {
    let state = AppState {
        selected: 12,
        screen: Screen::Game(GameId::Minesweeper),
        ..Default::default()
    };

    let index = CollectionIndex::from_state(&state, "1.0.0");
    assert_eq!(index.active_game, Some(GameId::Minesweeper));
    assert!(index.validate().is_ok());

    let encoded = serde_json::to_string(&index).expect("index serializes");
    let decoded: CollectionIndex = serde_json::from_str(&encoded).expect("index deserializes");
    let mut restored = AppState::default();
    decoded.apply_to(&mut restored);

    assert_eq!(restored.selected, 12);
    assert_eq!(restored.screen, Screen::Game(GameId::Minesweeper));
}

#[test]
fn malformed_index_is_rejected_before_apply() {
    let index = CollectionIndex {
        version: "".into(),
        selected: GameId::ALL.len(),
        active_game: None,
    };
    assert!(index.validate().is_err());
}
