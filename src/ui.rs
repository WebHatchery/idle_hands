//! Touch-first cabinet and 2048 presentation.

use crate::achievements_ui;
use crate::cabinet_ui;
use crate::daily_archive_ui;
use crate::domain::Direction;
use crate::drawer_info_ui;
use crate::favorites_ui;
use crate::finder_ui;
use crate::game_variant_ui;
use crate::library_ui;
use crate::lifecycle_pause_ui;
use crate::mobile_tutorial_ui;
use crate::notice_log_ui;
use crate::profile_ui;
use crate::records_ui;
use crate::responsive_cabinet;
use crate::responsive_landscape;
use crate::responsive_landscape_cabinet;
use crate::responsive_landscape_library;
use crate::responsive_landscape_rules;
use crate::responsive_library;
use crate::responsive_ui;
use crate::save_recovery_ui;
use crate::settings_ui;
use crate::statistics_ui;
use crate::tutorial_library_ui;
use crate::tutorial_ui;
pub use crate::ui_action::UiAction;
use crate::ui_game_routes;
#[path = "ui/game_2048.rs"]
mod game_2048;
#[path = "restart_modal.rs"]
mod restart_modal;
pub(crate) use game_2048::{draw_2048, game_clicks};
#[path = "ui/help.rs"]
mod help;
use crate::{
    data::GameData,
    state::{AppState, Screen},
};
pub(crate) use help::draw_help;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{
    end_frame_neighbours, note_neighbour, touch_area_for_scale, VirtualUi,
};

pub fn draw_rounded_panel(rect: Rect, radius: f32, fill: Color, border: Color) {
    const CORNER_SEGMENTS: usize = 6;

    let radius = radius.clamp(0., rect.w.min(rect.h) * 0.5);
    if radius == 0. {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., border);
        return;
    }

    let mut edge = Vec::with_capacity(CORNER_SEGMENTS * 4 + 1);
    for (center, start) in [
        (vec2(rect.x + radius, rect.y + radius), std::f32::consts::PI),
        (
            vec2(rect.right() - radius, rect.y + radius),
            std::f32::consts::PI * 1.5,
        ),
        (vec2(rect.right() - radius, rect.bottom() - radius), 0.),
        (
            vec2(rect.x + radius, rect.bottom() - radius),
            std::f32::consts::FRAC_PI_2,
        ),
    ] {
        for step in 0..=CORNER_SEGMENTS {
            let angle = start + std::f32::consts::FRAC_PI_2 * step as f32 / CORNER_SEGMENTS as f32;
            edge.push(center + vec2(angle.cos(), angle.sin()) * radius);
        }
    }

    let center = rect.center();
    for index in 0..edge.len() {
        draw_triangle(center, edge[index], edge[(index + 1) % edge.len()], fill);
        draw_line(
            edge[index].x,
            edge[index].y,
            edge[(index + 1) % edge.len()].x,
            edge[(index + 1) % edge.len()].y,
            2.,
            border,
        );
    }
}
use std::cell::Cell;
pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
pub const COMPACT_HEADER_TITLE_X: f32 = 90.0;
pub const COMPACT_HEADER_STATUS_X: f32 = 280.0;
thread_local! {
    static TOUCH_SCALE: Cell<f32> = const { Cell::new(1.0) };
}

#[cfg(test)]
#[path = "../tests/legacy/ui/tests.rs"]
mod tests;
#[derive(Clone, Copy, PartialEq, Eq)]
enum ForcedLayout {
    None,
    Desktop,
    CompactLandscape,
    Portrait,
}
thread_local! {
    static FORCED_LAYOUT: Cell<ForcedLayout> = const { Cell::new(ForcedLayout::None) };
}
pub fn viewport() -> VirtualUi {
    let (width, height) = layout_size();
    VirtualUi::new(width, height)
}
pub fn layout_size() -> (f32, f32) {
    if is_compact_landscape() {
        (responsive_landscape::WIDTH, responsive_landscape::HEIGHT)
    } else if is_portrait() {
        (responsive_ui::WIDTH, responsive_ui::HEIGHT)
    } else {
        (LOGICAL_WIDTH, LOGICAL_HEIGHT)
    }
}
pub fn is_portrait() -> bool {
    match FORCED_LAYOUT.with(Cell::get) {
        ForcedLayout::Desktop | ForcedLayout::CompactLandscape => return false,
        ForcedLayout::Portrait => return true,
        ForcedLayout::None => {}
    }
    display_height() > display_width() * 1.15
}
pub fn is_compact_landscape() -> bool {
    match FORCED_LAYOUT.with(Cell::get) {
        ForcedLayout::Desktop | ForcedLayout::Portrait => return false,
        ForcedLayout::CompactLandscape => return true,
        ForcedLayout::None => {}
    }
    display_width() <= 900. && display_width() > display_height() * 1.15
}

