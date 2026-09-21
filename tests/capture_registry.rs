//! Capture-scene routing regressions.

use idle_hands::testing::{modules::capture_registry::screen_for_scene, GameId, Screen};

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
    for scene in [
        "records_progress",
        "statistics",
        "tutorials",
        "finder",
        "profile",
        "drawer_info",
    ] {
        assert_eq!(screen_for_scene(scene), None);
    }
}
