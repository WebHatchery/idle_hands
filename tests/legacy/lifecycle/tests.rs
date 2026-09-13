//! Regression coverage for the tests module.

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

#[test]
fn pause_policy_requires_a_live_game_without_an_existing_modal() {
    assert!(should_pause_game(0.5, true, false, false, false, false));
    assert!(!should_pause_game(0.5, false, false, false, false, false));
    assert!(!should_pause_game(0.5, true, true, false, false, false));
    assert!(!should_pause_game(0.5, true, false, true, false, false));
    assert!(!should_pause_game(0.5, true, false, false, true, false));
    assert!(!should_pause_game(0.5, true, false, false, false, true));
}
