use super::*;

pub(super) fn draw_library(state: &AppState) {
    let title = cabinet_status::category_name(state.cabinet_filter);
    let availability = cabinet_status::availability_counts(state, state.cabinet_filter);
    let progress = cabinet_status::category_progress(state, state.cabinet_filter);
    text("<  COLLECTION", 260., 48., 12., crate::theme::SURFACE);
    text(title, 260., 82., 31., crate::theme::INK);
    text(
        &if crate::game_descriptor::is_demo_build() {
            format!(
                "{} · {}/{} done",
                availability.label(false),
                progress.completed,
                progress.total
            )
        } else {
            format!(
                "{} quiet games · {}/{} done",
                crate::cabinet_data::visible_games(state).len(),
                progress.completed,
                progress.total
            )
        },
        260.,
        104.,
        13.,
        crate::theme::SURFACE,
    );
    panel(
        Rect::new(760., 38., 180., 42.),
        crate::theme::PAPER_LIGHT,
        crate::theme::BORDER,
    );
    text(
        &format!(
            "SORT: {}",
            cabinet_status::CabinetSort::from_index(state.cabinet_sort).button_label()
        ),
        778.,
        64.,
        11.,
        crate::theme::INK,
    );
    for (rect, label, filter) in [
        (Rect::new(960., 38., 92., 42.), "ALL", 9),
        (Rect::new(1060., 38., 92., 42.), "OPEN", 1),
        (Rect::new(1160., 38., 92., 42.), "DONE", 2),
    ] {
        let fill = if state.cabinet_filter == filter {
            crate::theme::MOSS
        } else {
            crate::theme::PAPER_LIGHT
        };
        panel(rect, fill, crate::theme::BORDER);
        text(label, rect.x + 22., rect.y + 26., 11., crate::theme::INK);
        if filter != 9 {
            text(
                &cabinet_status::filter_count(state, filter).to_string(),
                rect.right() - 25.,
                rect.y + 26.,
                11.,
                crate::theme::INK,
            );
        }
    }
    let page = crate::cabinet_data::page(state, crate::cabinet_data::DESKTOP_PAGE_SIZE);
    for (index, game) in page.games.iter().copied().enumerate() {
        let rect = library_rect(index);
        panel(rect, crate::theme::PAPER_LIGHT, crate::theme::BORDER);
        draw_circle(
            rect.x + 25.,
            rect.y + 20.,
            14.,
            crate::theme::category_surface(game, true),
        );
        text(
            state.game_title(game),
            rect.x + 48.,
            rect.y + 18.,
            if state.game_title(game).len() > 18 {
                10.
            } else {
                13.
            },
            crate::theme::INK,
        );
        text(
            crate::storefront::cabinet_label(game, false),
            rect.x + 48.,
            rect.y + 34.,
            9.,
            if cabinet_status::availability(game).is_playable() {
                crate::theme::SURFACE
            } else {
                crate::theme::BRASS
            },
        );
        if cabinet_status::is_available(game) {
            let favorite = state.favorites.get(game.index()).copied().unwrap_or(false);
            text(
                if favorite { "*" } else { "+" },
                rect.right() - 27.,
                rect.y + 27.,
                19.,
                crate::theme::SURFACE_DARK,
            );
        }
        text(
            "INFO",
            rect.right() - 93.,
            rect.y + 26.,
            8.,
            crate::theme::BRASS,
        );
    }
    if page.has_previous() {
        small_button(
            Rect::new(960., 650., 92., 40.),
            "PREV",
            crate::theme::SURFACE_DARK,
        );
    }
    if page.has_next() {
        small_button(
            Rect::new(1060., 650., 92., 40.),
            "NEXT",
            crate::theme::SURFACE_DARK,
        );
    }
    text(
        &crate::cabinet_data::range_label(&page),
        260.,
        681.,
        10.,
        crate::theme::SURFACE,
    );
}
