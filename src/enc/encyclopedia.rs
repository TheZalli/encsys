//! Contains the `Encyclopedia` struct and an iterator to it's words.
use std::collections::{HashSet, HashMap, hash_map};
use std::iter::Iterator;
use std::sync::Arc;

use util::EncSysType;
use super::word::*;

/// A word manager that stores information about the tags associated with words.
pub struct Encyclopedia<W: Word> {
	// an association from word names into their tags
	word_map: HashMap<Arc<W::Name>, HashSet<Arc<W::Tag>> >,
}

impl<W: Word> Encyclopedia<W>
	where	W::Name: EncSysType,
			W::Tag: EncSysType,
{
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia<W> {
		Encyclopedia {
			word_map: HashMap::new(),
		}
	}

	/// Adds a new word to the encyclopedia's word map.
	pub fn add(&mut self, w: W) {
		self.word_map.insert(w.get_name(), w.get_tags());
	}

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get<U>(&self, name: U) -> Option<W>
		where U: Into<Arc<W::Name>>,
	{
		let name = name.into().clone();
		match self.word_map.get(&name) {
			Some(&ref set) => Some(Word::new_from_collection(name, set.clone())),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove<U>(&mut self, name: U)
		where U: Into<Arc<W::Name>>,
	{
		self.word_map.remove(&name.into());
	}

	/// Returns an iterator to the words
	pub fn iter(&self) -> WordIter<W> {
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
pub struct WordIter<'a, W: 'a + Word>
{
	map_iter: hash_map::Iter<'a, Arc<W::Name>, HashSet<Arc<W::Tag>> >,
}

impl<'a, W: Word> Iterator for WordIter<'a, W>
	where W::Tag: EncSysType
{
	type Item = W;

	fn next(&mut self) -> Option<Self::Item> {
		match self.map_iter.next() {
			Some((name, tags)) => Some(W::new_from_collection(name.clone(), tags.clone())),
			None => None,
		}
	}

}
