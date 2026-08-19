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
                let previous = self.state.snake.status;
                self.state.snake.tick(dt);
                if self.state.snake.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::Breakout) => {
                let previous = self.state.breakout.status;
                self.state.breakout.tick(dt);
                if self.state.breakout.status != previous {
                    self.finish_realtime_round();
                }
            }
            Screen::Game(GameId::TinyTowerDefence) => {
                let previous = self.state.tiny_tower_defence.phase;
                self.state.tiny_tower_defence.tick(dt);
                if self.state.tiny_tower_defence.phase != previous {
                    self.finish_realtime_round();
                }
            }
            _ => {}
        }
    }

    fn finish_realtime_round(&mut self) {
        self.update_records();
        self.save_autosave();
    }
}
