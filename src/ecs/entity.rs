use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use ecs::component::Comp;

#[derive(Debug)]
pub struct Entity<C, D>
	where	C: PartialEq + Eq + Clone + Debug + Hash,
			D: Clone + Debug,
{
	pub assoc_word_id: usize,
	pub comps: HashMap<Rc<C>, D>
}

impl<C, D> Entity<C, D>
	where	C: PartialEq + Eq + Clone + Debug + Hash,
			D: Clone + Debug,
{
	pub fn new(word_id: usize) -> Entity<C, D> {
		Entity{ assoc_word_id: word_id, comps: HashMap::new() }
	}

	pub fn insert_comp(&mut self, c: Comp<C, D>) {
		self.comps.insert(c.name, c.data);
	}
}
