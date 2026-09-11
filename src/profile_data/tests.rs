use super::*;

#[test]
fn profile_names_are_stable_and_bounded() {
    let state = AppState::default();

    assert_eq!(NAMES.len(), 8);
    assert_eq!(current_index(&state), Some(0));
    assert!(NAMES
        .iter()
        .all(|name| !name.is_empty() && name.len() <= 24));
}

#[test]
fn invalid_name_indices_fall_back_to_the_guest_plate() {
    assert_eq!(name(0), "Cabinet Guest");
    assert_eq!(name(255), "Cabinet Guest");
}

#[test]
fn saved_custom_names_remain_visible_without_a_selected_preset() {
    let state = AppState {
        profile_name: "Patient Player".to_owned(),
        ..AppState::default()
    };

    assert_eq!(current_index(&state), None);
}
