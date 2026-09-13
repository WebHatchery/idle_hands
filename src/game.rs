//! Application lifecycle and input routing.

use crate::input::PointerTracker;
use crate::sound::SoundBank;
use crate::{data::GameData, state::AppState};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::NotificationManager;

#[path = "game_action_dispatch.rs"]
mod game_action_dispatch;
#[path = "game_arcade_actions.rs"]
mod game_arcade_actions;
#[path = "game_board_actions.rs"]
mod game_board_actions;
#[path = "game_board_dispatch.rs"]
mod game_board_dispatch;
#[path = "game_capture.rs"]
mod game_capture;
#[path = "game_capture_depth.rs"]
mod game_capture_depth;
#[path = "game_capture_records.rs"]
mod game_capture_records;
#[path = "game_capture_rules.rs"]
mod game_capture_rules;
#[path = "game_capture_scenes.rs"]
mod game_capture_scenes;
#[path = "game_lifecycle.rs"]
mod game_lifecycle;
#[path = "game_navigation.rs"]
mod game_navigation;
#[path = "game_persistence.rs"]
mod game_persistence;
#[path = "game_progression.rs"]
pub(crate) mod game_progression;
#[path = "game_realtime.rs"]
mod game_realtime;
#[path = "game_render.rs"]
mod game_render;
#[path = "game_restart.rs"]
mod game_restart;
#[path = "game_settings.rs"]
mod game_settings;
#[path = "game_update.rs"]
mod game_update;

pub struct Game {
    pub(crate) data: GameData,
    pub(crate) state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    pointer: PointerTracker,
    pub(super) touch_was_active: bool,
    pub(super) capture_solitaire_peek: bool,
    sounds: SoundBank,
    transition: f32,
    confirmation_bypass: bool,
    pub(super) save_dirty: bool,
    pub(super) save_timer: f32,
    analytics: crate::analytics::GameAnalytics,
}
impl Game {
    pub async fn new() -> Self {
        macroquad_toolkit::rng::srand(get_time().to_bits());
        let data = match GameData::load() {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Idle Hands data load failed; using safe defaults: {error}");
                GameData::fallback()
            }
        };
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, crate::theme::SURFACE_DARK);
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        let state = AppState::new(&data);
        let mut game = Self {
            analytics: crate::analytics::GameAnalytics::disabled(&state),
            state,
            data,
            assets,
            notifications: NotificationManager::new(),
            pointer: PointerTracker::default(),
            touch_was_active: false,
            capture_solitaire_peek: false,
            sounds: SoundBank::load().await,
            transition: 0.,
            confirmation_bypass: false,
            save_dirty: false,
            save_timer: 0.,
        };
        game.initialize_launch_state();
        game.load_autosave();
        game.refresh_daily_challenge();
        game.analytics = crate::analytics::GameAnalytics::new(
            &game.state,
            !macroquad_toolkit::capture::capture_requested("IDLE_HANDS"),
        );
        game
    }
}

#[cfg(test)]
#[path = "../tests/legacy/game/tests.rs"]
mod tests;
