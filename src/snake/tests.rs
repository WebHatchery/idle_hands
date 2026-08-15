use super::*;

#[test]
fn seeded_snakes_repeat_and_start_with_food_outside_the_body() {
    let first = Snake::new(7);
    let second = Snake::new(7);
    assert_eq!(first.body, second.body);
    assert_eq!(first.food, second.food);
    assert!(!first.body.contains(&first.food));
}

#[test]
fn a_touch_step_advances_and_can_be_undone() {
    let mut game = Snake::new(1);
    let old_head = game.body[0];
    assert!(game.step(SnakeDirection::Right));
    assert_ne!(game.body[0], old_head);
    assert!(game.undo());
    assert_eq!(game.body[0], old_head);
    assert_eq!(game.moves, 0);
}

#[test]
fn opposite_turns_are_rejected_and_wall_collisions_lose() {
    let mut game = Snake::new(1);
    assert!(!game.step(SnakeDirection::Left));
    game.body = vec![15];
    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.status, SnakeStatus::Lost);
}
