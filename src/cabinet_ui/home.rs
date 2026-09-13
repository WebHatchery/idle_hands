use super::*;

pub(super) fn draw_home(state: &AppState, loaded: usize) {
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
        Rect::new(480., 154., 126., 40.),
        crate::theme::MOSS,
        crate::theme::BRASS,
    );
    text(
        crate::continue_data::action_label(state),
        495.,
        179.,
        crate::accessibility::text_size(12., state.large_text),
        crate::theme::CREAM,
    );
    stat_card(
        Rect::new(645., 92., 145., 118.),
        "FAVORITES",
        crate::cabinet_data::favorite_count(state),
        "games",
    );
    stat_card(
        Rect::new(805., 92., 145., 118.),
        "RECENT",
        state.recent_games.len(),
        "games",
    );
    panel(
        Rect::new(965., 92., 207., 118.),
        crate::theme::PAPER_LIGHT,
        crate::theme::BORDER,
    );
    text(
        "DAILY CHALLENGE",
        982.,
        122.,
        12.,
        crate::theme::SURFACE_DARK,
    );
    text(
        &crate::daily_challenge::status_label(
            state.games.daily_dungeon.day_key,
            state.games.daily_dungeon.challenge,
            state.games.daily_dungeon.phase,
        ),
        982.,
        154.,
        11.,
        crate::theme::INK,
    );
    text(
        &crate::daily_challenge::preview_action(
            state.games.daily_dungeon.phase,
            state.records.daily_score(state.games.daily_dungeon.day_key),
        ),
        982.,
        178.,
        14.,
        crate::theme::SURFACE,
    );
    text("Your collection", 260., 240., 15., crate::theme::INK);
    for (index, filter) in cabinet_status::CATEGORY_FILTERS.iter().copied().enumerate() {
        category_card(state, CATEGORY_RECTS[index], filter);
    }
    text("Recently played", 260., 560., 14., crate::theme::INK);
    let recent = recent_games(state);
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
            "PLAY  >",
            rect.x + 48.,
            rect.y + 45.,
            9.,
            crate::theme::SURFACE,
        );
    }
    let summary = crate::collection_summary::from_state(state);
    text(
        &format!(
            "{} stamps  ·  {}  ·  {} ({}%)  ·  {} textures",
            summary.stamps,
            summary.achievements_label(),
            summary.drawers_label(),
            summary.completion_percent(),
            loaded
        ),
        930.,
        694.,
        10.,
        crate::theme::SURFACE,
    );
}
