use super::*;

#[test]
fn portrait_title_leaves_room_before_the_rule_card() {
    crate::ui::with_portrait_layout(|| assert_eq!(title_size(), 18.));
}
