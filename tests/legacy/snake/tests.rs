//! Regression coverage for the tests module.

use idle_hands::testing::modules::snake::*;

#[test]
fn opposite_turns_are_rejected_and_wall_collisions_lose() {
    let mut game = Snake::new(1);
    assert!(!game.step(SnakeDirection::Left));
    game.body = vec![15];
    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.status, SnakeStatus::Lost);
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
fn wrap_mode_crosses_edges_instead_of_losing() {
    let mut game = Snake::new_with_mode(2, SnakeMode::Wrap);
    game.body = vec![15];
    game.direction = SnakeDirection::Right;

    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.body[0], 0);
    assert_eq!(game.status, SnakeStatus::Playing);
}

#[test]
fn gold_food_is_worth_three_and_undo_restores_its_kind() {
    let mut game = Snake::new(5);
    game.food = game.body[0] + 1;
    game.food_kind = FoodKind::Gold;
    let old_food = game.food;

    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.score, 3);
    assert_eq!(game.body.len(), 4);
    assert!(game.undo());
    assert_eq!(game.score, 0);
    assert_eq!(game.food, old_food);
    assert_eq!(game.food_kind, FoodKind::Gold);
}
