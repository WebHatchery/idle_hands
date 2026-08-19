use super::*;

#[test]
fn seeded_boards_repeat_and_have_four_brick_rows() {
    let first = Breakout::new(7);
    let second = Breakout::new(7);
    assert_eq!(first.bricks, second.bricks);
    assert_eq!(first.bricks.len(), 64);
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
fn missing_the_paddle_loses_the_round() {
    let mut game = Breakout::new(1);
    game.ball_x = 0;
    game.ball_y = HEIGHT - 2;
    game.velocity_x = 0;
    game.velocity_y = 1;
    assert!(game.step(PaddleMove::Right));
    assert_eq!(game.status, BreakoutStatus::Lost);
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
    let restored: Breakout = serde_json::from_value(value).unwrap();

    assert_eq!(restored.control, PaddleMove::Stay);
    assert!(!restored.paused);
}
