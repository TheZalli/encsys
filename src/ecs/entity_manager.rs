use std::collections::HashMap;
//use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use ecs::*;

/// An entity manager creates and manages entities.
#[derive(Debug)]
pub struct EntMan<C, D>
	where	C: Clone + PartialEq + Eq + Hash + Debug,
			D: Clone + PartialEq + Eq + Debug,
{
	// the components
	comps: HashMap<Rc<C>, Vec<Option<Rc<D>>>>,
	// the id's to the words that the entities are associated with
	assoc_word_ids: Vec<Option<usize>>,

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
	/// Returns the id given to the entity.
	pub fn add_ent(&mut self, e: Entity<C, D>) -> usize {
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

	/// Returns the entity associated with the given entity id.
	pub fn get_ent_by_id(&self, id: usize) -> Option<Entity<C, D>> {
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

	/// Removes the word with the highest id and frees it's id-slot.
	pub fn remove_last_id(&mut self) {
		self.assoc_word_ids.pop();
		for (_, vec) in self.comps.iter_mut() {
			vec.pop();
		}
		self.next_id -= 1;
		self.count -= 1;
	}

	/// Removes the entity with the given id.
	/// Leaves the given id slot unused, unless the last entity is removed.
	pub fn remove_ent_by_id(&mut self, id: usize) {
		if id >= self.next_id {
			// we're out of range
			return;
		}
		else if id == self.next_id - 1 {
			// we are removing the last entity
			return self.remove_last_id();
		}

		assert_eq!(self.assoc_word_ids.len(), self.next_id - 1);
		self.assoc_word_ids[id] == None;

		for (_, vec) in self.comps.iter_mut() {
			vec.get_mut(id).map(|x| *x = None);
		}
		self.next_id -= 1;
		self.count -= 1;
	}

	/// Returns the amount of entities stored.
	pub fn get_ent_count(&self) -> usize {
		self.count
	}

	/// Returns true if there are no entities stored.
	pub fn is_empty(&self) -> bool {
		self.count == 0
	}
}
