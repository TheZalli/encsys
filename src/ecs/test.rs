use std::iter::FromIterator;

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

#[test]
fn add_many_empty_entities() {
	let mut entman = EntMan::<(), ()>::new();

	let ent_am = 100;
	let foo = |x| 2*x + 1;

	let id_vec: Vec<usize> = (0..ent_am).map(|x| entman.add(Entity::new(foo(x)))).collect();

	let mut id_vec_clone = id_vec.clone();
	id_vec_clone.sort();
	id_vec_clone.dedup();

	assert_eq!(entman.get_end_id(), ent_am);
	assert_eq!(entman.get_count(), ent_am);

	assert_eq!(id_vec, id_vec_clone);
	assert_eq!(id_vec, Vec::from_iter(0..ent_am));

	// assert that we are iterating in the order of the id's
	for (i, ent) in entman.iter().enumerate() {
		assert_eq!(ent.get_word_id(), foo(i));
	}
}