pub fn display_width() -> f32 {
    match FORCED_LAYOUT.with(Cell::get) {
        ForcedLayout::Desktop => return LOGICAL_WIDTH,
        ForcedLayout::CompactLandscape => return crate::responsive_landscape::WIDTH,
        ForcedLayout::Portrait => return crate::responsive_ui::WIDTH,
        ForcedLayout::None => {}
    }
    macroquad::prelude::screen_width()
}

pub fn display_height() -> f32 {
    match FORCED_LAYOUT.with(Cell::get) {
        ForcedLayout::Desktop => return LOGICAL_HEIGHT,
        ForcedLayout::CompactLandscape => return crate::responsive_landscape::HEIGHT,
        ForcedLayout::Portrait => return crate::responsive_ui::HEIGHT,
        ForcedLayout::None => {}
    }
    macroquad::prelude::screen_height()
}

pub(crate) fn with_desktop_layout<T>(run: impl FnOnce() -> T) -> T {
    with_forced_layout(ForcedLayout::Desktop, run)
}

pub(crate) fn with_compact_landscape_layout<T>(run: impl FnOnce() -> T) -> T {
    with_forced_layout(ForcedLayout::CompactLandscape, run)
}

pub(crate) fn with_portrait_layout<T>(run: impl FnOnce() -> T) -> T {
    with_forced_layout(ForcedLayout::Portrait, run)
}

fn with_forced_layout<T>(layout: ForcedLayout, run: impl FnOnce() -> T) -> T {
    FORCED_LAYOUT.with(|forced| {
        let was_forced = forced.replace(layout);
        let result = run();
        forced.set(was_forced);
        result
    })
}
pub fn mouse() -> Vec2 {
    viewport()
        .screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1))
        .unwrap_or(vec2(-1000., -1000.))
}
pub fn physical_touch_rect(rect: Rect, scale: f32) -> Rect {
    touch_area_for_scale(rect, scale)
}
pub fn readable_text_size_for_scale(base: f32, scale: f32) -> f32 {
    let physical_floor = if base < 10. { 9. } else { 11. };
    if scale.is_finite() && scale > 0. {
        base.max(physical_floor / scale)
    } else {
        base
    }
}
pub fn readable_text_size(base: f32) -> f32 {
    TOUCH_SCALE.with(|scale| readable_text_size_for_scale(base, scale.get()))
}
pub fn draw_text(
    value: impl AsRef<str>,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) -> TextDimensions {
    macroquad_toolkit::ui::draw_ui_text(
        value.as_ref(),
        x,
        y,
        readable_text_size(size),
        crate::theme::text_color(color),
    )
}
pub fn measure_text(
    value: impl AsRef<str>,
    font: Option<&Font>,
    size: u16,
    scale: f32,
) -> TextDimensions {
    let readable_size = readable_text_size(size as f32).round() as u16;
    macroquad_toolkit::ui::measure_ui_text(value.as_ref(), font, readable_size, scale)
}
pub fn hit(rect: Rect, point: Vec2) -> bool {
    let scale = TOUCH_SCALE.with(Cell::get);
    let area = physical_touch_rect(rect, scale);
    note_neighbour(rect);
    area.contains(point)
}
/// Resolve a tap at the logical point captured by the pointer tracker.
///
/// Keeping the point supplied by the release event matters when the window is
/// letterboxed or the pointer moves out of the rendered viewport as the mouse
/// button is released. Re-reading the live mouse position can turn an otherwise
/// valid tap into the off-screen sentinel used by `mouse`.
pub fn clicks_at(state: &AppState, point: Vec2) -> Vec<UiAction> {
    TOUCH_SCALE.with(|scale| scale.set(viewport().scale));
    actions_at(state, point)
}

