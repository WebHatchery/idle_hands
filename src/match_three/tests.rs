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
        assert_eq!(game.move_limit(), expected.move_limit);
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
fn cleared_spaces_collapse_survivors_and_refill_only_from_the_top() {
    let mut game = MatchThree::new(4);
    let side = game.side();
    for row in 0..side {
        game.cells[row * side] = row as u8;
    }
    game.cells[2 * side] = EMPTY;
    game.cells[4 * side] = EMPTY;

    game.collapse_columns();

    assert_eq!(
        (2..side)
            .map(|row| game.cells[row * side])
            .collect::<Vec<_>>(),
        vec![0, 1, 3, 5, 6]
    );
    assert!(game.cells[0] < game.color_count());
    assert!(game.cells[side] < game.color_count());
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

#[test]
fn four_tile_match_creates_a_row_special_and_tracks_the_cascade() {
    let mut game = MatchThree::new(10);
    game.cells = quiet_board(game.side(), game.color_count());
    let side = game.side();
    let row = 3;
    for col in 1..=4 {
        game.cells[row * side + col] = 2;
    }
    let anchor = row * side + 3;

    game.resolve(anchor - 1, anchor, false);

    assert_eq!(game.special_at(anchor), MatchThreeSpecial::Row);
    assert!(game.score >= 30);
    assert!(game.last_cascade >= 1);
    assert_eq!(game.best_cascade, game.last_cascade);
}

#[test]
fn swapping_a_special_clears_its_line_even_without_a_color_match() {
    let mut game = MatchThree::new(11);
    game.cells = quiet_board(game.side(), game.color_count());
    let first = 3 * game.side() + 3;
    let second = first + game.side();
    game.specials[first] = MatchThreeSpecial::Row;

    assert!(game.tap(first));
    assert!(game.tap(second));

    assert!(game.score >= game.side() as u16 * 10);
    assert_eq!(game.moves, 1);
    assert!(game
        .specials
        .iter()
        .all(|special| *special == MatchThreeSpecial::None));
}

#[test]
fn final_unsuccessful_move_ends_the_round() {
    let mut game = MatchThree::new(12);
    game.cells = quiet_board(game.side(), game.color_count());
    game.target_score = u16::MAX;
    game.moves = game.move_limit() - 1;
    let first = 2 * game.side() + 2;
    game.specials[first] = MatchThreeSpecial::Column;

    assert!(game.tap(first));
    assert!(game.tap(first + 1));
    assert_eq!(game.phase, MatchThreePhase::Lost);
    assert_eq!(game.moves_left(), 0);
    assert_eq!(game.hint_swap(), None);
}

#[test]
fn reshuffle_is_deterministic_and_restores_a_playable_stable_board() {
    let mut first = MatchThree::new(13);
    first.cells.reverse();
    let mut second = first.clone();

    first.reshuffle();
    second.reshuffle();

    assert_eq!(first.cells, second.cells);
    assert_eq!(first.seed, second.seed);
    assert_eq!(first.reshuffles, 1);
    assert!(first.has_legal_swap());
    assert!(find_matches_for_side(&first.cells, first.side())
        .iter()
        .all(|matched| !matched));
}

#[test]
fn legacy_save_infers_move_limit_and_special_storage() {
    let game = MatchThree::new_with_difficulty(14, MatchThreeDifficulty::Expert);
    let mut saved = serde_json::to_value(game).unwrap();
    let object = saved.as_object_mut().unwrap();
    object.remove("move_limit");
    object.remove("specials");
    let mut restored: MatchThree = serde_json::from_value(saved).unwrap();

    assert_eq!(restored.move_limit(), 30);
    assert!(restored.specials.is_empty());
    assert!(restored.tap(0));
    assert_eq!(restored.specials.len(), restored.cells.len());
}

fn quiet_board(side: usize, colors: u8) -> Vec<u8> {
    (0..side * side)
        .map(|index| {
            let row = index / side;
            let col = index % side;
            ((row + col * 2) % colors as usize) as u8
        })
        .collect()
}
