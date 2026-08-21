use super::*;

#[test]
fn compact_tri_peaks_status_is_short_enough_for_the_header_lane() {
    assert!(compact_status_text(TriPeaksStatus::Playing).len() <= 12);
}
