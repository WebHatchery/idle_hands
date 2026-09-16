//! Regression coverage for the tests module.

use super::*;

#[test]
fn desktop_achievement_grid_stays_inside_its_panel() {
    crate::ui::with_desktop_layout(|| {
        let layout = layout();
        assert!(card_rect(layout, AchievementId::ALL.len() - 1).bottom() <= layout.panel.bottom());
    });
}
