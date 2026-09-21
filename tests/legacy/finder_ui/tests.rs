//! Regression coverage for the tests module.

use idle_hands::testing::finder_data;
use idle_hands::testing::modules::finder_ui::*;
use idle_hands::testing::state::{AppState, Screen};

#[test]
fn paging_controls_stop_at_both_edges() {
    let mut state = AppState {
        screen: Screen::Finder,
        ..AppState::default()
    };

    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(clicks(&state, vec2(680., 610.)).is_empty());
        assert!(matches!(
            clicks(&state, vec2(810., 610.)).as_slice(),
            [UiAction::LibraryScroll(12)]
        ));
        state.library_scroll = finder_data::scroll_limit(&state);
        assert!(clicks(&state, vec2(810., 610.)).is_empty());
        assert!(matches!(
            clicks(&state, vec2(680., 610.)).as_slice(),
            [UiAction::LibraryScroll(-12)]
        ));
    });
}

#[test]
fn accessibility_modes_keep_finder_cards_readable() {
    let high_contrast = AppState {
        high_contrast: true,
        ..AppState::default()
    };
    assert_eq!(
        card_fill(
            &high_contrast,
            idle_hands::testing::state::GameId::Solitaire,
            true
        ),
        idle_hands::testing::theme::SURFACE_DARK
    );

    assert!(
        idle_hands::testing::accessibility::text_size(14., true)
            > idle_hands::testing::accessibility::text_size(14., false),
        "large-text mode should enlarge the measured title size"
    );
}

#[test]
fn every_visible_finder_card_exposes_a_launch_action() {
    let state = AppState {
        screen: Screen::Finder,
        ..AppState::default()
    };

    let assert_cards = || {
        for index in 0..finder_data::visible_count() {
            let rect = card_rect(index);
            assert!(matches!(
                clicks(&state, rect.center()).as_slice(),
                [UiAction::Open(_)]
            ));
        }
    };
    idle_hands::testing::ui::with_desktop_layout(assert_cards);
    idle_hands::testing::ui::with_compact_landscape_layout(assert_cards);
    idle_hands::testing::ui::with_portrait_layout(assert_cards);
}
