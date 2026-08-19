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
    blocked.bridges = 0;
    blocked.resolve();
    assert_eq!(blocked.status, TriPeaksStatus::Stuck);
}

#[test]
fn tableau_runs_build_points_and_peaks_pay_a_bonus() {
    let mut game = TriPeaks::new(24);
    game.tableau = vec![None; 28];
    game.tableau[0] = Some(card(7));
    game.tableau[18] = Some(card(5));
    game.tableau[19] = Some(card(6));
    game.waste = vec![card(4)];
    game.stock.clear();

    assert!(game.tap(18));
    assert_eq!((game.run, game.points), (1, 10));
    assert!(game.tap(19));
    assert_eq!((game.run, game.best_run, game.points), (2, 2, 30));
    assert!(game.tap(0));
    assert_eq!((game.run, game.points), (3, 110));
    assert_eq!(game.status, TriPeaksStatus::Won);
}

#[test]
fn drawing_stock_breaks_a_run_and_undo_restores_it() {
    let mut game = TriPeaks::new(25);
    game.run = 4;
    game.best_run = 5;
    game.points = 90;
    assert!(game.draw_stock());
    assert_eq!(game.run, 0);
    assert!(game.undo());
    assert_eq!((game.run, game.best_run, game.points), (4, 5, 90));
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

#[test]
fn changing_rule_starts_fresh_and_reset_preserves_it() {
    let mut game = TriPeaks::new(28);
    game.moves = 8;
    game.set_rule(TriPeaksRule::Wrap, 29);
    assert_eq!(game.rule, TriPeaksRule::Wrap);
    assert_eq!(game.moves, 0);
    game.reset(30);
    assert_eq!(game.rule, TriPeaksRule::Wrap);
}

#[test]
fn legacy_saves_receive_depth_defaults() {
    let original = TriPeaks::new(31);
    let mut value = serde_json::to_value(&original).unwrap();
    for field in [
        "rule",
        "points",
        "run",
        "best_run",
        "bridges",
        "bridge_armed",
    ] {
        value.as_object_mut().unwrap().remove(field);
    }
    let restored: TriPeaks = serde_json::from_value(value).unwrap();
    assert_eq!(restored.rule, TriPeaksRule::Strict);
    assert_eq!(
        (restored.points, restored.run, restored.best_run),
        (0, 0, 0)
    );
    assert_eq!((restored.bridges, restored.bridge_armed), (1, false));
}
