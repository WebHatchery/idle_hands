//! Regression coverage for the tests module.

use idle_hands::testing::modules::ui::*;

#[test]
fn credits_round_trip_routes_through_every_layout() {
    let help_state = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Help,
        ..Default::default()
    };
    let credits_state = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Credits,
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
    let state = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Game(
            idle_hands::testing::state::GameId::Game2048,
        ),
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
    let state = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Game(
            idle_hands::testing::state::GameId::Game2048,
        ),
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
    let mut notice = idle_hands::testing::save_recovery::SaveRecoveryNotice::new();
    notice.record(true);
    let state = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Game(
            idle_hands::testing::state::GameId::Game2048,
        ),
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
    let settings = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Settings,
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

    let open = idle_hands::testing::state::AppState {
        screen: idle_hands::testing::state::Screen::Settings,
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
