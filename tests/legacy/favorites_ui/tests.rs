//! Regression coverage for the tests module.

use super::*;
use crate::state::AppState;

#[test]
fn favorite_card_remove_action_targets_the_starred_drawer() {
    let state = AppState {
        favorites: {
            let mut favorites = vec![false; GameId::ALL.len()];
            favorites[GameId::Solitaire.index()] = true;
            favorites
        },
        ..Default::default()
    };

    crate::ui::with_portrait_layout(|| {
        let point = favorite_remove_rect(list_card_rect(layout(), 0)).center();
        assert!(matches!(
            clicks(&state, point).as_slice(),
            [UiAction::ToggleFavorite(index)] if *index == GameId::Solitaire.index()
        ));
    });
}

#[test]
fn browse_tabs_switch_between_favorites_and_recent_at_each_size() {
    let state = AppState::default();
    crate::ui::with_desktop_layout(|| {
        let (favorites, recent) = browse_tab_rects();
        assert!(matches!(
            clicks(&state, favorites.center()).as_slice(),
            [UiAction::Favorites]
        ));
        assert!(matches!(
            clicks(&state, recent.center()).as_slice(),
            [UiAction::Recent]
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        let (favorites, recent) = browse_tab_rects();
        assert!(matches!(
            clicks(&state, favorites.center()).as_slice(),
            [UiAction::Favorites]
        ));
        assert!(matches!(
            clicks(&state, recent.center()).as_slice(),
            [UiAction::Recent]
        ));
    });
    crate::ui::with_portrait_layout(|| {
        let (favorites, recent) = browse_tab_rects();
        assert!(matches!(
            clicks(&state, favorites.center()).as_slice(),
            [UiAction::Favorites]
        ));
        assert!(matches!(
            clicks(&state, recent.center()).as_slice(),
            [UiAction::Recent]
        ));
    });
}
