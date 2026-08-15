use super::*;

#[test]
fn viewport_preserves_aspect_and_rejects_letterbox_taps() {
    let viewport = Viewport::new(1024., 768., 1280., 720.);
    assert!((viewport.scale - 0.8).abs() < f32::EPSILON);
    assert_eq!(viewport.screen_to_logical(Vec2::new(512., 10.)), None);
    assert_eq!(
        viewport.screen_to_logical(viewport.logical_to_screen(Vec2::new(640., 360.))),
        Some(Vec2::new(640., 360.))
    );
}

#[test]
fn pointer_tracker_distinguishes_taps_drags_and_cancelled_releases() {
    let mut tracker = PointerTracker::default();
    tracker.press(Some(Vec2::new(10., 10.)));
    assert_eq!(
        tracker.release(Some(Vec2::new(12., 12.))),
        Some(Gesture::Tap(Vec2::new(12., 12.)))
    );
    tracker.press(Some(Vec2::new(10., 10.)));
    assert_eq!(
        tracker.release(Some(Vec2::new(40., 10.))),
        Some(Gesture::Drag {
            start: Vec2::new(10., 10.),
            end: Vec2::new(40., 10.)
        })
    );
    tracker.press(Some(Vec2::new(10., 10.)));
    tracker.cancel();
    assert_eq!(tracker.release(Some(Vec2::new(40., 10.))), None);
}
