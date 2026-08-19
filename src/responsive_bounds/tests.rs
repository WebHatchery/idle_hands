use super::*;

#[test]
fn every_catalog_game_has_a_portrait_contract() {
    for game in GameId::ALL {
        assert_contract(game, Orientation::Portrait);
    }
}

#[test]
fn every_catalog_game_has_a_compact_landscape_contract() {
    for game in GameId::ALL {
        assert_contract(game, Orientation::CompactLandscape);
    }
}

#[test]
fn catalog_and_contracts_stay_in_lockstep() {
    assert_eq!(GameId::ALL.len(), 47);
    for game in GameId::ALL {
        assert!(game.title().chars().count() > 1);
    }
}
