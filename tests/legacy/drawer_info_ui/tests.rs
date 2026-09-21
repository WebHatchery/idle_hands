//! Regression coverage for the tests module.

use idle_hands::testing::modules::drawer_info_ui::*;
use idle_hands::testing::state::{AppState, GameId, Screen};

#[test]
fn drawer_info_actions_open_favorite_and_return() {
    let state = AppState {
        screen: Screen::DrawerInfo(GameId::Solitaire),
        ..AppState::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(
            matches!(clicks(&state, vec2(625., 525.)).as_slice(), [UiAction::Open(index)] if *index == GameId::Solitaire.index())
        );
        assert!(
            matches!(clicks(&state, vec2(815., 525.)).as_slice(), [UiAction::ToggleFavorite(index)] if *index == GameId::Solitaire.index())
        );
        assert!(matches!(
            clicks(&state, vec2(1015., 525.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn drawer_info_actions_follow_compact_and_portrait_layouts() {
    let state = AppState {
        screen: Screen::DrawerInfo(GameId::Solitaire),
        ..AppState::default()
    };
    let assert_actions = || {
        assert!(matches!(
            clicks(&state, open_rect().center()).as_slice(),
            [UiAction::Open(index)] if *index == GameId::Solitaire.index()
        ));
        assert!(matches!(
            clicks(&state, favorite_rect().center()).as_slice(),
            [UiAction::ToggleFavorite(index)] if *index == GameId::Solitaire.index()
        ));
        assert!(matches!(
            clicks(&state, back_rect().center()).as_slice(),
            [UiAction::Cabinet]
        ));
    };

    idle_hands::testing::ui::with_compact_landscape_layout(assert_actions);
    idle_hands::testing::ui::with_portrait_layout(assert_actions);
}

#[test]
fn drawer_info_layout_controls_fit_every_viewport() {
    let assert_layout = || {
        let (width, height) = idle_hands::testing::ui::layout_size();
        for rect in [open_rect(), favorite_rect(), back_rect()] {
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
    };
    idle_hands::testing::ui::with_desktop_layout(assert_layout);
    idle_hands::testing::ui::with_compact_landscape_layout(assert_layout);
    idle_hands::testing::ui::with_portrait_layout(assert_layout);
}

#[test]
fn favorite_action_matches_the_shared_availability_contract() {
    let state = AppState {
        screen: Screen::DrawerInfo(GameId::WordForge),
        ..AppState::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
        let actions = clicks(&state, vec2(815., 525.));
        if idle_hands::testing::cabinet_status::is_available(GameId::WordForge) {
            assert!(matches!(
                actions.as_slice(),
                [UiAction::ToggleFavorite(index)] if *index == GameId::WordForge.index()
            ));
        } else {
            assert!(actions.is_empty());
        }
    });
}
