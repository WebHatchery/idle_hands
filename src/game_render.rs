//! Host-level frame rendering for the game shell.

use super::Game;
use crate::{cosmetics, ui};
use macroquad::prelude::*;
use macroquad_toolkit::notifications::{NotificationAnchor, NotificationRenderConfig};

#[cfg(test)]
#[path = "../tests/legacy/game_render/tests.rs"]
mod tests;

impl Game {
    pub fn draw(&mut self) {
        clear_background(cosmetics::background(self.state.board_theme));
        let viewport = ui::viewport();
        let (layout_width, layout_height) = ui::layout_size();
        viewport.begin();
        ui::draw(
            &self.state,
            &self.data,
            self.assets.len(),
            self.assets.get_texture("cabinet_texture"),
            self.assets.get_texture("frogger_frog"),
            self.assets.get_texture("frogger_car"),
            self.notifications.history(),
        );
        crate::game_variant_ui::draw(&self.state);
        if self.transition > 0. {
            draw_rectangle(
                0.,
                0.,
                layout_width,
                layout_height,
                Color::new(0.02, 0.015, 0.035, self.transition),
            );
        }
        set_default_camera();
        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }
}
