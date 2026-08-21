use crate::data::GameData;
use crate::game_descriptor;
use crate::game_variants;
use crate::state::{AppState, GameId, Screen};
use crate::state_snapshots::GameSnapshot;

const HARNESS_SEED: u64 = 0x1D1E_5EED_6000_0001;

pub fn is_recovery_action(action: &crate::ui::UiAction) -> bool {
    let label = format!("{action:?}");
    matches!(
        label.as_str(),
        "Undo" | "Restart" | "Cancel" | "ConfirmRestart" | "New" | "Hint"
    ) || label.ends_with("Undo")
        || label.ends_with("New")
        || label.ends_with("Hint")
        || label.ends_with("Restart")
        || label.ends_with("Pause")
}

pub fn assert_game_contract(game: GameId) {
    let data = GameData::load().expect("embedded game data should load");
    let descriptor = game_descriptor::descriptor(game);
    assert_eq!(descriptor.id, game);
    assert_eq!(descriptor.index, game.index());
    assert!(descriptor.active);
    assert!(descriptor.has_variants);
    assert!(!descriptor.title.is_empty());
    assert!(!descriptor.subtitle.is_empty());

    let mut first = AppState::new_random(&data, HARNESS_SEED);
    let second = AppState::new_random(&data, HARNESS_SEED);
    first.screen = Screen::Game(game);
    first.selected = game.index();

    let first_snapshot = GameSnapshot::from_state(&first, game);
    let second_snapshot = GameSnapshot::from_state(&second, game);
    assert_eq!(
        serde_json::to_value(&first_snapshot).unwrap(),
        serde_json::to_value(&second_snapshot).unwrap(),
        "{} should start deterministically",
        game.title()
    );

    let initial_label = game_variants::label(&first, game);
    let mut labels = vec![initial_label.clone()];
    let mut cycle_count = 0;
    loop {
        game_variants::cycle(&mut first, &data, game);
        assert_eq!(first.screen, Screen::Game(game));
        assert_eq!(first.selected, game.index());
        let label = game_variants::label(&first, game);
        let snapshot = serde_json::to_value(GameSnapshot::from_state(&first, game)).unwrap();
        let mut restored = AppState::new(&data);
        GameSnapshot::from_state(&first, game).apply_to(&mut restored);
        assert_eq!(
            snapshot,
            serde_json::to_value(GameSnapshot::from_state(&restored, game)).unwrap(),
            "{} snapshot did not round-trip for variant {}",
            game.title(),
            label
        );
        cycle_count += 1;
        if label == initial_label {
            break;
        }
        assert!(
            !labels.iter().any(|seen| seen == &label),
            "{} variant label repeated before the cycle returned: {}",
            game.title(),
            label
        );
        labels.push(label);
        assert!(
            cycle_count < 32,
            "{} variant cycle did not return after {} steps",
            game.title(),
            cycle_count
        );
    }
    assert!(
        cycle_count >= 2,
        "{} variant cycle exposed fewer than two variants",
        game.title()
    );
}
