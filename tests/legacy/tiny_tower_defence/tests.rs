//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_waves_repeat() {
    let mut first = TinyTowerDefence::new(42);
    let mut second = TinyTowerDefence::new(42);
    assert!(first.start_or_advance());
    assert!(second.start_or_advance());
    assert_eq!(first.enemies, second.enemies);
    assert_eq!(first.seed, second.seed);
}

#[test]
fn building_and_upgrading_spend_gold() {
    let mut game = TinyTowerDefence::new(1);
    assert_eq!(game.tower_cost(8), Some(3));
    assert!(game.build_or_upgrade(8));
    assert_eq!(game.towers[8], 1);
    assert_eq!(game.gold, 9);
    assert_eq!(game.tower_cost(8), Some(3));
    assert!(game.build_or_upgrade(8));
    assert_eq!(game.towers[8], 2);
    assert_eq!(game.gold, 6);
    assert!(!game.build_or_upgrade(0));
}

#[test]
fn tower_shot_clears_enemy_and_opens_next_wave() {
    let mut game = TinyTowerDefence::new(1);
    game.towers[2] = 1;
    game.enemies = vec![Enemy {
        row: 0,
        column: 1,
        health: 1,
        kind: EnemyKind::Grunt,
        slow_ticks: 0,
    }];
    game.phase = TowerPhase::Wave;
    assert!(game.start_or_advance());
    assert!(game.enemies.is_empty());
    assert_eq!(game.score, 10);
    assert_eq!(game.wave, 2);
    assert_eq!(game.phase, TowerPhase::Build);
}

#[test]
fn leaking_enemies_cost_lives_and_can_lose() {
    let mut game = TinyTowerDefence::new(1);
    game.lives = 1;
    game.enemies = vec![Enemy {
        row: 0,
        column: (WIDTH - 2) as u8,
        health: 4,
        kind: EnemyKind::Grunt,
        slow_ticks: 0,
    }];
    game.phase = TowerPhase::Wave;
    assert!(game.start_or_advance());
    assert_eq!(game.lives, 0);
    assert_eq!(game.phase, TowerPhase::Lost);
}

#[test]
fn undo_restores_build_and_target_wave_can_win() {
    let mut game = TinyTowerDefence::new(1);
    assert!(game.build_or_upgrade(8));
    assert!(game.undo());
    assert_eq!(game.towers[8], 0);
    assert_eq!(game.gold, STARTING_GOLD);

    game.towers[2] = 3;
    game.wave = TARGET_WAVE;
    game.enemies = vec![Enemy {
        row: 0,
        column: 1,
        health: 1,
        kind: EnemyKind::Grunt,
        slow_ticks: 0,
    }];
    game.phase = TowerPhase::Wave;
    assert!(game.start_or_advance());
    assert!(game.won());
}

#[test]
fn hint_recommends_build_then_wave_control_without_mutating_the_tower() {
    let mut game = TinyTowerDefence::new(1);
    let before = game.clone();

    assert_eq!(
        game.hint_action(),
        Some(TowerHint::Build(17, TowerKind::Bolt))
    );
    assert_eq!(game.towers, before.towers);
    assert_eq!(game.gold, before.gold);

    assert!(game.start_or_advance());
    let before_wave = game.clone();
    assert_eq!(game.hint_action(), Some(TowerHint::WaveControl));
    assert_eq!(game.enemies, before_wave.enemies);
    assert_eq!(game.tick, before_wave.tick);
}

#[test]
fn hint_is_empty_after_tower_defence_ends() {
    let mut game = TinyTowerDefence::new(1);
    game.phase = TowerPhase::Won;

    assert_eq!(game.hint_action(), None);
}

#[test]
fn elapsed_time_advances_waves_and_pause_stops_the_invaders() {
    let mut game = TinyTowerDefence::new(1);
    assert!(game.start_or_advance());
    let start = game.enemies.clone();

    assert!(!game.tick(0.39));
    assert_eq!(game.enemies, start);
    assert!(game.tick(0.02));
    assert_ne!(game.enemies, start);

    assert!(game.start_or_toggle_pause());
    let paused = game.enemies.clone();
    assert!(!game.tick(1.));
    assert_eq!(game.enemies, paused);
    assert!(game.start_or_toggle_pause());
    assert!(game.tick(0.41));
    assert_ne!(game.enemies, paused);
}

