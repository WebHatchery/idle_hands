use super::*;
use crate::cards::Card;

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
        profile_name: "Patient Player".into(),
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
    state.sliding_puzzle.moves = 7;
    state.records.sliding_puzzle_best_moves = Some(7);
    state.mastermind.row = 2;
    state.records.mastermind_best_rows = Some(2);
    state.spider.moves = 9;
    state.records.spider_best_moves = Some(9);
    state.word_search.moves = 4;
    state.records.word_search_best_moves = Some(4);
    state.hangman.moves = 6;
    state.records.hangman_best_moves = Some(6);
    state.connect_four.moves = 8;
    state.records.connect_four_best_moves = Some(8);
    state.checkers.moves = 10;
    state.records.checkers_best_moves = Some(10);
    state.peg_solitaire.moves = 11;
    state.records.peg_solitaire_best_moves = Some(11);
    state.mahjong_solitaire.moves = 12;
    state.records.mahjong_solitaire_best_moves = Some(12);
    state.snake.moves = 13;
    state.records.snake_best_score = Some(4);
    state.breakout.moves = 14;
    state.records.breakout_best_score = Some(5);
    state.higher_lower.moves = 15;
    state.records.higher_lower_best_score = Some(6);
    state.klondike_golf.moves = 16;
    state.records.klondike_golf_best_moves = Some(16);
    state.blackjack.player.push(Card {
        rank: 10,
        suit: 0,
        face_up: true,
    });
    state.records.blackjack_best_wins = Some(2);
    state.spider_solitaire.moves = 17;
    state.records.spider_solitaire_best_moves = Some(17);
    state.dungeon_sweeper.moves = 18;
    state.records.dungeon_sweeper_best_moves = Some(18);
    state.potion_2048.score = 19;
    state.records.potion_2048_best_score = Some(19);
    state.tiny_tower_defence.wave = 4;
    state.records.tiny_tower_defence_best_wave = Some(4);
    state.one_room_roguelike.score = 23;
    state.records.one_room_roguelike_best_score = Some(23);
    state.daily_dungeon.score = 31;
    state.records.daily_dungeon_best_score = Some(31);
    state.nim.moves = 6;
    state.records.nim_best_moves = Some(6);
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
    assert_eq!(restored.profile_name, "Patient Player");
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
    assert_eq!(restored.sliding_puzzle.moves, 7);
    assert_eq!(restored.records.sliding_puzzle_best_moves, Some(7));
    assert_eq!(restored.mastermind.row, 2);
    assert_eq!(restored.records.mastermind_best_rows, Some(2));
    assert_eq!(restored.spider.moves, 9);
    assert_eq!(restored.records.spider_best_moves, Some(9));
    assert_eq!(restored.word_search.moves, 4);
    assert_eq!(restored.records.word_search_best_moves, Some(4));
    assert_eq!(restored.hangman.moves, 6);
    assert_eq!(restored.records.hangman_best_moves, Some(6));
    assert_eq!(restored.connect_four.moves, 8);
    assert_eq!(restored.records.connect_four_best_moves, Some(8));
    assert_eq!(restored.checkers.moves, 10);
    assert_eq!(restored.records.checkers_best_moves, Some(10));
    assert_eq!(restored.peg_solitaire.moves, 11);
    assert_eq!(restored.records.peg_solitaire_best_moves, Some(11));
    assert_eq!(restored.mahjong_solitaire.moves, 12);
    assert_eq!(restored.records.mahjong_solitaire_best_moves, Some(12));
    assert_eq!(restored.snake.moves, 13);
    assert_eq!(restored.records.snake_best_score, Some(4));
    assert_eq!(restored.breakout.moves, 14);
    assert_eq!(restored.records.breakout_best_score, Some(5));
    assert_eq!(restored.higher_lower.moves, 15);
    assert_eq!(restored.records.higher_lower_best_score, Some(6));
    assert_eq!(restored.klondike_golf.moves, 16);
    assert_eq!(restored.records.klondike_golf_best_moves, Some(16));
    assert_eq!(restored.blackjack.player.len(), 3);
    assert_eq!(restored.records.blackjack_best_wins, Some(2));
    assert_eq!(restored.spider_solitaire.moves, 17);
    assert_eq!(restored.records.spider_solitaire_best_moves, Some(17));
    assert_eq!(restored.dungeon_sweeper.moves, 18);
    assert_eq!(restored.records.dungeon_sweeper_best_moves, Some(18));
    assert_eq!(restored.potion_2048.score, 19);
    assert_eq!(restored.records.potion_2048_best_score, Some(19));
    assert_eq!(restored.tiny_tower_defence.wave, 4);
    assert_eq!(restored.records.tiny_tower_defence_best_wave, Some(4));
    assert_eq!(restored.one_room_roguelike.score, 23);
    assert_eq!(restored.records.one_room_roguelike_best_score, Some(23));
    assert_eq!(restored.daily_dungeon.score, 31);
    assert_eq!(restored.records.daily_dungeon_best_score, Some(31));
    assert_eq!(restored.nim.moves, 6);
    assert_eq!(restored.records.nim_best_moves, Some(6));
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
    source.sliding_puzzle.moves = 23;
    source.mastermind.row = 3;
    source.spider.moves = 25;
    source.word_search.moves = 26;
    source.hangman.moves = 27;
    source.connect_four.moves = 28;
    source.checkers.moves = 29;
    source.peg_solitaire.moves = 30;
    source.mahjong_solitaire.moves = 31;
    source.snake.moves = 32;
    source.breakout.moves = 33;
    source.higher_lower.moves = 34;
    source.klondike_golf.moves = 35;
    source.blackjack.player.push(Card {
        rank: 9,
        suit: 1,
        face_up: true,
    });
    source.spider_solitaire.moves = 36;
    source.dungeon_sweeper.moves = 38;
    source.potion_2048.score = 40;
    source.tiny_tower_defence.wave = 4;
    source.one_room_roguelike.score = 40;
    source.daily_dungeon.score = 42;
    source.nim.moves = 43;

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
    source.sliding_puzzle.moves = 23;
    source.mastermind.row = 3;
    source.spider.moves = 25;
    source.word_search.moves = 26;
    source.hangman.moves = 27;
    source.connect_four.moves = 28;
    source.checkers.moves = 29;
    source.peg_solitaire.moves = 30;
    source.mahjong_solitaire.moves = 31;
    source.snake.moves = 32;
    source.breakout.moves = 33;
    source.higher_lower.moves = 34;
    source.klondike_golf.moves = 35;
    source.blackjack.player.push(Card {
        rank: 8,
        suit: 2,
        face_up: true,
    });
    source.spider_solitaire.moves = 37;
    source.dungeon_sweeper.moves = 39;
    source.potion_2048.score = 41;
    source.tiny_tower_defence.wave = 5;
    source.one_room_roguelike.score = 41;
    source.daily_dungeon.score = 43;
    source.nim.moves = 44;

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
    assert_eq!(restored.sliding_puzzle.moves, 23);
    assert_eq!(restored.mastermind.row, 3);
    assert_eq!(restored.spider.moves, 25);
    assert_eq!(restored.word_search.moves, 26);
    assert_eq!(restored.hangman.moves, 27);
    assert_eq!(restored.connect_four.moves, 28);
    assert_eq!(restored.checkers.moves, 29);
    assert_eq!(restored.peg_solitaire.moves, 30);
    assert_eq!(restored.mahjong_solitaire.moves, 31);
    assert_eq!(restored.snake.moves, 32);
    assert_eq!(restored.breakout.moves, 33);
    assert_eq!(restored.higher_lower.moves, 34);
    assert_eq!(restored.klondike_golf.moves, 35);
    assert_eq!(restored.blackjack.player.len(), 3);
    assert_eq!(restored.spider_solitaire.moves, 37);
    assert_eq!(restored.dungeon_sweeper.moves, 39);
    assert_eq!(restored.potion_2048.score, 41);
    assert_eq!(restored.tiny_tower_defence.wave, 5);
    assert_eq!(restored.one_room_roguelike.score, 41);
    assert_eq!(restored.daily_dungeon.score, 43);
    assert_eq!(restored.nim.moves, 44);
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
        "favorites",
        "high_contrast",
        "large_text",
        "lights_out",
        "tic_tac_toe",
        "memory_pairs",
        "sliding_puzzle",
        "mastermind",
        "spider",
        "word_search",
        "hangman",
        "connect_four",
        "checkers",
        "peg_solitaire",
        "mahjong_solitaire",
        "snake",
        "breakout",
        "higher_lower",
        "klondike_golf",
        "blackjack",
        "spider_solitaire",
        "dungeon_sweeper",
        "potion_2048",
        "tiny_tower_defence",
        "one_room_roguelike",
        "daily_dungeon",
        "nim",
    ] {
        object.remove(field);
    }
    let migrated: CollectionSave = serde_json::from_value(value).unwrap();
    let mut restored = AppState::default();
    migrated.apply_to(&mut restored);
    assert_eq!(restored.stamps, 0);
    assert_eq!(
        restored.achievements,
        vec![false; crate::progression::AchievementId::ALL.len()]
    );
    assert_eq!(restored.card_back, 0);
    assert_eq!(restored.board_theme, 0);
    assert_eq!(restored.sound_set, 0);
    assert_eq!(restored.cabinet_decoration, 0);
    assert!(!restored.high_contrast);
    assert!(!restored.large_text);
    assert_eq!(restored.favorites, vec![false; GameId::ALL.len()]);
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

    let mut legacy =
        serde_json::to_value(ProfileSave::from_state(&AppState::default(), "1.0.0")).unwrap();
    legacy["achievements"] =
        serde_json::json!([true, false, true, false, true, false, true, false, true, false]);
    let migrated_achievements: ProfileSave = serde_json::from_value(legacy).unwrap();
    let mut achievement_state = AppState::default();
    migrated_achievements.apply_to(&mut achievement_state);
    assert_eq!(
        achievement_state.achievements.len(),
        crate::progression::AchievementId::ALL.len()
    );
    assert!(achievement_state.achievements[0]);
    assert!(
        !achievement_state.achievements
            [crate::progression::AchievementId::Game(GameId::WordLadder).index()]
    );
}

