//! Regression coverage for the tests module.

use super::*;

#[test]
fn embedded_data_loads() {
    let data = GameData::load().unwrap();

    assert!(!data.config.game_name.is_empty());
    assert!(!data.config.display_name.is_empty());
}
