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
    let object = value.as_object_mut().unwrap();
    object.remove("paused");
    object.remove("mode");
    object.remove("obstacles");
    object.remove("food_kind");
    let restored: Snake = serde_json::from_value(value).unwrap();

    assert!(!restored.paused);
    assert_eq!(restored.mode, SnakeMode::Classic);
    assert!(restored.obstacles.is_empty());
    assert_eq!(restored.food_kind, FoodKind::Berry);
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
fn garden_mode_places_repeatable_rocks_away_from_the_opening() {
    let first = Snake::new_with_mode(3, SnakeMode::Garden);
    let second = Snake::new_with_mode(3, SnakeMode::Garden);

    assert_eq!(first.obstacles, second.obstacles);
    assert_eq!(first.obstacles.len(), GARDEN_ROCKS);
    assert!(first
        .obstacles
        .iter()
        .all(|cell| !first.body.contains(cell)));
    assert!(!first.obstacles.contains(&first.food));
    assert!(first.obstacles.iter().all(|cell| {
        let row = i32::from(*cell) / WIDTH;
        let column = i32::from(*cell) % WIDTH;
        row != HEIGHT / 2 || !(WIDTH / 2 - 3..=WIDTH / 2 + 3).contains(&column)
    }));
}

#[test]
fn touching_a_garden_rock_ends_the_round() {
    let mut game = Snake::new_with_mode(4, SnakeMode::Garden);
    let head = game.body[0];
    let next = head + 1;
    game.obstacles = vec![next];

    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.status, SnakeStatus::Lost);
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

#[test]
fn every_fifth_point_prepares_gold_food_and_speed_pressure() {
    let mut game = Snake::new(6);
    game.score = 3;
    let opening_interval = game.move_interval();
    game.food = game.body[0] + 1;
    game.food_kind = FoodKind::Berry;

    assert!(game.step(SnakeDirection::Right));
    assert_eq!(game.score, 4);
    assert_eq!(game.food_kind, FoodKind::Gold);
    game.score = 10;
    assert!(game.move_interval() < opening_interval);
    assert_eq!(game.speed_stage(), 3);
}

#[test]
fn wrap_hint_uses_the_short_route_across_the_edge() {
    let mut game = Snake::new_with_mode(7, SnakeMode::Wrap);
    game.body = vec![0, 1, 2];
    game.direction = SnakeDirection::Up;
    game.food = (WIDTH - 1) as u16;

    assert_eq!(game.hint_direction(), Some(SnakeDirection::Left));
}

#[test]
fn reset_keeps_the_selected_variant() {
    let mut game = Snake::new_with_mode(8, SnakeMode::Garden);
    game.reset(9);
    let expected = Snake::new_with_mode(9, SnakeMode::Garden);

    assert_eq!(game.mode, SnakeMode::Garden);
    assert_eq!(game.obstacles.len(), GARDEN_ROCKS);
    assert_eq!(game.body, expected.body);
    assert_eq!(game.obstacles, expected.obstacles);
    assert_eq!(game.food, expected.food);
    assert_eq!(game.seed, expected.seed);
}
