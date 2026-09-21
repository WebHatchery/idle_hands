//! Regression coverage for the tests module.

use idle_hands::testing::modules::daily_dungeon::*;

#[test]
fn exit_wins_only_after_all_runes() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[CELLS - 1] = DailyTile::Exit;
    dungeon.runes_found = RUNES as u8;
    dungeon.player = CELLS - 2;
    assert!(dungeon.move_in(Direction::Right));
    assert!(dungeon.won());
    assert_eq!(dungeon.score, 92);
}

#[test]
fn three_traps_can_end_the_run() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Trap;
    dungeon.tiles[2] = DailyTile::Trap;
    dungeon.tiles[3] = DailyTile::Trap;
    assert!(dungeon.move_in(Direction::Right));
    assert!(dungeon.move_in(Direction::Right));
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.hearts, 0);
    assert_eq!(dungeon.phase, DailyPhase::Lost);
}

#[test]
fn scout_reveals_adjacent_cells_and_undo_restores_the_charge() {
    let mut dungeon = DailyDungeon::new(0);
    let scouts = dungeon.scouts;

    assert!(dungeon.scout());
    assert!(dungeon.revealed[1]);
    assert!(dungeon.revealed[SIZE]);
    assert_eq!(dungeon.scouts, scouts - 1);
    assert_eq!(dungeon.moves, 1);
    assert!(dungeon.undo());
    assert!(!dungeon.revealed[1]);
    assert_eq!(dungeon.scouts, scouts);
}

#[test]
fn legacy_saves_receive_a_rule_and_two_scouts() {
    let mut value = serde_json::to_value(DailyDungeon::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("rule");
    value.as_object_mut().unwrap().remove("scouts");
    let loaded: DailyDungeon = serde_json::from_value(value).unwrap();

    assert_eq!(loaded.rule, DailyRule::Wayfinder);
    assert_eq!(loaded.scouts, 2);
}
