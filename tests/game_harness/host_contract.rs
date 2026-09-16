//! Shared host contracts use the catalog as inputs, with one case per behavior.

use idle_hands::testing::{
    cycle_variant, descriptor, variant_label, AppState, GameData, GameId, GameSnapshot, Screen,
};
use std::collections::HashSet;

const SEED: u64 = 0x1D1E_5EED_6000_0001;

#[test]
fn catalog_metadata_identifies_every_game_and_its_unique_save_key() {
    let mut keys = HashSet::new();
    for game in GameId::ALL {
        let entry = descriptor(game);
        assert_eq!(entry.id, game, "{game:?}");
        assert_eq!(entry.index, game.index(), "{game:?}");
        assert!(entry.active && entry.has_variants, "{game:?}");
        assert!(
            !entry.title.is_empty() && !entry.subtitle.is_empty(),
            "{game:?}"
        );
        assert!(
            keys.insert(entry.save_key),
            "duplicate save key for {game:?}"
        );
    }
}

#[test]
fn constructing_the_host_with_the_same_seed_repeats_every_game() {
    let data = GameData::load().unwrap();
    let first = AppState::new_random(&data, SEED);
    let second = AppState::new_random(&data, SEED);
    for game in GameId::ALL {
        assert_eq!(
            serde_json::to_value(GameSnapshot::from_state(&first, game)).unwrap(),
            serde_json::to_value(GameSnapshot::from_state(&second, game)).unwrap(),
            "{game:?}"
        );
    }
}

fn visit_variants(game: GameId, mut visit: impl FnMut(&AppState, &str)) {
    let data = GameData::load().unwrap();
    let mut state = AppState::new_random(&data, SEED);
    state.screen = Screen::Game(game);
    state.selected = game.index();
    let initial = variant_label(&state, game);
    let mut labels = HashSet::from([initial.clone()]);
    visit(&state, &initial);
    for _ in 0..32 {
        cycle_variant(&mut state, &data, game);
        let label = variant_label(&state, game);
        visit(&state, &label);
        if label == initial {
            assert!(labels.len() >= 2, "{game:?} must expose multiple variants");
            return;
        }
        assert!(
            labels.insert(label),
            "{game:?} repeated a variant before wrapping"
        );
    }
    panic!("{game:?} variant cycle did not return within 32 steps");
}

#[test]
fn variant_cycles_return_to_the_start_without_leaving_the_selected_game() {
    for game in GameId::ALL {
        visit_variants(game, |state, label| {
            assert!(!label.is_empty(), "{game:?}");
            assert_eq!(state.screen, Screen::Game(game), "{game:?}: {label}");
            assert_eq!(state.selected, game.index(), "{game:?}: {label}");
        });
    }
}

#[test]
fn every_variant_snapshot_survives_serialization_and_restore() {
    for game in GameId::ALL {
        visit_variants(game, |state, label| {
            let value = serde_json::to_value(GameSnapshot::from_state(state, game)).unwrap();
            let snapshot: GameSnapshot = serde_json::from_value(value.clone()).unwrap();
            let mut restored = AppState::default();
            snapshot.apply_to(&mut restored);
            assert_eq!(
                value,
                serde_json::to_value(GameSnapshot::from_state(&restored, game)).unwrap(),
                "{game:?}: {label}"
            );
        });
    }
}
