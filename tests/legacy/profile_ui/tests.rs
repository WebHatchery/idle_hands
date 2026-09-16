//! Regression coverage for the tests module.

use super::*;
use crate::state::{AppState, Screen};

#[test]
fn profile_choices_route_to_visible_touch_actions() {
    let state = AppState {
        screen: Screen::Profile,
        ..AppState::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(&state, name_rects()[3].center()).as_slice(),
            [UiAction::SetProfileName(3)]
        ));
        assert!(matches!(
            clicks(&state, back_rect().center()).as_slice(),
            [UiAction::Settings]
        ));
    });
}

#[test]
fn profile_cards_and_back_fit_every_viewport() {
    let assert_layout = || {
        let (width, height) = crate::ui::layout_size();
        for rect in name_rects() {
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        let back = back_rect();
        assert!(back.right() <= width && back.bottom() <= height);
    };
    crate::ui::with_desktop_layout(assert_layout);
    crate::ui::with_compact_landscape_layout(assert_layout);
    crate::ui::with_portrait_layout(assert_layout);
}

#[test]
fn high_contrast_nameplates_use_neutral_selected_luminance() {
    let state = AppState {
        high_contrast: true,
        ..AppState::default()
    };
    let selected = profile_card_fill(&state, true);
    let unselected = profile_card_fill(&state, false);

    assert_eq!(selected.r, selected.g);
    assert_eq!(selected.g, selected.b);
    assert!(selected.r > unselected.r);
}
