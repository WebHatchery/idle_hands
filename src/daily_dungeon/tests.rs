use super::*;

#[test]
fn seeded_challenges_repeat() {
    let first = DailyDungeon::new(42);
    let second = DailyDungeon::new(42);
    assert_eq!(first.challenge, second.challenge);
    assert_eq!(first.tiles, second.tiles);
    assert_eq!(first.revealed, second.revealed);
}

#[test]
fn runes_score_and_traps_are_single_use() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Rune;
    dungeon.tiles[2] = DailyTile::Trap;
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.runes_found, 1);
    assert_eq!(dungeon.score, 15);
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.hearts, 2);
    assert_eq!(dungeon.tiles[2], DailyTile::Floor);
    assert_eq!(dungeon.score, 17);
}

#[test]
fn exit_wins_only_after_all_runes() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[CELLS - 1] = DailyTile::Exit;
    dungeon.runes_found = RUNES as u8;
    dungeon.player = CELLS - 2;
    assert!(dungeon.move_in(Direction::Right));
    assert!(dungeon.won());
    assert_eq!(dungeon.score, 50);
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
fn undo_restores_reveal_and_reset_changes_challenge() {
    let mut dungeon = DailyDungeon::new(1);
    assert!(dungeon.move_in(Direction::Right));
    assert!(dungeon.undo());
    assert_eq!(dungeon.player, 0);
    assert!(!dungeon.revealed[1]);
    let challenge = dungeon.challenge;
    dungeon.reset(2);
    assert_ne!(challenge, dungeon.challenge);
}

#[test]
fn hint_moves_toward_the_nearest_rune_without_mutating_the_run() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Rune;
    let before = dungeon.clone();

    assert_eq!(dungeon.hint_direction(), Some(Direction::Right));
    assert_eq!(dungeon.player, before.player);
    assert_eq!(dungeon.revealed, before.revealed);
    assert_eq!(dungeon.hearts, before.hearts);
}

#[test]
fn hint_is_empty_after_daily_run_ends() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.phase = DailyPhase::Won;

    assert_eq!(dungeon.hint_direction(), None);
}
