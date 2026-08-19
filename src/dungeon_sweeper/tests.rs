use super::*;

#[test]
fn seeded_dungeons_repeat_and_keep_start_safe() {
    let first = DungeonSweeper::new(42);
    let second = DungeonSweeper::new(42);
    assert_eq!(first.cells, second.cells);
    let mut revealed = first;
    assert!(revealed.reveal(0));
    assert!(!matches!(revealed.cells[0], DungeonCell::Trap));
    assert_eq!(revealed.cells.len(), 64);
}

#[test]
fn flags_and_undo_restore_the_previous_path() {
    let mut game = DungeonSweeper::new(7);
    assert!(game.reveal(0));
    assert!(game.toggle_flag(1));
    assert_eq!(game.flagged_count(), 1);
    assert!(game.reveal(2));
    assert!(game.undo());
    assert!(matches!(game.cells[2], DungeonCell::Hidden));
    assert_eq!(game.moves, 1);
}

#[test]
fn flag_and_first_reveal_generation_both_undo_cleanly() {
    let mut flags = DungeonSweeper::new(7);
    assert!(flags.toggle_flag(1));
    assert!(flags.undo());
    assert_eq!(flags.cells[1], DungeonCell::Hidden);

    let mut reveal = DungeonSweeper::new(7);
    let seed = reveal.seed;
    assert!(reveal.reveal(0));
    assert!(reveal.undo());
    assert!(!reveal.first_reveal);
    assert_eq!(reveal.seed, seed);
    assert!(reveal.relics.is_empty());
    assert!(reveal.cells.iter().all(|cell| *cell == DungeonCell::Hidden));
    assert!(reveal.reveal(0));
    assert_eq!(
        reveal
            .cells
            .iter()
            .filter(|cell| matches!(cell, DungeonCell::Trap))
            .count(),
        12
    );
}

#[test]
fn exit_waits_for_relics_and_peril_traps_end_the_run() {
    let mut won = DungeonSweeper::new(2);
    won.cells = vec![DungeonCell::Hidden; 64];
    won.first_reveal = true;
    won.status = DungeonStatus::Playing;
    won.required_relics = 2;
    won.relics = vec![1, 2];
    assert!(won.reveal(won.exit));
    assert_eq!(won.status, DungeonStatus::Playing);
    assert!(won.reveal(1));
    assert!(won.reveal(2));
    assert_eq!(won.status, DungeonStatus::Won);

    let mut lost = DungeonSweeper::new_with_difficulty(3, DungeonDifficulty::Peril);
    assert!(lost.reveal(0));
    let trap = lost
        .cells
        .iter()
        .position(|cell| matches!(cell, DungeonCell::Trap))
        .unwrap();
    assert!(lost.reveal(trap));
    assert_eq!(lost.status, DungeonStatus::Lost);
}

#[test]
fn matching_flags_chord_open_adjacent_rooms_and_undo() {
    let mut game = DungeonSweeper::new(4);
    game.cells = vec![DungeonCell::Hidden; 64];
    game.cells[0] = DungeonCell::Revealed(1);
    game.cells[1] = DungeonCell::FlaggedTrap;
    game.cells[8] = DungeonCell::Hidden;
    game.status = DungeonStatus::Playing;
    game.first_reveal = true;
    game.seed = 99;

    assert!(game.chord(0));
    assert!(matches!(game.cells[8], DungeonCell::Revealed(1)));
    assert_eq!(game.moves, 1);
    assert!(game.undo());
    assert!(matches!(game.cells[8], DungeonCell::Hidden));
    assert_eq!(game.moves, 0);
}

#[test]
fn chord_requires_matching_flags_and_revealed_taps_use_it() {
    let mut game = DungeonSweeper::new(5);
    game.cells = vec![DungeonCell::Hidden; 64];
    game.cells[0] = DungeonCell::Revealed(1);
    game.cells[1] = DungeonCell::Trap;
    game.cells[8] = DungeonCell::Hidden;
    game.status = DungeonStatus::Playing;
    game.first_reveal = true;

    assert!(!game.chord(0));
    assert_eq!(game.moves, 0);
    game.cells[1] = DungeonCell::FlaggedTrap;
    assert!(game.reveal(0));
    assert!(matches!(game.cells[8], DungeonCell::Revealed(1)));
}

