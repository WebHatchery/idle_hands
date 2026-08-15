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
    assert_eq!(game.status, PyramidStatus::Stuck);
}
