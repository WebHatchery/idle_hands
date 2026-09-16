//! Regression coverage for the tests module.

use super::*;
use crate::state::GameId;

#[test]
fn rows_are_alphabetical_and_cover_the_collection() {
    let rows = rows(0);

    assert_eq!(rows.len(), GameId::ALL.len());
    assert!(rows
        .windows(2)
        .all(|pair| pair[0].title() <= pair[1].title()));
}

#[test]
fn alphabet_buckets_partition_titles() {
    let bucketed = (1..=4).map(|filter| rows(filter).len()).sum::<usize>();

    assert_eq!(bucketed, GameId::ALL.len());
    assert!(rows(1).iter().all(|game| matches_filter(*game, 1)));
    assert!(rows(4).iter().all(|game| matches_filter(*game, 4)));
}
