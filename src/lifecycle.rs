//! Frame-gap thresholds used to detect a return from an unfocused window.

pub const FRAME_GAP_SECONDS: f32 = 0.5;

pub fn should_pause_for_gap(frame_seconds: f32) -> bool {
    frame_seconds.is_finite() && frame_seconds >= FRAME_GAP_SECONDS
}

#[cfg(test)]
mod tests;
