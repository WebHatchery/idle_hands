//! Frame-driven updates for the cabinet's real-time games.

use super::Game;
use crate::state::{GameId, Screen};

impl Game {
    pub(super) fn tick_realtime(&mut self, dt: f32) {
        if self.state.lifecycle_paused
            || self.state.tutorial.is_some()
            || self.state.confirm_restart
            || self.state.confirm_reset
        {
            return;
        }
        match self.state.screen {
            Screen::Game(GameId::Snake) => {
                let previous = self.state.games.snake.status;
                self.state.games.snake.tick(dt);
                if self.state.games.snake.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::Breakout) => {
                let previous = (
                    self.state.games.breakout.status,
                    self.state.games.breakout.level,
                    self.state.games.breakout.lives,
                    self.state.games.breakout.serve_ready,
                );
                self.state.games.breakout.tick(dt);
                let current = (
                    self.state.games.breakout.status,
                    self.state.games.breakout.level,
                    self.state.games.breakout.lives,
                    self.state.games.breakout.serve_ready,
                );
                if current != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::TinyTowerDefence) => {
                let previous = self.state.games.tiny_tower_defence.phase;
                self.state.games.tiny_tower_defence.tick(dt);
                if self.state.games.tiny_tower_defence.phase != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::SpaceInvaders) => {
                let previous = self.state.games.space_invaders.status;
                self.state.games.space_invaders.tick(dt);
                if self.state.games.space_invaders.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::Asteroids) => {
                let previous = self.state.games.asteroids.status;
                self.state.games.asteroids.tick(dt);
                if self.state.games.asteroids.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::Frogger) => {
                let previous = self.state.games.frogger.status;
                self.state.games.frogger.tick(dt);
                if self.state.games.frogger.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::MunchMaze) => {
                let previous = self.state.games.munch_maze.status;
                self.state.games.munch_maze.tick(dt);
                if self.state.games.munch_maze.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::BlockStack) => {
                let previous = self.state.games.block_stack.status;
                self.state.games.block_stack.tick(dt);
                if self.state.games.block_stack.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::TerrainCannon) => {
                let previous = self.state.games.terrain_cannon.status;
                self.state.games.terrain_cannon.tick(dt);
                if self.state.games.terrain_cannon.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::FlingFury) => {
                let previous = self.state.games.fling_fury.status;
                self.state.games.fling_fury.tick(dt);
                if self.state.games.fling_fury.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::PaddleDuel) => {
                let previous = self.state.games.paddle_duel.status;
                self.state.games.paddle_duel.tick(dt);
                if self.state.games.paddle_duel.status != previous {
                    self.finish_realtime_round();
                }
            }
            _ => {}
        }
    }

    fn finish_realtime_round(&mut self) {
        self.update_records();
        self.request_autosave();
    }
}
