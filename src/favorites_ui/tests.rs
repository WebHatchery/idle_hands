use super::*;
use crate::state::AppState;

#[test]
fn favorite_shelf_uses_a_touch_safe_capacity_at_each_size() {
    let mut state = AppState::default();
    state.favorites.fill(true);

    crate::ui::with_desktop_layout(|| {
        assert_eq!(visible_rows(&state).len(), DESKTOP_VISIBLE_GAMES);
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert_eq!(visible_rows(&state).len(), 10);
    });
    crate::ui::with_portrait_layout(|| {
        assert_eq!(visible_rows(&state).len(), 8);
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
fn empty_recent_shelf_has_no_clear_action_at_any_size() {
    let state = AppState {
        recent_view: true,
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(clicks(&state, quick_action_rect().center()).is_empty());
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(clicks(&state, quick_action_rect().center()).is_empty());
    });
    crate::ui::with_portrait_layout(|| {
        assert!(clicks(&state, quick_action_rect().center()).is_empty());
    });
}

#[test]
fn short_shelves_have_no_inert_paging_controls() {
    let state = AppState {
        recent_view: true,
        recent_games: vec![GameId::Solitaire],
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| assert!(scroll_rects(&state).is_none()));
    crate::ui::with_compact_landscape_layout(|| assert!(scroll_rects(&state).is_none()));
    crate::ui::with_portrait_layout(|| assert!(scroll_rects(&state).is_none()));
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
fn quick_browse_info_lanes_open_drawer_details_at_each_size() {
    let favorite_state = AppState {
        favorites: {
            let mut favorites = vec![false; GameId::ALL.len()];
            favorites[GameId::Solitaire.index()] = true;
            favorites
        },
        ..Default::default()
    };
    let recent_state = AppState {
        recent_view: true,
        recent_games: vec![GameId::Solitaire],
        ..Default::default()
    };

    let assert_lanes = || {
        let rect = list_card_rect(layout(), 0);
        assert!(matches!(
            clicks(&favorite_state, info_rect(rect, false).center()).as_slice(),
            [UiAction::Inspect(index)] if *index == GameId::Solitaire.index()
        ));
        assert!(matches!(
            clicks(&recent_state, info_rect(rect, true).center()).as_slice(),
            [UiAction::Inspect(index)] if *index == GameId::Solitaire.index()
        ));
    };

    crate::ui::with_desktop_layout(assert_lanes);
    crate::ui::with_compact_landscape_layout(assert_lanes);
    crate::ui::with_portrait_layout(assert_lanes);
}

#[test]
fn quick_browse_info_lanes_keep_their_touch_geometry_clear() {
    let assert_geometry = || {
        let card = list_card_rect(layout(), 0);
        let favorite_info = info_rect(card, false);
        let remove = favorite_remove_rect(card);
        let recent_info = info_rect(card, true);

        assert!(favorite_info.x >= card.x);
        assert!(favorite_info.right() <= card.right());
        assert!(favorite_info.right() < remove.x);
        assert!(recent_info.x >= card.x);
        assert!(recent_info.right() < card.right());
    };

    crate::ui::with_desktop_layout(assert_geometry);
    crate::ui::with_compact_landscape_layout(assert_geometry);
    crate::ui::with_portrait_layout(assert_geometry);
}

#[test]
fn restricted_quick_browse_cards_still_open_drawer_details() {
    let game = if crate::game_descriptor::is_demo_build() {
        GameId::SpiderSolitaire
    } else {
        GameId::Solitaire
    };
    let favorite_state = AppState {
        favorites: {
            let mut favorites = vec![false; GameId::ALL.len()];
            favorites[game.index()] = true;
            favorites
        },
        ..Default::default()
    };
    let recent_state = AppState {
        recent_view: true,
        recent_games: vec![game],
        ..Default::default()
    };

    let assert_details = || {
        let card = list_card_rect(layout(), 0);
        assert!(matches!(
            clicks(&favorite_state, info_rect(card, false).center()).as_slice(),
            [UiAction::Inspect(index)] if *index == game.index()
        ));
        assert!(matches!(
            clicks(&recent_state, info_rect(card, true).center()).as_slice(),
            [UiAction::Inspect(index)] if *index == game.index()
        ));
    };

    crate::ui::with_desktop_layout(assert_details);
    crate::ui::with_compact_landscape_layout(assert_details);
    crate::ui::with_portrait_layout(assert_details);
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

#[test]
fn completed_browse_cards_can_show_their_best_clear_time() {
    let mut state = AppState::default();
    state.records.ensure_time_slots();
    state.records.elapsed_seconds[GameId::Solitaire.index()] = 42;
    state.records.record_time(GameId::Solitaire.index());

    assert_eq!(
        browse_status_label(&state, GameId::Solitaire, "COMPLETE"),
        "DONE 0m 42s"
    );
    assert_eq!(
        browse_status_label(&state, GameId::FreeCell, "PLAY NOW"),
        "PLAY NOW"
    );
}
