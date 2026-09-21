//! Regression coverage for the tests module.

use idle_hands::testing::modules::profile_ui::*;
use idle_hands::testing::state::{AppState, Screen};

#[test]
fn profile_choices_route_to_visible_touch_actions() {
    let state = AppState {
        screen: Screen::Profile,
        ..AppState::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
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
        let (width, height) = idle_hands::testing::ui::layout_size();
        for rect in name_rects() {
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        let back = back_rect();
        assert!(back.right() <= width && back.bottom() <= height);
    };
    idle_hands::testing::ui::with_desktop_layout(assert_layout);
    idle_hands::testing::ui::with_compact_landscape_layout(assert_layout);
    idle_hands::testing::ui::with_portrait_layout(assert_layout);
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
