use std::collections::{HashSet, hash_set};
use std::iter::Iterator;
use std::rc::Rc;
use std::fmt::Debug;

use EncSysType;

/// A word that has a name and associated tags.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Word<N, T>
	where	N: Clone + PartialEq + Eq + Debug,
			T: EncSysType + Debug,
{
	name: Rc<N>,
	tags: HashSet<Rc<T>>,
}

impl<N, T> Word<N, T>
	where	N: Clone + PartialEq + Eq + Debug,
			T: EncSysType + Debug,
{
	pub fn new<U>(name: U, tags: HashSet<Rc<T>>) -> Word<N, T>
		where	U: Into<Rc<N>>,
	{
		Word { name: name.into(), tags: tags }
	}

	/// Creates a new empty word.
	pub fn new_empty<U>(name: U) -> Word<N, T>
		where U: Into<Rc<N>>
	{
		Word { name: name.into(), tags: HashSet::new() }
	}

	pub fn from_collection<U, V, W>(name: U, coll: V) -> Word<N, T>
		where	U: Into<Rc<N>>,
				V: IntoIterator<Item = W>,
				W: Into<Rc<T>>,
	{
		Word {
			name: name.into(),
			tags: coll.into_iter().map(&Into::into).collect(),
		}
	}

	/// Returns the words name.
	pub fn get_name(&self) -> Rc<N> {
		self.name.clone()
	}

	pub fn get_tags(&self) -> HashSet<Rc<T>> {
		self.tags.clone()
	}

	/// Adds the given tag to the word, replacing any previous tag with the same value.
	pub fn add_tag<U>(&mut self, tag: U)
		where U: Into<Rc<T>>
	{
		// note that the duplicates aren't removed here but lazily when needed.
		// this is so that we don't have to sort
		self.tags.insert(tag.into());
	}

	/// Returns true if the word has the given tag.
	pub fn has_tag<U>(&self, tag: U) -> bool
		where U: Into<Rc<T>>
	{
		self.tags.contains(&tag.into())
	}

	/// Returns the amount of tags stored.
	pub fn tag_amount(&self) -> usize {
		self.tags.len()
	}

	/// Returns true if the word has no tags.
	pub fn is_empty(&self) -> bool {
		self.tags.is_empty()
	}

	/// Returns an iterator over the tags of the word.
	pub fn iter(&self) -> TagIter<T> {
		TagIter{ set_iter: self.tags.iter() }
	}
}

/// An iterator wrapper class for the `hash_set`'s iterator.
pub struct TagIter<'a, T: 'a> {
	pub set_iter: hash_set::Iter<'a, Rc<T>>
}

impl<'a, T> Iterator for TagIter<'a, T> {
	type Item = Rc<T>;
	fn next(&mut self) -> Option<Self::Item> {
		self.set_iter.next().cloned()
	}
}
