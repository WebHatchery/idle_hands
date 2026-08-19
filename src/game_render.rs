//! Host-level frame rendering for the game shell.

use super::Game;
use crate::{cosmetics, ui};
use macroquad::prelude::*;
use macroquad_toolkit::notifications::{NotificationAnchor, NotificationRenderConfig};

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
        );
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
        ) {
            return;
        }
        let (width, _) = ui::layout_size();
        let rect = if ui::is_portrait() {
            Rect::new(width - 158., 8., 150., 34.)
        } else if ui::is_compact_landscape() {
            Rect::new(width - 172., 5., 164., 34.)
        } else {
            Rect::new(width - 190., 16., 178., 38.)
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE_DARK);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., crate::theme::BRASS);
        let current = self.state.records.current_time(game.index());
        let best = self
            .state
            .records
            .best_time(game.index())
            .map_or("—".to_owned(), format_clock);
        let label = format!("TIME {}  •  BEST {}", format_clock(current), best);
        crate::ui::draw_text(
            label,
            rect.x + 8.,
            rect.y + rect.h * 0.66,
            if ui::is_portrait() { 9. } else { 11. },
            crate::theme::CREAM,
        );
    }
}

use crate::state::Screen;

fn format_clock(seconds: u32) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}
