//! Storefront copy for the demo upgrade path.
//!
//! Keeping this boundary separate means a later itch.io / Steam chooser can
//! replace the single-store prompt without changing cabinet availability.

pub const PRIMARY_STORE: &str = "itch.io";
pub const LOCKED_LABEL: &str = "FULL VERSION · BUY ON ITCH.IO";
pub const COMPACT_LOCKED_LABEL: &str = "FULL VERSION · ITCH.IO";

pub use crate::storefront_data::GameAvailability;

pub fn availability(game: crate::state::GameId) -> GameAvailability {
    crate::storefront_data::availability(game)
}

pub fn build_badge() -> Option<String> {
    crate::storefront_data::build_badge_for(crate::game_descriptor::is_demo_build())
}

pub fn cabinet_label(game: crate::state::GameId, compact: bool) -> &'static str {
    match availability(game) {
        GameAvailability::DemoRestricted if compact => COMPACT_LOCKED_LABEL,
        GameAvailability::DemoRestricted => LOCKED_LABEL,
        _ => crate::storefront_data::cabinet_label(game, compact),
    }
}

pub fn action_label(game: crate::state::GameId) -> &'static str {
    availability(game).action_label()
}

pub fn availability_message(game_title: &str, availability: GameAvailability) -> String {
    match availability {
        GameAvailability::Playable => format!("{game_title} is ready to open"),
        GameAvailability::DemoRestricted => purchase_message(game_title),
        GameAvailability::ComingSoon => format!("{game_title} is coming soon"),
    }
}

pub fn purchase_message(game_title: &str) -> String {
    format!("{game_title} is in the full version — buy Idle Hands on {PRIMARY_STORE}")
}

#[cfg(test)]
mod tests;
