//! Regression coverage for the tests module.

use super::*;
use crate::state::GameId;

fn scope(screen: Screen, layer: PointerLayer) -> PointerScope {
    PointerScope { screen, layer }
}

#[test]
fn pointer_tracker_distinguishes_taps_drags_and_cancelled_releases() {
    let mut tracker = PointerTracker::default();
    let cabinet = scope(Screen::Cabinet, PointerLayer::Board);
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    assert_eq!(
        tracker.release(Some(Vec2::new(12., 12.)), cabinet),
        Some(Gesture::Tap(Vec2::new(12., 12.)))
    );
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    assert_eq!(
        tracker.release(Some(Vec2::new(40., 10.)), cabinet),
        Some(Gesture::Drag {
            start: Vec2::new(10., 10.),
            end: Vec2::new(40., 10.)
        })
    );
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    tracker.cancel();
    assert_eq!(tracker.release(Some(Vec2::new(40., 10.)), cabinet), None);
}

#[test]
fn long_press_requires_the_threshold_and_does_not_override_a_drag() {
    let mut tracker = PointerTracker::default();
    let cabinet = scope(Screen::Cabinet, PointerLayer::Board);
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    tracker.tick(LONG_PRESS_SECONDS - 0.01);
    assert_eq!(
        tracker.release(Some(Vec2::new(10., 10.)), cabinet),
        Some(Gesture::Tap(Vec2::new(10., 10.)))
    );
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    tracker.tick(LONG_PRESS_SECONDS);
    assert_eq!(
        tracker.release(Some(Vec2::new(10., 10.)), cabinet),
        Some(Gesture::LongPress(Vec2::new(10., 10.)))
    );
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    tracker.tick(LONG_PRESS_SECONDS * 2.);
    assert!(matches!(
        tracker.release(Some(Vec2::new(40., 10.)), cabinet),
        Some(Gesture::Drag { .. })
    ));
}

#[test]
fn a_second_touch_cancels_single_pointer_capture() {
    assert!(!should_cancel_for_touch_count(0));
    assert!(!should_cancel_for_touch_count(1));
    assert!(should_cancel_for_touch_count(2));
    assert!(should_cancel_for_touch_count(5));
}

#[test]
fn scope_changes_cancel_captured_gestures_before_release() {
    let mut tracker = PointerTracker::default();
    let cabinet = scope(Screen::Cabinet, PointerLayer::Board);
    let game = scope(Screen::Game(GameId::Game2048), PointerLayer::Board);
    let tutorial = scope(Screen::Game(GameId::Game2048), PointerLayer::Tutorial);

    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    assert!(tracker.sync_scope(game));
    assert_eq!(tracker.release(Some(Vec2::new(40., 10.)), game), None);

    tracker.press(Some(Vec2::new(10., 10.)), game);
    assert!(tracker.sync_scope(tutorial));
    assert_eq!(tracker.release(Some(Vec2::new(10., 10.)), tutorial), None);
}

#[test]
fn state_scope_tracks_the_topmost_interaction_layer() {
    let mut state = AppState::default();
    assert_eq!(
        PointerScope::from_state(&state),
        scope(Screen::Cabinet, PointerLayer::Board)
    );

    state.screen = Screen::Game(GameId::Game2048);
    assert_eq!(
        PointerScope::from_state(&state),
        scope(Screen::Game(GameId::Game2048), PointerLayer::Board)
    );
    state.tutorial = Some(GameId::Game2048);
    assert_eq!(
        PointerScope::from_state(&state),
        scope(Screen::Game(GameId::Game2048), PointerLayer::Tutorial)
    );
    state.tutorial = None;
    state.confirm_restart = true;
    state.pending_restart = Some(crate::ui::UiAction::Restart);
    assert_eq!(
        PointerScope::from_state(&state),
        scope(
            Screen::Game(GameId::Game2048),
            PointerLayer::RestartConfirmation
        )
    );
    state.confirm_restart = false;
    state.pending_restart = None;
    state.confirm_reset = true;
    assert_eq!(
        PointerScope::from_state(&state),
        scope(
            Screen::Game(GameId::Game2048),
            PointerLayer::ResetConfirmation
        )
    );
    state.lifecycle_paused = true;
    assert_eq!(
        PointerScope::from_state(&state),
        scope(Screen::Game(GameId::Game2048), PointerLayer::LifecyclePause)
    );
}
