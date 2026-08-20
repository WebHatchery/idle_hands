//! Frame-driven updates for the cabinet's real-time games.

use super::Game;
use crate::state::{GameId, Screen};

impl Game {
    pub(super) fn tick_realtime(&mut self, dt: f32) {
        if self.state.tutorial.is_some() || self.state.confirm_restart || self.state.confirm_reset {
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
            _ => {}
        }
    }

    fn finish_realtime_round(&mut self) {
        self.update_records();
        self.request_autosave();
    }
}
