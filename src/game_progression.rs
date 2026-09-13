//! Record, feedback, and movement helpers for the game host.

use super::Game;
use crate::domain::Direction;
use crate::{
    sound::SoundCue,
    state::{GameId, Screen},
};

#[path = "game_progression/completion.rs"]
mod completion;
#[path = "game_progression/records.rs"]
mod records;

pub(crate) use completion::round_is_complete;

impl Game {
    pub(super) fn tick_elapsed(&mut self, dt: f32) {
        self.state.records.ensure_time_slots();
        if self.state.lifecycle_paused {
            return;
        }
        let Screen::Game(game) = self.state.screen else {
            return;
        };
        if !is_timed_game(game)
            || self.state.tutorial.is_some()
            || self.state.confirm_restart
            || self.state.confirm_reset
            || round_is_complete(&self.state, game)
        {
            return;
        }
        self.state.records.elapsed_remainder += dt.max(0.0);
        let whole_seconds = self.state.records.elapsed_remainder.floor() as u32;
        if whole_seconds > 0 {
            self.state.records.elapsed_remainder -= whole_seconds as f32;
            self.state.records.elapsed_seconds[game.index()] =
                self.state.records.elapsed_seconds[game.index()].saturating_add(whole_seconds);
        }
    }

    pub(super) fn reset_elapsed(&mut self) {
        if let Screen::Game(game) = self.state.screen {
            self.state.records.reset_time(game.index());
            self.state.records.elapsed_remainder = 0.0;
        }
    }

    pub(super) fn play_feedback(&self, cue: SoundCue) {
        let volume = crate::audio_settings::volume(self.state.sound, self.state.sound_level);
        if volume > 0.0 {
            self.sounds.play(self.state.sound_set, cue, volume);
        }
    }

    pub(super) fn try_move(&mut self, direction: Direction) {
        if self.state.games.game.move_in(direction) && self.state.games.game.won() {
            self.notifications
                .success("2048 reached — keep playing or start a fresh board");
        }
        self.update_records();
        self.request_autosave();
    }
}

pub(super) fn is_timed_game(game: GameId) -> bool {
    !matches!(
        game,
        GameId::Snake
            | GameId::Breakout
            | GameId::TinyTowerDefence
            | GameId::SpaceInvaders
            | GameId::Asteroids
            | GameId::Frogger
            | GameId::MunchMaze
            | GameId::BlockStack
            | GameId::TerrainCannon
            | GameId::FlingFury
            | GameId::PaddleDuel
    )
}
