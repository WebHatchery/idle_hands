//! Responsive tutorial index for replaying and discovering drawer lessons.

use crate::{state::AppState, tutorial_library_data, ui::UiAction};
use macroquad::prelude::*;

pub fn panel(state: &AppState, rect: Rect, fill: Color) {
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

pub fn text(state: &AppState, value: impl AsRef<str>, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(
        value,
        x,
        y,
        crate::accessibility::text_size(size, state.large_text),
        color,
    );
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(filter_rect(), point) {
        return vec![UiAction::ToggleTutorialFilter];
    }
    let (previous, next, back) = control_rects();
    if crate::ui::hit(back, point) {
        return vec![UiAction::Help];
    }
    if crate::ui::hit(previous, point) {
        return vec![UiAction::LibraryScroll(-1)];
    }
    if crate::ui::hit(next, point) {
        return vec![UiAction::LibraryScroll(1)];
    }
    let start = state
        .library_scroll
        .min(tutorial_library_data::scroll_limit(state));
    let rows = tutorial_library_data::rows(state);
    for index in 0..tutorial_library_data::visible_count() {
        let Some(row) = rows.get(start + index) else {
            break;
        };
        if crate::ui::hit(card_rect(index), point) {
            return vec![UiAction::OpenTutorial(row.game.index())];
        }
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

pub fn draw_desktop(state: &AppState) {
    panel(
        state,
        Rect::new(120., 40., 1040., 640.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(state, 170., 105., "Replay every lesson");
    draw_summary(state, 170., 134., 18.);
    draw_filter_button(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn draw_compact(state: &AppState) {
    panel(
        state,
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(state, 40., 47., "Replay every lesson");
    draw_summary(state, 40., 65., 11.);
    draw_filter_button(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn draw_portrait(state: &AppState) {
    panel(
        state,
        Rect::new(8., 16., 344., 688.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(state, 20., 60., "Replay lessons");
    draw_summary(state, 20., 86., 11.);
    draw_filter_button(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn heading(state: &AppState, x: f32, y: f32, subtitle: &str) {
    text(state, "TUTORIALS", x, y, 32., accent(state));
    text(state, subtitle, x, y + 29., 15., secondary(state));
}

pub fn draw_summary(state: &AppState, x: f32, y: f32, size: f32) {
    text(
        state,
        format!(
            "{} seen  ·  {} new  ·  TAP A DRAWER TO REPLAY",
            tutorial_library_data::seen_count(state),
            tutorial_library_data::new_count(state)
        ),
        x,
        y,
        size,
        secondary(state),
    );
}

pub fn draw_cards(state: &AppState) {
    let start = state
        .library_scroll
        .min(tutorial_library_data::scroll_limit(state));
    let rows = tutorial_library_data::rows(state);
    for (index, row) in rows
        .iter()
        .skip(start)
        .take(tutorial_library_data::visible_count())
        .enumerate()
    {
        let rect = card_rect(index);
        panel(
            state,
            rect,
            if row.seen {
                crate::theme::SURFACE_DARK
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        let title_size = if crate::ui::is_portrait() { 12. } else { 14. };
        text(
            state,
            state.game_title(row.game),
            rect.x + 12.,
            rect.y + if crate::ui::is_portrait() { 19. } else { 25. },
            title_size,
            crate::theme::CREAM,
        );
        text(
            state,
            if row.seen {
                "REPLAY LESSON"
            } else {
                "NEW LESSON"
            },
            rect.x + 12.,
            rect.bottom() - 10.,
            if crate::ui::is_portrait() { 9. } else { 10. },
            if row.seen {
                secondary(state)
            } else {
                accent(state)
            },
        );
    }
    if rows.is_empty() {
        text(
            state,
            "No lessons are available.",
            40.,
            180.,
            16.,
            secondary(state),
        );
    }
}

pub fn draw_controls(state: &AppState) {
    let (previous, next, back) = control_rects();
    button(state, previous, "PREV");
    button(state, next, "NEXT");
    button(state, back, "BACK");
    text(
        state,
        tutorial_library_data::page_label(
            state
                .library_scroll
                .min(tutorial_library_data::scroll_limit(state)),
            tutorial_library_data::rows(state).len(),
        ),
        page_label_position(),
        page_label_y(),
        11.,
        secondary(state),
    );
}

pub fn draw_filter_button(state: &AppState) {
    button(
        state,
        filter_rect(),
        if state.tutorial_filter {
            "SHOW ALL"
        } else {
            "NEW ONLY"
        },
    );
}

pub fn button(state: &AppState, rect: Rect, label: &str) {
    panel(state, rect, crate::theme::MOSS_DARK);
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

pub fn card_rect(index: usize) -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(
            40. + (index % 2) as f32 * 390.,
            92. + (index / 2) as f32 * 52.,
            370.,
            44.,
        )
    } else if crate::ui::is_portrait() {
        Rect::new(20., 112. + index as f32 * 52., 320., 44.)
    } else {
        Rect::new(
            170. + (index % 4) as f32 * 235.,
            170. + (index / 4) as f32 * 92.,
            215.,
            76.,
        )
    }
}

pub fn control_rects() -> (Rect, Rect, Rect) {
    if crate::ui::is_compact_landscape() {
        (
            Rect::new(430., 320., 100., 44.),
            Rect::new(545., 320., 100., 44.),
            Rect::new(690., 320., 110., 44.),
        )
    } else if crate::ui::is_portrait() {
        (
            Rect::new(10., 602., 100., 44.),
            Rect::new(250., 602., 100., 44.),
            Rect::new(10., 714., 150., 44.),
        )
    } else {
        (
            Rect::new(650., 590., 110., 48.),
            Rect::new(780., 590., 110., 48.),
            Rect::new(930., 590., 180., 48.),
        )
    }
}

pub fn filter_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(650., 20., 150., 44.)
    } else if crate::ui::is_portrait() {
        Rect::new(190., 42., 150., 44.)
    } else {
        Rect::new(930., 80., 180., 44.)
    }
}

pub fn page_label_position() -> f32 {
    if crate::ui::is_portrait() {
        125.
    } else if crate::ui::is_compact_landscape() {
        255.
    } else {
        490.
    }
}

pub fn page_label_y() -> f32 {
    if crate::ui::is_portrait() {
        630.
    } else if crate::ui::is_compact_landscape() {
        347.
    } else {
        620.
    }
}

pub fn accent(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::BRASS
    }
}

pub fn secondary(state: &AppState) -> Color {
    if state.high_contrast {
        WHITE
    } else {
        crate::theme::SECONDARY
    }
}