pub fn actions_at(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.lifecycle_paused {
        return lifecycle_pause_ui::clicks(p);
    }
    if state.tutorial.is_some() {
        if is_compact_landscape() {
            return mobile_tutorial_ui::tutorial_clicks(p, true);
        }
        if is_portrait() {
            return mobile_tutorial_ui::tutorial_clicks(p, false);
        }
        return tutorial_ui::clicks(p);
    }
    if state.screen.is_game()
        && ((is_compact_landscape() && mobile_tutorial_ui::replay_clicks(p, true))
            || (is_portrait() && mobile_tutorial_ui::replay_clicks(p, false))
            || (!is_portrait() && hit(tutorial_ui::REPLAY_RECT, p)))
    {
        return vec![UiAction::ReplayTutorial];
    }
    if state.confirm_restart && state.pending_restart.is_some() {
        return restart_modal::clicks(p);
    }
    if state.notice_log_view {
        return notice_log_ui::clicks(p);
    }
    if let Some(action) = notice_log_ui::button_click(state, p) {
        return vec![action];
    }
    if state.save_recovery.is_some() {
        if let Some(action) = save_recovery_ui::clicks(p) {
            return vec![action];
        }
    }
    if state.game_setup_open {
        return game_variant_ui::setup_clicks(state, p)
            .into_iter()
            .collect();
    }
    if crate::dense_focus_ui::is_open(state) {
        return crate::dense_focus_ui::clicks(state, p);
    }
    if let Some(action) = crate::dense_focus_ui::entry_action(state, p) {
        return vec![action];
    }
    if game_variant_ui::clicks(state, p) {
        return vec![UiAction::ToggleGameSetup];
    }
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => responsive_landscape_cabinet::clicks(state, p),
        Screen::Cabinet if is_portrait() => responsive_cabinet::clicks(state, p),
        Screen::Cabinet => cabinet_ui::clicks(state, p),
        Screen::Finder => finder_ui::clicks(state, p),
        Screen::Profile => profile_ui::clicks(state, p),
        Screen::DrawerInfo(_) => drawer_info_ui::clicks(state, p),
        Screen::Game(_) => ui_game_routes::clicks(state, p),
        Screen::Help => {
            if is_compact_landscape() {
                responsive_landscape_library::help_clicks(p)
            } else if is_portrait() {
                responsive_library::help_clicks(p)
            } else if hit(Rect::new(400., 635., 180., 48.), p) {
                vec![UiAction::Tutorials]
            } else if hit(Rect::new(1030., 635., 180., 48.), p) {
                vec![UiAction::Cabinet]
            } else if hit(Rect::new(600., 635., 180., 48.), p) {
                vec![UiAction::Rules]
            } else if hit(Rect::new(800., 635., 180., 48.), p) {
                vec![UiAction::Credits]
            } else {
                vec![]
            }
        }
        Screen::Records if state.achievements_view => achievements_ui::clicks(state, p),
        Screen::Records if state.daily_archive_view => daily_archive_ui::clicks(state, p),
        Screen::Records if state.favorites_view || state.recent_view => {
            favorites_ui::clicks(state, p)
        }
        Screen::Records if is_compact_landscape() => {
            responsive_landscape_library::records_clicks(state, p)
        }
        Screen::Records if is_portrait() => responsive_library::records_clicks(state, p),
        Screen::Records => records_ui::records_clicks(state, p),
        Screen::Statistics => statistics_ui::clicks(state, p),
        Screen::Tutorials => tutorial_library_ui::clicks(state, p),
        Screen::Rules if is_compact_landscape() => {
            responsive_landscape_rules::rules_clicks(state, p)
        }
        Screen::Rules if is_portrait() => responsive_library::rules_clicks(state, p),
        Screen::Rules => library_ui::rules_clicks(state, p),
        Screen::Credits if is_compact_landscape() => {
            responsive_landscape_library::credits_clicks(p)
        }
        Screen::Credits if is_portrait() => responsive_library::credits_clicks(p),
        Screen::Credits => library_ui::credits_clicks(p),
        Screen::Settings if is_compact_landscape() => {
            responsive_landscape_library::settings_clicks(state, p)
        }
        Screen::Settings if is_portrait() => responsive_ui::settings_clicks(state, p),
        Screen::Settings => settings_ui::settings_clicks(state, p),
    }
}
pub fn draw(
    state: &AppState,
    data: &GameData,
    loaded_assets: usize,
    cabinet_texture: Option<&Texture2D>,
    frogger_frog: Option<&Texture2D>,
    frogger_car: Option<&Texture2D>,
    notification_history: &[macroquad_toolkit::notifications::LoggedNotification],
) {
    TOUCH_SCALE.with(|scale| scale.set(viewport().scale));
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => {
            responsive_landscape_cabinet::draw(state, data, loaded_assets, cabinet_texture)
        }
        Screen::Cabinet if is_portrait() => {
            responsive_cabinet::draw(state, data, loaded_assets, cabinet_texture)
        }
        Screen::Cabinet => cabinet_ui::draw(state, loaded_assets, cabinet_texture),
        Screen::Finder => finder_ui::draw(state),
        Screen::Profile => profile_ui::draw(state),
        Screen::DrawerInfo(_) => drawer_info_ui::draw(state),
        Screen::Game(_) => ui_game_routes::draw(state, frogger_frog, frogger_car),
        Screen::Help if is_compact_landscape() => responsive_landscape_library::draw_help(state),
        Screen::Help if is_portrait() => responsive_library::draw_help(state),
        Screen::Help => draw_help(state),
        Screen::Records if state.achievements_view => achievements_ui::draw(state),
        Screen::Records if state.daily_archive_view => daily_archive_ui::draw(state),
        Screen::Records if state.favorites_view || state.recent_view => favorites_ui::draw(state),
        Screen::Records if is_compact_landscape() => {
            responsive_landscape_library::draw_records(state)
        }
        Screen::Records if is_portrait() => responsive_library::draw_records(state),
        Screen::Records => records_ui::draw_records(state),
        Screen::Statistics => statistics_ui::draw(state),
        Screen::Tutorials => tutorial_library_ui::draw(state),
        Screen::Rules if is_compact_landscape() => responsive_landscape_rules::draw_rules(state),
        Screen::Rules if is_portrait() => responsive_library::draw_rules(state),
        Screen::Rules => library_ui::draw_rules(state),
        Screen::Credits if is_compact_landscape() => {
            responsive_landscape_library::draw_credits(state)
        }
        Screen::Credits if is_portrait() => responsive_library::draw_credits(state),
        Screen::Credits => library_ui::draw_credits(state),
        Screen::Settings if is_compact_landscape() => {
            responsive_landscape_library::draw_settings(state)
        }
        Screen::Settings if is_portrait() => responsive_ui::draw_settings(state),
        Screen::Settings => settings_ui::draw_settings(state),
    }
    if let Some(game) = state.tutorial {
        if is_compact_landscape() {
            mobile_tutorial_ui::draw_tutorial(
                game,
                &state.content,
                true,
                state.large_text,
                state.high_contrast,
            );
        } else if is_portrait() {
            mobile_tutorial_ui::draw_tutorial(
                game,
                &state.content,
                false,
                state.large_text,
                state.high_contrast,
            );
        } else {
            tutorial_ui::draw_overlay(game, &state.content, state.large_text, state.high_contrast);
        }
    } else if state.screen.is_game() {
        if is_compact_landscape() {
            mobile_tutorial_ui::draw_replay_button(true, state.large_text, state.high_contrast);
        } else if is_portrait() {
            mobile_tutorial_ui::draw_replay_button(false, state.large_text, state.high_contrast);
        } else {
            tutorial_ui::draw_replay_button(state.large_text, state.high_contrast);
        }
    }
    if state.confirm_restart && state.pending_restart.is_some() {
        restart_modal::draw(state);
    }
    notice_log_ui::draw_button(state);
    notice_log_ui::draw_log(state, notification_history);
    save_recovery_ui::draw(state);
    if state.lifecycle_paused {
        lifecycle_pause_ui::draw(state);
    }
    // Prime neighbour-aware target growth from the visible action map. Input
    // is handled before drawing, so the next frame can expand small controls
    // without allowing adjacent targets to claim the same physical point.
    let _ = actions_at(state, vec2(-10_000., -10_000.));
    end_frame_neighbours();
}
fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(s, x, y, crate::ui::readable_text_size(size), color);
}
fn panel(r: Rect, fill: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, crate::theme::drawer_surface(fill));
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., crate::theme::BORDER)
}
