//! Terminal game state is part of the persisted game snapshot.

use idle_hands::testing::state::{AppState, GameId, GameSnapshot, Screen};

#[test]
fn terminal_snapshot_round_trip_preserves_status_and_score() {
    let mut source = AppState {
        screen: Screen::Game(GameId::MatchThree),
        ..AppState::default()
    };
    source.games.match_three.phase = idle_hands::testing::match_three::MatchThreePhase::Lost;
    source.games.match_three.score = 42;

    let snapshot = GameSnapshot::from_state(&source, GameId::MatchThree);
    let encoded = serde_json::to_value(&snapshot).expect("terminal snapshot serializes");
    let decoded: GameSnapshot =
        serde_json::from_value(encoded).expect("terminal snapshot deserializes");
    let mut restored = AppState::default();
    decoded.apply_to(&mut restored);

    assert_eq!(
        restored.games.match_three.phase,
        idle_hands::testing::match_three::MatchThreePhase::Lost
    );
    assert_eq!(restored.games.match_three.score, 42);
}
