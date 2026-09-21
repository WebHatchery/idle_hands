//! Regression coverage for the tests module.

use idle_hands::testing::modules::achievements_ui::*;
use idle_hands::testing::progression::AchievementId;

#[test]
fn desktop_achievement_grid_stays_inside_its_panel() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        let layout = layout();
        assert!(card_rect(layout, AchievementId::ALL.len() - 1).bottom() <= layout.panel.bottom());
    });
}
