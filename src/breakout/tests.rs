//! Regression coverage for the tests module.

use super::*;

#[test]
fn seeded_boards_repeat_and_have_four_brick_rows() {
    let first = Breakout::new(7);
    let second = Breakout::new(7);
    assert_eq!(first.bricks, second.bricks);
    assert_eq!(first.brick_health, second.brick_health);
    assert_eq!(first.bricks.len(), 64);
    assert_eq!(first.remaining_bricks(), 64);
    assert!(first.brick_health.iter().all(|health| *health == 1));
}

#[test]
fn a_touch_step_moves_the_paddle_and_undo_restores_it() {
    let mut game = Breakout::new(1);
    let paddle = game.paddle;
    assert!(game.step(PaddleMove::Left));
    assert!(game.paddle < paddle);
    assert!(game.undo());
    assert_eq!(game.paddle, paddle);
    assert_eq!(game.moves, 0);
}

#[test]
fn missing_the_paddle_costs_a_life_and_prepares_a_visible_serve() {
    let mut game = Breakout::new(1);
    game.ball_x = 0;
    game.ball_y = HEIGHT - 2;
    game.velocity_x = 0;
    game.velocity_y = 1;
    assert!(game.step(PaddleMove::Right));
    assert_eq!(game.status, BreakoutStatus::Playing);
    assert_eq!(game.lives, 2);
    assert!(game.paused);
    assert!(game.serve_ready);
    assert_eq!(game.ball_y, HEIGHT - 3);
    assert!(game.undo());
    assert_eq!(game.lives, 3);
}

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
fn hint_tracks_projected_ball_without_mutating_the_round() {
    let mut game = Breakout::new(1);
    game.ball_x = 2;
    game.ball_y = 2;
    game.paddle = 8;
    let before = game.clone();

    assert_eq!(game.hint_move(), Some(PaddleMove::Left));
    assert_eq!(game.ball_x, before.ball_x);
    assert_eq!(game.ball_y, before.ball_y);
    assert_eq!(game.paddle, before.paddle);
    assert_eq!(game.moves, before.moves);
}

#[test]
fn hint_is_empty_after_breakout_ends() {
    let mut game = Breakout::new(1);
    game.status = BreakoutStatus::Won;

    assert_eq!(game.hint_move(), None);
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
fn legacy_saves_default_to_an_active_breakout() {
    let mut value = serde_json::to_value(Breakout::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("control");
    value.as_object_mut().unwrap().remove("paused");
    value.as_object_mut().unwrap().remove("brick_health");
    value.as_object_mut().unwrap().remove("lives");
    value.as_object_mut().unwrap().remove("level");
    value.as_object_mut().unwrap().remove("serve_ready");
    let mut restored: Breakout = serde_json::from_value(value).unwrap();

    assert_eq!(restored.control, PaddleMove::Stay);
    assert!(!restored.paused);
    assert_eq!(restored.lives, 3);
    assert_eq!(restored.level, 1);
    assert!(restored.brick_health.is_empty());
    restored.ensure_runtime();
    assert_eq!(restored.brick_health.len(), restored.bricks.len());
}

#[test]
fn later_seeded_walls_add_gaps_and_armored_bricks() {
    let mut first = Breakout::new(17);
    first.level = 2;
    first.build_wall();
    let mut second = Breakout::new(17);
    second.level = 2;
    second.build_wall();

    assert_eq!(first.brick_health, second.brick_health);
    assert!(first.remaining_bricks() < BRICK_COUNT);
    assert!(first.brick_health.contains(&0));
    assert!(first.brick_health.contains(&2));
}

#[test]
fn alternate_wall_launches_at_the_requested_level() {
    let game = Breakout::new_with_level(42, 3);
    assert_eq!(game.level, 3);
    assert!(game.remaining_bricks() > 0);
    assert!(game.brick_health.iter().any(|health| *health > 1));
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
fn clearing_a_wall_advances_to_a_faster_pattern_and_waits_for_launch() {
    let mut game = Breakout::new(19);
    game.bricks.fill(false);
    game.brick_health.fill(0);
    game.complete_wall();

    assert_eq!(game.level, 2);
    assert_eq!(game.status, BreakoutStatus::Playing);
    assert!(game.remaining_bricks() > 0);
    assert!(game.serve_ready);
    assert!(game.paused);
    assert!(game.toggle_pause());
    assert!(!game.serve_ready);
    assert!(!game.paused);
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

#[test]
fn paddle_edges_steer_the_rebound() {
    let mut game = Breakout::new(21);
    game.precise_ball_x = game.precise_paddle + 1.5;
    game.precise_ball_y = f32::from(HEIGHT - 1) - 0.001;
    game.velocity_x = -1;
    game.velocity_y = 1;

    game.simulate_step(0.001);

    assert_eq!(game.velocity_y, -1);
    assert_eq!(game.velocity_x, 1);
}
