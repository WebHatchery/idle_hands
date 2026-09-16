//! Regression coverage for the tests module.

use super::*;

#[test]
fn saved_custom_names_remain_visible_without_a_selected_preset() {
    let state = AppState {
        profile_name: "Patient Player".to_owned(),
        ..AppState::default()
    };

    assert_eq!(current_index(&state), None);
}

#[test]
fn display_names_are_safe_for_the_identity_plate() {
    assert_eq!(display_name("  "), "Cabinet Guest");
    assert_eq!(display_name("quiet\nsolver"), "quietsolver");
    assert_eq!(
        display_name("abcdefghijklmnopqrstuvwxyz"),
        "abcdefghijklmnopqrstuvw…"
    );
}
