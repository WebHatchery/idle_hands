use super::*;

pub(super) fn draw_home(state: &AppState, _loaded: usize) {
    text("Good evening", 260., 55., 30., crate::theme::INK);
    if let Some(badge) = crate::storefront::build_badge() {
        text(&badge, 1000., 86., 11., crate::theme::BRASS);
    }
    text(
        "Pick a game and unwind.",
        260.,
        78.,
        15.,
        crate::theme::SURFACE,
    );
    small_button(
        Rect::new(1190., 28., 52., 48.),
        "SET",
        crate::theme::SURFACE_DARK,
    );
    small_button(
        Rect::new(1132., 28., 52., 48.),
        "?",
        crate::theme::SURFACE_DARK,
    );
    small_button(
        Rect::new(1030., 28., 92., 48.),
        "FIND",
        crate::theme::SURFACE_DARK,
    );
    panel(CONTINUE, crate::theme::MOSS_DARK, crate::theme::BRASS);
    if state.high_contrast {
        draw_rectangle_lines(CONTINUE.x, CONTINUE.y, CONTINUE.w, CONTINUE.h, 3., WHITE);
    }
    text(
        crate::continue_data::title(state),
        278.,
        116.,
        crate::accessibility::text_size(11., state.large_text),
        crate::theme::BRASS,
    );
    let selected = crate::continue_data::preferred_game(state);
    text(
        state.game_title(selected),
        278.,
        153.,
        crate::accessibility::text_size(27., state.large_text),
        crate::theme::CREAM,
    );
    text(
        state.game_subtitle(selected),
        278.,
        178.,
        13.,
        crate::theme::SECONDARY,
    );
    panel(
        Rect::new(1000., 139., 150., 48.),
        crate::theme::MOSS,
        crate::theme::BRASS,
    );
    text(
        crate::continue_data::action_label(state),
        1018.,
        169.,
        crate::accessibility::text_size(12., state.large_text),
        crate::theme::CREAM,
    );
    text("Browse by category", 260., 240., 15., crate::theme::INK);
    for (index, filter) in cabinet_status::CATEGORY_FILTERS.iter().copied().enumerate() {
        category_card(state, CATEGORY_RECTS[index], filter);
    }
    let recent = recent_games(state);
    if !recent.is_empty() {
        text("Recently played", 260., 560., 14., crate::theme::INK);
        for (index, game) in recent.iter().take(7).copied().enumerate() {
            let rect = recent_rect(index);
            panel(rect, crate::theme::PAPER_LIGHT, crate::theme::BORDER);
            draw_circle(
                rect.x + 25.,
                rect.y + 25.,
                15.,
                crate::theme::category_surface(game, true),
            );
            text(
                &fit_recent_title(state.game_title(game), rect.right() - (rect.x + 48.) - 9.),
                rect.x + 48.,
                rect.y + 24.,
                12.,
                crate::theme::INK,
            );
            text(
                "OPEN",
                rect.x + 48.,
                rect.y + 45.,
                9.,
                crate::theme::SURFACE,
            );
        }
    }
}
