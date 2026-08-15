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
