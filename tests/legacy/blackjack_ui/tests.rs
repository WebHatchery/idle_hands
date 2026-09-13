//! Regression coverage for the tests module.

use super::*;

#[test]
fn compact_blackjack_status_is_short_enough_for_the_header_lane() {
    let status = compact_status_text(BlackjackStatus::Playing, 13);
    assert!(status.len() <= 10);
}

#[test]
fn portrait_blackjack_status_uses_compact_copy() {
    assert_eq!(
        status_line_text(BlackjackStatus::Playing, 13, 17, false, true),
        "Total 13"
    );
    assert!(status_line_text(BlackjackStatus::Playing, 13, 17, false, false).contains("dealer"));
}

#[test]
fn round_summary_stays_below_the_player_hand() {
    for (compact, portrait) in [(true, false), (false, true), (false, false)] {
        assert!(round_summary_y(compact, portrait) >= player_hand_bottom(compact, portrait) + 12.);
    }
}
