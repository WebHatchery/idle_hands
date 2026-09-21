//! Regression coverage for the tests module.

use idle_hands::testing::modules::breakout::*;

#[test]
fn missing_the_final_ball_loses_the_run() {
    let mut game = Breakout::new(2);
    game.lives = 1;
    game.ball_x = 0;
    game.ball_y = HEIGHT - 2;
    game.velocity_x = 0;
    game.velocity_y = 1;

    assert!(game.step(PaddleMove::Right));
    assert_eq!(game.lives, 0);
    assert_eq!(game.status, BreakoutStatus::Lost);
    assert!(!game.serve_ready);
}

#[test]
fn elapsed_time_moves_the_ball_and_pause_stops_the_round() {
    let mut game = Breakout::new(1);
    let start = game.ball_position();

    assert!(game.tick(0.1));
    assert_ne!(game.ball_position(), start);

    assert!(game.toggle_pause());
    let paused = game.ball_position();
    assert!(!game.tick(1.));
    assert_eq!(game.ball_position(), paused);
    assert!(game.toggle_pause());
    assert!(game.tick(0.1));
    assert_ne!(game.ball_position(), paused);
}

#[test]
fn armored_bricks_score_each_hit_and_survive_until_health_is_spent() {
    let mut game = Breakout::new(18);
    game.level = 2;
    game.build_wall();
    let index = game
        .brick_health
        .iter()
        .position(|health| *health == 2)
        .unwrap();

    game.damage_brick(index);
    assert!(game.bricks[index]);
    assert_eq!(game.brick_health[index], 1);
    assert_eq!(game.score, 1);
    game.damage_brick(index);
    assert!(!game.bricks[index]);
    assert_eq!(game.brick_health[index], 0);
    assert_eq!(game.score, 2);
}

#[test]
fn clearing_the_third_wall_wins_the_run() {
    let mut game = Breakout::new(20);
    game.level = Breakout::target_level();
    game.bricks.fill(false);
    game.brick_health.fill(0);

    game.complete_wall();

    assert_eq!(game.status, BreakoutStatus::Won);
    assert!(!game.serve_ready);
}
