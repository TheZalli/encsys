use std::collections::{HashSet, HashMap, hash_map};
use std::iter::Iterator;
use std::sync::Arc;

use util::EncSysType;
use super::word::*;

/// A word manager that stores information about the tags associated with words.
pub struct Encyclopedia<N, T>
	where	N: EncSysType,
			T: EncSysType,
{
	// an association from word names into their tags
	word_map: HashMap<Arc<N>, HashSet<Arc<T>> >,
}

impl<N, T> Encyclopedia<N, T>
	where	N: EncSysType,
			T: EncSysType,
{
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia<N, T> {
		Encyclopedia {
			word_map: HashMap::new(),
		}
	}

	/// Adds a new word to the encyclopedia's word map.
	pub fn add(&mut self, w: Word<N, T>) {
		self.word_map.insert(w.get_name(), w.get_tags());
	}

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get<U>(&self, name: U) -> Option<Word<N, T>>
		where U: Into<Arc<N>>,
	{
		let name = name.into().clone();
		match self.word_map.get(&name) {
			Some(&ref set) => Some(Word::new(name, set.clone())),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove<U>(&mut self, name: U)
		where U: Into<Arc<N>>,
	{
		self.word_map.remove(&name.into());
	}

	/// Returns an iterator to the words
	pub fn iter(&self) -> WordIter<N, T> {
		WordIter{ map_iter: self.word_map.iter() }
	}

	/// Returns the amount of words stored.
	pub fn amount(&self) -> usize {
		self.word_map.len()
	}

	/// Tells if the encyclopedia has no words.
	pub fn is_empty(&self) -> bool {
		self.word_map.is_empty()
	}
}

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a, N: 'a, T: 'a>
	where	N: EncSysType,
			T: EncSysType,
{
	map_iter: hash_map::Iter<'a, Arc<N>, HashSet<Arc<T>> >
}

impl<'a, N, T> Iterator for WordIter<'a, N, T>
	where	N: EncSysType,
			T: EncSysType,
{
	type Item = Word<N, T>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.map_iter.next() {
			Some((name, tags)) => Some(Word::new(name.clone(), tags.clone() )),
			None => None,
		}
	}

}
