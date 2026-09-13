//! Regression coverage for the tests module.

use super::{is_demo_game, GameCategory, ALL, DEMO_GAMES_PER_CATEGORY};
use crate::state::GameId;

#[test]
fn registry_matches_the_closed_game_collection() {
    assert_eq!(ALL.len(), GameId::ALL.len());
    for (index, descriptor) in ALL.iter().enumerate() {
        assert_eq!(descriptor.id, GameId::ALL[index]);
        assert_eq!(descriptor.index, index);
        assert_eq!(descriptor.id.save_key(), descriptor.save_key);
        assert!(descriptor.active);
        assert!(!descriptor.title.is_empty());
        assert!(!descriptor.subtitle.is_empty());
    }
}

#[test]
fn save_keys_are_unique() {
    for (index, descriptor) in ALL.iter().enumerate() {
        assert!(
            ALL[..index]
                .iter()
                .all(|other| other.save_key != descriptor.save_key),
            "duplicate save key {}",
            descriptor.save_key
        );
    }
}

#[test]
fn demo_contains_five_games_from_every_category() {
    for category in [
        GameCategory::Cards,
        GameCategory::Logic,
        GameCategory::Board,
        GameCategory::Word,
        GameCategory::Arcade,
        GameCategory::Misc,
    ] {
        assert_eq!(
            ALL.iter()
                .filter(|descriptor| descriptor.category == category)
                .filter(|descriptor| is_demo_game(descriptor.id))
                .count(),
            DEMO_GAMES_PER_CATEGORY
        );
    }
}

#[test]
fn demo_selection_totals_thirty_unique_games() {
    assert_eq!(
        ALL.iter()
            .filter(|descriptor| is_demo_game(descriptor.id))
            .count(),
        30
    );
}
