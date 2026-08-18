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
fn harder_difficulties_use_larger_boards() {
    assert_eq!(DotsBoxes::new_with_difficulty(1, DotsDifficulty::Standard).boxes.len(), 16);
    assert_eq!(DotsBoxes::new_with_difficulty(1, DotsDifficulty::Hard).boxes.len(), 25);
    assert_eq!(DotsBoxes::new_with_difficulty(1, DotsDifficulty::Expert).boxes.len(), 36);
}

#[test]
fn rejects_used_edges_and_undo_restores_the_turn() {
    let mut game = DotsBoxes::new(2);
    assert!(game.play(Edge::Horizontal(0)));
    assert!(!game.play(Edge::Horizontal(0)));
    assert!(game.undo());
    assert!(!game.horizontal[0]);
    assert_eq!(game.current_player, 0);
}

#[test]
fn cpu_turn_is_deterministic_and_can_finish_a_board() {
    let mut first = DotsBoxes::new(42);
    let mut second = DotsBoxes::new(42);
    for index in 0..(HORIZONTAL + VERTICAL) {
        if first.phase != DotsPhase::Playing {
            break;
        }
        let edge = if index < 20 {
            Edge::Horizontal(index)
        } else {
            Edge::Vertical(index - 20)
        };
        first.play(edge);
        second.play(edge);
    }
    assert_eq!(first.horizontal, second.horizontal);
    assert_eq!(first.vertical, second.vertical);
    assert_ne!(first.phase, DotsPhase::Playing);
    assert!(first.scores.iter().sum::<u8>() as usize == BOX_COUNT);
}

#[test]
fn hint_prefers_a_box_closing_edge_without_mutating_the_board() {
    let mut game = DotsBoxes::new(1);
    game.horizontal[0] = true;
    game.horizontal[4] = true;
    game.vertical[0] = true;
    let before = game.clone();

    assert_eq!(game.hint_edge(), Some(Edge::Vertical(1)));
    assert_eq!(game.horizontal, before.horizontal);
    assert_eq!(game.vertical, before.vertical);
    assert_eq!(game.scores, before.scores);
}

#[test]
fn hint_is_empty_after_dots_and_boxes_ends() {
    let mut game = DotsBoxes::new(1);
    game.phase = DotsPhase::Won;

    assert_eq!(game.hint_edge(), None);
}