#[test]
fn trap_flags_round_trip_and_wrong_chord_can_lose() {
    let mut flags = DungeonSweeper::new(6);
    assert!(flags.reveal(0));
    let trap = flags
        .cells
        .iter()
        .position(|cell| matches!(cell, DungeonCell::Trap))
        .unwrap();
    assert!(flags.toggle_flag(trap));
    assert!(matches!(flags.cells[trap], DungeonCell::FlaggedTrap));
    assert!(flags.toggle_flag(trap));
    assert!(matches!(flags.cells[trap], DungeonCell::Trap));

    let mut game = DungeonSweeper::new(7);
    game.cells = vec![DungeonCell::Hidden; 64];
    game.cells[0] = DungeonCell::Revealed(1);
    game.cells[1] = DungeonCell::Flagged;
    game.cells[8] = DungeonCell::Trap;
    game.status = DungeonStatus::Playing;
    game.first_reveal = true;
    game.hearts = 1;
    assert!(game.chord(0));
    assert_eq!(game.status, DungeonStatus::Lost);
}

#[test]
fn hint_recommends_a_safe_start_then_an_uncollected_relic_without_mutating() {
    let mut game = DungeonSweeper::new(8);
    assert_eq!(game.hint_cell(), Some(0));
    assert!(!game.first_reveal);

    assert!(game.reveal(0));
    let before = game.cells.clone();
    let hint = game.hint_cell().unwrap();

    assert!(matches!(game.cells[hint], DungeonCell::Hidden));
    assert!(!matches!(game.cells[hint], DungeonCell::Trap));
    assert!(game.relics.contains(&hint));
    assert_eq!(game.cells, before);
}

#[test]
fn hint_is_empty_after_dungeon_ends() {
    let mut game = DungeonSweeper::new(8);
    game.status = DungeonStatus::Lost;

    assert_eq!(game.hint_cell(), None);
}

#[test]
fn difficulty_changes_traps_hearts_and_relic_requirements() {
    let explorer = DungeonSweeper::new_with_difficulty(1, DungeonDifficulty::Explorer);
    let delver = DungeonSweeper::new_with_difficulty(1, DungeonDifficulty::Delver);
    let peril = DungeonSweeper::new_with_difficulty(1, DungeonDifficulty::Peril);

    assert_eq!(
        (explorer.traps, explorer.hearts, explorer.relic_total()),
        (10, 3, 2)
    );
    assert_eq!(
        (delver.traps, delver.hearts, delver.relic_total()),
        (12, 2, 3)
    );
    assert_eq!((peril.traps, peril.hearts, peril.relic_total()), (15, 1, 3));
}

#[test]
fn first_reveal_places_repeatable_safe_relics() {
    let mut first = DungeonSweeper::new(42);
    let mut second = DungeonSweeper::new(42);
    assert!(first.reveal(0));
    assert!(second.reveal(0));

    assert_eq!(first.relics, second.relics);
    assert_eq!(first.relics.len(), 3);
    assert!(first
        .relics
        .iter()
        .all(|index| *index != 0 && *index != first.exit && !first.is_trap(*index)));
}

#[test]
fn delver_can_survive_one_trap_and_undo_it() {
    let mut game = DungeonSweeper::new(9);
    assert!(game.reveal(0));
    let trap = game
        .cells
        .iter()
        .position(|cell| matches!(cell, DungeonCell::Trap))
        .unwrap();

    assert!(game.reveal(trap));
    assert_eq!(game.hearts, 1);
    assert_eq!(game.status, DungeonStatus::Playing);
    assert!(game.undo());
    assert_eq!(game.hearts, 2);
    assert!(matches!(game.cells[trap], DungeonCell::Trap));
}

#[test]
fn collecting_a_relic_is_visible_to_undo() {
    let mut game = DungeonSweeper::new(1);
    game.cells = vec![DungeonCell::Hidden; 64];
    game.first_reveal = true;
    game.status = DungeonStatus::Playing;
    game.required_relics = 1;
    game.relics = vec![1];

    assert!(game.reveal(1));
    assert_eq!(game.relics_found(), 1);
    assert!(game.undo());
    assert_eq!(game.relics_found(), 0);
}

#[test]
fn reset_keeps_difficulty_and_legacy_saves_keep_an_open_exit() {
    let mut game = DungeonSweeper::new_with_difficulty(1, DungeonDifficulty::Explorer);
    game.reset(2);
    assert_eq!(game.difficulty, DungeonDifficulty::Explorer);

    let mut value = serde_json::to_value(DungeonSweeper::new(1)).unwrap();
    for field in [
        "difficulty",
        "hearts",
        "relics",
        "collected_relics",
        "required_relics",
    ] {
        value.as_object_mut().unwrap().remove(field);
    }
    let loaded: DungeonSweeper = serde_json::from_value(value).unwrap();
    assert_eq!(loaded.difficulty, DungeonDifficulty::Delver);
    assert_eq!(loaded.hearts, 2);
    assert_eq!(loaded.relic_total(), 0);
}
