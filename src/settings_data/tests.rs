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

#[test]
fn cosmetic_action_mapping_matches_shared_row_order() {
    assert!(matches!(
        cosmetic_action(0),
        Some(crate::ui_action::UiAction::CycleCardBack)
    ));
    assert!(matches!(
        cosmetic_action(1),
        Some(crate::ui_action::UiAction::CycleBoardTheme)
    ));
    assert!(matches!(
        cosmetic_action(2),
        Some(crate::ui_action::UiAction::CycleSoundSet)
    ));
    assert!(matches!(
        cosmetic_action(3),
        Some(crate::ui_action::UiAction::CycleCabinetDecoration)
    ));
    assert!(cosmetic_action(4).is_none());
}

#[test]
fn accessibility_labels_use_consistent_meanings() {
    let labels = accessibility_labels(&AppState {
        sound: true,
        reduced_motion: true,
        high_contrast: true,
        large_text: true,
        ..AppState::default()
    });
    assert_eq!(labels.sound, "On");
    assert_eq!(labels.volume, "Full");
    assert_eq!(labels.motion, "Reduced");
    assert_eq!(labels.contrast, "On");
    assert_eq!(labels.text, "Large");

    let defaults = accessibility_labels(&AppState::default());
    assert_eq!(defaults.motion, "Full");
    assert_eq!(defaults.text, "Normal");

    let quiet = accessibility_labels(&AppState {
        sound_level: 0,
        ..AppState::default()
    });
    assert_eq!(quiet.volume, "Quiet");

    let muted_state = AppState {
        sound: false,
        ..AppState::default()
    };
    let muted = accessibility_labels(&muted_state);
    assert_eq!(muted.volume, "Off");
    assert_eq!(volume_meter(&muted_state), "····");
}
