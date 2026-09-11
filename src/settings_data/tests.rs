use super::*;
use crate::cosmetics::CosmeticKind;

#[test]
fn cosmetic_rows_follow_state_values_and_stamp_unlocks() {
    let state = AppState {
        stamps: 4,
        card_back: 1,
        board_theme: 1,
        ..AppState::default()
    };

    let rows = cosmetic_rows(&state);

    assert_eq!(rows[0].kind, CosmeticKind::CardBack);
    assert_eq!(rows[0].option.name, "Moss");
    assert_eq!(rows[0].unlocked, 2);
    assert_eq!(rows[1].option.name, "Moss felt");
    assert_eq!(rows[1].unlocked, 2);
}

#[test]
fn cosmetic_rows_expose_next_thresholds_and_all_open_state() {
    let closed = cosmetic_rows(&AppState::default());
    assert_eq!(next_label(closed[0]), "NEXT 2 STAMPS");

    let open = cosmetic_rows(&AppState {
        stamps: 8,
        ..AppState::default()
    });
    assert_eq!(next_label(open[0]), "ALL OPEN");
    assert_eq!(open[3].total, 3);
}

#[test]
fn cosmetic_rows_recover_locked_saved_selection_for_display() {
    let state = AppState {
        stamps: 0,
        board_theme: 2,
        ..AppState::default()
    };

    let rows = cosmetic_rows(&state);

    assert_eq!(rows[1].current, 0);
    assert_eq!(rows[1].option.name, "Walnut felt");
}
