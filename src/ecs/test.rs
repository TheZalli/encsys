use ecs::*;

#[test]
fn add_then_remove_ent() {
	let mut entman = EntMan::<(), ()>::new();
	assert!(entman.is_empty());

	let ent = Entity::new(123);

	let id = entman.add_ent(ent.clone());

	assert_eq!(entman.get_ent_count(), 1);
	assert_eq!(entman.get_ent_by_id(id), Some(ent));

	entman.remove_ent_by_id(id);
	assert!(entman.is_empty());
}
