use super::*;
use crate::state::{AppState, GameId, Screen};

#[test]
fn back_and_paging_controls_route_in_desktop() {
    let state = AppState {
        screen: Screen::Tutorials,
        ..AppState::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(&state, vec2(950., 610.)).as_slice(),
            [UiAction::Help]
        ));
        assert!(matches!(
            clicks(&state, vec2(680., 610.)).as_slice(),
            [UiAction::LibraryScroll(-1)]
        ));
        assert!(matches!(
            clicks(&state, vec2(810., 610.)).as_slice(),
            [UiAction::LibraryScroll(1)]
        ));
    });
}

#[test]
fn tapping_a_tutorial_card_routes_to_that_canonical_game() {
    let state = AppState {
        screen: Screen::Tutorials,
        ..AppState::default()
    };

    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(&state, vec2(30., 130.)).as_slice(),
            [UiAction::OpenTutorial(index)] if *index == GameId::Solitaire.index()
        ));
    });
}
