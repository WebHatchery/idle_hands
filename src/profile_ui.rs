//! Responsive touch-first profile identity editor.

use crate::{profile_data, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
#[path = "../tests/legacy/profile_ui/tests.rs"]
mod tests;

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    for (index, rect) in name_rects().iter().copied().enumerate() {
        if crate::ui::hit(rect, point) {
            return vec![UiAction::SetProfileName(index as u8)];
        }
    }
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Settings];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    if crate::ui::is_compact_landscape() {
        draw_compact(state);
    } else if crate::ui::is_portrait() {
        draw_portrait(state);
    } else {
        draw_desktop(state);
    }
}

fn draw_desktop(state: &AppState) {
    panel(state, Rect::new(180., 40., 920., 640.));
    heading(state, 230., 100., 38., "PROFILE");
    current_copy(state, 230., 132., 17.);
    text(
        state,
        "TAP A NAME TO USE IT",
        230.,
        164.,
        14.,
        secondary(state),
    );
    draw_names(state);
    button(state, back_rect(), "BACK", crate::theme::MOSS_DARK);
}

fn draw_compact(state: &AppState) {
    panel(state, Rect::new(20., 12., 804., 365.));
    heading(state, 40., 47., 25., "PROFILE");
    current_copy(state, 40., 72., 12.);
    text(
        state,
        "TAP A NAME TO USE IT",
        40.,
        92.,
        10.,
        secondary(state),
    );
    draw_names(state);
    button(state, back_rect(), "BACK", crate::theme::MOSS_DARK);
}

fn draw_portrait(state: &AppState) {
    panel(state, Rect::new(8., 16., 344., 688.));
    heading(state, 20., 60., 28., "PROFILE");
    current_copy(state, 20., 87., 12.);
    text(
        state,
        "TAP A NAME TO USE IT",
        20.,
        106.,
        10.,
        secondary(state),
    );
    draw_names(state);
    button(state, back_rect(), "BACK", crate::theme::MOSS_DARK);
}

fn heading(state: &AppState, x: f32, y: f32, size: f32, value: &str) {
    text(state, value, x, y, size, accent(state));
}

fn current_copy(state: &AppState, x: f32, y: f32, size: f32) {
    text(
        state,
        format!(
            "CURRENT NAME: {}",
            profile_data::display_name(&state.profile_name).to_uppercase()
        ),
        x,
        y,
        size,
        crate::theme::CREAM,
    );
}

fn draw_names(state: &AppState) {
    let selected = profile_data::current_index(state);
    for (index, rect) in name_rects().iter().copied().enumerate() {
        let fill = profile_card_fill(state, selected == Some(index));
        panel_fill(state, rect, fill);
        text(
            state,
            profile_data::name(&state.content, index as u8),
            rect.x + 14.,
            rect.y + rect.h * 0.62,
            if crate::ui::is_portrait() { 11. } else { 14. },
            crate::theme::CREAM,
        );
        if selected == Some(index) {
            text(
                state,
                "CURRENT",
                rect.right() - 70.,
                rect.y + 17.,
                8.,
                accent(state),
            );
        }
    }
}

fn button(state: &AppState, rect: Rect, label: &str, fill: Color) {
    panel_fill(state, rect, fill);
    let width = crate::ui::measure_text(label, None, 12, 1.).width;
    text(
        state,
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + rect.h * 0.64,
        12.,
        WHITE,
    );
}

fn panel(state: &AppState, rect: Rect) {
    panel_fill(state, rect, crate::theme::BACKGROUND_DEEP);
}

fn panel_fill(state: &AppState, rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BORDER
        },
    );
}

fn text(state: &AppState, value: impl AsRef<str>, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(
        value,
        x,
        y,
        crate::accessibility::text_size(size, state.large_text),
        color,
    );
}

fn name_rects() -> [Rect; 8] {
    if crate::ui::is_portrait() {
        std::array::from_fn(|index| Rect::new(20., 125. + index as f32 * 50., 320., 42.))
    } else if crate::ui::is_compact_landscape() {
        std::array::from_fn(|index| {
            Rect::new(
                40. + (index % 2) as f32 * 390.,
                110. + (index / 2) as f32 * 48.,
                370.,
                40.,
            )
        })
    } else {
        std::array::from_fn(|index| {
            Rect::new(
                230. + (index % 4) as f32 * 195.,
                205. + (index / 4) as f32 * 105.,
                175.,
                84.,
            )
        })
    }
}

fn back_rect() -> Rect {
    if crate::ui::is_portrait() {
        Rect::new(10., 602., 150., 44.)
    } else if crate::ui::is_compact_landscape() {
        Rect::new(690., 320., 110., 44.)
    } else {
        Rect::new(230., 550., 170., 50.)
    }
}

fn accent(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::BRASS
    }
}

fn secondary(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::SECONDARY
    }
}

fn profile_card_fill(state: &AppState, selected: bool) -> Color {
    if state.high_contrast {
        if selected {
            Color::new(0.30, 0.30, 0.30, 1.)
        } else {
            Color::new(0.12, 0.12, 0.12, 1.)
        }
    } else if selected {
        crate::theme::MOSS_DARK
    } else {
        crate::theme::SURFACE_DARK
    }
}
