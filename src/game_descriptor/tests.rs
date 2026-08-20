use super::ALL;
use crate::state::GameId;

#[test]
fn registry_matches_the_closed_game_collection() {
    assert_eq!(ALL.len(), GameId::ALL.len());
    for (index, descriptor) in ALL.iter().enumerate() {
        assert_eq!(descriptor.id, GameId::ALL[index]);
        assert_eq!(descriptor.index, index);
        assert_eq!(descriptor.id.save_key(), descriptor.save_key);
        assert!(descriptor.active);
        assert!(!descriptor.title.is_empty());
        assert!(!descriptor.subtitle.is_empty());
    }
}

#[test]
fn save_keys_are_unique() {
    for (index, descriptor) in ALL.iter().enumerate() {
        assert!(
            ALL[..index]
                .iter()
                .all(|other| other.save_key != descriptor.save_key),
            "duplicate save key {}",
            descriptor.save_key
        );
    }
}
