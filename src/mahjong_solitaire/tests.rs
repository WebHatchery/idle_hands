use super::*;

#[test]
fn seeded_layouts_repeat_with_eighteen_pairs() {
    let first = MahjongSolitaire::new(7);
    let second = MahjongSolitaire::new(7);
    assert_eq!(first.tiles, second.tiles);
    assert_eq!(first.tiles.len(), TILE_COUNT);
    assert_eq!(first.tiles.iter().filter(|tile| !tile.removed).count(), 36);
}

#[test]
fn matching_available_tiles_remove_as_a_pair_and_undo() {
    let mut game = MahjongSolitaire::new(1);
    let (first, second) = (0..game.tiles.len())
        .filter(|&index| game.available(index))
        .find_map(|first| {
            (first + 1..game.tiles.len())
                .find(|&index| {
                    game.available(index) && game.tiles[index].kind == game.tiles[first].kind
                })
                .map(|second| (first, second))
        })
        .unwrap();
    assert!(game.tap(first));
    assert!(game.tap(second));
    assert!(game.tiles[first].removed);
    assert!(game.tiles[second].removed);
    assert!(game.undo());
    assert!(!game.tiles[first].removed);
}

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
fn hint_pair_finds_free_matching_tiles_without_mutating() {
    let game = MahjongSolitaire::new(1);
    let before = game.tiles.clone();
    let (first, second) = game.hint_pair().unwrap();
    assert!(game.available(first));
    assert!(game.available(second));
    assert_eq!(game.tiles[first].kind, game.tiles[second].kind);
    assert_eq!(game.tiles, before);
}

#[test]
fn temple_layout_is_deterministic_and_keeps_thirty_six_tiles() {
    let temple = MahjongSolitaire::new_with_layout(54, MahjongLayout::Temple);
    assert_eq!(temple.tiles.len(), TILE_COUNT);
    assert_eq!(
        temple.tiles,
        MahjongSolitaire::new_with_layout(54, MahjongLayout::Temple).tiles
    );
    assert_ne!(
        temple
            .tiles
            .iter()
            .map(|tile| (tile.x, tile.y, tile.layer))
            .collect::<Vec<_>>(),
        MahjongSolitaire::new(54)
            .tiles
            .iter()
            .map(|tile| (tile.x, tile.y, tile.layer))
            .collect::<Vec<_>>()
    );
}

#[test]
fn every_layout_starts_with_a_free_matching_pair() {
    for layout in MahjongLayout::ALL {
        let game = MahjongSolitaire::new_with_layout(91, layout);
        assert_eq!(game.status, MahjongStatus::Playing);
        assert!(game.hint_pair().is_some(), "{layout:?} has no opening pair");
    }
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
