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

#[test]
fn hint_prefers_a_safe_direction_toward_food_without_mutating() {
    let mut game = Snake::new(1);
    game.food = 8;
    let before = game.clone();

    assert_eq!(game.hint_direction(), Some(SnakeDirection::Up));
    assert_eq!(game.body, before.body);
    assert_eq!(game.direction, before.direction);
    assert_eq!(game.food, before.food);
    assert_eq!(game.moves, before.moves);
}

#[test]
fn hint_is_empty_after_the_snake_finishes() {
    let mut game = Snake::new(1);
    game.status = SnakeStatus::Won;

    assert_eq!(game.hint_direction(), None);
}

#[test]
fn elapsed_time_drives_movement_and_pause_stops_the_coil() {
    let mut game = Snake::new(1);
    let start = game.body.clone();

    assert!(!game.tick(0.19));
    assert_eq!(game.body, start);
    assert!(game.tick(0.02));
    assert_ne!(game.body, start);

    assert!(game.toggle_pause());
    let paused = game.body.clone();
    assert!(!game.tick(1.));
    assert_eq!(game.body, paused);
    assert!(game.toggle_pause());
    assert!(game.tick(0.21));
    assert_ne!(game.body, paused);
}

#[test]
fn legacy_saves_default_to_an_active_coil() {
    let mut value = serde_json::to_value(Snake::new(1)).unwrap();
    value.as_object_mut().unwrap().remove("paused");
    let restored: Snake = serde_json::from_value(value).unwrap();

    assert!(!restored.paused);
}
