//! Regression coverage for the tests module.

use super::*;
use crate::state::GameId;

fn scope(screen: Screen, layer: PointerLayer) -> PointerScope {
    PointerScope { screen, layer }
}

#[test]
fn device_matrix_keeps_rendering_and_input_on_one_transform() {
    use macroquad_toolkit::ui::VirtualUi;

    let devices = [
        (320., 568., 360., 780.),
        (390., 844., 360., 780.),
        (568., 320., 840., 390.),
        (844., 390., 840., 390.),
        (768., 1024., 360., 780.),
        (1024., 768., 1280., 720.),
    ];
    for (screen_w, screen_h, logical_w, logical_h) in devices {
        let viewport = VirtualUi::from_screen_size(logical_w, logical_h, screen_w, screen_h);
        let center = Vec2::new(logical_w / 2., logical_h / 2.);
        let screen_center = viewport.ui_to_screen(center);
        let mapped = viewport.screen_to_ui_checked(screen_center).unwrap();
        assert!((mapped - center).length() < 0.001);
        assert_eq!(
            viewport.viewport_for_dpi(1.),
            (
                viewport.offset.x.round() as i32,
                viewport.offset.y.round() as i32,
                (logical_w * viewport.scale).round() as i32,
                (logical_h * viewport.scale).round() as i32,
            )
        );
        assert_eq!(
            viewport
                .screen_to_ui_checked(Vec2::new(viewport.offset.x - 1., viewport.offset.y - 1.)),
            None
        );
    }
}

#[test]
fn device_matrix_keeps_shared_button_hits_at_least_44_physical_points() {
    use macroquad::prelude::Rect;
    use macroquad_toolkit::ui::{VirtualUi, MIN_TARGET};

    let devices = [
        (320., 568., 360., 780.),
        (390., 844., 360., 780.),
        (568., 320., 840., 390.),
        (844., 390., 840., 390.),
        (768., 1024., 360., 780.),
        (1024., 768., 1280., 720.),
    ];
    for (screen_w, screen_h, logical_w, logical_h) in devices {
        let viewport = VirtualUi::from_screen_size(logical_w, logical_h, screen_w, screen_h);
        let area = crate::ui::physical_touch_rect(Rect::new(100., 100., 100., 30.), viewport.scale);
        assert!(area.w * viewport.scale >= MIN_TARGET - 0.01);
        assert!(area.h * viewport.scale >= MIN_TARGET - 0.01);
    }
}

#[test]
fn smallest_phone_matrix_keeps_body_text_physically_readable() {
    use macroquad_toolkit::ui::VirtualUi;

    for (screen_w, screen_h, logical_w, logical_h) in [
        (320., 568., 360., 780.),
        (568., 320., 844., 390.),
        (1024., 768., 1280., 720.),
    ] {
        let viewport = VirtualUi::from_screen_size(logical_w, logical_h, screen_w, screen_h);
        let body = crate::ui::readable_text_size_for_scale(10., viewport.scale);
        let caption = crate::ui::readable_text_size_for_scale(8., viewport.scale);
        assert!(body * viewport.scale >= 11. - 0.01);
        assert!(caption * viewport.scale >= 9. - 0.01);
    }
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
fn drag_distance_keeps_the_boundary_between_tap_and_drag_explicit() {
    let mut tracker = PointerTracker::default();
    let cabinet = scope(Screen::Cabinet, PointerLayer::Board);
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    assert!(matches!(
        tracker.release(Some(Vec2::new(10. + DRAG_DISTANCE, 10.)), cabinet),
        Some(Gesture::Tap(_))
    ));
    tracker.press(Some(Vec2::new(10., 10.)), cabinet);
    assert!(matches!(
        tracker.release(Some(Vec2::new(10. + DRAG_DISTANCE + 0.01, 10.)), cabinet),
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
