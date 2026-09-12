//! Public contract tests for the bounded undo history.

use idle_hands::undo::UndoStack;

#[test]
fn history_is_bounded_and_keeps_newest_entries() {
    let mut history = UndoStack::with_capacity(2);
    history.push(1);
    history.push(2);
    history.push(3);

    assert_eq!(history.len(), 2);
    assert_eq!(history.pop(), Some(3));
    assert_eq!(history.pop(), Some(2));
    assert!(history.is_empty());
}
