use super::*;

#[test]
fn seeded_rooms_repeat() {
    let first = OneRoomRoguelike::new(42);
    let second = OneRoomRoguelike::new(42);
    assert_eq!(first.player, second.player);
    assert_eq!(first.enemies, second.enemies);
    assert_eq!(first.treasure, second.treasure);
}

#[test]
fn strike_damages_and_then_clears_adjacent_enemy() {
    let mut game = OneRoomRoguelike::new(1);
    game.enemies = vec![RoomEnemy {
        position: game.player - 1,
        health: 3,
        damage: 0,
    }];
    assert!(game.strike());
    assert_eq!(game.enemies[0].health, 1);
    assert!(game.strike());
    assert!(game.enemies.is_empty());
    assert_eq!(game.phase, RoomPhase::Stairs);
    assert_eq!(game.score, 10);
}

#[test]
fn potion_restores_health_and_enemy_can_defeat_player() {
    let mut game = OneRoomRoguelike::new(1);
    game.health = 4;
    game.enemies = vec![RoomEnemy {
        position: game.player - 1,
        health: 3,
        damage: 2,
    }];
    assert!(game.drink_potion());
    assert_eq!(game.health, 6);
    game.health = 1;
    assert!(game.strike());
    assert_eq!(game.phase, RoomPhase::Lost);
}

#[test]
fn clearing_room_and_reaching_stairs_enters_a_harder_room() {
    let mut game = OneRoomRoguelike::new(1);
    game.enemies.clear();
    game.treasure = EMPTY;
    game.player = game.exit + 1;
    assert!(game.move_in(Direction::Left));
    assert_eq!(game.phase, RoomPhase::Exploring);
    assert_eq!(game.room, 2);
    assert_eq!(game.score, 50);
    assert_eq!(game.enemies.len(), 5);
    assert!(game.enemies.iter().any(|enemy| enemy.damage == 1));
}

#[test]
fn cleared_room_hint_points_to_stairs() {
    let mut game = OneRoomRoguelike::new(1);
    game.enemies.clear();
    assert!(game.move_in(Direction::Up));

    assert_eq!(game.phase, RoomPhase::Stairs);
    assert!(matches!(game.hint_action(), Some(RogueHint::Move(_))));
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
fn hint_prioritizes_combat_and_preserves_the_room() {
    let mut game = OneRoomRoguelike::new(1);
    game.enemies = vec![RoomEnemy {
        position: game.player - 1,
        health: 3,
        damage: 1,
    }];
    let before = game.clone();

    assert_eq!(game.hint_action(), Some(RogueHint::Strike));
    assert_eq!(game.player, before.player);
    assert_eq!(game.health, before.health);
    assert_eq!(game.enemies, before.enemies);
}

#[test]
fn hint_recommends_potion_when_health_is_low() {
    let mut game = OneRoomRoguelike::new(1);
    game.health = 3;
    game.enemies = vec![RoomEnemy {
        position: game.player - 1,
        health: 3,
        damage: 1,
    }];

    assert_eq!(game.hint_action(), Some(RogueHint::Potion));
}

#[test]
fn hint_is_empty_after_the_room_ends() {
    let mut game = OneRoomRoguelike::new(1);
    game.phase = RoomPhase::Won;

    assert_eq!(game.hint_action(), None);
}
