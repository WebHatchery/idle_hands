use super::*;

#[test]
fn portrait_scoreline_uses_short_header_copy() {
    let status = portrait_score_status(0, 0, 0);

    assert_eq!(status, "Red 0  •  Blue 0  •  0 moves");
    assert!(status.chars().count() < 30);
}
