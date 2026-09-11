//! Touch-first collection statistics shelf.

use crate::{state::AppState, stats_data, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

fn panel(state: &AppState, rect: Rect, fill: Color) {
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

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    crate::ui::hit(back_rect(), point)
        .then_some(vec![UiAction::Records])
        .unwrap_or_default()
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
    let summary = stats_data::from_state(state);
    panel(
        state,
        Rect::new(120., 40., 1040., 640.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(
        state,
        "STATISTICS",
        "A quiet ledger of how the cabinet is played",
        170.,
        105.,
        170.,
        134.,
    );
    let cards = [
        (
            "PLAYTIME",
            crate::state_records::format_duration(summary.total_playtime_seconds),
        ),
        (
            "DRAWERS PLAYED",
            format!("{}/{}", summary.played_games, summary.total_games),
        ),
        (
            "DRAWERS CLEARED",
            format!("{}/{}", summary.completed_games, summary.total_games),
        ),
        (
            "DAILY CLEARS",
            format!("{}/{}", summary.daily_clears, summary.daily_attempts),
        ),
    ];
    for (index, (label, value)) in cards.iter().enumerate() {
        metric_card(
            state,
            Rect::new(170. + index as f32 * 235., 170., 215., 72.),
            label,
            value,
        );
    }
    panel(
        state,
        Rect::new(170., 270., 490., 250.),
        crate::theme::SURFACE_DARK,
    );
    text(state, "COLLECTION RHYTHM", 195., 305., 17., accent(state));
    text(
        state,
        format!("Completion {}%", stats_data::completion_percent(summary)),
        195.,
        345.,
        25.,
        WHITE,
    );
    text(
        state,
        format!("{} favorites", summary.favorite_games),
        195.,
        390.,
        18.,
        crate::theme::CREAM,
    );
    text(
        state,
        format!("{} recent drawers", summary.recent_games),
        195.,
        420.,
        18.,
        crate::theme::CREAM,
    );
    text(
        state,
        format!("{} daily routes attempted", summary.daily_attempts),
        195.,
        450.,
        18.,
        crate::theme::CREAM,
    );
    draw_ledger(state, summary, Rect::new(690., 270., 440., 110.), 16.);
    draw_top_playtime(state, Rect::new(690., 400., 440., 120.), 3, 14.);
    back_button(state, back_rect());
}

fn draw_compact(state: &AppState) {
    let summary = stats_data::from_state(state);
    panel(
        state,
        Rect::new(20., 12., 804., 365.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(state, "STATISTICS", "Collection rhythm", 40., 47., 40., 63.);
    let cards = [
        (
            "TIME",
            crate::state_records::format_duration(summary.total_playtime_seconds),
        ),
        (
            "PLAYED",
            format!("{}/{}", summary.played_games, summary.total_games),
        ),
        (
            "CLEARED",
            format!("{}/{}", summary.completed_games, summary.total_games),
        ),
        (
            "DAILY",
            format!("{}/{}", summary.daily_clears, summary.daily_attempts),
        ),
    ];
    for (index, (label, value)) in cards.iter().enumerate() {
        metric_card(
            state,
            Rect::new(40. + index as f32 * 190., 78., 175., 48.),
            label,
            value,
        );
    }
    panel(
        state,
        Rect::new(40., 145., 360., 145.),
        crate::theme::SURFACE_DARK,
    );
    text(state, "COLLECTION", 58., 174., 14., accent(state));
    text(
        state,
        format!("{}% complete", stats_data::completion_percent(summary)),
        58.,
        204.,
        17.,
        WHITE,
    );
    text(
        state,
        format!(
            "{} favorites  ·  {} recent",
            summary.favorite_games, summary.recent_games
        ),
        58.,
        232.,
        13.,
        crate::theme::CREAM,
    );
    text(
        state,
        format!("{} daily routes attempted", summary.daily_attempts),
        58.,
        258.,
        13.,
        crate::theme::CREAM,
    );
    draw_ledger(state, summary, Rect::new(420., 145., 360., 145.), 14.);
    back_button(state, back_rect());
}

fn draw_portrait(state: &AppState) {
    let summary = stats_data::from_state(state);
    panel(
        state,
        Rect::new(8., 12., 344., 700.),
        crate::theme::BACKGROUND_DEEP,
    );
    heading(state, "STATISTICS", "Collection rhythm", 20., 58., 20., 77.);
    let cards = [
        (
            "PLAYTIME",
            crate::state_records::format_duration(summary.total_playtime_seconds),
        ),
        (
            "PLAYED",
            format!("{}/{}", summary.played_games, summary.total_games),
        ),
        (
            "CLEARED",
            format!("{}/{}", summary.completed_games, summary.total_games),
        ),
        (
            "DAILY",
            format!("{}/{}", summary.daily_clears, summary.daily_attempts),
        ),
    ];
    for (index, (label, value)) in cards.iter().enumerate() {
        let x = 20. + (index % 2) as f32 * 165.;
        let y = 98. + (index / 2) as f32 * 66.;
        metric_card(state, Rect::new(x, y, 155., 56.), label, value);
    }
    panel(
        state,
        Rect::new(20., 240., 320., 105.),
        crate::theme::SURFACE_DARK,
    );
    text(state, "COLLECTION", 34., 268., 14., accent(state));
    text(
        state,
        format!("{}% complete", stats_data::completion_percent(summary)),
        34.,
        296.,
        18.,
        WHITE,
    );
    text(
        state,
        format!(
            "{} favorites  ·  {} recent",
            summary.favorite_games, summary.recent_games
        ),
        34.,
        323.,
        12.,
        crate::theme::CREAM,
    );
    draw_ledger(state, summary, Rect::new(20., 360., 320., 118.), 12.);
    draw_top_playtime(state, Rect::new(20., 493., 320., 190.), 5, 12.);
    back_button(state, back_rect());
}

fn heading(
    state: &AppState,
    title: &str,
    subtitle: &str,
    title_x: f32,
    title_y: f32,
    subtitle_x: f32,
    subtitle_y: f32,
) {
    text(state, title, title_x, title_y, 32., accent(state));
    text(
        state,
        subtitle,
        subtitle_x,
        subtitle_y,
        15.,
        secondary(state),
    );
}

fn metric_card(state: &AppState, rect: Rect, label: &str, value: &str) {
    panel(state, rect, crate::theme::SURFACE_DARK);
    text(
        state,
        label,
        rect.x + 12.,
        rect.y + 22.,
        11.,
        secondary(state),
    );
    text(state, value, rect.x + 12., rect.y + 51., 20., WHITE);
}

fn draw_ledger(state: &AppState, summary: stats_data::StatisticsSummary, rect: Rect, size: f32) {
    panel(state, rect, crate::theme::SURFACE_DARK);
    text(
        state,
        "TIME LEDGER",
        rect.x + 14.,
        rect.y + 26.,
        size,
        accent(state),
    );
    let fastest = summary.fastest_clear.map_or_else(
        || "—".to_owned(),
        |row| {
            format!(
                "{} {}",
                stats_data::short_title(row.game, if rect.w < 330. { 18 } else { 28 }),
                duration(row.seconds)
            )
        },
    );
    let longest = summary.longest_session.map_or_else(
        || "—".to_owned(),
        |row| {
            format!(
                "{} {}",
                stats_data::short_title(row.game, if rect.w < 330. { 18 } else { 28 }),
                duration(row.seconds)
            )
        },
    );
    text(
        state,
        "FASTEST CLEAR",
        rect.x + 14.,
        rect.y + 52.,
        10.,
        secondary(state),
    );
    text(state, fastest, rect.x + 14., rect.y + 69., 12., WHITE);
    text(
        state,
        "LONGEST SESSION",
        rect.x + 14.,
        rect.y + 89.,
        10.,
        secondary(state),
    );
    text(state, longest, rect.x + 14., rect.y + 106., 12., WHITE);
}

fn draw_top_playtime(state: &AppState, rect: Rect, limit: usize, size: f32) {
    panel(state, rect, crate::theme::SURFACE_DARK);
    text(
        state,
        "TOP DRAWERS BY TIME",
        rect.x + 14.,
        rect.y + 26.,
        size,
        accent(state),
    );
    let rows = stats_data::top_playtime(state, limit);
    let row_spacing = if rect.h < 180. { 22. } else { 30. };
    let row_size = if rect.h < 180. { 10. } else { 12. };
    for (index, row) in rows.iter().enumerate() {
        let y = rect.y + 54. + index as f32 * row_spacing;
        text(
            state,
            format!(
                "{}. {}",
                index + 1,
                stats_data::short_title(row.game, if rect.w < 330. { 17 } else { 24 })
            ),
            rect.x + 14.,
            y,
            row_size,
            crate::theme::CREAM,
        );
        text(
            state,
            duration(row.seconds),
            rect.right() - 76.,
            y,
            row_size,
            accent(state),
        );
    }
    if rows.is_empty() {
        text(
            state,
            "No playtime recorded yet.",
            rect.x + 14.,
            rect.y + 58.,
            12.,
            secondary(state),
        );
    }
}

fn duration(seconds: u32) -> String {
    crate::state_records::format_duration(u64::from(seconds))
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

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(690., 320., 110., 44.)
    } else if crate::ui::is_portrait() {
        Rect::new(10., 714., 150., 44.)
    } else {
        Rect::new(930., 590., 180., 48.)
    }
}

fn back_button(state: &AppState, rect: Rect) {
    panel(state, rect, crate::theme::MOSS_DARK);
    text(
        state,
        "BACK",
        rect.x + (rect.w - 40.) * 0.5,
        rect.y + rect.h * 0.64,
        16.,
        WHITE,
    );
}
