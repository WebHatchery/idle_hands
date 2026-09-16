//! Regression coverage for the tests module.

use super::{is_demo_game, GameCategory, ALL, DEMO_GAMES_PER_CATEGORY};

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
