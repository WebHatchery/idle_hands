use super::*;
use crate::achievements_data;

#[test]
fn achievement_filter_counts_partition_the_shelf() {
    let mut state = AppState::default();
    state.achievements[AchievementId::FirstFinish.index()] = true;
    state.achievements[AchievementId::FullCabinet.index()] = true;
    assert_eq!(
        achievements_data::filter_count(&state, 0),
        AchievementId::ALL.len()
    );
    assert_eq!(achievements_data::filter_count(&state, 1), 2);
    assert_eq!(
        achievements_data::filter_count(&state, 2),
        AchievementId::ALL.len() - 2
    );
    assert_eq!(filter_button_label(1, &state), "EARNED 2");
}

#[test]
fn desktop_achievement_grid_stays_inside_its_panel() {
    crate::ui::with_desktop_layout(|| {
        let layout = layout();
        assert!(card_rect(layout, AchievementId::ALL.len() - 1).bottom() <= layout.panel.bottom());
    });
}
