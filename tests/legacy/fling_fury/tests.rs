//! Regression coverage for the tests module.

use super::*;

#[test]
fn impact_knocks_blocks_and_can_start_a_target_fall() {
    let mut game = FlingFury::new(9);
    game.shot = Some(FlingShot {
        x: 20.5,
        y: 12.5,
        vx: 2.,
        vy: 0.,
    });
    game.advance_one(0.016);
    assert!(game.blocks[0].knocked);

    game.targets[0].x = game.blocks[0].x + 0.5;
    game.targets[0].y = game.blocks[0].y + 0.5;
    game.advance_one(0.016);
    assert!(game.targets[0].falling || !game.targets[0].alive);
}

#[test]
fn winning_a_level_opens_the_next_authored_fort() {
    let mut game = FlingFury::new(10);
    game.status = FlingStatus::Won;
    assert!(game.next_level());
    assert_eq!(game.level_number(), 2);
    assert_eq!(game.targets.len(), 4);
    assert_eq!(game.shots_remaining, 5);
}

#[test]
fn won_rounds_report_a_clear_star_rating_for_the_resume_screen() {
    let mut game = FlingFury::new(11);
    game.status = FlingStatus::Won;
    game.moves = 3;
    assert_eq!(game.stars(), 2);
    game.moves = 2;
    assert_eq!(game.stars(), 3);
}

#[test]
fn spending_the_last_shot_enters_a_loss_state_when_targets_remain() {
    let mut game = FlingFury::new(12);
    game.shots_remaining = 0;
    game.shot = None;
    game.advance_one(0.016);
    assert_eq!(game.status, FlingStatus::Lost);
    assert_eq!(game.stars(), 0);
}
