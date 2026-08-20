//! Capture-scene routing derived from the cabinet descriptor registry.

use crate::state::{GameId, Screen};

/// Resolve the canonical game prefix used by capture fixtures. Variant and
/// accessibility suffixes stay fixtures, while the game identity lives once
/// in `GameDescriptor::save_key`.
pub fn screen_for_scene(scene: &str) -> Option<Screen> {
    if scene == "gameplay" {
        return Some(Screen::Game(GameId::Game2048));
    }
    if let Some(game) = GameId::ALL
        .into_iter()
        .find(|game| scene == game.save_key())
    {
        return Some(Screen::Game(game));
    }
    GameId::ALL
        .into_iter()
        .filter(|game| {
            scene
                .strip_prefix(game.save_key())
                .is_some_and(|suffix| suffix.starts_with('_'))
        })
        .max_by_key(|game| game.save_key().len())
        .map(Screen::Game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_catalog_game_has_a_canonical_capture_prefix() {
        for game in GameId::ALL {
            assert_eq!(screen_for_scene(game.save_key()), Some(Screen::Game(game)));
        }
    }

    #[test]
    fn fixture_suffixes_share_their_game_route() {
        assert_eq!(
            screen_for_scene("word_ladder_routes"),
            Some(Screen::Game(GameId::WordLadder))
        );
        assert_eq!(screen_for_scene("records_progress"), None);
    }
}
