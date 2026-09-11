//! Host-level frame rendering for the game shell.

use super::Game;
use crate::{cosmetics, ui};
use macroquad::prelude::*;
use macroquad_toolkit::notifications::{NotificationAnchor, NotificationRenderConfig};

#[cfg(test)]
#[path = "game_render/tests.rs"]
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
        self.draw_time_badge();
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

    fn draw_time_badge(&self) {
        let Screen::Game(game) = self.state.screen else {
            return;
        };
        if matches!(
            game,
            crate::state::GameId::Snake
                | crate::state::GameId::Breakout
                | crate::state::GameId::TinyTowerDefence
                | crate::state::GameId::SpaceInvaders
                | crate::state::GameId::Asteroids
                | crate::state::GameId::Frogger
                | crate::state::GameId::MunchMaze
                | crate::state::GameId::BlockStack
                | crate::state::GameId::TerrainCannon
                | crate::state::GameId::FlingFury
                | crate::state::GameId::PaddleDuel
        ) {
            return;
        }
        if !time_badge_is_visible() {
            return;
        }
        let rect = time_badge_rect();
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE_DARK);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., crate::theme::BRASS);
        let current = self.state.records.current_time(game.index());
        let best = self
            .state
            .records
            .best_time(game.index())
            .map_or("—".to_owned(), |seconds| {
                crate::state_records::format_duration(u64::from(seconds))
            });
        let label = format!(
            "TIME {}  •  BEST {}",
            crate::state_records::format_duration(u64::from(current)),
            best
        );
        crate::ui::draw_text(
            label,
            rect.x + 8.,
            rect.y + rect.h * 0.66,
            if ui::is_portrait() { 9. } else { 11. },
            crate::theme::CREAM,
        );
    }
}

fn time_badge_is_visible() -> bool {
    !ui::is_compact_landscape()
}

fn time_badge_rect() -> Rect {
    if ui::is_portrait() {
        Rect::new(95., 8., 140., 34.)
    } else {
        Rect::new(700., 16., 178., 38.)
    }
}

use crate::state::Screen;
