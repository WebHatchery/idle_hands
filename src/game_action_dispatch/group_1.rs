use super::*;

impl Game {
    pub(super) fn apply_shell_group_1(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::Open(index) => self.open_game(index),
            ui::UiAction::ContinueGame => {
                self.open_game(crate::continue_data::preferred_game(&self.state).index());
            }
            ui::UiAction::ToggleFavorite(index) => self.toggle_favorite(index),
            ui::UiAction::ClearRecent => {
                self.state.recent_games.clear();
                self.state.library_scroll = 0;
                self.notifications.info("Recent shelf cleared");
            }
            ui::UiAction::CabinetFilter(filter) => {
                self.state.cabinet_filter = filter.min(9);
                self.state.cabinet_scroll = 0;
            }
            ui::UiAction::FinderFilter(filter) => {
                self.state.cabinet_filter = crate::finder_data::normalize_filter(filter);
                self.state.library_scroll = 0;
            }
            ui::UiAction::CabinetSort => {
                let sort =
                    crate::cabinet_status::CabinetSort::from_index(self.state.cabinet_sort).next();
                self.state.cabinet_sort = sort.index();
                self.state.cabinet_scroll = 0;
                self.notifications
                    .info(format!("Cabinet order: {}", sort.label()));
            }
            ui::UiAction::CabinetScroll(delta) => {
                let page_size = crate::cabinet_data::page_size_for_layout(
                    crate::ui::is_portrait(),
                    crate::ui::is_compact_landscape(),
                );
                let limit = crate::cabinet_data::scroll_limit(&self.state, page_size);
                self.state.cabinet_scroll = self
                    .state
                    .cabinet_scroll
                    .saturating_add_signed(delta as isize)
                    .min(limit);
            }
            ui::UiAction::LibraryScroll(delta) => {
                self.state.library_scroll = self
                    .state
                    .library_scroll
                    .saturating_add_signed(delta as isize)
                    .min(self.library_scroll_limit());
            }
            ui::UiAction::DailyArchiveScroll(delta) => {
                let page_size = crate::daily_archive_ui::page_size();
                self.state.daily_archive_scroll = self
                    .state
                    .daily_archive_scroll
                    .saturating_add_signed(delta as isize)
                    .min(
                        self.state
                            .records
                            .daily_results
                            .len()
                            .saturating_sub(page_size),
                    );
            }
            ui::UiAction::DailyArchiveOpen(day) => self.open_archived_daily_day(day),
            ui::UiAction::RecordsFilter(filter) => {
                self.state.records_filter = if crate::records_data::FILTERS.contains(&filter) {
                    filter
                } else {
                    0
                };
                self.state.library_scroll = 0;
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
