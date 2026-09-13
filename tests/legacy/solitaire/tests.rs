//! Regression coverage for the tests module.

use super::*;
use crate::cards::Card;

#[test]
fn seeded_deals_are_repeatable_and_complete() {
    let left = Solitaire::new(9);
    let right = Solitaire::new(9);
    assert_eq!(left.tableau, right.tableau);
    assert_eq!(
        left.tableau.iter().map(Vec::len).sum::<usize>() + left.stock.len(),
        52
    );
}

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
fn selecting_the_same_card_again_releases_it() {
    let mut game = Solitaire::default();
    game.waste.push(Card {
        rank: 1,
        suit: 0,
        face_up: true,
    });
    assert!(game.select_waste());
    assert!(game.tap_waste());
    assert_eq!(game.selected, None);

    game.tableau[0] = vec![Card {
        rank: 13,
        suit: 0,
        face_up: true,
    }];
    assert!(game.select_tableau(0, 0));
    assert!(game.tap_tableau(0, 0));
    assert_eq!(game.selected, None);
}

#[test]
fn drawing_from_stock_clears_a_waste_selection() {
    let mut game = Solitaire::default();
    game.waste.push(Card {
        rank: 1,
        suit: 0,
        face_up: true,
    });
    assert!(game.select_waste());
    game.draw_stock();
    assert_eq!(game.selected, None);
}

#[test]
fn tapping_another_face_up_tableau_card_replaces_an_invalid_selection() {
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
    assert!(game.tap_tableau(1, 0));
    assert_eq!(game.selected, Some(CardSource::Tableau(1, 0)));
}

#[test]
fn launch_ruleset_is_draw_one_with_unlimited_redeals() {
    let game = Solitaire::default();
    assert_eq!(game.ruleset, SolitaireRuleset::DrawOneUnlimited);
    assert_eq!(game.ruleset.label(), "DRAW 1 · UNLIMITED REDEALS");
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
fn select_then_select_moves_a_legal_tableau_card() {
    let mut game = Solitaire {
        tableau: vec![
            vec![Card {
                rank: 5,
                suit: 0,
                face_up: true,
            }],
            vec![Card {
                rank: 6,
                suit: 2,
                face_up: true,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ],
        ..Default::default()
    };
    assert!(game.select_tableau(0, 0));
    assert!(game.move_to_tableau(1));
    assert_eq!(game.tableau[1].len(), 2);
    assert_eq!(game.tableau[1][1].rank, 5);
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

#[test]
fn invalid_foundation_suit_is_rejected_without_losing_selection() {
    let mut game = Solitaire::default();
    game.waste.push(Card {
        rank: 1,
        suit: 0,
        face_up: true,
    });
    assert!(game.select_waste());
    assert!(!game.move_to_foundation(4));
    assert_eq!(game.selected, Some(CardSource::Waste));
}
