use super::*;

#[test]
fn default_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!solitaire(&state).is_empty());
    assert_eq!(solitaire(&state), solitaire(&state));
}

#[test]
fn default_freecell_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!freecell(&state).is_empty());
    assert_eq!(freecell(&state), freecell(&state));
}
