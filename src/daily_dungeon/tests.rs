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
fn calendar_challenges_keep_the_day_identity_and_rule() {
    let first = DailyDungeon::new_for_day(20_042);
    let second = DailyDungeon::new_for_day(20_042);
    let next = DailyDungeon::new_for_day(20_043);

    assert_eq!(first.day_key, 20_042);
    assert_eq!(first.challenge, 42);
    assert_eq!(first.rule, DailyRule::Daredevil);
    assert_eq!(first.seed, second.seed);
    assert_eq!(first.tiles, second.tiles);
    assert_ne!(first.seed, next.seed);
    assert_ne!(first.tiles, next.tiles);
}

#[test]
fn runes_score_and_traps_are_single_use() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Rune;
    dungeon.tiles[2] = DailyTile::Trap;
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.runes_found, 1);
    assert_eq!(dungeon.score, 17);
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.hearts, 2);
    assert_eq!(dungeon.tiles[2], DailyTile::Floor);
    assert_eq!(dungeon.score, 21);
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
    dungeon.scouts = 0;
    let before = dungeon.clone();

    assert_eq!(
        dungeon.hint_action(),
        Some(DailyHint::Move(Direction::Right))
    );
    assert_eq!(dungeon.player, before.player);
    assert_eq!(dungeon.revealed, before.revealed);
    assert_eq!(dungeon.hearts, before.hearts);
}

#[test]
fn hint_is_empty_after_daily_run_ends() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.phase = DailyPhase::Won;

    assert_eq!(dungeon.hint_action(), None);
}

#[test]
fn daily_rules_change_traps_springs_and_lantern_supply() {
    let wayfinder = DailyDungeon::new(0);
    let forager = DailyDungeon::new(1);
    let daredevil = DailyDungeon::new(2);

    assert_eq!(
        (wayfinder.rule, wayfinder.scouts),
        (DailyRule::Wayfinder, 3)
    );
    assert_eq!((forager.rule, forager.scouts), (DailyRule::Forager, 2));
    assert_eq!(
        (daredevil.rule, daredevil.scouts),
        (DailyRule::Daredevil, 1)
    );
    assert_eq!(
        wayfinder
            .tiles
            .iter()
            .filter(|tile| **tile == DailyTile::Trap)
            .count(),
        8
    );
    assert_eq!(
        forager
            .tiles
            .iter()
            .filter(|tile| **tile == DailyTile::Spring)
            .count(),
        2
    );
    assert_eq!(
        daredevil
            .tiles
            .iter()
            .filter(|tile| **tile == DailyTile::Trap)
            .count(),
        9
    );
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
fn scouting_trades_bravery_points_for_information() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Rune;

    assert!(dungeon.scout());
    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.score, 15);
}

#[test]
fn spring_restores_one_heart_and_is_single_use() {
    let mut dungeon = DailyDungeon::new(1);
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[1] = DailyTile::Spring;
    dungeon.hearts = 1;

    assert!(dungeon.move_in(Direction::Right));
    assert_eq!(dungeon.hearts, 2);
    assert_eq!(dungeon.tiles[1], DailyTile::Floor);
    assert_eq!(dungeon.score, 10);
}

#[test]
fn hint_scouts_first_then_avoids_a_revealed_trap() {
    let mut dungeon = DailyDungeon::new(1);
    assert_eq!(dungeon.hint_action(), Some(DailyHint::Scout));

    dungeon.scouts = 0;
    dungeon.player = 7;
    dungeon.tiles = vec![DailyTile::Floor; CELLS];
    dungeon.tiles[8] = DailyTile::Trap;
    dungeon.tiles[9] = DailyTile::Rune;
    dungeon.revealed = vec![false; CELLS];
    dungeon.revealed[8] = true;
    assert_ne!(
        dungeon.hint_action(),
        Some(DailyHint::Move(Direction::Right))
    );
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
