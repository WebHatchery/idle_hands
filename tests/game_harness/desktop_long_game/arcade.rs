//! Deterministic capture scenarios for the cabinet game suite.

use idle_hands::testing::games::asteroids::{Asteroids, ShipDirection as AsteroidDirection};
use idle_hands::testing::games::block_stack::{BlockMove, BlockStack};
use idle_hands::testing::games::breakout::{Breakout, PaddleMove as BreakoutMove};
use idle_hands::testing::games::fling_fury::FlingFury;
use idle_hands::testing::games::frogger::Frogger;
use idle_hands::testing::games::munch_maze::MunchMaze;
use idle_hands::testing::games::paddle_duel::{PaddleDuel, PaddleMove as DuelMove};
use idle_hands::testing::games::potion_2048::Potion2048;
use idle_hands::testing::games::snake::Snake;
use idle_hands::testing::games::space_invaders::{ShipDirection, SpaceInvaders};
use idle_hands::testing::games::terrain_cannon::TerrainCannon;
use idle_hands::testing::Direction;
use idle_hands::testing::GameId;

use super::{assert_serializable, SESSION_SEED};

pub(super) fn run(game: GameId) {
    match game {
        GameId::Snake => snake(),
        GameId::Breakout => breakout(),
        GameId::Potion2048 => potion_2048(),
        GameId::SpaceInvaders => space_invaders(),
        GameId::Asteroids => asteroids(),
        GameId::Frogger => frogger(),
        GameId::MunchMaze => munch_maze(),
        GameId::BlockStack => block_stack(),
        GameId::TerrainCannon => terrain_cannon(),
        GameId::FlingFury => fling_fury(),
        GameId::PaddleDuel => paddle_duel(),
        _ => panic!("arcade session was assigned the wrong game"),
    }
}

fn snake() {
    let mut game = Snake::new(SESSION_SEED);
    let mut ticks = 0;
    for round in 0..12 {
        for _ in 0..128 {
            if let Some(direction) = game.hint_direction() {
                game.set_direction(direction);
            }
            game.tick(0.2);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 512,
        "Snake should retain direction and tick updates"
    );
    assert_serializable(&game);
}

fn breakout() {
    let mut game = Breakout::new(SESSION_SEED);
    let mut ticks = 0;
    for round in 0..12 {
        for _ in 0..128 {
            let movement = game.hint_move().unwrap_or(BreakoutMove::Stay);
            game.set_control(movement);
            game.tick(0.16);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 512,
        "Breakout should retain paddle control across long play"
    );
    assert_serializable(&game);
}

fn potion_2048() {
    let mut game = Potion2048::new(SESSION_SEED);
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut moves = 0;
    for round in 0..12 {
        for direction in directions.into_iter().cycle().take(96) {
            if game.move_in(direction) {
                moves += 1;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 64,
        "Potion 2048 should retain repeated board movement"
    );
    assert_serializable(&game);
}

fn space_invaders() {
    let mut game = SpaceInvaders::new(SESSION_SEED);
    let controls = [
        ShipDirection::Left,
        ShipDirection::Stay,
        ShipDirection::Right,
    ];
    let mut ticks = 0;
    for round in 0..8 {
        for step in 0..256 {
            game.set_control(controls[step % controls.len()]);
            game.fire();
            game.tick(0.1);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 512,
        "Space Invaders should retain fire and wave ticks"
    );
    assert_serializable(&game);
}

fn asteroids() {
    let mut game = Asteroids::new(SESSION_SEED);
    let controls = [
        AsteroidDirection::Left,
        AsteroidDirection::Stay,
        AsteroidDirection::Right,
    ];
    let mut ticks = 0;
    for round in 0..8 {
        for step in 0..256 {
            game.set_control(controls[step % controls.len()]);
            game.fire();
            game.tick(0.1);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 512,
        "Asteroids should retain fire and movement ticks"
    );
    assert_serializable(&game);
}

fn frogger() {
    let mut game = Frogger::new(SESSION_SEED);
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut actions = 0;
    for round in 0..12 {
        for step in 0..96 {
            game.move_player(directions[step % directions.len()]);
            game.tick(0.2);
            actions += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 384,
        "Frogger should retain movement and traffic updates"
    );
    assert_serializable(&game);
}

fn munch_maze() {
    let mut game = MunchMaze::new(SESSION_SEED);
    let directions = [
        Direction::Left,
        Direction::Up,
        Direction::Right,
        Direction::Down,
    ];
    let mut ticks = 0;
    for round in 0..12 {
        for step in 0..128 {
            game.set_direction(directions[step % directions.len()]);
            game.tick(0.2);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 512,
        "Munch Maze should retain direction and ghost ticks"
    );
    assert_serializable(&game);
}

fn block_stack() {
    let mut game = BlockStack::new(SESSION_SEED);
    let moves = [
        BlockMove::Left,
        BlockMove::Rotate,
        BlockMove::Right,
        BlockMove::Drop,
    ];
    let mut actions = 0;
    for round in 0..8 {
        for step in 0..128 {
            game.apply_move(moves[step % moves.len()]);
            game.tick(0.6);
            actions += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 512,
        "Block Stack should retain piece actions and gravity"
    );
    assert_serializable(&game);
}

fn terrain_cannon() {
    let mut game = TerrainCannon::new(SESSION_SEED);
    let mut shots = 0;
    for round in 0..12 {
        for step in 0..48 {
            game.adjust_angle(if step % 2 == 0 { 2 } else { -1 });
            game.adjust_power(if step % 3 == 0 { 3 } else { -2 });
            if game.fire() {
                shots += 1;
            }
            for _ in 0..8 {
                game.tick(0.2);
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        shots >= 24,
        "Terrain Cannon should retain repeated fire cycles"
    );
    assert_serializable(&game);
}

fn fling_fury() {
    let mut game = FlingFury::new(SESSION_SEED);
    let mut shots = 0;
    for round in 0..12 {
        for step in 0..48 {
            game.adjust_angle(if step % 2 == 0 { 2 } else { -1 });
            game.adjust_power(if step % 3 == 0 { 3 } else { -2 });
            if game.fire() {
                shots += 1;
            }
            for _ in 0..8 {
                game.tick(0.2);
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(shots >= 24, "Fling Fury should retain repeated fire cycles");
    assert_serializable(&game);
}

fn paddle_duel() {
    let mut game = PaddleDuel::new(SESSION_SEED);
    let controls = [DuelMove::Up, DuelMove::Stay, DuelMove::Down];
    let mut ticks = 0;
    for round in 0..12 {
        for step in 0..256 {
            game.set_control(controls[step % controls.len()]);
            game.tick(0.05);
            ticks += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        ticks >= 1024,
        "Paddle Duel should retain control through long rallies"
    );
    assert_serializable(&game);
}
