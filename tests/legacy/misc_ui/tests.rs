//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_misc_titles_and_metrics_use_the_header_lane() {
    let game = MiscGame::new(0x4D49_5343_0001, MiscKind::OrbitOrder);

    crate::ui::with_portrait_layout(|| {
        assert_eq!(title_size(), 21.);
        assert_eq!(metrics_text(&game, true), "R0 • S0 • M0");
    });
}

#[test]
fn compact_misc_controls_stay_clear_of_the_panel() {
    crate::ui::with_compact_landscape_layout(|| assert_layout_is_clear(844., 390.));
}

#[test]
fn portrait_misc_controls_stay_clear_of_the_panel() {
    crate::ui::with_portrait_layout(|| assert_layout_is_clear(360., 780.));
}

#[test]
fn desktop_misc_controls_stay_clear_of_the_panel() {
    crate::ui::with_desktop_layout(|| assert_layout_is_clear(1280., 720.));
}

fn assert_layout_is_clear(width: f32, height: f32) {
    let layout = layout();
    let controls = [
        layout.hint,
        layout.undo,
        layout.clear,
        layout.submit,
        layout.new_game,
    ];

    assert!(controls
        .iter()
        .all(|control| !control.overlaps(&layout.panel)));
    for (index, left) in controls.iter().enumerate() {
        assert!(controls[index + 1..]
            .iter()
            .all(|right| !left.overlaps(right)));
        assert!(left.x >= 0. && left.y >= 0.);
        assert!(left.right() <= width && left.bottom() <= height);
    }
}

#[test]
fn all_eight_planets_have_separate_touch_targets() {
    for panel in [
        Rect::new(15., 100., 300., 310.),
        Rect::new(320., 105., 520., 310.),
    ] {
        let targets = orbit_rects(panel, 8);
        assert_eq!(targets.len(), 8);
        for (i, target) in targets.iter().enumerate() {
            assert!(target.w >= 44. && target.h >= 44.);
            assert!(panel.contains(vec2(target.x, target.y)));
            assert!(panel.contains(vec2(target.right(), target.bottom())));
            assert!(targets[i + 1..].iter().all(|other| !target.overlaps(other)));
        }
    }
}
