use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use ecs::component::Comp;

#[derive(Debug)]
/// An entity derived from a word containing components that describe it.
pub struct Entity<C, D>
	where	C: PartialEq + Eq + Clone + Debug + Hash,
			D: Clone + Debug,
{
	assoc_word_id: usize,
	comps: HashMap<Rc<C>, Rc<D>>
}

impl<C, D> Entity<C, D>
	where	C: PartialEq + Eq + Clone + Debug + Hash,
			D: Clone + Debug,
{
	/// Creates a new entity that is derived from a word with the given id.
	pub fn new(word_id: usize) -> Entity<C, D> {
		Entity{ assoc_word_id: word_id, comps: HashMap::new() }
	}

	/// Returns the id of the word where this entity is derived from.
	pub fn get_word_id(&self) -> usize {
		self.assoc_word_id
	}

	/// Inserts the given component into the word.
	pub fn insert_comp(&mut self, c: Comp<C, D>) {
		self.comps.insert(c.name, c.data);
	}

	/// Gets the data of the component with the given name or returns None if no such component was found.
	pub fn get_comp_data(&self, comp_name: Rc<C>) -> Option<Rc<D>> {
		self.comps.get(&comp_name).map(&Rc::clone)
	}

	/// Returns true if this entity has an entity with the given name
	pub fn has_comp(&self, comp_name: Rc<C>) -> bool {
		self.comps.contains_key(&comp_name)
	}

	/// Tells how many components does this entity own.
	pub fn comp_amount(&self) -> usize {
		self.comps.len()
	}

	/// Tells if this is an entity with no components.
	pub fn is_empty(&self) -> bool {
		self.comps.is_empty()
	}

	pub fn iter(&self) -> EntCompIter<C, D> {
		EntCompIter{ comp_iter: self.comps.iter() }
	}
}

/// An iterator that goes through all of the components in an entity.
pub struct EntCompIter<'a, C, D>
	where	C: 'a + PartialEq + Eq + Clone + Debug + Hash,
			D: 'a + Clone + Debug,
{
	comp_iter: hash_map::Iter<'a, Rc<C>, Rc<D>>,
}


impl<'a, C, D> Iterator for EntCompIter<'a, C, D>
	where	C: PartialEq + Eq + Clone + Debug + Hash,
			D: Clone + Debug,
{
	type Item = Comp<C, D>;

	fn next(&mut self) -> Option<Self::Item> {
		self.comp_iter.next().map(|x: (&Rc<C>, &Rc<D>)| Comp::new(x.0.clone(), x.1.clone() ))
	}
}
