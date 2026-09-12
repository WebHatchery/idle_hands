//! Contract tests for the crate-level runtime boundary.

use idle_hands::{Game, LOGICAL_HEIGHT, LOGICAL_WIDTH};

#[test]
fn the_platform_shell_can_reach_the_runtime_and_window_contract() {
    let _runtime_type: Option<Game> = None;
    assert!(std::hint::black_box(LOGICAL_WIDTH) > 0.0);
    assert!(std::hint::black_box(LOGICAL_HEIGHT) > 0.0);
}
