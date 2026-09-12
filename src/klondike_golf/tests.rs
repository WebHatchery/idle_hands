//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_deals_repeat_with_seven_columns() {
    let first = KlondikeGolf::new(7);
    let second = KlondikeGolf::new(7);
    assert_eq!(first.tableau, second.tableau);
    assert_eq!(first.tableau.len(), 7);
    assert_eq!(first.tableau.iter().map(Vec::len).sum::<usize>(), 35);
}

#[test]
fn adjacent_top_cards_move_to_waste_and_undo() {
    let mut game = KlondikeGolf::new(1);
    let waste = game.waste.last().unwrap().rank;
    let column = (0..7)
        .find(|&column| game.tableau[column].last().unwrap().rank.abs_diff(waste) == 1)
        .unwrap();
    assert!(game.tap_column(column));
    assert!(game.undo());
    assert_eq!(game.moves, 0);
}

#[test]
fn stock_draw_changes_the_waste() {
    let mut game = KlondikeGolf::new(2);
    let before = game.waste.last().unwrap().rank;
    assert!(game.draw_stock());
    assert_ne!(game.waste.last().unwrap().rank, before);
}

#[test]
fn relaxed_rule_allows_a_matching_rank() {
    let mut game = KlondikeGolf::new_with_rule(55, GolfRule::Relaxed);
    let waste = game.waste.last().unwrap().rank;
    game.tableau[0].last_mut().unwrap().rank = waste;
    assert!(game.tap_column(0));
}