#[test]
fn default_state_has_no_reset_confirmation() {
    assert!(!AppState::default().confirm_reset);
}

#[test]
fn tutorial_state_covers_late_games_and_migrates_short_saves() {
    let mut state = AppState::default();
    let late_game = GameId::MatchThree;
    state.selected = late_game.index();
    state.tutorial_seen[late_game.index()] = true;
    let collection = CollectionSave::from_state(&state, "1.0.0");
    let mut collection_state = AppState::default();
    collection.apply_to(&mut collection_state);
    assert_eq!(collection_state.selected, late_game.index());
    let profile = ProfileSave::from_state(&state, "1.0.0");
    let mut restored = AppState::default();
    profile.apply_to(&mut restored);
    assert!(restored.tutorial_seen[late_game.index()]);
    assert_eq!(restored.tutorial_seen.len(), GameId::ALL.len());

    let mut old = serde_json::to_value(ProfileSave::from_state(&state, "1.0.0")).unwrap();
    old["tutorial_seen"] =
        serde_json::json!([true, false, true, false, false, false, false, false]);
    let migrated: ProfileSave = serde_json::from_value(old).unwrap();
    assert_eq!(migrated.tutorial_seen.len(), 8);
    assert!(migrated.tutorial_seen[0]);
    let mut migrated_state = AppState::default();
    migrated.apply_to(&mut migrated_state);
    assert_eq!(migrated_state.tutorial_seen.len(), GameId::ALL.len());
    assert!(!migrated_state.tutorial_seen[GameId::MatchThree.index()]);
}

