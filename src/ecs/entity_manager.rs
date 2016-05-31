use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use ecs::entity::Entity;

/// An entity manager creates and manages entities
#[derive(Debug)]
pub struct EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	// the components
	comps: HashMap<C, Vec<D>>,
	// the id's to the words that the entities are associated with
	assoc_word_ids: Vec<usize>,
}

impl<C, D> EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	pub fn new() -> EntMan<C, D> {
		EntMan { comps: HashMap::new(), assoc_word_ids: Vec::new()}
	}

	pub fn add_entity(&self, e: Entity<C, D>) {
		unimplemented!()
	}
}
