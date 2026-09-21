//! Regression coverage for the tests module.

use idle_hands::testing::modules::tutorial_library_ui::*;
use idle_hands::testing::state::{AppState, GameId, Screen};
use idle_hands::testing::tutorial_library_data;

#[test]
fn tapping_a_tutorial_card_routes_to_that_canonical_game() {
    let state = AppState {
        screen: Screen::Tutorials,
        ..AppState::default()
    };

    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(&state, vec2(30., 130.)).as_slice(),
            [UiAction::OpenTutorial(index)] if *index == GameId::Solitaire.index()
        ));
    });
}

#[test]
fn new_only_filter_is_a_touchable_toggle() {
    let state = AppState {
        screen: Screen::Tutorials,
        ..AppState::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(&state, vec2(950., 100.)).as_slice(),
            [UiAction::ToggleTutorialFilter]
        ));
    });
}

#[test]
fn lesson_cards_and_controls_stay_inside_each_viewport() {
    let assert_layout = || {
        let (width, height) = idle_hands::testing::ui::layout_size();
        for index in 0..tutorial_library_data::visible_count() {
            let rect = card_rect(index);
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        let (previous, next, back) = control_rects();
        for rect in [previous, next, back] {
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
    };
    idle_hands::testing::ui::with_desktop_layout(assert_layout);
    let assert_layout = || {
        let (width, height) = idle_hands::testing::ui::layout_size();
        for index in 0..tutorial_library_data::visible_count() {
            let rect = card_rect(index);
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        let (previous, next, back) = control_rects();
        for rect in [previous, next, back] {
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
    };
    idle_hands::testing::ui::with_compact_landscape_layout(assert_layout);
    let assert_layout = || {
        let (width, height) = idle_hands::testing::ui::layout_size();
        for index in 0..tutorial_library_data::visible_count() {
            let rect = card_rect(index);
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        let (previous, next, back) = control_rects();
        for rect in [previous, next, back] {
            assert!(rect.x >= 0. && rect.y >= 0.);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
    };
    idle_hands::testing::ui::with_portrait_layout(assert_layout);
}
