//! Regression coverage for the tests module.

use super::*;

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
