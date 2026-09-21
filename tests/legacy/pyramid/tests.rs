//! Regression coverage for the tests module.

use idle_hands::testing::modules::pyramid::*;

fn card(rank: u8) -> Card {
    Card {
        rank,
        suit: 0,
        face_up: true,
    }
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
