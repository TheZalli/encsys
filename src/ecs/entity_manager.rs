use std::collections::HashMap;
//use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use ecs::entity::Entity;

/// An entity manager creates and manages entities
#[derive(Debug)]
pub struct EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	// the components
	comps: HashMap<Rc<C>, Vec<Option<D>>>,
	// the id's to the words that the entities are associated with
	assoc_word_ids: Vec<usize>,

	next_id: usize,
	count: usize,
}

impl<C, D> EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	pub fn new() -> EntMan<C, D> {
		EntMan { comps: HashMap::new(), assoc_word_ids: Vec::new(), next_id: 0, count: 0}
	}

	pub fn add_entity(&mut self, e: Entity<C, D>) {
		let current_id = self.next_id;
		self.next_id += 1;
		self.count += 1;

		self.assoc_word_ids.push(e.assoc_word_id);

		for (name, data) in e.comps {
			let vec = self.comps.entry(name).or_insert(Vec::new());
			vec.resize(self.next_id, None);
			vec[current_id] = Some(data);
		}
	}
}
