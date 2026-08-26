use super::*;

#[test]
fn first_session_steps_emit_once_when_they_advance() {
    assert_eq!(
        progress_signals(false, true, 0, 1, 0, 1, 0, 1, false, false),
        vec![
            ProgressSignal::FirstDrawerOpened,
            ProgressSignal::TutorialCompleted,
            ProgressSignal::FirstDrawerCompleted,
        ]
    );
    assert!(progress_signals(true, true, 1, 1, 1, 1, 1, 1, true, false).is_empty());
}

#[test]
fn cabinet_depth_reports_crossed_thresholds() {
    assert_eq!(
        progress_signals(true, true, 4, 4, 9, 10, 7, 7, true, false),
        vec![ProgressSignal::TenDrawersCompleted]
    );
    assert_eq!(
        progress_signals(true, true, 4, 4, 29, 30, 29, 30, true, true),
        vec![
            ProgressSignal::DemoCompleted,
            ProgressSignal::ThirtyDrawersCompleted,
        ]
    );
    assert_eq!(
        progress_signals(true, true, 4, 4, 59, 60, 30, 30, true, false),
        vec![ProgressSignal::CabinetCompleted]
    );
}

#[test]
fn active_play_requires_recent_input_and_no_overlay_or_pause() {
    let mut state = AppState::default();
    state.screen = Screen::Game(GameId::Snake);
    assert!(is_active_play(&state, 1.0));
    assert!(!is_active_play(&state, 0.0));

    state.games.snake.paused = true;
    assert!(!is_active_play(&state, 1.0));
    state.games.snake.paused = false;
    state.tutorial = Some(GameId::Snake);
    assert!(!is_active_play(&state, 1.0));
}
