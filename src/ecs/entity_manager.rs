use std::collections::HashMap;
//use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use {EncSysContainer, EncSysType};
use ecs::*;

/// An entity manager creates and manages entities.
#[derive(Debug)]
pub struct EntMan<C, D>
	where	C: EncSysType + Hash + Debug,
			D: EncSysType + Debug,
{
	// the components
	comps: HashMap<Rc<C>, Vec<Option<Rc<D>>>>,
	// the id's to the words that the entities are associated with
	assoc_word_ids: Vec<Option<usize>>,

	next_id: usize,
	count: usize,
}

impl<C, D> EntMan<C, D>
	where	C: EncSysType + Hash + Debug,
			D: EncSysType + Debug,
{
	/// Creates an empty entity manager.
	pub fn new() -> EntMan<C, D> {
		EntMan { comps: HashMap::new(), assoc_word_ids: Vec::new(), next_id: 0, count: 0}
	}
}

impl<C, D> EncSysContainer<Entity<C, D>> for EntMan<C, D>
	where	C: EncSysType + Hash + Debug,
			D: EncSysType + Debug,
{
	fn add(&mut self, e: Entity<C, D>) -> usize {
		let current_id = self.next_id;
		self.next_id += 1;
		self.count += 1;

		// Assumes no id slots are freed before.
		self.assoc_word_ids.push(Some(e.get_word_id()));

		for (name, data) in e.iter().map(&Comp::into) {
			let vec = self.comps.entry(name).or_insert(Vec::new());
			vec.resize(self.next_id, None);
			vec[current_id] = Some(data);
		}

		return current_id;
	}

	fn get_by_id(&self, id: usize) -> Option<Entity<C, D>> {
		// check if the id is out of bounds
		if id >= self.next_id {
			return None;
		}

		assert_eq!(self.assoc_word_ids.len(), self.next_id);

		let word_id =
			match self.assoc_word_ids.get(id) {
				Some(&Some(x)) => x,
				_ => return None,
			};

		// the returned entity
		let mut ent = Entity::<C, D>::new(word_id);

		// check the components associated with the id
		for (name, vec) in self.comps.iter() {
			match vec.get(id) {
				Some(&Some(ref data)) => ent.insert_comp(Comp::new(name.clone(), data.clone())),
				_ => {}
			}
		}

		// return the entity
		// we know the entity was found because we found it's word id
		Some(ent)
	}

	fn remove_last_id(&mut self) {
		self.assoc_word_ids.pop();
		for (_, vec) in self.comps.iter_mut() {
			vec.pop();
		}
		self.next_id -= 1;
		self.count -= 1;
	}

	/// Leaves the given id slot unused, unless the last entity is removed.
	fn remove_by_id(&mut self, id: usize) {
		if id >= self.next_id {
			// we're out of range
			return;
		}
		else if id == self.next_id - 1 {
			// we are removing the last entity
			return self.remove_last_id();
		}

		self.assoc_word_ids[id] == None;

		for (_, vec) in self.comps.iter_mut() {
			vec.get_mut(id).map(|x| *x = None);
		}
		self.next_id -= 1;
		self.count -= 1;
	}

	fn get_end_id(&self) -> usize {
		self.next_id
	}

	fn get_count(&self) -> usize {
		self.count
	}

	fn is_empty(&self) -> bool {
		self.count == 0
	}
}
