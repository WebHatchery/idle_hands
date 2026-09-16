//! Regression coverage for the tests module.

use super::*;

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

fn quiet_board(side: usize, colors: u8) -> Vec<u8> {
    (0..side * side)
        .map(|index| {
            let row = index / side;
            let col = index % side;
            ((row + col * 2) % colors as usize) as u8
        })
        .collect()
}
