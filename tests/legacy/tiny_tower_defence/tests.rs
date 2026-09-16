//! Regression coverage for the tests module.

use super::*;

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
