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
fn portrait_wrapping_keeps_long_instructions_readable() {
    for game in GameId::ALL {
        for instruction in tutorial_ui::instructions(game) {
            assert!(wrap(instruction, 39).iter().all(|line| line.len() <= 39));
        }
    }
}
