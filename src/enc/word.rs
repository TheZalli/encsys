use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::rc::Rc;

use EncSysType;
use enc::tag::*;

/// A word that has a name and associated tags.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Word<W, N, I>
	where	N: EncSysType + Debug + Hash,
			I: EncSysType + Debug,
{
	name: Rc<W>,
	tags: HashMap<Rc<N>, Option<Rc<I>>>
}

impl<W, N, I> Word<W, N, I>
	where	N: EncSysType + Debug + Hash,
			I: EncSysType + Debug,
{
	/// Creates a new empty word.
	pub fn new<T>(name: T) -> Word<W, N, I>
		where T: Into<Rc<W>> + EncSysType + Debug
	{
		Word{ name: name.into(), tags: HashMap::new() }
	}

	/// Creates a new word from the given name and the given tag vector
	pub fn from_tag_vec<T>(name: T, vec: Vec<Tag<N, I>>) -> Word<W, N, I>
		where T: Into<Rc<W>> + EncSysType + Debug
	{
		Word {
			name: name.into(),
			tags: vec.into_iter().map(|t: Tag<N, I>| (t.name, t.data)).collect(),
		}
	}

	/// Returns the words name.
	pub fn get_name(&self) -> Rc<W> {
		self.name.clone()
	}

	/// Transforms the word into a vector of it's tags.
	pub fn to_tag_vec(self) -> Vec<Tag<N, I>> {
		self.tags.into_iter().map(|(n, i)| Tag::reconstruct(n, i)).collect()
	}

	/// Adds the given tag to the word, replacing any previous tag with the same name.
	pub fn add_tag(&mut self, t: Tag<N, I>) {
		self.tags.insert(t.name, t.data);
	}

	/// Adds the given tag to the word, WITHOUT replacing any previous tags.
	/// Returns true if the tag was added and false otherwise.
	pub fn add_new_tag(&mut self, t: Tag<N, I>) -> bool {
		match self.tags.entry(t.get_name()) {
			Entry::Occupied(_) => false,
			Entry::Vacant(x) => {
				x.insert(t.get_data());
				true
			}
		}
	}

	// maybe change the type N for this and the three next functions into Into<Rc<N>>

	/// Returns the `TagData` struct associated with the tag with the given name.
	/// Returns `None` if no such tag was found.
	pub fn get_tag_data(&self, name: &N) -> Option<Rc<I>> {
		match self.tags.get(name) {
			Some(&ref data) => data.clone(),
			_ => None
		}
	}


	/// Returns true if the word has the given nullary tag.
	pub fn has_nullary(&self, name: N) -> bool {
		self.tags.get(&name) == None
	}

	/// Returns true if the word has the given tag.
	pub fn has_tag(&self, name: N) -> bool {
		self.tags.contains_key(&name)
	}


	/// Returns true if the word has no tags.
	pub fn is_empty(&self) -> bool {
		self.tags.is_empty()
	}
}

/*impl<'a, W, N, I> Extend<&'a Tag<N, I>> for Word<W, N, I>
	where	N: EncSysType + Debug + Hash,
			I: EncSysType + Debug,
{
	fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'a Tag<N, I>> {
		self.tags.extend(iter.into_iter().map(|t: &Tag<N, I>| (t.name.clone(), t.data.clone()) ))
	}
}*/
