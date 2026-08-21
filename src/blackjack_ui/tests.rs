use super::*;

#[test]
fn compact_blackjack_status_is_short_enough_for_the_header_lane() {
    let status = compact_status_text(BlackjackStatus::Playing, 13);
    assert!(status.len() <= 10);
}
