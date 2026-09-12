//! Regression coverage for the tests module.

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
        health: 4,
        damage: 0,
        kind: EnemyKind::Guard,
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
        kind: EnemyKind::Guard,
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
        kind: EnemyKind::Guard,
    }];

    assert_eq!(game.hint_action(), Some(RogueHint::Potion));
}

#[test]
fn hint_is_empty_after_the_room_ends() {
    let mut game = OneRoomRoguelike::new(1);
    game.phase = RoomPhase::Won;

    assert_eq!(game.hint_action(), None);
}

#[test]
fn hero_classes_change_attack_endurance_and_potions() {
    let blade = OneRoomRoguelike::new_with_class(1, HeroClass::Blade);
    let warden = OneRoomRoguelike::new_with_class(1, HeroClass::Warden);
    let alchemist = OneRoomRoguelike::new_with_class(1, HeroClass::Alchemist);

    assert_eq!(
        (blade.health, blade.attack_damage(), blade.potions),
        (10, 3, 2)
    );
    assert_eq!(
        (warden.health, warden.attack_damage(), warden.potions),
        (14, 2, 2)
    );
    assert_eq!(
        (
            alchemist.health,
            alchemist.attack_damage(),
            alchemist.potions
        ),
        (10, 2, 3)
    );
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
fn stalker_closes_distance_before_attacking() {
    let mut game = OneRoomRoguelike::new(1);
    game.room = 2;
    game.health = 5;
    game.enemies = vec![RoomEnemy {
        position: game.player - 2,
        health: 3,
        damage: 1,
        kind: EnemyKind::Stalker,
    }];

    assert!(game.drink_potion());
    assert_eq!(game.enemies[0].position, game.player - 1);
    assert_eq!(game.health, 8);
}

#[test]
fn deeper_rooms_mix_stalkers_and_brutes() {
    let mut game = OneRoomRoguelike::new(7);
    game.room = 3;
    game.place_room();

    assert!(game
        .enemies
        .iter()
        .any(|enemy| enemy.kind == EnemyKind::Stalker));
    assert!(game
        .enemies
        .iter()
        .any(|enemy| enemy.kind == EnemyKind::Brute));
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

#[test]
fn reset_preserves_the_chosen_hero() {
    let mut game = OneRoomRoguelike::new_with_class(1, HeroClass::Warden);
    game.reset(2);
    assert_eq!(game.hero_class, HeroClass::Warden);
    assert_eq!(game.health, 14);
}

#[test]
fn legacy_saves_default_to_blade_and_guard() {
    let mut value = serde_json::to_value(OneRoomRoguelike::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("hero_class");
    for enemy in value["enemies"].as_array_mut().unwrap() {
        enemy.as_object_mut().unwrap().remove("kind");
    }
    let loaded: OneRoomRoguelike = serde_json::from_value(value).unwrap();

    assert_eq!(loaded.hero_class, HeroClass::Blade);
    assert!(loaded
        .enemies
        .iter()
        .all(|enemy| enemy.kind == EnemyKind::Guard));
}
