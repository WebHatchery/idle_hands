//! Idle Hands analytics policy and shared-client wiring.

use crate::{
    game_descriptor, progression,
    state::{AppState, GameId, Screen},
};
use macroquad_toolkit::analytics::{AnalyticsClient, AnalyticsConfig};

const ACTIVE_INPUT_GRACE_SECONDS: f32 = 30.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProgressSignal {
    FirstDrawerOpened,
    TutorialCompleted,
    FirstDrawerCompleted,
    TenDrawersCompleted,
    DemoCompleted,
    ThirtyDrawersCompleted,
    CabinetCompleted,
}

#[derive(Debug, Clone, Copy)]
struct ProgressState {
    drawer_open: bool,
    tutorials: usize,
    completed: usize,
    demo_completed: usize,
}

#[derive(Debug, Clone, Copy)]
struct ProgressSignalsInput {
    before: ProgressState,
    after: ProgressState,
    tutorial_emitted: bool,
    demo_build: bool,
}

pub struct GameAnalytics {
    client: Option<AnalyticsClient>,
    recent_input_seconds: f32,
    opened_any_drawer: bool,
    tutorial_count: usize,
    completed_count: usize,
    demo_completed_count: usize,
    tutorial_emitted: bool,
}

impl GameAnalytics {
    pub fn new(state: &AppState, enabled: bool) -> Self {
        let mut analytics = Self::disabled(state);
        if enabled && analytics_enabled() {
            analytics.client = Some(AnalyticsClient::new(analytics_config()));
        }
        analytics
    }

    pub fn disabled(state: &AppState) -> Self {
        Self {
            client: None,
            recent_input_seconds: 0.0,
            opened_any_drawer: !state.recent_games.is_empty(),
            tutorial_count: tutorial_count(state),
            completed_count: progression::completed_games(&state.records),
            demo_completed_count: completed_demo_games(state),
            tutorial_emitted: false,
        }
    }

    pub fn note_input(&mut self) {
        self.recent_input_seconds = ACTIVE_INPUT_GRACE_SECONDS;
    }

    pub fn sync_progress(&mut self, state: &AppState) {
        self.opened_any_drawer |= state.screen.is_game();
        self.tutorial_count = tutorial_count(state);
        self.completed_count = progression::completed_games(&state.records);
        self.demo_completed_count = completed_demo_games(state);
    }

    pub fn update(&mut self, dt: f32, state: &AppState) {
        self.recent_input_seconds = (self.recent_input_seconds - dt.max(0.0)).max(0.0);
        let signals = self.observe_progress(state);
        let active = is_active_play(state, self.recent_input_seconds);
        let Some(client) = self.client.as_mut() else {
            return;
        };
        for signal in signals {
            match signal {
                ProgressSignal::FirstDrawerOpened => client.milestone("first_drawer_opened"),
                ProgressSignal::TutorialCompleted => client.milestone("tutorial_completed"),
                ProgressSignal::FirstDrawerCompleted => client.milestone("first_drawer_completed"),
                ProgressSignal::TenDrawersCompleted => client.milestone("ten_drawers_completed"),
                ProgressSignal::DemoCompleted => client.demo_completed(),
                ProgressSignal::ThirtyDrawersCompleted => {
                    client.milestone("thirty_drawers_completed")
                }
                ProgressSignal::CabinetCompleted => client.game_completed(),
            }
        }
        client.update(dt, active);
    }

    pub fn end_session(&mut self) {
        if let Some(client) = self.client.as_mut() {
            client.end_session();
            client.update(0.0, false);
        }
    }

    fn observe_progress(&mut self, state: &AppState) -> Vec<ProgressSignal> {
        let current_tutorials = tutorial_count(state);
        let current_completed = progression::completed_games(&state.records);
        let current_demo_completed = completed_demo_games(state);
        let signals = progress_signals(ProgressSignalsInput {
            before: ProgressState {
                drawer_open: self.opened_any_drawer,
                tutorials: self.tutorial_count,
                completed: self.completed_count,
                demo_completed: self.demo_completed_count,
            },
            after: ProgressState {
                drawer_open: state.screen.is_game(),
                tutorials: current_tutorials,
                completed: current_completed,
                demo_completed: current_demo_completed,
            },
            tutorial_emitted: self.tutorial_emitted,
            demo_build: game_descriptor::is_demo_build(),
        });
        self.opened_any_drawer |= state.screen.is_game();
        if current_tutorials > self.tutorial_count {
            self.tutorial_emitted = true;
        }
        self.tutorial_count = current_tutorials;
        self.completed_count = current_completed;
        self.demo_completed_count = current_demo_completed;
        signals
    }
}