#[test]
fn legacy_saves_default_to_an_active_wave() {
    let mut value = serde_json::to_value(TinyTowerDefence::new(1)).unwrap();
    let object = value.as_object_mut().unwrap();
    object.remove("paused");
    object.remove("tower_kinds");
    object.remove("selected_kind");
    let restored: TinyTowerDefence = serde_json::from_value(value).unwrap();

    assert!(!restored.paused);
    assert!(restored.tower_kinds.is_empty());
    assert_eq!(restored.selected_kind, TowerKind::Bolt);
    assert_eq!(restored.tower_kind(8), TowerKind::Bolt);
}

#[test]
fn tower_selector_sets_role_cost_and_new_towers_keep_that_role() {
    let mut game = TinyTowerDefence::new(2);
    assert!(game.select_kind(TowerKind::Frost));
    assert_eq!(game.tower_cost(8), Some(4));
    assert!(game.build_or_upgrade(8));
    assert_eq!(game.tower_kind(8), TowerKind::Frost);
    assert_eq!(game.gold, 8);
    assert_eq!(game.tower_cost(8), Some(4));

    assert!(game.select_kind(TowerKind::Burst));
    assert!(game.build_or_upgrade(8));
    assert_eq!(game.tower_kind(8), TowerKind::Frost);
    assert!(game.undo());
    assert_eq!(game.towers[8], 1);
    assert_eq!(game.tower_kind(8), TowerKind::Frost);
    assert_eq!(game.selected_kind, TowerKind::Burst);
}

#[test]
fn frost_tower_holds_an_enemy_in_place_while_dealing_damage() {
    let mut game = TinyTowerDefence::new(3);
    game.towers[2] = 1;
    game.tower_kinds[2] = TowerKind::Frost;
    game.enemies = vec![Enemy {
        row: 0,
        column: 1,
        health: 3,
        kind: EnemyKind::Grunt,
        slow_ticks: 0,
    }];
    game.phase = TowerPhase::Wave;

    assert!(game.start_or_advance());
    assert_eq!(game.enemies[0].health, 2);
    assert_eq!(game.enemies[0].column, 1);
}

#[test]
fn burst_tower_splashes_the_target_and_adjacent_lanes() {
    let mut game = TinyTowerDefence::new(4);
    let tower = 2 * WIDTH + 3;
    game.towers[tower] = 1;
    game.tower_kinds[tower] = TowerKind::Burst;
    game.enemies = (1..=3)
        .map(|row| Enemy {
            row,
            column: 2,
            health: 1,
            kind: EnemyKind::Grunt,
            slow_ticks: 0,
        })
        .collect();
    game.phase = TowerPhase::Wave;

    assert!(game.start_or_advance());
    assert!(game.enemies.is_empty());
    assert_eq!(game.score, 30);
}

#[test]
fn swift_enemies_move_twice_and_later_waves_mix_the_roster() {
    let mut game = TinyTowerDefence::new(5);
    game.enemies = vec![Enemy {
        row: 0,
        column: 0,
        health: 5,
        kind: EnemyKind::Swift,
        slow_ticks: 0,
    }];
    game.phase = TowerPhase::Wave;
    assert!(game.start_or_advance());
    assert_eq!(game.enemies[0].column, 2);

    let mut later = TinyTowerDefence::new(5);
    later.wave = 5;
    assert!(later.start_or_advance());
    assert!(later
        .enemies
        .iter()
        .any(|enemy| enemy.kind == EnemyKind::Swift));
    assert!(later
        .enemies
        .iter()
        .any(|enemy| enemy.kind == EnemyKind::Armored));
}

#[test]
fn build_hint_changes_role_as_the_wave_roster_deepens() {
    let mut game = TinyTowerDefence::new(6);
    game.wave = 3;

    assert!(matches!(
        game.hint_action(),
        Some(TowerHint::Build(_, TowerKind::Frost))
    ));
    game.wave = 5;
    assert!(matches!(
        game.hint_action(),
        Some(TowerHint::Build(_, TowerKind::Burst))
    ));
}

#[test]
fn legacy_enemy_defaults_to_an_unhindered_grunt() {
    let enemy = Enemy {
        row: 2,
        column: 3,
        health: 4,
        kind: EnemyKind::Armored,
        slow_ticks: 2,
    };
    let mut value = serde_json::to_value(enemy).unwrap();
    let object = value.as_object_mut().unwrap();
    object.remove("kind");
    object.remove("slow_ticks");
    let restored: Enemy = serde_json::from_value(value).unwrap();

    assert_eq!(restored.kind, EnemyKind::Grunt);
    assert_eq!(restored.slow_ticks, 0);
}
