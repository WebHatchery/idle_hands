//! Persisted game snapshots must survive JSON serialization and restoration.

use idle_hands::testing::{AppState, GameData, GameId, GameSnapshot};

const SEED: u64 = 0x1D1E_5EED_6000_0001;

#[test]
fn every_game_snapshot_survives_serialization_and_restore() {
    let data = GameData::load().expect("embedded game data loads");
    let source = AppState::new_random(&data, SEED);

    for game in GameId::ALL {
        let snapshot = GameSnapshot::from_state(&source, game);
        let encoded = serde_json::to_value(&snapshot).expect("game snapshot serializes");
        let decoded: GameSnapshot =
            serde_json::from_value(encoded.clone()).expect("game snapshot deserializes");
        let mut restored = AppState::default();
        decoded.apply_to(&mut restored);

        assert_eq!(
            encoded,
            serde_json::to_value(GameSnapshot::from_state(&restored, game))
                .expect("restored game snapshot serializes"),
            "snapshot for {game:?} changed after restore"
        );
    }
}
