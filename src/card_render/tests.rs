use super::*;

#[test]
fn card_labels_are_safe_for_catalog_and_invalid_values() {
    assert_eq!(rank_label(1), "A");
    assert_eq!(rank_label(13), "K");
    assert_eq!(rank_label(14), "?");
    assert_eq!(suit_label(0), "♣");
    assert_eq!(suit_label(4), "?");
}

#[test]
fn selection_pulse_respects_selection_and_reduced_motion() {
    assert_eq!(selection_pulse(1., false, false), 0.);
    assert_eq!(selection_pulse(1., true, true), 0.);
    assert!((0. ..=1.).contains(&selection_pulse(0.25, true, false)));
}
