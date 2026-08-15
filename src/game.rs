//! Application lifecycle and input routing.

use crate::ui;
use crate::{
    data::GameData,
    state::{AppState, Direction, GameId, Screen},
};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};

pub struct Game {
    pub data: GameData,
    pub state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    drag_start: Option<Vec2>,
}
impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Idle Hands embedded data failed to load");
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.25, 0.18, 0.35, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        Self {
            data,
            state: AppState::default(),
            assets,
            notifications: NotificationManager::new(),
            drag_start: None,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(ui::mouse());
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = self.drag_start.take() {
                let end = ui::mouse();
                if self.state.screen == Screen::Game(GameId::Game2048)
                    && (end - start).length() > 32.0
                {
                    self.try_move(swipe_direction(end - start));
                } else {
                    for action in ui::clicks(&self.state) {
                        self.apply(action);
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state.screen = Screen::Cabinet;
            self.state.confirm_restart = false;
        }
        if self.state.screen == Screen::Game(GameId::Game2048) {
            for (key, direction) in [
                (KeyCode::Up, Direction::Up),
                (KeyCode::Right, Direction::Right),
                (KeyCode::Down, Direction::Down),
                (KeyCode::Left, Direction::Left),
            ] {
                if is_key_pressed(key) {
                    self.try_move(direction);
                }
            }
        }
        let _ = dt;
    }
    pub fn draw(&mut self) {
        clear_background(Color::new(0.035, 0.028, 0.055, 1.0));
        ui::draw(&self.state, &self.data, self.assets.len());
        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }
    fn apply(&mut self, action: ui::UiAction) {
        match action {
            ui::UiAction::Open(index) => {
                self.state.selected = index;
                let id = GameId::ALL[index];
                if id == GameId::Game2048 {
                    self.state.screen = Screen::Game(id);
                } else {
                    self.notifications
                        .info(format!("{} is coming soon", id.title()));
                }
            }
            ui::UiAction::Cabinet => self.state.screen = Screen::Cabinet,
            ui::UiAction::Help => self.state.screen = Screen::Help,
            ui::UiAction::Settings => self.state.screen = Screen::Settings,
            ui::UiAction::Move(direction) => self.try_move(direction),
            ui::UiAction::Undo => {
                if self.state.game.undo() {
                    self.notifications.info("One move undone");
                }
            }
            ui::UiAction::Restart => self.state.confirm_restart = true,
            ui::UiAction::ConfirmRestart => {
                self.state.game = crate::state::Game2048::new(self.state.game.seed.wrapping_add(1));
                self.state.confirm_restart = false;
            }
            ui::UiAction::Cancel => self.state.confirm_restart = false,
            ui::UiAction::ToggleSound => self.state.sound = !self.state.sound,
            ui::UiAction::ToggleMotion => self.state.reduced_motion = !self.state.reduced_motion,
        }
    }
    fn try_move(&mut self, direction: Direction) {
        if self.state.game.move_in(direction) && self.state.game.won() {
            self.notifications
                .success("2048 reached — keep playing or start a fresh board");
        }
    }
}
fn swipe_direction(delta: Vec2) -> Direction {
    if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if delta.y > 0.0 {
        Direction::Down
    } else {
        Direction::Up
    }
}
