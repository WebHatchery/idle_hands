//! Frame-gap thresholds used to detect a return from an unfocused window.

pub const FRAME_GAP_SECONDS: f32 = 0.5;

pub fn should_pause_for_gap(frame_seconds: f32) -> bool {
    frame_seconds.is_finite() && frame_seconds >= FRAME_GAP_SECONDS
}

pub fn should_pause_game(
    frame_seconds: f32,
    is_game_screen: bool,
    tutorial_visible: bool,
    restart_confirmation: bool,
    reset_confirmation: bool,
    already_paused: bool,
) -> bool {
    should_pause_for_gap(frame_seconds)
        && is_game_screen
        && !tutorial_visible
        && !restart_confirmation
        && !reset_confirmation
        && !already_paused
}

#[cfg(test)]
#[path = "../tests/legacy/lifecycle/tests.rs"]
mod tests;
