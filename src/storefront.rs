//! Storefront copy for the demo upgrade path.
//!
//! Keeping this boundary separate means a later itch.io / Steam chooser can
//! replace the single-store prompt without changing cabinet availability.

pub const PRIMARY_STORE: &str = "itch.io";
pub const LOCKED_LABEL: &str = "FULL VERSION · BUY ON ITCH.IO";
pub const COMPACT_LOCKED_LABEL: &str = "FULL VERSION · ITCH.IO";

pub fn purchase_message(game_title: &str) -> String {
    format!("{game_title} is in the full version — buy Idle Hands on {PRIMARY_STORE}")
}

#[cfg(test)]
mod tests;
