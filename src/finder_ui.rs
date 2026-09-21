//! Responsive touch-first drawer finder.

use crate::{finder_data, state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    for (index, rect) in filter_rects().iter().copied().enumerate() {
        if crate::ui::hit(rect, point) {
            return vec![UiAction::FinderFilter(index as u8)];
        }
    }
    let (previous, next, back) = control_rects();
    if crate::ui::hit(back, point) {
        return vec![UiAction::Cabinet];
    }
    let start = state.library_scroll.min(finder_data::scroll_limit(state));
    if crate::ui::hit(previous, point) && start > 0 {
        return vec![UiAction::LibraryScroll(
            -(finder_data::visible_count() as i8),
        )];
    }
    if crate::ui::hit(next, point) && start < finder_data::scroll_limit(state) {
        return vec![UiAction::LibraryScroll(finder_data::visible_count() as i8)];
    }
    for (index, game) in finder_data::page(state).into_iter().enumerate() {
        if crate::ui::hit(info_rect(index), point) {
            return vec![UiAction::Inspect(game.index())];
        }
        if crate::ui::hit(card_rect(index), point) {
            return vec![UiAction::Open(game.index())];
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
    panel(state, Rect::new(120., 30., 1040., 650.));
    heading(state, 170., 90., 32., "FIND A DRAWER");
    text(
        state,
        "Browse every game by name",
        170.,
        115.,
        15.,
        secondary(state),
    );
    draw_filters(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn draw_compact(state: &AppState) {
    panel(state, Rect::new(20., 12., 804., 365.));
    heading(state, 40., 47., 22., "FIND A DRAWER");
    text(
        state,
        "Browse by name · TAP A DRAWER TO OPEN",
        40.,
        67.,
        10.,
        secondary(state),
    );
    draw_filters(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn draw_portrait(state: &AppState) {
    panel(state, Rect::new(8., 16., 344., 688.));
    heading(state, 20., 60., 25., "FIND A DRAWER");
    text(state, "Browse by name", 20., 85., 11., secondary(state));
    draw_filters(state);
    draw_cards(state);
    draw_controls(state);
}

pub fn heading(state: &AppState, x: f32, y: f32, size: f32, value: &str) {
    text(state, value, x, y, size, accent(state));
}

pub fn draw_filters(state: &AppState) {
    for (index, rect) in filter_rects().iter().copied().enumerate() {
        let selected = state.cabinet_filter == index as u8;
        button(
            state,
            rect,
            finder_data::filter_label(index as u8),
            if selected {
                crate::theme::MOSS_DARK
            } else {
                crate::theme::SURFACE_DARK
            },
        );
    }
}

pub fn draw_cards(state: &AppState) {
    for (index, game) in finder_data::page(state).into_iter().enumerate() {
        let rect = card_rect(index);
        let available = crate::storefront::availability(game).is_playable();
        panel_fill(state, rect, card_fill(state, game, available));
        let title_size = if crate::ui::is_portrait() { 11. } else { 14. };
        text(
            state,
            fit_title(
                state.game_title(game),
                rect.w - 24.,
                title_size,
                state.large_text,
            ),
            rect.x + 12.,
            rect.y + if crate::ui::is_portrait() { 19. } else { 27. },
            title_size,
            crate::theme::CREAM,
        );
        let status = finder_data::status_label(state, game, crate::ui::is_portrait());
        text(
            state,
            status,
            rect.x + 12.,
            rect.bottom() - if crate::ui::is_portrait() { 8. } else { 11. },
            if crate::ui::is_portrait() { 8. } else { 10. },
            if available {
                crate::theme::SECONDARY
            } else {
                accent(state)
            },
        );
        text(
            state,
            "INFO",
            rect.right() - 42.,
            rect.y + 25.,
            8.,
            accent(state),
        );
    }
    if finder_data::page(state).is_empty() {
        text(
            state,
            "No drawers match this shelf.",
            empty_message_position().0,
            empty_message_position().1,
            15.,
            secondary(state),
        );
    }
}

pub fn draw_controls(state: &AppState) {
    let (previous, next, back) = control_rects();
    button(state, previous, "PREV", crate::theme::SURFACE_DARK);
    button(state, next, "NEXT", crate::theme::SURFACE_DARK);
    button(state, back, "BACK", crate::theme::MOSS_DARK);
    let label = finder_data::page_label(state);
    let (x, y) = page_label_position();
    text(state, label, x, y, 11., secondary(state));
}

pub fn button(state: &AppState, rect: Rect, label: &str, fill: Color) {
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

pub fn panel(state: &AppState, rect: Rect) {
    panel_fill(state, rect, crate::theme::BACKGROUND_DEEP);
}

pub fn panel_fill(state: &AppState, rect: Rect, fill: Color) {
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

pub fn card_rect(index: usize) -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(
            40. + (index % 2) as f32 * 390.,
            123. + (index / 2) as f32 * 48.,
            370.,
            40.,
        )
    } else if crate::ui::is_portrait() {
        Rect::new(20., 205. + index as f32 * 44., 320., 40.)
    } else {
        Rect::new(
            170. + (index % 4) as f32 * 235.,
            175. + (index / 4) as f32 * 100.,
            215.,
            82.,
        )
    }
}

pub fn info_rect(index: usize) -> Rect {
    let rect = card_rect(index);
    Rect::new(rect.right() - 48., rect.y, 48., rect.h)
}

pub fn filter_rects() -> [Rect; 5] {
    if crate::ui::is_portrait() {
        [
            Rect::new(20., 100., 96., 38.),
            Rect::new(120., 100., 96., 38.),
            Rect::new(220., 100., 96., 38.),
            Rect::new(20., 144., 96., 38.),
            Rect::new(120., 144., 96., 38.),
        ]
    } else if crate::ui::is_compact_landscape() {
        std::array::from_fn(|index| Rect::new(40. + index as f32 * 145., 80., 135., 36.))
    } else {
        std::array::from_fn(|index| Rect::new(170. + index as f32 * 170., 130., 155., 40.))
    }
}

pub fn control_rects() -> (Rect, Rect, Rect) {
    if crate::ui::is_portrait() {
        (
            Rect::new(10., 602., 100., 44.),
            Rect::new(250., 602., 100., 44.),
            Rect::new(10., 658., 150., 44.),
        )
    } else if crate::ui::is_compact_landscape() {
        (
            Rect::new(430., 320., 100., 44.),
            Rect::new(545., 320., 100., 44.),
            Rect::new(690., 320., 110., 44.),
        )
    } else {
        (
            Rect::new(650., 590., 110., 48.),
            Rect::new(780., 590., 110., 48.),
            Rect::new(930., 590., 180., 48.),
        )
    }
}

pub fn page_label_position() -> (f32, f32) {
    if crate::ui::is_portrait() {
        (125., 630.)
    } else if crate::ui::is_compact_landscape() {
        (255., 347.)
    } else {
        (490., 620.)
    }
}

pub fn empty_message_position() -> (f32, f32) {
    if crate::ui::is_portrait() {
        (20., 220.)
    } else if crate::ui::is_compact_landscape() {
        (40., 145.)
    } else {
        (170., 205.)
    }
}

pub fn fit_title(title: &str, max_width: f32, size: f32, large_text: bool) -> String {
    let measure_size = if large_text { size * 1.18 } else { size };
    if crate::ui::measure_text(title, None, measure_size as u16, 1.).width <= max_width {
        return title.to_owned();
    }
    let mut prefix = title.to_owned();
    while !prefix.is_empty() {
        prefix.pop();
        let candidate = format!("{prefix}…");
        if crate::ui::measure_text(&candidate, None, measure_size as u16, 1.).width <= max_width {
            return candidate;
        }
    }
    "…".to_owned()
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

pub fn card_fill(state: &AppState, game: crate::state::GameId, available: bool) -> Color {
    if state.high_contrast || !available {
        crate::theme::SURFACE_DARK
    } else {
        crate::theme::category_surface(game, true)
    }
}
