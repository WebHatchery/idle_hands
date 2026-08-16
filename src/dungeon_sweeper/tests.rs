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
fn exit_wins_and_traps_lose() {
    let mut won = DungeonSweeper::new(2);
    assert!(won.reveal(won.exit));
    assert_eq!(won.status, DungeonStatus::Won);

    let mut lost = DungeonSweeper::new(3);
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
    assert!(game.chord(0));
    assert_eq!(game.status, DungeonStatus::Lost);
}

#[test]
fn hint_recommends_exit_then_a_safe_hidden_room_without_mutating() {
    let mut game = DungeonSweeper::new(8);
    assert_eq!(game.hint_cell(), Some(game.exit));
    assert!(!game.first_reveal);

    assert!(game.reveal(0));
    let before = game.cells.clone();
    let hint = game.hint_cell().unwrap();

    assert!(matches!(game.cells[hint], DungeonCell::Hidden));
    assert!(!matches!(game.cells[hint], DungeonCell::Trap));
    assert_eq!(game.cells, before);
}

#[test]
fn hint_is_empty_after_dungeon_ends() {
    let mut game = DungeonSweeper::new(8);
    game.status = DungeonStatus::Lost;

    assert_eq!(game.hint_cell(), None);
}
