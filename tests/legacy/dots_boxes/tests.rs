//! Regression coverage for the tests module.

use super::*;

#[test]
fn claims_a_box_and_keeps_the_turn() {
    let mut game = DotsBoxes::new(1);
    game.horizontal[0] = true;
    game.horizontal[4] = true;
    game.vertical[0] = true;
    assert_eq!(game.current_player, 0);
    assert!(game.play(Edge::Vertical(1)));
    assert_eq!(game.boxes[0], 1);
    assert_eq!(game.scores, [1, 0]);
}

#[test]
fn hard_and_expert_cpu_avoid_gifts_that_standard_may_take() {
    for difficulty in [DotsDifficulty::Hard, DotsDifficulty::Expert] {
        let mut game = DotsBoxes::new_with_difficulty(1, difficulty);
        let side = game.side();
        game.horizontal[0] = true;
        game.horizontal[side] = true;
        game.current_player = 1;

        let edge = game.choose_cpu_edge().unwrap();
        assert_eq!(game.edge_risk(edge), 0, "{}", difficulty.label());
    }
}

#[test]
fn three_sided_unclaimed_box_reports_danger() {
    let mut game = DotsBoxes::new(1);
    game.horizontal[0] = true;
    game.horizontal[4] = true;
    game.vertical[0] = true;

    assert_eq!(game.box_edge_count(0, 0), 3);
    assert_eq!(game.box_edge_count(99, 99), 0);
}

#[test]
fn undo_and_legacy_saves_restore_edge_ownership_safely() {
    let mut game = DotsBoxes::new(1);
    assert!(game.play(Edge::Horizontal(0)));
    assert!(game.undo());
    assert_eq!(game.edge_owner(Edge::Horizontal(0)), 0);

    let mut value = serde_json::to_value(DotsBoxes::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("horizontal_owners");
    value.as_object_mut().unwrap().remove("vertical_owners");
    let mut loaded: DotsBoxes = serde_json::from_value(value).unwrap();
    assert!(loaded.horizontal_owners.is_empty());
    assert!(loaded.play(Edge::Horizontal(0)));
    assert_eq!(loaded.horizontal_owners.len(), loaded.horizontal.len());
    assert_eq!(loaded.vertical_owners.len(), loaded.vertical.len());
}
