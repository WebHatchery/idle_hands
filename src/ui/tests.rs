use super::*;

#[test]
fn compact_header_title_starts_after_the_cabinet_breadcrumb() {
    let breadcrumb_budget = 76.;
    let title_x = std::hint::black_box(COMPACT_HEADER_TITLE_X);
    assert!(title_x > breadcrumb_budget);
    assert!(title_x < 494.);
    let status_x = std::hint::black_box(COMPACT_HEADER_STATUS_X);
    assert!(status_x + 190. < 494.);
}

#[test]
fn desktop_help_routes_all_visible_buttons() {
    let state = crate::state::AppState {
        screen: crate::state::Screen::Help,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(650., 650.)).as_slice(),
            [UiAction::Rules]
        ));
        assert!(matches!(
            actions_at(&state, vec2(850., 650.)).as_slice(),
            [UiAction::Credits]
        ));
        assert!(matches!(
            actions_at(&state, vec2(1080., 650.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn credits_round_trip_routes_through_every_layout() {
    let help_state = crate::state::AppState {
        screen: crate::state::Screen::Help,
        ..Default::default()
    };
    let credits_state = crate::state::AppState {
        screen: crate::state::Screen::Credits,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&help_state, vec2(850., 650.)).as_slice(),
            [UiAction::Credits]
        ));
        assert!(matches!(
            actions_at(&credits_state, vec2(1100., 660.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&help_state, vec2(160., 550.)).as_slice(),
            [UiAction::Credits]
        ));
        assert!(matches!(
            actions_at(&credits_state, vec2(50., 670.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&help_state, vec2(600., 300.)).as_slice(),
            [UiAction::Credits]
        ));
        assert!(matches!(
            actions_at(&credits_state, vec2(400., 330.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn restart_modal_blocks_game_background_in_every_layout() {
    let state = crate::state::AppState {
        screen: crate::state::Screen::Game(crate::state::GameId::Game2048),
        confirm_restart: true,
        pending_restart: Some(UiAction::Restart),
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(actions_at(&state, vec2(100., 100.)).is_empty());
    });
    with_portrait_layout(|| {
        assert!(actions_at(&state, vec2(10., 100.)).is_empty());
    });
    with_compact_landscape_layout(|| {
        assert!(actions_at(&state, vec2(10., 100.)).is_empty());
    });
}
