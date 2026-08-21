use super::*;

#[test]
fn starts_with_a_live_ball_and_score() {
    let game = PaddleDuel::new(7);
    assert_eq!(game.player_score, 0);
    assert!(game.ball_vx > 0.);
}

#[test]
fn touch_control_advances_and_state_round_trips() {
    let mut game = PaddleDuel::new(8);
    game.set_control(PaddleMove::Up);
    assert!(game.tick(0.2));
    assert!(game.moves > 0);
    let restored: PaddleDuel = serde_json::from_value(serde_json::to_value(game).unwrap()).unwrap();
    assert_eq!(restored.status, PaddleStatus::Playing);
}
