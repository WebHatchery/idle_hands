//! Desktop collection dashboard and category library.

use crate::{
    cabinet_status,
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const SIDEBAR_W: f32 = 220.;
const MAIN: Rect = Rect::new(230., 10., 1040., 700.);
const CONTINUE: Rect = Rect::new(260., 92., 912., 118.);
const CATEGORY_RECTS: [Rect; 6] = [
    Rect::new(260., 252., 292., 132.),
    Rect::new(570., 252., 292., 132.),
    Rect::new(880., 252., 292., 132.),
    Rect::new(260., 400., 292., 132.),
    Rect::new(570., 400., 292., 132.),
    Rect::new(880., 400., 292., 132.),
];
#[cfg(test)]
#[path = "../tests/legacy/cabinet_ui/tests.rs"]
mod tests;

pub fn draw(state: &AppState, loaded: usize, cabinet_texture: Option<&Texture2D>) {
    draw_wood_frame();
    draw_sidebar(state);
    draw_rectangle(MAIN.x, MAIN.y, MAIN.w, MAIN.h, crate::theme::PAPER);
    draw_rectangle_lines(MAIN.x, MAIN.y, MAIN.w, MAIN.h, 3., crate::theme::BORDER);
    if let Some(texture) = cabinet_texture {
        draw_texture_ex(
            texture,
            MAIN.x,
            MAIN.y,
            Color::new(1., 1., 1., 0.14),
            DrawTextureParams {
                dest_size: Some(vec2(MAIN.w, MAIN.h)),
                ..Default::default()
            },
        );
    }
    draw_paper_grain();
    if state.cabinet_filter == 0 {
        home::draw_home(state, loaded);
    } else {
        library::draw_library(state);
    }
}

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    for (index, rect) in sidebar_rects().iter().enumerate() {
        if crate::ui::hit(*rect, p) {
            return match index {
                0 | 2 => vec![UiAction::CabinetFilter(0)],
                1 => vec![UiAction::CabinetFilter(9)],
                3 => vec![UiAction::Favorites],
                4 => vec![UiAction::Recent],
                5 => vec![UiAction::Open(GameId::DailyDungeon.index())],
                _ => vec![UiAction::Records],
            };
        }
    }
    if crate::ui::hit(Rect::new(1190., 28., 52., 48.), p) {
        return vec![UiAction::Settings];
    }
    if crate::ui::hit(Rect::new(1132., 28., 52., 48.), p) {
        return vec![UiAction::Help];
    }
    if state.cabinet_filter == 0 {
        home_clicks(state, p)
    } else {
        library_clicks(state, p)
    }
}

mod home;
mod library;

fn draw_sidebar(state: &AppState) {
    text("IDLE", 48., 61., 34., crate::theme::CREAM);
    text("HANDS", 48., 94., 34., crate::theme::CREAM);
    text("quiet games for", 48., 119., 12., crate::theme::SECONDARY);
    text("idle hands", 48., 136., 12., crate::theme::SECONDARY);
    let labels = [
        "HOME",
        "ALL GAMES",
        "CATEGORIES",
        "FAVORITES",
        "RECENT",
        "DAILY CHALLENGE",
        "RECORDS",
    ];
    for (index, rect) in sidebar_rects().iter().copied().enumerate() {
        let selected =
            (index == 0 && state.cabinet_filter == 0) || (index == 1 && state.cabinet_filter == 9);
        if selected {
            panel(rect, crate::theme::MOSS_DARK, crate::theme::BRASS);
        }
        text(
            labels[index],
            rect.x + 38.,
            rect.y + 28.,
            13.,
            if selected {
                crate::theme::CREAM
            } else {
                crate::theme::SECONDARY
            },
        );
        text(
            match index {
                0 => "H",
                1 => "#",
                2 => "=",
                3 => "*",
                4 => "R",
                5 => "D",
                _ => "!",
            },
            rect.x + 13.,
            rect.y + 29.,
            14.,
            crate::theme::BRASS,
        );
    }
    panel(
        Rect::new(18., 618., 184., 58.),
        crate::theme::SURFACE_DARK,
        crate::theme::BORDER,
    );
    let profile_name = crate::profile_data::display_name(&state.profile_name);
    text(&profile_name, 34., 650., 13., crate::theme::CREAM);
}

fn category_card(state: &AppState, rect: Rect, filter: u8) {
    let sample = GameId::ALL
        .iter()
        .copied()
        .find(|game| cabinet_status::category_filter(*game) == filter)
        .unwrap_or(GameId::Solitaire);
    panel(
        rect,
        crate::theme::category_surface(sample, false),
        crate::theme::BORDER,
    );
    text(
        crate::cabinet_data::category_initial(filter),
        rect.x + 22.,
        rect.y + 42.,
        25.,
        crate::theme::BRASS,
    );
    text(
        cabinet_status::category_name(filter),
        rect.x + 72.,
        rect.y + 43.,
        24.,
        crate::theme::CREAM,
    );
    text(
        &format!(
            "{} games  ·  {}/{} done",
            cabinet_status::filter_count(state, filter),
            cabinet_status::category_progress(state, filter).completed,
            cabinet_status::category_progress(state, filter).total
        ),
        rect.x + 72.,
        rect.y + 70.,
        12.,
        crate::theme::SECONDARY,
    );
    let progress = cabinet_status::category_progress(state, filter);
    let ratio = if progress.total == 0 {
        0.
    } else {
        progress.completed as f32 / progress.total as f32
    };
    draw_rectangle(
        rect.x + 72.,
        rect.y + 82.,
        168.,
        5.,
        crate::theme::PAPER_LIGHT,
    );
    draw_rectangle(
        rect.x + 72.,
        rect.y + 82.,
        168. * ratio,
        5.,
        if progress.is_complete() {
            crate::theme::MOSS
        } else {
            crate::theme::BRASS
        },
    );
    let prompt = cabinet_status::next_unfinished_game(state, filter)
        .map(|game| format!("NEXT: {}", state.game_title(game)))
        .unwrap_or_else(|| "COMPLETE · EXPLORE  >".to_owned());
    text(
        &fit_recent_title(&prompt, rect.w - 96.),
        rect.x + 72.,
        rect.y + 105.,
        11.,
        crate::theme::BRASS,
    );
    crate::mascots::draw_for_filter(filter, vec2(rect.right() - 43., rect.y + 66.), 0.95);
}

