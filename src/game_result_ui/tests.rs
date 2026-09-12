//! Regression coverage for the tests module.

use super::*;
use crate::state::{AppState, GameId, GameSnapshot, Screen};

fn game_state(game: GameId) -> AppState {
    AppState {
        screen: Screen::Game(game),
        ..AppState::default()
    }
}

#[test]
fn win_loss_and_stuck_states_have_explicit_result_kinds() {
    let mut state = game_state(GameId::Minesweeper);
    state.games.minesweeper.status = crate::minesweeper::MineStatus::Won;
    assert_eq!(info(&state).unwrap().kind, ResultKind::Won);

    state.games.minesweeper.status = crate::minesweeper::MineStatus::Lost;
    assert_eq!(info(&state).unwrap().kind, ResultKind::Lost);

    let mut board = game_state(GameId::Game2048);
    board.games.game.cells = (1..=16).collect();
    assert_eq!(info(&board).unwrap().kind, ResultKind::Stuck);
}

#[test]
fn autosaved_terminal_status_survives_serialized_snapshot_restore() {
    let mut saved = game_state(GameId::MatchThree);
    saved.games.match_three.phase = crate::match_three::MatchThreePhase::Lost;
    saved.games.match_three.score = 42;
    let snapshot = GameSnapshot::from_state(&saved, GameId::MatchThree);
    let encoded = serde_json::to_value(&snapshot).expect("terminal snapshot should serialize");
    let restored_snapshot: GameSnapshot =
        serde_json::from_value(encoded).expect("terminal snapshot should deserialize");

    let mut restored = game_state(GameId::MatchThree);
    restored_snapshot.apply_to(&mut restored);
    assert_eq!(info(&restored).unwrap().kind, ResultKind::Lost);
    assert!(info(&restored).unwrap().stats.contains("SCORE 42"));
}

#[test]
fn timed_result_cards_show_the_run_and_personal_best() {
    let mut state = game_state(GameId::Minesweeper);
    state.games.minesweeper.status = crate::minesweeper::MineStatus::Won;
    state.records.ensure_time_slots();
    state.records.elapsed_seconds[GameId::Minesweeper.index()] = 61;
    state.records.best_time_seconds[GameId::Minesweeper.index()] = Some(54);
    let stats = info(&state).unwrap().stats;
    assert!(stats.contains("RUN 1m 01s"));
    assert!(stats.contains("BEST 0m 54s"));
}

#[test]
fn every_result_layout_keeps_two_touch_targets_inside_the_viewport() {
    let state = {
        let mut state = game_state(GameId::TicTacToe);
        state.games.tic_tac_toe.status = crate::tic_tac_toe::TicTacToeStatus::Draw;
        state
    };
    crate::ui::with_desktop_layout(|| assert_layout(&state, 1280., 720.));
    crate::ui::with_compact_landscape_layout(|| assert_layout(&state, 844., 390.));
    crate::ui::with_portrait_layout(|| assert_layout(&state, 360., 780.));
}

fn assert_layout(state: &AppState, width: f32, height: f32) {
    let result = info(state).unwrap();
    let layout = layout();
    assert!(layout.primary.w >= 44.);
    assert!(layout.primary.h >= 44.);
    assert!(layout.secondary.w >= 44.);
    assert!(layout.secondary.h >= 44.);
    assert!(layout.primary.right() <= width);
    assert!(layout.primary.bottom() <= height);
    assert!(layout.secondary.right() <= width);
    assert!(layout.secondary.bottom() <= height);
    assert!(matches!(
        clicks(state, layout.primary.center()).unwrap().as_slice(),
        [UiAction::TicTacToeNew]
    ));
    assert!(matches!(
        clicks(state, layout.secondary.center()).unwrap().as_slice(),
        [UiAction::Cabinet]
    ));
    assert_eq!(result.kind, ResultKind::Draw);
}

#[test]
fn fling_fury_keeps_its_existing_result_surface() {
    let state = game_state(GameId::FlingFury);
    assert!(info(&state).is_none());
}
