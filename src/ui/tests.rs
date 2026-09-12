//! Regression coverage for the tests module.

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
            actions_at(&state, vec2(450., 650.)).as_slice(),
            [UiAction::Tutorials]
        ));
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

#[test]
fn lifecycle_pause_routes_only_resume_touch_in_every_layout() {
    let state = crate::state::AppState {
        screen: crate::state::Screen::Game(crate::state::GameId::Game2048),
        lifecycle_paused: true,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(640., 462.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
        assert!(actions_at(&state, vec2(100., 100.)).is_empty());
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(180., 439.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
        assert!(actions_at(&state, vec2(10., 100.)).is_empty());
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(420., 276.)).as_slice(),
            [UiAction::ResumeLifecycle]
        ));
        assert!(actions_at(&state, vec2(10., 100.)).is_empty());
    });
}

#[test]
fn save_recovery_dismiss_routes_through_every_layout() {
    let mut notice = crate::save_recovery::SaveRecoveryNotice::new();
    notice.record(true);
    let state = crate::state::AppState {
        screen: crate::state::Screen::Game(crate::state::GameId::Game2048),
        save_recovery: Some(notice),
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(855., 164.)).as_slice(),
            [UiAction::DismissSaveRecovery]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(244., 182.)).as_slice(),
            [UiAction::DismissSaveRecovery]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&state, vec2(602., 149.)).as_slice(),
            [UiAction::DismissSaveRecovery]
        ));
    });
}

#[test]
fn notice_log_routes_open_and_close_through_every_layout() {
    let settings = crate::state::AppState {
        screen: crate::state::Screen::Settings,
        ..Default::default()
    };
    let open_points = [vec2(1035., 584.), vec2(180., 634.), vec2(690., 290.)];
    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&settings, open_points[0]).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&settings, open_points[1]).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&settings, open_points[2]).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });

    let open = crate::state::AppState {
        screen: crate::state::Screen::Settings,
        notice_log_view: true,
        ..Default::default()
    };
    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&open, vec2(920., 614.)).as_slice(),
            [UiAction::ToggleNoticeLog]
        ));
    });
}

#[test]
fn statistics_entry_routes_from_records_in_every_layout() {
    let records = crate::state::AppState {
        screen: crate::state::Screen::Records,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&records, vec2(1000., 280.)).as_slice(),
            [UiAction::Statistics]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&records, vec2(70., 145.)).as_slice(),
            [UiAction::Statistics]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&records, vec2(200., 20.)).as_slice(),
            [UiAction::Statistics]
        ));
    });
}

#[test]
fn tutorial_entry_routes_from_help_in_every_layout() {
    let help = crate::state::AppState {
        screen: crate::state::Screen::Help,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&help, vec2(450., 650.)).as_slice(),
            [UiAction::Tutorials]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&help, vec2(50., 495.)).as_slice(),
            [UiAction::Tutorials]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&help, vec2(350., 300.)).as_slice(),
            [UiAction::Tutorials]
        ));
    });
}

#[test]
fn finder_entry_and_return_are_touchable_in_every_layout() {
    let cabinet = crate::state::AppState {
        screen: crate::state::Screen::Cabinet,
        ..Default::default()
    };
    let finder = crate::state::AppState {
        screen: crate::state::Screen::Finder,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&cabinet, vec2(1070., 50.)).as_slice(),
            [UiAction::Finder]
        ));
        assert!(matches!(
            actions_at(&finder, vec2(950., 610.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&cabinet, vec2(220., 30.)).as_slice(),
            [UiAction::Finder]
        ));
        assert!(matches!(
            actions_at(&finder, vec2(50., 675.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&cabinet, vec2(600., 20.)).as_slice(),
            [UiAction::Finder]
        ));
        assert!(matches!(
            actions_at(&finder, vec2(740., 340.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn profile_editor_entry_and_return_are_touchable_in_every_layout() {
    let settings = crate::state::AppState {
        screen: crate::state::Screen::Settings,
        ..Default::default()
    };
    let profile = crate::state::AppState {
        screen: crate::state::Screen::Profile,
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&settings, vec2(500., 170.)).as_slice(),
            [UiAction::Profile]
        ));
        assert!(matches!(
            actions_at(&profile, vec2(315., 575.)).as_slice(),
            [UiAction::Settings]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&settings, vec2(250., 110.)).as_slice(),
            [UiAction::Profile]
        ));
        assert!(matches!(
            actions_at(&profile, vec2(85., 624.)).as_slice(),
            [UiAction::Settings]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&settings, vec2(520., 285.)).as_slice(),
            [UiAction::Profile]
        ));
        assert!(matches!(
            actions_at(&profile, vec2(745., 342.)).as_slice(),
            [UiAction::Settings]
        ));
    });
}

#[test]
fn drawer_info_return_routes_through_the_shared_ui_dispatcher() {
    let drawer_info = crate::state::AppState {
        screen: crate::state::Screen::DrawerInfo(crate::state::GameId::Solitaire),
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&drawer_info, vec2(1015., 525.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&drawer_info, vec2(292., 624.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&drawer_info, vec2(745., 337.)).as_slice(),
            [UiAction::Cabinet]
        ));
    });
}

#[test]
fn quick_browse_info_routes_through_records_in_every_layout() {
    let favorites = crate::state::AppState {
        screen: crate::state::Screen::Records,
        favorites_view: true,
        favorites: {
            let mut favorites = vec![false; crate::state::GameId::ALL.len()];
            favorites[crate::state::GameId::Solitaire.index()] = true;
            favorites
        },
        ..Default::default()
    };
    let recent = crate::state::AppState {
        screen: crate::state::Screen::Records,
        recent_view: true,
        recent_games: vec![crate::state::GameId::Solitaire],
        ..Default::default()
    };

    with_desktop_layout(|| {
        assert!(matches!(
            actions_at(&favorites, vec2(279., 211.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
        assert!(matches!(
            actions_at(&recent, vec2(321., 211.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
    });
    with_compact_landscape_layout(|| {
        assert!(matches!(
            actions_at(&favorites, vec2(324., 92.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
        assert!(matches!(
            actions_at(&recent, vec2(366., 92.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
    });
    with_portrait_layout(|| {
        assert!(matches!(
            actions_at(&favorites, vec2(276., 182.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
        assert!(matches!(
            actions_at(&recent, vec2(318., 182.)).as_slice(),
            [UiAction::Inspect(index)] if *index == crate::state::GameId::Solitaire.index()
        ));
    });
}