fn home_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(1030., 28., 92., 48.), p) {
        return vec![UiAction::Finder];
    }
    if crate::ui::hit(CONTINUE, p) {
        return vec![UiAction::ContinueGame];
    }
    for (index, rect) in CATEGORY_RECTS.iter().copied().enumerate() {
        if crate::ui::hit(rect, p) {
            return vec![UiAction::CabinetFilter(3 + index as u8)];
        }
    }
    for (index, game) in recent_games(state).iter().take(7).copied().enumerate() {
        if crate::ui::hit(recent_rect(index), p) {
            return vec![UiAction::Open(game.index())];
        }
    }
    vec![]
}

fn library_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(760., 38., 180., 42.), p) {
        return vec![UiAction::CabinetSort];
    }
    for (rect, filter) in [
        (Rect::new(960., 38., 92., 42.), 9),
        (Rect::new(1060., 38., 92., 42.), 1),
        (Rect::new(1160., 38., 92., 42.), 2),
    ] {
        if crate::ui::hit(rect, p) {
            return vec![UiAction::CabinetFilter(filter)];
        }
    }
    if crate::ui::hit(Rect::new(250., 20., 190., 70.), p) {
        return vec![UiAction::CabinetFilter(0)];
    }
    let page = crate::cabinet_data::page(state, crate::cabinet_data::DESKTOP_PAGE_SIZE);
    if page.has_previous() && crate::ui::hit(Rect::new(960., 650., 92., 40.), p) {
        return vec![UiAction::CabinetScroll(
            -crate::cabinet_data::DESKTOP_PAGE_STEP,
        )];
    }
    if page.has_next() && crate::ui::hit(Rect::new(1060., 650., 92., 40.), p) {
        return vec![UiAction::CabinetScroll(
            crate::cabinet_data::DESKTOP_PAGE_STEP,
        )];
    }
    for (index, game) in page.games.iter().copied().enumerate() {
        let rect = library_rect(index);
        if crate::ui::hit(Rect::new(rect.right() - 96., rect.y, 48., rect.h), p) {
            return vec![UiAction::Inspect(game.index())];
        }
        if cabinet_status::is_available(game)
            && crate::ui::hit(Rect::new(rect.right() - 48., rect.y, 48., rect.h), p)
        {
            return vec![UiAction::ToggleFavorite(game.index())];
        }
        if crate::ui::hit(rect, p) {
            return vec![UiAction::Open(game.index())];
        }
    }
    vec![]
}

fn recent_games(state: &AppState) -> Vec<GameId> {
    state.recent_games.clone()
}
fn recent_rect(index: usize) -> Rect {
    Rect::new(260. + index as f32 * 137., 576., 126., 58.)
}
fn fit_recent_title(title: &str, max_width: f32) -> String {
    const SIZE: u16 = 12;
    const ELLIPSIS: &str = "…";

    if crate::ui::measure_text(title, None, SIZE, 1.).width <= max_width {
        return title.to_owned();
    }

    let mut prefix = title.to_owned();
    while !prefix.is_empty() {
        prefix.pop();
        let candidate = format!("{prefix}{ELLIPSIS}");
        if crate::ui::measure_text(&candidate, None, SIZE, 1.).width <= max_width {
            return candidate;
        }
    }
    ELLIPSIS.to_owned()
}
fn library_rect(index: usize) -> Rect {
    Rect::new(
        260. + (index % 4) as f32 * 246.,
        125. + (index / 4) as f32 * 47.,
        232.,
        40.,
    )
}
fn sidebar_rects() -> [Rect; 7] {
    std::array::from_fn(|index| Rect::new(18., 160. + index as f32 * 54., 184., 44.))
}

fn draw_wood_frame() {
    draw_rectangle(0., 0., 1280., 720., crate::theme::BACKGROUND_DEEP);
    for y in (8..720).step_by(18) {
        draw_line(
            0.,
            y as f32,
            SIDEBAR_W,
            y as f32 + 3.,
            1.,
            Color::new(0.28, 0.19, 0.12, 0.34),
        );
    }
    draw_line(SIDEBAR_W, 0., SIDEBAR_W, 720., 5., crate::theme::BORDER);
}
fn draw_paper_grain() {
    for y in (24..700).step_by(22) {
        draw_line(
            MAIN.x + 8.,
            y as f32,
            MAIN.right() - 8.,
            y as f32 + 1.,
            1.,
            Color::new(0.35, 0.25, 0.14, 0.07),
        );
    }
}
fn small_button(rect: Rect, label: &str, fill: Color) {
    panel(rect, fill, crate::theme::BORDER);
    text(label, rect.x + 13., rect.y + 29., 10., crate::theme::CREAM);
}
fn panel(rect: Rect, fill: Color, border: Color) {
    crate::ui::draw_rounded_panel(rect, 10., fill, border);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, size, color);
}
