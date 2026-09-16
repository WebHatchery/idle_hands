//! Tile victories remain distinct from score milestones and survive saved games.

use idle_hands::testing::{
    cabinet_status, completed_games,
    games::game_2048::{Game2048, Game2048Size},
    AppState, Direction, GameId, GameSnapshot,
};

fn assert_complete(state: &AppState, expected: bool) {
    assert_eq!(state.records.completed_2048(), expected);
    assert_eq!(
        cabinet_status(state, GameId::Game2048) == "COMPLETE",
        expected
    );
    assert_eq!(completed_games(&state.records), usize::from(expected));
}

#[test]
fn merging_the_winning_tile_records_completion_on_both_board_sizes() {
    for size in Game2048Size::ALL {
        let mut state = AppState::default();
        state.games.game = Game2048::new_with_size(7, size);
        state.games.game.cells.fill(0);
        state.games.game.cells[..2].copy_from_slice(&[1024, 1024]);
        state.records.ensure_time_slots();
        state.records.elapsed_seconds[GameId::Game2048.index()] = 123;
        assert_complete(&state, false);

        assert!(state.games.game.move_in(Direction::Left));
        state.records.record_2048(&state.games.game);

        assert_complete(&state, true);
        assert_eq!(state.records.best_time(GameId::Game2048.index()), Some(123));
        assert_eq!(state.records.best_2048, 2048);
    }
}

#[test]
fn score_milestones_do_not_count_as_tile_victories() {
    for score in [2048, 4096, 20000] {
        let mut state = AppState::default();
        state.games.game.cells.fill(0);
        state.games.game.cells[0] = 1024;
        state.games.game.score = score;
        state.games.game.best = score;
        state.records.record_2048(&state.games.game);

        assert_eq!(state.records.best_2048, score);
        assert_complete(&state, false);
    }
}

#[test]
fn completion_survives_undo_restart_and_record_serialization() {
    let mut state = AppState::default();
    state.games.game.cells.fill(0);
    state.games.game.cells[..2].copy_from_slice(&[1024, 1024]);
    state.games.game.move_in(Direction::Left);
    state.records.record_2048(&state.games.game);
    assert!(state.games.game.undo());
    state.records.record_2048(&state.games.game);
    assert_complete(&state, true);

    state.games.game = Game2048::new(8);
    state.records.reset_time(GameId::Game2048.index());
    state.records.record_2048(&state.games.game);
    let restored = AppState {
        records: serde_json::from_value(serde_json::to_value(&state.records).unwrap()).unwrap(),
        ..Default::default()
    };
    assert_complete(&restored, true);
    assert_eq!(restored.records.best_2048, 2048);
}

#[test]
fn restored_winning_boards_repair_missing_completion_records() {
    for tile in [2048, 4096] {
        let mut saved = AppState::default();
        saved.games.game.cells.fill(0);
        saved.games.game.cells[0] = tile;
        let value =
            serde_json::to_value(GameSnapshot::from_state(&saved, GameId::Game2048)).unwrap();
        let snapshot: GameSnapshot = serde_json::from_value(value).unwrap();
        let mut restored = AppState::default();
        snapshot.apply_to(&mut restored);
        restored.records.record_2048(&restored.games.game);
        assert_complete(&restored, true);
    }
}

#[test]
fn historical_win_times_remain_complete_without_the_winning_board() {
    let mut state = AppState::default();
    state.records.record_time(GameId::Game2048.index());
    state.records.record_2048(&state.games.game);
    assert!(!state.games.game.won());
    assert_complete(&state, true);
}
