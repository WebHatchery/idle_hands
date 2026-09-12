//! Regression coverage for the tests module.

use super::*;
use crate::data::GameData;
use crate::state::GameId;

#[test]
fn every_registered_game_has_three_touch_first_lines() {
    let content = GameData::load().unwrap().content;
    for game in GameId::ALL {
        let lines = instructions(&content, game);
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
        assert!(lines.iter().all(|line| {
            let lower = line.to_ascii_lowercase();
            !lower.contains("keyboard") && !lower.contains("press key") && !lower.contains("click")
        }));
    }
}
