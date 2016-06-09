use EncSysContainer;
use ecs::*;

#[test]
fn add_then_remove_ent() {
	let mut entman = EntMan::<(), ()>::new();
	assert!(entman.is_empty());

	let ent = Entity::new(123);

	let id = entman.add(ent.clone());

	assert_eq!(entman.get_count(), 1);
	assert_eq!(entman.get_by_id(id), Some(ent));

	entman.remove_by_id(id);
	assert!(entman.is_empty());
}
