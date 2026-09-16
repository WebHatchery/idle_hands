//! Shared rule-card metadata and deterministic variant rotation.

mod cycle;
mod labels;

pub use cycle::cycle;
pub(crate) use labels::configured_label;
pub use labels::label;

fn next<T: Copy + PartialEq>(all: &[T], current: T) -> T {
    let index = all.iter().position(|item| *item == current).unwrap_or(0);
    all[(index + 1) % all.len()]
}
