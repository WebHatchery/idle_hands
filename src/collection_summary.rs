//! One shared snapshot for the collection-wide progress numbers.

use crate::{progression, progression::AchievementId, state::AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollectionSummary {
    pub completed_games: usize,
    pub total_games: usize,
    pub earned_achievements: usize,
    pub total_achievements: usize,
    pub stamps: u16,
    pub total_playtime_seconds: u64,
    pub active_games: usize,
    pub fastest_seconds: Option<u32>,
}

pub fn from_state(state: &AppState) -> CollectionSummary {
    let time = state.records.time_summary();
    CollectionSummary {
        completed_games: progression::completed_games(&state.records),
        total_games: crate::state::GameId::ALL.len(),
        earned_achievements: state.achievements.iter().filter(|earned| **earned).count(),
        total_achievements: AchievementId::ALL.len(),
        stamps: state.stamps,
        total_playtime_seconds: time.total_seconds,
        active_games: time.active_games,
        fastest_seconds: time.fastest_seconds,
    }
}

impl CollectionSummary {
    pub fn drawers_label(self) -> String {
        format!("{}/{} drawers", self.completed_games, self.total_games)
    }

    pub fn achievements_label(self) -> String {
        format!(
            "{}/{} achievements",
            self.earned_achievements, self.total_achievements
        )
    }

    pub fn progress_label(self) -> String {
        format!(
            "Drawers {}/{}  ·  Achievements {}/{}  ·  Stamps {}",
            self.completed_games,
            self.total_games,
            self.earned_achievements,
            self.total_achievements,
            self.stamps
        )
    }

    pub fn fastest_label(self) -> String {
        self.fastest_seconds.map_or_else(
            || "Fastest —".to_owned(),
            |seconds| {
                format!(
                    "Fastest {}",
                    crate::state_records::format_duration(u64::from(seconds))
                )
            },
        )
    }

    pub fn completion_percent(self) -> usize {
        self.completed_games
            .saturating_mul(100)
            .checked_div(self.total_games)
            .unwrap_or(0)
    }

    pub fn achievement_percent(self) -> usize {
        self.earned_achievements
            .saturating_mul(100)
            .checked_div(self.total_achievements)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests;
