use super::*;

#[test]
fn new_board_has_two_tiles_and_is_seeded() {
    let first = Game2048::new(42);
    let second = Game2048::new(42);
    assert_eq!(first.cells, second.cells);
    assert_eq!(first.cells.iter().filter(|&&value| value != 0).count(), 2);
}

#[test]
fn horizontal_move_merges_once_and_can_undo() {
    let mut game = Game2048::new(1);
    game.cells = [2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(game.move_in(Direction::Left));
    assert_eq!(&game.cells[0..3], &[4, 4, 0]);
    assert!(game.undo());
    assert_eq!(game.cells[0], 2);
}

#[test]
fn blocked_board_has_no_available_move() {
    let mut game = Game2048::new(1);
    game.cells = [2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2];
    assert!(!game.can_move());
}

#[test]
fn collection_save_round_trips_game_and_profile_state() {
    let mut state = AppState {
        profile_name: "Quiet Player".into(),
        ..Default::default()
    };
    state.game.score = 128;
    state.mine_records[0] = Some(42);
    state.records.best_2048 = 128;
    state.records.fivefold_best_total = 275;
    state.lights_out.moves = 4;
    state.records.lights_out_best_moves = Some(4);
    state.tic_tac_toe.moves = 3;
    state.records.tic_tac_toe_best_moves = Some(3);
    state.memory_pairs.moves = 5;
    state.records.memory_pairs_best_moves = Some(5);
    state.achievements[0] = true;
    state.stamps = 3;
    state.card_back = 1;
    state.board_theme = 1;
    state.sound_set = 1;
    state.cabinet_decoration = 1;
    state.high_contrast = true;
    state.large_text = true;
    state.tutorial_seen[6] = true;
    let save = CollectionSave::from_state(&state, "1.0.0");
    let mut restored = AppState::default();
    save.apply_to(&mut restored);
    assert_eq!(restored.profile_name, "Quiet Player");
    assert_eq!(restored.game.score, 128);
    assert_eq!(restored.mine_records[0], Some(42));
    assert_eq!(restored.records.best_2048, 128);
    assert_eq!(restored.records.fivefold_best_total, 275);
    assert_eq!(restored.lights_out.moves, 4);
    assert_eq!(restored.records.lights_out_best_moves, Some(4));
    assert_eq!(restored.tic_tac_toe.moves, 3);
    assert_eq!(restored.records.tic_tac_toe_best_moves, Some(3));
    assert_eq!(restored.memory_pairs.moves, 5);
    assert_eq!(restored.records.memory_pairs_best_moves, Some(5));
    assert!(restored.achievements[0]);
    assert_eq!(restored.stamps, 3);
    assert_eq!(restored.card_back, 1);
    assert_eq!(restored.board_theme, 1);
    assert_eq!(restored.sound_set, 1);
    assert_eq!(restored.cabinet_decoration, 1);
    assert!(restored.high_contrast);
    assert!(restored.large_text);
    assert!(restored.tutorial_seen[6]);
}

#[test]
fn profile_and_game_snapshots_round_trip_independently() {
    let mut state = AppState {
        profile_name: "Separate Slots".into(),
        ..Default::default()
    };
    state.game.score = 77;
    state.records.best_2048 = 77;
    let profile = ProfileSave::from_state(&state, "1.0.0");
    let snapshot = GameSnapshot::from_state(&state, GameId::Game2048);
    let mut restored = AppState::default();
    profile.apply_to(&mut restored);
    snapshot.apply_to(&mut restored);
    assert_eq!(restored.profile_name, "Separate Slots");
    assert_eq!(restored.game.score, 77);
    assert_eq!(GameId::Yahtzee.save_key(), "fivefold");
}

#[test]
fn every_game_snapshot_round_trips_its_own_state() {
    let mut source = AppState::default();
    source.game.score = 77;
    source.minesweeper.seed = 91;
    source.sudoku.selected = Some(4);
    source.nonogram.moves = 12;
    source.solitaire.moves = 8;
    source.freecell.moves = 9;
    source.fivefold.roll_number = 2;
    source.reversi.turn = 2;
    source.lights_out.moves = 17;
    source.tic_tac_toe.moves = 19;
    source.memory_pairs.moves = 21;

    for game in GameId::ALL {
        let snapshot = GameSnapshot::from_state(&source, game);
        let before = serde_json::to_value(&snapshot).unwrap();
        let mut restored = AppState::default();
        snapshot.apply_to(&mut restored);
        let after = serde_json::to_value(GameSnapshot::from_state(&restored, game)).unwrap();
        assert_eq!(before, after, "snapshot changed for {}", game.title());
    }
}

#[test]
fn independent_snapshots_restore_all_games_without_overwriting_each_other() {
    let mut source = AppState::default();
    source.game.score = 11;
    source.minesweeper.seed = 22;
    source.sudoku.moves = 33;
    source.nonogram.moves = 44;
    source.solitaire.moves = 55;
    source.freecell.moves = 66;
    source.fivefold.roll_number = 2;
    source.reversi.turn = 2;
    source.lights_out.moves = 17;
    source.tic_tac_toe.moves = 19;
    source.memory_pairs.moves = 21;

    let snapshots = GameId::ALL
        .iter()
        .map(|&game| GameSnapshot::from_state(&source, game))
        .collect::<Vec<_>>();
    let mut restored = AppState::default();
    for snapshot in snapshots {
        snapshot.apply_to(&mut restored);
    }

    assert_eq!(restored.game.score, 11);
    assert_eq!(restored.minesweeper.seed, 22);
    assert_eq!(restored.sudoku.moves, 33);
    assert_eq!(restored.nonogram.moves, 44);
    assert_eq!(restored.solitaire.moves, 55);
    assert_eq!(restored.freecell.moves, 66);
    assert_eq!(restored.fivefold.roll_number, 2);
    assert_eq!(restored.reversi.turn, 2);
    assert_eq!(restored.lights_out.moves, 17);
    assert_eq!(restored.tic_tac_toe.moves, 19);
    assert_eq!(restored.memory_pairs.moves, 21);
}

#[test]
fn older_saves_default_new_progression_and_cosmetic_fields() {
    let save = CollectionSave::from_state(&AppState::default(), "1.0.0");
    let mut value = serde_json::to_value(save).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in [
        "achievements",
        "stamps",
        "card_back",
        "board_theme",
        "sound_set",
        "cabinet_decoration",
        "high_contrast",
        "large_text",
        "lights_out",
        "tic_tac_toe",
        "memory_pairs",
    ] {
        object.remove(field);
    }
    let migrated: CollectionSave = serde_json::from_value(value).unwrap();
    let mut restored = AppState::default();
    migrated.apply_to(&mut restored);
    assert_eq!(restored.stamps, 0);
    assert_eq!(restored.achievements, [false; 10]);
    assert_eq!(restored.card_back, 0);
    assert_eq!(restored.board_theme, 0);
    assert_eq!(restored.sound_set, 0);
    assert_eq!(restored.cabinet_decoration, 0);
    assert!(!restored.high_contrast);
    assert!(!restored.large_text);
}

#[test]
fn older_profile_saves_default_accessibility_fields() {
    let save = ProfileSave::from_state(&AppState::default(), "1.0.0");
    let mut value = serde_json::to_value(save).unwrap();
    value.as_object_mut().unwrap().remove("high_contrast");
    value.as_object_mut().unwrap().remove("large_text");
    let migrated: ProfileSave = serde_json::from_value(value).unwrap();
    let mut restored = AppState::default();
    migrated.apply_to(&mut restored);
    assert!(!restored.high_contrast);
    assert!(!restored.large_text);
}

#[test]
fn default_state_has_no_reset_confirmation() {
    assert!(!AppState::default().confirm_reset);
}
