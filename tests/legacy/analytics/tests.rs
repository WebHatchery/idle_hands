//! Regression coverage for the tests module.

use super::*;

#[test]
fn shipping_configuration_keeps_analytics_disabled() {
    assert!(!analytics_enabled());
}

#[test]
fn first_session_steps_emit_once_when_they_advance() {
    assert_eq!(
        progress_signals(ProgressSignalsInput {
            before: ProgressState {
                drawer_open: false,
                tutorials: 0,
                completed: 0,
                demo_completed: 0,
            },
            after: ProgressState {
                drawer_open: true,
                tutorials: 1,
                completed: 1,
                demo_completed: 1,
            },
            tutorial_emitted: false,
            demo_build: false,
        }),
        vec![
            ProgressSignal::FirstDrawerOpened,
            ProgressSignal::TutorialCompleted,
            ProgressSignal::FirstDrawerCompleted,
        ]
    );
    assert!(progress_signals(ProgressSignalsInput {
        before: ProgressState {
            drawer_open: true,
            tutorials: 1,
            completed: 1,
            demo_completed: 1,
        },
        after: ProgressState {
            drawer_open: true,
            tutorials: 1,
            completed: 1,
            demo_completed: 1,
        },
        tutorial_emitted: true,
        demo_build: false,
    })
    .is_empty());
}

#[test]
fn cabinet_depth_reports_crossed_thresholds() {
    assert_eq!(
        progress_signals(ProgressSignalsInput {
            before: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 9,
                demo_completed: 7,
            },
            after: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 10,
                demo_completed: 7,
            },
            tutorial_emitted: true,
            demo_build: false,
        }),
        vec![ProgressSignal::TenDrawersCompleted]
    );
    assert_eq!(
        progress_signals(ProgressSignalsInput {
            before: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 29,
                demo_completed: 29,
            },
            after: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 30,
                demo_completed: 30,
            },
            tutorial_emitted: true,
            demo_build: true,
        }),
        vec![
            ProgressSignal::DemoCompleted,
            ProgressSignal::ThirtyDrawersCompleted,
        ]
    );
    assert_eq!(
        progress_signals(ProgressSignalsInput {
            before: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 59,
                demo_completed: 30,
            },
            after: ProgressState {
                drawer_open: true,
                tutorials: 4,
                completed: 60,
                demo_completed: 30,
            },
            tutorial_emitted: true,
            demo_build: false,
        }),
        vec![ProgressSignal::CabinetCompleted]
    );
}

#[test]
fn active_play_requires_recent_input_and_no_overlay_or_pause() {
    let mut state = AppState {
        screen: Screen::Game(GameId::Snake),
        ..AppState::default()
    };
    assert!(is_active_play(&state, 1.0));
    assert!(!is_active_play(&state, 0.0));

    state.games.snake.paused = true;
    assert!(!is_active_play(&state, 1.0));
    state.games.snake.paused = false;
    state.lifecycle_paused = true;
    assert!(!is_active_play(&state, 1.0));
    state.lifecycle_paused = false;
    state.tutorial = Some(GameId::Snake);
    assert!(!is_active_play(&state, 1.0));
}
