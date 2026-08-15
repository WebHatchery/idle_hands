use super::*;

fn card(rank: u8) -> Card {
    Card {
        rank,
        suit: 0,
        face_up: true,
    }
}

#[test]
fn seeded_deals_are_repeatable_and_use_the_full_deck() {
    let first = TriPeaks::new(19);
    let second = TriPeaks::new(19);
    assert_eq!(first.tableau, second.tableau);
    assert_eq!(first.tableau.len(), 28);
    assert_eq!(first.stock.len() + first.waste.len(), 24);
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
fn stock_draw_and_undo_restore_the_previous_state() {
    let mut game = TriPeaks::new(21);
    let stock = game.stock.len();
    assert!(game.draw_stock());
    assert_eq!(game.stock.len(), stock - 1);
    assert!(game.undo());
    assert_eq!(game.stock.len(), stock);
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
    blocked.resolve();
    assert_eq!(blocked.status, TriPeaksStatus::Stuck);
}
