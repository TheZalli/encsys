//! Contains the `Encyclopedia` struct and an iterator to it's words.
use std::collections::{HashSet, HashMap, hash_map};
use std::iter::Iterator;
use std::borrow::Cow;
use std::ops::Deref;

use enc::word::*;
use enc::ling::LingTag;

/// A word manager that stores information about the tags associated with words.
pub struct Encyclopedia {
	/// An association from word names into their tags.
	word_map: HashMap<String, HashSet<LingTag>>,
}

impl Encyclopedia
{
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia {
		Encyclopedia {
			word_map: HashMap::new(),
		}
	}

	/// Adds a new word to the encyclopedia's word map.
	pub fn add(&mut self, word: Word) {
		self.word_map.insert(
			word.get_name().into_owned(),
			word.get_tags().into_iter().map(|x| x.clone().into_owned()).collect()
		);
	}

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get<'a, U>(&'a self, name: U) -> Option<Word<'a>>
		where  U: 'a + AsRef<str> + Into<Cow<'a, str>>
	{
		match self.word_map.get(name.as_ref()) {
			Some(&ref set) => Some(Word::new_from_collection(name.into(), set)),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove<'a, U: 'a + AsRef<str>>(&'a mut self, name: U) {
		self.word_map.remove(name.as_ref());
	}

	/// Returns an iterator to the words
	pub fn iter<'a>(&'a self) -> WordIter<'a> {
		WordIter{ iter: self.word_map.iter() }
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
pub struct WordIter<'a> {
	iter: hash_map::Iter<'a, String, HashSet<LingTag> >,
}

impl<'a> Iterator for WordIter<'a> {
	type Item = Word<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.iter.next() {
			Some((name, tags)) => Some(Word::new_from_collection(name.deref(), tags)),
			None => None,
		}
	}

}
