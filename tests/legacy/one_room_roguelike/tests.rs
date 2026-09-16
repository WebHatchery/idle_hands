//! Regression coverage for the tests module.

use super::*;

#[test]
fn potion_restores_health_and_enemy_can_defeat_player() {
    let mut game = OneRoomRoguelike::new(1);
    game.health = 4;
    game.enemies = vec![RoomEnemy {
        position: game.player - 1,
        health: 4,
        damage: 2,
        kind: EnemyKind::Guard,
    }];
    assert!(game.drink_potion());
    assert_eq!(game.health, 6);
    game.health = 1;
    assert!(game.strike());
    assert_eq!(game.phase, RoomPhase::Lost);
}

#[test]
fn undo_restores_action_and_reset_reseeds_room() {
    let mut game = OneRoomRoguelike::new(1);
    let original = game.player;
    assert!(game.move_in(Direction::Up));
    assert!(game.undo());
    assert_eq!(game.player, original);
    game.reset(2);
    assert_ne!(game.enemies, OneRoomRoguelike::new(1).enemies);
}

#[test]
fn alchemist_heals_more_and_warden_softens_each_hit() {
    let mut alchemist = OneRoomRoguelike::new_with_class(1, HeroClass::Alchemist);
    alchemist.health = 2;
    alchemist.enemies.clear();
    assert!(alchemist.drink_potion());
    assert_eq!(alchemist.health, 8);

    let mut warden = OneRoomRoguelike::new_with_class(1, HeroClass::Warden);
    warden.health = 10;
    warden.enemies = vec![RoomEnemy {
        position: warden.player - 1,
        health: 5,
        damage: 2,
        kind: EnemyKind::Brute,
    }];
    assert!(warden.strike());
    assert_eq!(warden.health, 9);
}

#[test]
fn fifth_staircase_completes_the_run() {
    let mut game = OneRoomRoguelike::new(1);
    game.room = OneRoomRoguelike::target_room();
    game.enemies.clear();
    game.treasure = EMPTY;
    game.player = game.exit + 1;

    assert!(game.move_in(Direction::Left));
    assert_eq!(game.phase, RoomPhase::Won);
    assert_eq!(game.room, OneRoomRoguelike::target_room());
    assert_eq!(game.score, 90);
}
