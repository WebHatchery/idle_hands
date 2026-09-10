use super::*;
use crate::daily_dungeon::DailyPhase;

#[test]
fn day_seed_and_challenge_are_stable() {
    assert_eq!(seed_for_day(20_000), seed_for_day(20_000));
    assert_ne!(seed_for_day(20_000), seed_for_day(20_001));
    assert_eq!(challenge_for_day(20_123), 123);
}

#[test]
fn legacy_label_uses_the_saved_challenge_number() {
    assert_eq!(label(0, 42), "DAY 0042");
    assert_eq!(label(20_042, 7), "DAY 20042");
}

#[test]
fn status_label_explains_the_daily_state() {
    assert_eq!(
        status_label(20_042, 7, DailyPhase::Exploring),
        "DAY 20042 · READY"
    );
    assert_eq!(
        status_label(20_042, 7, DailyPhase::Won),
        "DAY 20042 · CLEARED"
    );
}
