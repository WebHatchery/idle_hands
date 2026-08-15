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

#[test]
fn default_pyramid_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!pyramid(&state).is_empty());
    assert_eq!(pyramid(&state), pyramid(&state));
}

#[test]
fn pyramid_hint_finds_an_exposed_pair() {
    let mut state = AppState::default();
    state.pyramid.pyramid = vec![None; 28];
    state.pyramid.pyramid[26] = Some(crate::cards::Card {
        rank: 5,
        suit: 0,
        face_up: true,
    });
    state.pyramid.pyramid[27] = Some(crate::cards::Card {
        rank: 8,
        suit: 1,
        face_up: true,
    });
    state.pyramid.stock.clear();
    assert_eq!(pyramid(&state), "Pair exposed cards 27 and 28.");
}

#[test]
fn default_tri_peaks_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!tri_peaks(&state).is_empty());
    assert_eq!(tri_peaks(&state), tri_peaks(&state));
}

#[test]
fn default_klondike_golf_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!klondike_golf(&state).is_empty());
    assert_eq!(klondike_golf(&state), klondike_golf(&state));
}

#[test]
fn default_spider_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!spider_solitaire(&state).is_empty());
    assert_eq!(spider_solitaire(&state), spider_solitaire(&state));
}