#[test]
fn favorites_round_trip_and_short_legacy_profiles_are_normalized() {
    let mut state = AppState::default();
    state.favorites[GameId::Spider.index()] = true;
    state.favorites[GameId::Nim.index()] = true;
    let profile = ProfileSave::from_state(&state, "1.0.0");
    let mut restored = AppState::default();
    profile.apply_to(&mut restored);
    assert!(restored.favorites[GameId::Spider.index()]);
    assert!(restored.favorites[GameId::Nim.index()]);
    assert_eq!(restored.favorites.len(), GameId::ALL.len());

    let mut old = serde_json::to_value(ProfileSave::from_state(&state, "1.0.0")).unwrap();
    old["favorites"] = serde_json::json!([true, false]);
    let migrated: ProfileSave = serde_json::from_value(old).unwrap();
    let mut migrated_state = AppState::default();
    migrated.apply_to(&mut migrated_state);
    assert_eq!(migrated_state.favorites.len(), GameId::ALL.len());
    assert!(migrated_state.favorites[0]);
    assert!(!migrated_state.favorites[GameId::Nim.index()]);
}

#[test]
fn recent_games_round_trip_deduplicates_and_caps_history() {
    let state = AppState {
        recent_games: vec![
            GameId::WordLadder,
            GameId::Spider,
            GameId::WordLadder,
            GameId::Nim,
            GameId::Solitaire,
            GameId::Game2048,
            GameId::Sudoku,
        ],
        ..Default::default()
    };
    let profile = ProfileSave::from_state(&state, "1.0.0");
    let mut restored = AppState::default();
    profile.apply_to(&mut restored);
    assert_eq!(
        restored.recent_games,
        vec![
            GameId::WordLadder,
            GameId::Spider,
            GameId::Nim,
            GameId::Solitaire,
            GameId::Game2048
        ]
    );

    let mut old = serde_json::to_value(ProfileSave::from_state(&state, "1.0.0")).unwrap();
    old["recent_games"] = serde_json::json!([
        "Spider",
        "Spider",
        "Nim",
        "WordLadder",
        "Solitaire",
        "Game2048",
        "Sudoku"
    ]);
    let migrated: ProfileSave = serde_json::from_value(old).unwrap();
    let mut migrated_state = AppState::default();
    migrated.apply_to(&mut migrated_state);
    assert_eq!(migrated_state.recent_games.len(), 5);
    assert_eq!(migrated_state.recent_games[0], GameId::Spider);
    assert_eq!(migrated_state.recent_games[1], GameId::Nim);
}
