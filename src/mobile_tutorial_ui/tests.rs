use super::*;

#[test]
fn every_game_has_three_touch_specific_instructions() {
    for game in GameId::ALL {
        let lines = tutorial_ui::instructions(game);
        assert_eq!(lines.len(), 3);
        assert!(
            lines.iter().any(|line| {
                line.contains("Tap")
                    || line.contains("tap")
                    || line.contains("Swipe")
                    || line.contains("Drag")
            }),
            "{} has no direct touch instruction: {:?}",
            game.title(),
            lines
        );
        assert!(lines.iter().all(|line| {
            let lower = line.to_ascii_lowercase();
            !lower.contains("dismiss") && !lower.contains("confirm")
        }));
    }
}

#[test]
fn responsive_continue_buttons_stay_inside_their_panels() {
    assert!(contains(PORTRAIT_PANEL, continue_rect(false)));
    assert!(contains(LANDSCAPE_PANEL, continue_rect(true)));
}

fn contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}
