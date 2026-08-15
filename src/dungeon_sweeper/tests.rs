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
