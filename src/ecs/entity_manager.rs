use std::collections::HashMap;
//use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use ecs::entity::Entity;
use ecs::component::Comp;

/// An entity manager creates and manages entities.
#[derive(Debug)]
pub struct EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	// the components
	comps: HashMap<Rc<C>, Vec<Option<Rc<D>>>>,
	// the id's to the words that the entities are associated with
	assoc_word_ids: Vec<usize>,

	next_id: usize,
	count: usize,
}

impl<C, D> EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	/// Creates an empty entity manager.
	pub fn new() -> EntMan<C, D> {
		EntMan { comps: HashMap::new(), assoc_word_ids: Vec::new(), next_id: 0, count: 0}
	}

	/// Adds the given entity to the entity manager.
	pub fn add_entity(&mut self, e: Entity<C, D>) {
		let current_id = self.next_id;
		self.next_id += 1;
		self.count += 1;

		self.assoc_word_ids.push(e.get_word_id());

		for (name, data) in e.iter().map(&Comp::into) {
			let vec = self.comps.entry(name).or_insert(Vec::new());
			vec.resize(self.next_id, None);
			vec[current_id] = Some(data);
		}
	}

	/// Returns the entity associated with the given id.
	pub fn get_ent_by_id(&self, id: usize) -> Option<Entity<C, D>> {
		// check if the id is out of bounds
		if id >= self.next_id {
			return None;
		}

		// the returned entity
		let mut ent = Entity::<C, D>::new(id);

		// check the components associated with the id
		for (name, vec) in self.comps.iter() {
			match vec.get(id) {
				Some(&Some(ref data)) => ent.insert_comp(Comp::new(name.clone(), data.clone())),
				_ => {}
			}
		}

		// return the entity if it was found
		if ent.is_empty() {
			None
		}
		else {
			Some(ent)
		}
	}
}
