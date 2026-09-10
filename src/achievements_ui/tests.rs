use super::*;

#[test]
fn achievement_filter_counts_partition_the_shelf() {
    let mut state = AppState::default();
    state.achievements[AchievementId::FirstFinish.index()] = true;
    state.achievements[AchievementId::FullCabinet.index()] = true;
    assert_eq!(filter_count(0, &state), AchievementId::ALL.len());
    assert_eq!(filter_count(1, &state), 2);
    assert_eq!(filter_count(2, &state), AchievementId::ALL.len() - 2);
    assert_eq!(filter_button_label(1, &state), "EARNED 2");
}
