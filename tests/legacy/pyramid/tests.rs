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
fn seeded_deals_are_repeatable_and_have_full_pyramid() {
    let first = Pyramid::new(7);
    let second = Pyramid::new(7);
    assert_eq!(first.pyramid, second.pyramid);
    assert_eq!(first.stock.len(), 24);
    assert_eq!(
        first.pyramid.iter().filter(|card| card.is_some()).count(),
        28
    );
}

#[test]
fn exposed_cards_can_pair_to_thirteen() {
    let mut game = Pyramid::new(1);
    game.pyramid = vec![None; 28];
    game.pyramid[21] = Some(card(5));
    game.pyramid[22] = Some(card(8));
    assert!(game.tap(21));
    assert!(game.tap(22));
    assert_eq!(game.moves, 1);
    assert_eq!(game.status, PyramidStatus::Won);
}

#[test]
fn kings_clear_alone_and_stock_can_pair_with_pyramid() {
    let mut game = Pyramid::new(2);
    game.pyramid = vec![None; 28];
    game.pyramid[27] = Some(card(13));
    assert!(game.tap(27));
    assert_eq!(game.status, PyramidStatus::Won);

    let mut game = Pyramid::new(3);
    game.pyramid = vec![None; 28];
    game.pyramid[27] = Some(card(5));
    game.stock.clear();
    game.waste = vec![card(8)];
    assert!(game.tap(WASTE_INDEX));
    assert!(game.tap(27));
    assert_eq!(game.status, PyramidStatus::Won);
}

#[test]
fn stock_draw_and_undo_restore_the_previous_state() {
    let mut game = Pyramid::new(4);
    let before = game.stock.len();
    assert!(game.draw_stock());
    assert_eq!(game.stock.len(), before - 1);
    assert!(game.undo());
    assert_eq!(game.stock.len(), before);
    assert!(game.waste.is_empty());
}

#[test]
fn blocked_empty_stock_becomes_stuck() {
    let mut game = Pyramid::new(5);
    game.pyramid = vec![None; 28];
    game.pyramid[27] = Some(card(2));
    game.stock.clear();
    game.waste.clear();
    game.resolve();
    assert_eq!(game.status, PyramidStatus::Stuck);
}

#[test]
fn a_waste_king_keeps_an_empty_stock_playable() {
    let mut game = Pyramid::new(6);
    game.pyramid = vec![None; 28];
    game.pyramid[27] = Some(card(2));
    game.stock.clear();
    game.waste = vec![card(13)];
    game.resolve();
    assert_eq!(game.status, PyramidStatus::Playing);
    assert!(game.tap(WASTE_INDEX));
    game.redeals_remaining = 0;
    game.resolve();
    assert_eq!(game.status, PyramidStatus::Stuck);
}

#[test]
fn consecutive_clears_build_a_scoring_chain() {
    let mut game = Pyramid::new(7);
    game.pyramid = vec![None; 28];
    for (index, rank) in [(21, 5), (22, 8), (23, 6), (24, 7)] {
        game.pyramid[index] = Some(card(rank));
    }
    assert!(game.tap(21));
    assert!(game.tap(22));
    assert_eq!((game.combo, game.points), (1, 20));
    assert!(game.tap(23));
    assert!(game.tap(24));
    assert_eq!((game.combo, game.best_combo, game.points), (2, 2, 60));
}

#[test]
fn draw_three_turns_three_stock_cards_and_breaks_the_chain() {
    let mut game = Pyramid::new(8);
    game.draw_rule = PyramidDraw::Three;
    game.combo = 4;
    let before = game.stock.len();
    assert!(game.draw_stock());
    assert_eq!(game.stock.len(), before - 3);
    assert_eq!(game.waste.len(), 3);
    assert_eq!(game.combo, 0);
}

#[test]
fn empty_stock_can_recycle_the_waste_once() {
    let mut game = Pyramid::new(9);
    game.stock.clear();
    game.waste = vec![card(2), card(4), card(6)];
    assert!(game.draw_stock());
    assert_eq!(game.redeals_remaining, 0);
    assert!(game.waste.is_empty());
    assert_eq!(game.stock.len(), 3);
    assert!(game.draw_stock());
    game.stock.clear();
    game.waste.clear();
    assert!(!game.draw_stock());
}

#[test]
fn pair_count_and_partner_preview_use_only_exposed_cards() {
    let mut game = Pyramid::new(10);
    game.pyramid = vec![None; 28];
    game.pyramid[21] = Some(card(5));
    game.pyramid[22] = Some(card(8));
    game.pyramid[23] = Some(card(13));
    assert!(game.legal_pair(21, 22));
    assert_eq!(game.available_pair_count(), 2);
}

#[test]
fn legacy_saves_receive_draw_one_scoring_and_one_recycle() {
    let original = Pyramid::new(11);
    let mut value = serde_json::to_value(&original).unwrap();
    let object = value.as_object_mut().unwrap();
    for field in [
        "draw_rule",
        "points",
        "combo",
        "best_combo",
        "redeals_remaining",
    ] {
        object.remove(field);
    }
    let restored: Pyramid = serde_json::from_value(value).unwrap();
    assert_eq!(restored.draw_rule, PyramidDraw::One);
    assert_eq!((restored.points, restored.combo), (0, 0));
    assert_eq!(restored.redeals_remaining, 1);
}
