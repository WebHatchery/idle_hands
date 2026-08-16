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
    }];
    game.phase = TowerPhase::Wave;
    assert!(game.start_or_advance());
    assert!(game.won());
}

#[test]
fn hint_recommends_build_then_advance_without_mutating_the_tower() {
    let mut game = TinyTowerDefence::new(1);
    let before = game.clone();

    assert_eq!(game.hint_action(), Some(TowerHint::Build(17)));
    assert_eq!(game.towers, before.towers);
    assert_eq!(game.gold, before.gold);

    assert!(game.start_or_advance());
    let before_wave = game.clone();
    assert_eq!(game.hint_action(), Some(TowerHint::Advance));
    assert_eq!(game.enemies, before_wave.enemies);
    assert_eq!(game.tick, before_wave.tick);
}

#[test]
fn hint_is_empty_after_tower_defence_ends() {
    let mut game = TinyTowerDefence::new(1);
    game.phase = TowerPhase::Won;

    assert_eq!(game.hint_action(), None);
}
