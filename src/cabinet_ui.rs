//! Desktop collection dashboard and category library.

use crate::{
    cabinet_status,
    data::GameData,
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const SIDEBAR_W: f32 = 220.;
const MAIN: Rect = Rect::new(230., 10., 1040., 700.);
const CONTINUE: Rect = Rect::new(260., 92., 365., 118.);
const CATEGORY_RECTS: [Rect; 6] = [
    Rect::new(260., 252., 292., 132.),
    Rect::new(570., 252., 292., 132.),
    Rect::new(880., 252., 292., 132.),
    Rect::new(260., 400., 292., 132.),
    Rect::new(570., 400., 292., 132.),
    Rect::new(880., 400., 292., 132.),
];
const LIBRARY_PAGE_SIZE: usize = 44;
const LIBRARY_PAGE_STEP: i8 = 11;

#[cfg(test)]
mod tests;

pub fn draw(
    state: &AppState,
    _data: &GameData,
    loaded: usize,
    cabinet_texture: Option<&Texture2D>,
) {
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
        draw_home(state, loaded);
    } else {
        draw_library(state);
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

fn draw_home(state: &AppState, loaded: usize) {
    text("Good evening", 260., 55., 30., crate::theme::INK);
    if crate::game_descriptor::is_demo_build() {
        text("DEMO · 30 GAMES", 1000., 55., 13., crate::theme::BRASS);
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
    panel(CONTINUE, crate::theme::MOSS_DARK, crate::theme::BRASS);
    text("CONTINUE PLAYING", 278., 116., 11., crate::theme::BRASS);
    let selected = GameId::ALL[state.selected.min(GameId::ALL.len() - 1)];
    text(selected.title(), 278., 153., 27., crate::theme::CREAM);
    text(
        selected.subtitle(),
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
    text("CONTINUE  >", 495., 179., 12., crate::theme::CREAM);
    stat_card(
        Rect::new(645., 92., 145., 118.),
        "FAVORITES",
        favorite_count(state),
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
            &fit_recent_title(game.title(), rect.right() - (rect.x + 48.) - 9.),
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
    text(
        &format!(
            "{} stamps  ·  {}/{} achievements  ·  {} games  ·  {} textures",
            state.stamps,
            state.achievements.iter().filter(|earned| **earned).count(),
            crate::progression::AchievementId::ALL.len(),
            GameId::ALL.len(),
            loaded
        ),
        930.,
        694.,
        10.,
        crate::theme::SURFACE,
    );
}

fn draw_library(state: &AppState) {
    let title = cabinet_status::category_name(state.cabinet_filter);
    let (playable, full) = cabinet_status::availability_counts(state, state.cabinet_filter);
    text("<  COLLECTION", 260., 48., 12., crate::theme::SURFACE);
    text(title, 260., 82., 31., crate::theme::INK);
    text(
        &if crate::game_descriptor::is_demo_build() {
            format!("{playable} playable · {full} in full version")
        } else {
            format!("{} quiet games", visible_games(state).len())
        },
        260.,
        104.,
        13.,
        crate::theme::SURFACE,
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
    }
    let games = visible_games(state);
    let start = library_page_start(games.len(), state.cabinet_scroll);
    for (index, game) in games
        .iter()
        .copied()
        .skip(start)
        .take(LIBRARY_PAGE_SIZE)
        .enumerate()
    {
        let rect = library_rect(index);
        panel(rect, crate::theme::PAPER_LIGHT, crate::theme::BORDER);
        draw_circle(
            rect.x + 25.,
            rect.y + 20.,
            14.,
            crate::theme::category_surface(game, true),
        );
        text(
            game.title(),
            rect.x + 48.,
            rect.y + 18.,
            if game.title().len() > 18 { 10. } else { 13. },
            crate::theme::INK,
        );
        text(
            if cabinet_status::is_available(game) {
                game.subtitle()
            } else {
                crate::storefront::LOCKED_LABEL
            },
            rect.x + 48.,
            rect.y + 34.,
            9.,
            if cabinet_status::is_available(game) {
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
    }
    let end = (start + LIBRARY_PAGE_SIZE).min(games.len());
    small_button(
        Rect::new(960., 650., 92., 40.),
        "PREV",
        crate::theme::SURFACE_DARK,
    );
    small_button(
        Rect::new(1060., 650., 92., 40.),
        "NEXT",
        crate::theme::SURFACE_DARK,
    );
    text(
        &format!(
            "{}-{} OF {}",
            start + usize::from(!games.is_empty()),
            end,
            games.len()
        ),
        260.,
        681.,
        10.,
        crate::theme::SURFACE,
    );
}

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
        Rect::new(18., 606., 184., 76.),
        crate::theme::SURFACE_DARK,
        crate::theme::BORDER,
    );
    text(&state.profile_name, 34., 636., 13., crate::theme::CREAM);
    text(
        &format!(
            "{} stamps · {}/{} goals",
            state.stamps,
            state.achievements.iter().filter(|earned| **earned).count(),
            crate::progression::AchievementId::ALL.len()
        ),
        34.,
        659.,
        10.,
        crate::theme::SECONDARY,
    );
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
        ["C", "L", "B", "W", "A", "M"][(filter - 3) as usize],
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
    text(
        "EXPLORE  >",
        rect.x + 72.,
        rect.y + 105.,
        11.,
        crate::theme::BRASS,
    );
    crate::mascots::draw_for_filter(filter, vec2(rect.right() - 43., rect.y + 66.), 0.95);
}

fn stat_card(rect: Rect, label: &str, count: usize, unit: &str) {
    panel(rect, crate::theme::PAPER_LIGHT, crate::theme::BORDER);
    text(
        label,
        rect.x + 15.,
        rect.y + 28.,
        11.,
        crate::theme::SURFACE_DARK,
    );
    text(
        &count.to_string(),
        rect.x + 54.,
        rect.y + 70.,
        25.,
        crate::theme::INK,
    );
    text(unit, rect.x + 55., rect.y + 91., 10., crate::theme::SURFACE);
}

fn home_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(CONTINUE, p) {
        return vec![UiAction::ContinueGame];
    }
    if crate::ui::hit(Rect::new(645., 92., 145., 118.), p) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(Rect::new(805., 92., 145., 118.), p) {
        return vec![UiAction::Recent];
    }
    if crate::ui::hit(Rect::new(965., 92., 207., 118.), p) {
        return vec![UiAction::Open(GameId::DailyDungeon.index())];
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
    if crate::ui::hit(Rect::new(960., 650., 92., 40.), p) {
        return vec![UiAction::CabinetScroll(-LIBRARY_PAGE_STEP)];
    }
    if crate::ui::hit(Rect::new(1060., 650., 92., 40.), p) {
        return vec![UiAction::CabinetScroll(LIBRARY_PAGE_STEP)];
    }
    let games = visible_games(state);
    let start = library_page_start(games.len(), state.cabinet_scroll);
    for (index, game) in games
        .iter()
        .copied()
        .skip(start)
        .take(LIBRARY_PAGE_SIZE)
        .enumerate()
    {
        let rect = library_rect(index);
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

fn visible_games(state: &AppState) -> Vec<GameId> {
    GameId::ALL
        .iter()
        .copied()
        .filter(|game| cabinet_status::matches_filter(state, *game, state.cabinet_filter))
        .collect()
}

fn library_page_start(total: usize, scroll: usize) -> usize {
    scroll.min(total.saturating_sub(LIBRARY_PAGE_SIZE))
}
fn recent_games(state: &AppState) -> Vec<GameId> {
    if state.recent_games.is_empty() {
        GameId::ALL[..7].to_vec()
    } else {
        state.recent_games.clone()
    }
}
fn favorite_count(state: &AppState) -> usize {
    state.favorites.iter().filter(|value| **value).count()
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
