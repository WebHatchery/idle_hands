use super::*;
use crate::state::AppState;

#[test]
fn favorite_shelf_uses_a_touch_safe_capacity_at_each_size() {
    let mut state = AppState::default();
    state.favorites.fill(true);

    crate::ui::with_desktop_layout(|| {
        assert_eq!(visible_games(&state).len(), DESKTOP_VISIBLE_GAMES);
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert_eq!(visible_games(&state).len(), 10);
    });
    crate::ui::with_portrait_layout(|| {
        assert_eq!(visible_games(&state).len(), 8);
    });
}

#[test]
fn recent_shelf_clear_action_is_touchable_at_each_size() {
    let state = AppState {
        recent_view: true,
        recent_games: vec![GameId::Solitaire],
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(matches!(
            clicks(&state, quick_action_rect().center()).as_slice(),
            [UiAction::ClearRecent]
        ));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(matches!(
            clicks(&state, quick_action_rect().center()).as_slice(),
            [UiAction::ClearRecent]
        ));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(matches!(
            clicks(&state, quick_action_rect().center()).as_slice(),
            [UiAction::ClearRecent]
        ));
    });
}

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
fn browse_summary_separates_open_finished_and_locked_drawers() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);
    let games = vec![GameId::Solitaire, GameId::FreeCell];

    let summary = browse_summary(&state, &games);
    assert_eq!(
        summary,
        BrowseSummary {
            total: 2,
            open: 1,
            done: 1,
            locked: 0,
        }
    );
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

#[test]
fn portrait_status_copy_stays_short_and_meaningful() {
    assert_eq!(short_status("COMPLETE"), "DONE");
    assert_eq!(short_status("FULL VERSION"), "FULL");
    assert_eq!(short_status("IN PROGRESS"), "OPEN");
}
