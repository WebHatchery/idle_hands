//! Regression coverage for the tests module.

use idle_hands::testing::modules::daily_archive_ui::*;

#[test]
fn archive_paging_buttons_stop_at_the_history_edges() {
    let mut state = AppState::default();
    for day in 1..=11 {
        state
            .records
            .record_daily_result(day, day as u32, day % 2 == 0);
    }
    idle_hands::testing::ui::with_desktop_layout(|| {
        let layout = layout();
        assert!(clicks(&state, layout.previous.center()).is_empty());
        assert!(matches!(
            clicks(&state, layout.next.center()).as_slice(),
            [UiAction::DailyArchiveScroll(10)]
        ));
        state.daily_archive_scroll = 1;
        assert!(matches!(
            clicks(&state, layout.previous.center()).as_slice(),
            [UiAction::DailyArchiveScroll(-10)]
        ));
        state.daily_archive_scroll = 10;
        assert!(clicks(&state, layout.next.center()).is_empty());
    });
}

#[test]
fn archive_rows_offer_their_exact_day_at_each_touch_layout() {
    let mut state = AppState::default();
    state.records.record_daily_result(42, 120, true);

    idle_hands::testing::ui::with_desktop_layout(|| {
        let layout = layout();
        assert!(matches!(
            clicks(&state, archive_rect(layout, 0).center()).as_slice(),
            [UiAction::DailyArchiveOpen(42)]
        ));
    });
    idle_hands::testing::ui::with_compact_landscape_layout(|| {
        let layout = layout();
        assert!(matches!(
            clicks(&state, archive_rect(layout, 0).center()).as_slice(),
            [UiAction::DailyArchiveOpen(42)]
        ));
    });
    idle_hands::testing::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(matches!(
            clicks(&state, archive_rect(layout, 0).center()).as_slice(),
            [UiAction::DailyArchiveOpen(42)]
        ));
    });
}
