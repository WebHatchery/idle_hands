//! Regression coverage for the tests module.

use super::*;

fn card(rank: u8) -> Card {
    Card {
        rank,
        suit: 0,
        face_up: true,
    }
}

#[test]
fn bottom_cards_expose_the_peak_and_follow_rank_rules() {
    let mut game = TriPeaks::new(20);
    game.tableau = vec![None; 28];
    game.tableau[0] = Some(card(7));
    game.tableau[3] = Some(card(4));
    game.tableau[4] = Some(card(5));
    game.tableau[9] = Some(card(2));
    game.tableau[10] = Some(card(3));
    game.tableau[18] = Some(card(6));
    game.waste = vec![card(7)];
    game.stock.clear();
    assert!(!game.exposed(0));
    assert!(game.exposed(18));
    assert!(game.can_play(18));
    assert!(game.tap(18));
    assert_eq!(game.moves, 1);
}

#[test]
fn clearing_the_table_wins_and_blocked_tableau_sticks() {
    let mut game = TriPeaks::new(22);
    game.tableau = vec![None; 28];
    game.tableau[27] = Some(card(5));
    game.waste = vec![card(6)];
    game.stock.clear();
    assert!(game.tap(27));
    assert_eq!(game.status, TriPeaksStatus::Won);

    let mut blocked = TriPeaks::new(23);
    blocked.tableau = vec![None; 28];
    blocked.tableau[27] = Some(card(2));
    blocked.waste = vec![card(9)];
    blocked.stock.clear();
    blocked.bridges = 0;
    blocked.resolve();
    assert_eq!(blocked.status, TriPeaksStatus::Stuck);
}

#[test]
fn wrap_rule_connects_ace_and_king() {
    let mut game = TriPeaks::new(26);
    game.tableau = vec![None; 28];
    game.tableau[27] = Some(card(1));
    game.waste = vec![card(13)];
    game.stock.clear();
    game.bridges = 0;

    assert!(!game.can_play(27));
    game.rule = TriPeaksRule::Wrap;
    assert!(game.can_play(27));
    assert_eq!(game.playable_count(), 1);
}

#[test]
fn bridge_opens_any_exposed_card_and_undo_refunds_it() {
    let mut game = TriPeaks::new(27);
    game.tableau = vec![None; 28];
    game.tableau[26] = Some(card(3));
    game.tableau[27] = Some(card(9));
    game.waste = vec![card(5)];
    game.stock.clear();
    game.resolve();

    assert_eq!(game.status, TriPeaksStatus::Playing);
    assert_eq!(game.playable_count(), 0);
    assert!(game.toggle_bridge());
    assert_eq!(game.playable_count(), 2);
    assert!(game.tap(27));
    assert_eq!((game.bridges, game.bridge_armed), (0, false));
    assert!(game.undo());
    assert_eq!((game.bridges, game.bridge_armed), (1, true));
    assert_eq!(game.playable_count(), 2);
}
