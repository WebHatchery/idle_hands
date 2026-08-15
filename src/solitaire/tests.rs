use super::*;

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
