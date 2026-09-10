//! Stable calendar identity for the Daily Dungeon.

const SECONDS_PER_DAY: f64 = 86_400.0;
const SEED_VERSION: u64 = 1;

pub fn current_day() -> u64 {
    (macroquad::miniquad::date::now().max(0.0) / SECONDS_PER_DAY).floor() as u64
}

pub fn seed_for_day(day: u64) -> u64 {
    let mut value = day
        .wrapping_add(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(SEED_VERSION);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

pub fn challenge_for_day(day: u64) -> u32 {
    (day % 10_000) as u32
}

pub fn label(day: u64, fallback_challenge: u32) -> String {
    if day == 0 {
        format!("DAY {fallback_challenge:04}")
    } else {
        format!("DAY {day}")
    }
}

pub fn status_label(
    day: u64,
    fallback_challenge: u32,
    phase: crate::daily_dungeon::DailyPhase,
) -> String {
    let state = match phase {
        crate::daily_dungeon::DailyPhase::Exploring => "READY",
        crate::daily_dungeon::DailyPhase::Won => "CLEARED",
        crate::daily_dungeon::DailyPhase::Lost => "CLOSED",
    };
    format!("{} · {state}", label(day, fallback_challenge))
}

pub fn preview_action(phase: crate::daily_dungeon::DailyPhase, best_score: Option<u32>) -> String {
    match phase {
        crate::daily_dungeon::DailyPhase::Exploring => best_score.map_or_else(
            || "dungeon awaits  >".to_owned(),
            |score| format!("BEST {score}  ·  OPEN  >"),
        ),
        crate::daily_dungeon::DailyPhase::Won | crate::daily_dungeon::DailyPhase::Lost => {
            best_score.map_or_else(
                || "replay route  >".to_owned(),
                |score| format!("BEST {score}  ·  REPLAY  >"),
            )
        }
    }
}

#[cfg(test)]
mod tests;
