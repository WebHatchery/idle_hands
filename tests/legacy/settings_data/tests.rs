//! Regression coverage for the tests module.

use idle_hands::testing::cosmetics::CosmeticKind;
use idle_hands::testing::modules::settings_data::*;

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
