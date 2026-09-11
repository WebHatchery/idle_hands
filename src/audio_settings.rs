//! Shared sound-level labels and playback volume for the profile settings.

pub const DEFAULT_LEVEL: u8 = 2;
const LEVEL_COUNT: u8 = 4;

pub const fn default_level() -> u8 {
    DEFAULT_LEVEL
}

pub const fn normalize(level: u8) -> u8 {
    if level >= LEVEL_COUNT {
        LEVEL_COUNT - 1
    } else {
        level
    }
}

pub const fn next_level(level: u8) -> u8 {
    (normalize(level) + 1) % LEVEL_COUNT
}

pub const fn label(enabled: bool, level: u8) -> &'static str {
    if !enabled {
        "Off"
    } else {
        match normalize(level) {
            0 => "Quiet",
            1 => "Clear",
            2 => "Full",
            _ => "Bright",
        }
    }
}

pub const fn volume(enabled: bool, level: u8) -> f32 {
    if !enabled {
        0.0
    } else {
        match normalize(level) {
            0 => 0.25,
            1 => 0.5,
            2 => 0.75,
            _ => 1.0,
        }
    }
}

#[cfg(test)]
mod tests;
