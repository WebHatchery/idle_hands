//! Regression coverage for the tests module.

use idle_hands::testing::cards::Card;
use idle_hands::testing::modules::solitaire::*;

#[test]
fn stock_draw_and_undo_restore_the_deal() {
    let mut game = Solitaire::default();
    let stock = game.stock.len();
    game.draw_stock();
    assert_eq!(game.waste.len(), 1);
    assert!(game.undo());
    assert_eq!(game.waste.len(), 0);
    assert_eq!(game.stock.len(), stock);
}

#[test]
fn draw_three_ruleset_keeps_the_waste_order_for_redeals() {
    let mut game = Solitaire::with_ruleset(9, SolitaireRuleset::DrawThreeUnlimited);
    game.draw_stock();
    assert_eq!(game.waste.len(), 3);
    let drawn = game.waste.clone();
    while !game.stock.is_empty() {
        game.draw_stock();
    }
    game.draw_stock();
    assert_eq!(game.stock.last(), drawn.first());
}

#[test]
fn foundation_requires_the_next_card_of_the_same_suit() {
    let mut game = Solitaire::default();
    game.waste.push(Card {
        rank: 2,
        suit: 0,
        face_up: true,
    });
    game.select_waste();
    assert!(!game.move_to_foundation(0));
    game.waste.pop();
    game.waste.push(Card {
        rank: 1,
        suit: 0,
        face_up: true,
    });
    game.select_waste();
    assert!(game.move_to_foundation(0));
    assert_eq!(game.foundations[0], 1);
}

#[test]
fn rejected_tableau_destination_keeps_the_selected_source() {
    let mut game = Solitaire::default();
    game.tableau[0] = vec![Card {
        rank: 5,
        suit: 0,
        face_up: true,
    }];
    game.tableau[1] = vec![Card {
        rank: 9,
        suit: 2,
        face_up: true,
    }];
    assert!(game.select_tableau(0, 0));
    assert!(!game.move_to_tableau(1));
    assert_eq!(game.selected, Some(CardSource::Tableau(0, 0)));
}
