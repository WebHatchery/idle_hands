//! Regression coverage for the tests module.

use super::*;

#[test]
fn a_board_without_remaining_pairs_becomes_stuck() {
    let mut game = MahjongSolitaire::new(1);
    for tile in &mut game.tiles {
        tile.removed = true;
    }
    game.tiles[0].removed = false;
    game.tiles[1].removed = false;
    game.tiles[0].kind = 1;
    game.tiles[1].kind = 2;
    game.resolve();
    assert_eq!(game.status, MahjongStatus::Stuck);
}

#[test]
fn seeded_classic_and_temple_boards_clear_from_hints() {
    for &seed in &[0_u64, 1, 7, 54, u64::MAX] {
        for layout in MahjongLayout::ALL {
            let mut game = MahjongSolitaire::new_with_layout(seed, layout);
            let mut pairs = 0;
            while let Some((first, second)) = game.hint_pair() {
                assert!(game.tap(first));
                assert!(game.tap(second));
                pairs += 1;
                assert!(pairs <= PAIR_COUNT);
            }
            assert_eq!(pairs, PAIR_COUNT, "{layout:?} seed {seed} stopped early");
            assert_eq!(game.status, MahjongStatus::Won);
        }
    }
}

#[test]
fn reset_keeps_the_selected_layout() {
    let mut game = MahjongSolitaire::new_with_layout(7, MahjongLayout::Temple);
    game.reset(8);
    assert_eq!(game.layout, MahjongLayout::Temple);
    assert_eq!(game.status, MahjongStatus::Playing);
    assert!(game.hint_pair().is_some());
}

#[test]
fn coverage_only_blocks_a_tile_at_the_same_stack_position() {
    let lower = Tile {
        kind: 0,
        x: 2,
        y: 1,
        layer: 0,
        removed: false,
    };
    let same_position = Tile { layer: 1, ..lower };
    let adjacent_position = Tile {
        x: 3,
        ..same_position
    };

    assert!(overlaps(&lower, &same_position));
    assert!(!overlaps(&lower, &adjacent_position));
}
