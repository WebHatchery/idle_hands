use super::*;

#[test]
fn seeded_boards_are_repeatable_and_start_without_matches() {
    let first = MatchThree::new(3);
    assert_eq!(first.cells, MatchThree::new(3).cells);
    assert!(find_matches_for_side(&first.cells, first.side())
        .iter()
        .all(|&matched| !matched));
}

#[test]
fn harder_difficulties_use_larger_fields_and_more_colors() {
    let data = crate::data::GameData::load().unwrap();
    let games: Vec<_> = MatchThreeDifficulty::ALL
        .into_iter()
        .map(|difficulty| MatchThree::new_with_config(3, difficulty, &data.puzzles.match_three))
        .collect();
    for (game, expected) in games
        .iter()
        .zip(data.puzzles.match_three.difficulties.iter())
    {
        assert_eq!(game.cells.len(), expected.side * expected.side);
        assert_eq!(game.color_count(), expected.colors);
        assert_eq!(game.target_score(), expected.target_score);
    }
    let standard = &games[0];
    let hard = &games[1];
    let expert = &games[2];
    assert!(standard.color_count() < hard.color_count());
    assert!(hard.color_count() < expert.color_count());
    assert!(find_matches_for_side(&expert.cells, expert.side())
        .iter()
        .all(|&matched| !matched));
}

#[test]
fn adjacent_swap_that_creates_a_match_scores_and_moves() {
    let mut game = MatchThree::new(4);
    game.cells = vec![
        1, 2, 1, 0, 0, 0, 2, 1, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2,
        3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2,
    ];
    assert!(game.tap(2));
    assert!(game.tap(9));
    assert!(game.score > 0);
    assert_eq!(game.moves, 1);
}

#[test]
fn rejects_non_adjacent_and_matchless_swaps() {
    let mut game = MatchThree::new(5);
    assert!(game.tap(0));
    assert!(!game.tap(8));
    assert_eq!(game.selected, Some(0));
    game.selected = None;
    assert!(game.tap(0));
    assert!(!game.tap(1));
}

#[test]
fn undo_restores_a_successful_swap_and_reset_clears_progress() {
    let mut game = MatchThree::new(6);
    let before = game.cells.clone();
    let mut moved = false;
    let side = game.side();
    'outer: for first in 0..game.cells.len() {
        for second in 0..game.cells.len() {
            if adjacent_for_side(first, second, side) {
                game.selected = Some(first);
                if game.tap(second) {
                    moved = true;
                    break 'outer;
                }
                game.cells = before.clone();
                game.selected = None;
            }
        }
    }
    assert!(moved);
    assert!(game.undo());
    assert_eq!(game.cells, before);
    game.reset(7);
    assert_eq!(game.score, 0);
    assert_eq!(game.moves, 0);
}

#[test]
fn hint_returns_the_highest_scoring_legal_swap_without_mutating_the_board() {
    let mut game = MatchThree::new(8);
    game.cells = vec![
        1, 2, 1, 0, 0, 0, 2, 1, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2,
        3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2,
    ];
    game.selected = Some(4);
    let before = game.clone();

    assert_eq!(game.hint_swap(), Some((1, 8)));
    assert_eq!(game.hint_swap(), Some((1, 8)));
    assert_eq!(game.cells, before.cells);
    assert_eq!(game.selected, before.selected);
    assert_eq!(game.score, before.score);
    assert_eq!(game.moves, before.moves);
    assert_eq!(game.seed, before.seed);
}

#[test]
fn won_board_has_no_hint() {
    let mut game = MatchThree::new(9);
    game.phase = MatchThreePhase::Won;
    assert_eq!(game.hint_swap(), None);
}