fn analytics_enabled() -> bool {
    option_env!("IDLE_HANDS_ANALYTICS_ENABLED") == Some("true")
}

fn analytics_config() -> AnalyticsConfig {
    let endpoint = env!("IDLE_HANDS_ANALYTICS_ENDPOINT");
    let write_key = env!("IDLE_HANDS_ANALYTICS_WRITE_KEY");
    #[cfg(target_arch = "wasm32")]
    {
        AnalyticsConfig::web(
            endpoint,
            write_key,
            "idle_hands",
            env!("CARGO_PKG_VERSION"),
            "webhatchery",
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        AnalyticsConfig {
            endpoint: endpoint.to_owned(),
            write_key: write_key.to_owned(),
            game: "idle_hands".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            platform: "windows".to_owned(),
            source: "webhatchery".to_owned(),
            heartbeat_seconds: 60.0,
            flush_seconds: 30.0,
        }
    }
}

fn tutorial_count(state: &AppState) -> usize {
    state.tutorial_seen.iter().filter(|seen| **seen).count()
}

fn completed_demo_games(state: &AppState) -> usize {
    GameId::ALL
        .into_iter()
        .filter(|game| game_descriptor::is_demo_game(*game))
        .filter(|game| progression::game_complete(&state.records, *game))
        .count()
}

fn is_active_play(state: &AppState, recent_input_seconds: f32) -> bool {
    let Screen::Game(game) = state.screen else {
        return false;
    };
    recent_input_seconds > 0.0
        && state.tutorial.is_none()
        && !state.confirm_restart
        && !state.confirm_reset
        && !state.lifecycle_paused
        && !crate::game::game_progression::round_is_complete(state, game)
        && !is_paused(state, game)
}

fn is_paused(state: &AppState, game: GameId) -> bool {
    match game {
        GameId::Snake => state.games.snake.paused,
        GameId::Breakout => state.games.breakout.paused,
        GameId::TinyTowerDefence => state.games.tiny_tower_defence.paused,
        GameId::SpaceInvaders => state.games.space_invaders.paused,
        GameId::Asteroids => state.games.asteroids.paused,
        GameId::Frogger => state.games.frogger.paused,
        GameId::MunchMaze => state.games.munch_maze.paused,
        GameId::BlockStack => state.games.block_stack.paused,
        GameId::TerrainCannon => state.games.terrain_cannon.paused,
        GameId::FlingFury => state.games.fling_fury.paused,
        GameId::PaddleDuel => state.games.paddle_duel.paused,
        _ => false,
    }
}

fn progress_signals(input: ProgressSignalsInput) -> Vec<ProgressSignal> {
    let mut signals = Vec::new();
    if !input.before.drawer_open && input.after.drawer_open {
        signals.push(ProgressSignal::FirstDrawerOpened);
    }
    if !input.tutorial_emitted && input.after.tutorials > input.before.tutorials {
        signals.push(ProgressSignal::TutorialCompleted);
    }
    if input.before.completed == 0 && input.after.completed > 0 {
        signals.push(ProgressSignal::FirstDrawerCompleted);
    }
    if input.before.completed < 10 && input.after.completed >= 10 {
        signals.push(ProgressSignal::TenDrawersCompleted);
    }
    let demo_total = GameId::ALL
        .into_iter()
        .filter(|game| game_descriptor::is_demo_game(*game))
        .count();
    if input.demo_build
        && input.before.demo_completed < demo_total
        && input.after.demo_completed >= demo_total
    {
        signals.push(ProgressSignal::DemoCompleted);
    }
    if input.before.completed < 30 && input.after.completed >= 30 {
        signals.push(ProgressSignal::ThirtyDrawersCompleted);
    }
    if input.before.completed < GameId::ALL.len() && input.after.completed >= GameId::ALL.len() {
        signals.push(ProgressSignal::CabinetCompleted);
    }
    signals
}

#[cfg(test)]
mod tests;
