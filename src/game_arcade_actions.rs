//! Action routing for continuously moving arcade drawers.

use super::Game;
use crate::ui::UiAction;

pub(super) fn apply(game: &mut Game, action: &UiAction) -> bool {
    match action {
        UiAction::SnakeStep(direction) => {
            game.state.games.snake.set_direction(*direction);
        }
        UiAction::SnakeMode(mode) => {
            let seed = game.state.games.snake.seed.wrapping_add(1);
            game.state.games.snake = crate::snake::Snake::new_with_mode(seed, *mode);
        }
        UiAction::SnakePause => {
            game.state.games.snake.toggle_pause();
        }
        UiAction::SnakeHint => {
            game.state.card_hint = Some(crate::card_hints::snake(&game.state));
        }
        UiAction::SnakeUndo => {
            game.state.games.snake.undo();
        }
        UiAction::SnakeNew => {
            let seed = game.state.games.snake.seed.wrapping_add(1);
            game.state.games.snake.reset(seed);
        }
        UiAction::SpaceInvadersStep(direction) => {
            game.state.games.space_invaders.set_control(*direction);
        }
        UiAction::SpaceInvadersFire => {
            game.state.games.space_invaders.fire();
        }
        UiAction::SpaceInvadersPause => {
            game.state.games.space_invaders.toggle_pause();
        }
        UiAction::SpaceInvadersUndo => {
            game.state.games.space_invaders.undo();
        }
        UiAction::SpaceInvadersNew => {
            let seed = game.state.games.space_invaders.seed.wrapping_add(1);
            game.state.games.space_invaders.reset(seed);
        }
        UiAction::AsteroidsStep(direction) => {
            game.state.games.asteroids.set_control(*direction);
        }
        UiAction::AsteroidsFire => {
            game.state.games.asteroids.fire();
        }
        UiAction::AsteroidsPause => {
            game.state.games.asteroids.toggle_pause();
        }
        UiAction::AsteroidsUndo => {
            game.state.games.asteroids.undo();
        }
        UiAction::AsteroidsNew => {
            let seed = game.state.games.asteroids.seed.wrapping_add(1);
            game.state.games.asteroids.reset(seed);
        }
        UiAction::FroggerMove(direction) => {
            game.state.games.frogger.move_player(*direction);
        }
        UiAction::FroggerPause => {
            game.state.games.frogger.toggle_pause();
        }
        UiAction::FroggerUndo => {
            game.state.games.frogger.undo();
        }
        UiAction::FroggerNew => {
            let seed = game.state.games.frogger.seed.wrapping_add(1);
            game.state.games.frogger.reset(seed);
        }
        UiAction::BreakoutStep(movement) => {
            game.state.games.breakout.set_control(*movement);
        }
        UiAction::BreakoutPause => {
            game.state.games.breakout.toggle_pause();
        }
        UiAction::BreakoutHint => {
            game.state.card_hint = Some(crate::card_hints::breakout(&game.state));
        }
        UiAction::BreakoutUndo => {
            game.state.games.breakout.undo();
        }
        UiAction::BreakoutNew => {
            let seed = game.state.games.breakout.seed.wrapping_add(1);
            game.state.games.breakout.reset(seed);
        }
        UiAction::MunchMove(direction) => game.state.games.munch_maze.set_direction(*direction),
        UiAction::MunchPause => {
            game.state.games.munch_maze.toggle_pause();
        }
        UiAction::MunchUndo => {
            game.state.games.munch_maze.undo();
        }
        UiAction::MunchNew => {
            let seed = game.state.games.munch_maze.seed.wrapping_add(1);
            game.state.games.munch_maze.reset(seed);
        }
        UiAction::BlockMove(movement) => {
            game.state.games.block_stack.apply_move(*movement);
        }
        UiAction::BlockPause => {
            game.state.games.block_stack.toggle_pause();
        }
        UiAction::BlockUndo => {
            game.state.games.block_stack.undo();
        }
        UiAction::BlockNew => {
            let seed = game.state.games.block_stack.seed.wrapping_add(1);
            game.state.games.block_stack.reset(seed);
        }
        UiAction::CannonAngle(delta) => {
            game.state.games.terrain_cannon.adjust_angle(*delta);
        }
        UiAction::CannonPower(delta) => {
            game.state.games.terrain_cannon.adjust_power(*delta);
        }
        UiAction::CannonFire => {
            game.state.games.terrain_cannon.fire();
        }
        UiAction::CannonPause => {
            game.state.games.terrain_cannon.toggle_pause();
        }
        UiAction::CannonUndo => {
            game.state.games.terrain_cannon.undo();
        }
        UiAction::CannonNew => {
            let seed = game.state.games.terrain_cannon.seed.wrapping_add(1);
            game.state.games.terrain_cannon.reset(seed);
        }
        UiAction::FlingAngle(delta) => {
            game.state.games.fling_fury.adjust_angle(*delta);
        }
        UiAction::FlingPower(delta) => {
            game.state.games.fling_fury.adjust_power(*delta);
        }
        UiAction::FlingFire => {
            game.state.games.fling_fury.fire();
        }
        UiAction::FlingPause => {
            game.state.games.fling_fury.toggle_pause();
        }
        UiAction::FlingUndo => {
            game.state.games.fling_fury.undo();
        }
        UiAction::FlingNew => {
            let seed = game.state.games.fling_fury.seed.wrapping_add(1);
            game.state.games.fling_fury.reset(seed);
        }
        UiAction::FlingNext => {
            game.state.games.fling_fury.next_level();
        }
        UiAction::PaddleMove(movement) => {
            game.state.games.paddle_duel.set_control(*movement);
        }
        UiAction::PaddlePause => {
            game.state.games.paddle_duel.toggle_pause();
        }
        UiAction::PaddleUndo => {
            game.state.games.paddle_duel.undo();
        }
        UiAction::PaddleNew => {
            let seed = game.state.games.paddle_duel.seed.wrapping_add(1);
            game.state.games.paddle_duel.reset(seed);
        }
        _ => return false,
    }
    true
}
