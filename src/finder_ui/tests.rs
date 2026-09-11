use super::*;
use crate::state::{AppState, Screen};

#[test]
fn filters_and_navigation_are_touchable_on_desktop() {
    let mut state = AppState {
        screen: Screen::Finder,
        ..AppState::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(&state, vec2(345., 145.)).as_slice(),
            [UiAction::FinderFilter(1)]
        ));
        state.library_scroll = 12;
        assert!(matches!(
            clicks(&state, vec2(680., 610.)).as_slice(),
            [UiAction::LibraryScroll(-12)]
        ));
        assert!(matches!(
            clicks(&state, vec2(950., 610.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn paging_controls_stop_at_both_edges() {
    let mut state = AppState {
        screen: Screen::Finder,
        ..AppState::default()
    };

    crate::ui::with_desktop_layout(|| {
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
fn drawer_cards_open_their_alphabetical_game() {
    let state = AppState {
        screen: Screen::Finder,
        ..AppState::default()
    };

    crate::ui::with_portrait_layout(|| {
        let first = finder_data::page(&state)[0].index();
        assert!(
            matches!(clicks(&state, vec2(30., 220.)).as_slice(), [UiAction::Open(index)] if *index == first)
        );
    });
}

#[test]
fn all_controls_and_cards_fit_each_viewport() {
    let assert_layout = || {
        let (width, height) = crate::ui::layout_size();
        for index in 0..finder_data::visible_count() {
            let rect = card_rect(index);
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        for rect in filter_rects() {
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
        for rect in [control_rects().0, control_rects().1, control_rects().2] {
            assert!(rect.right() <= width && rect.bottom() <= height);
        }
    };
    crate::ui::with_desktop_layout(assert_layout);
    crate::ui::with_compact_landscape_layout(assert_layout);
    crate::ui::with_portrait_layout(assert_layout);
}

#[test]
fn accessibility_modes_keep_finder_cards_readable() {
    let high_contrast = AppState {
        high_contrast: true,
        ..AppState::default()
    };
    assert_eq!(
        card_fill(&high_contrast, crate::state::GameId::Solitaire, true),
        crate::theme::SURFACE_DARK
    );

    assert!(
        crate::accessibility::text_size(14., true) > crate::accessibility::text_size(14., false),
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
    crate::ui::with_desktop_layout(assert_cards);
    crate::ui::with_compact_landscape_layout(assert_cards);
    crate::ui::with_portrait_layout(assert_cards);
}
