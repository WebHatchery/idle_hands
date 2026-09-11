use super::*;
use crate::state::GameId;

#[test]
fn every_registered_game_has_three_touch_first_lines() {
    for game in GameId::ALL {
        let lines = instructions(game);
        assert_eq!(lines.len(), 3);
        assert!(lines.iter().any(|line| {
            line.contains("Tap")
                || line.contains("tap")
                || line.contains("Swipe")
                || line.contains("Drag")
        }));
        assert!(lines.iter().all(|line| {
            let lower = line.to_ascii_lowercase();
            !lower.contains("dismiss") && !lower.contains("confirm")
        }));
    }
}
