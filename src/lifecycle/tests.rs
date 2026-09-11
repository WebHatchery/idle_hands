use super::*;

#[test]
fn ordinary_frames_do_not_pause_the_cabinet() {
    assert!(!should_pause_for_gap(0.1));
    assert!(!should_pause_for_gap(FRAME_GAP_SECONDS - f32::EPSILON));
}

#[test]
fn long_or_invalid_frames_are_only_paused_when_the_gap_is_real() {
    assert!(should_pause_for_gap(FRAME_GAP_SECONDS));
    assert!(should_pause_for_gap(3.0));
    assert!(!should_pause_for_gap(f32::NAN));
    assert!(!should_pause_for_gap(f32::INFINITY));
}
