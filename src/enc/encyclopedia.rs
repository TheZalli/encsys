//! Contains the `Encyclopedia` struct and an iterator to it's words.
use std::collections::{HashSet, HashMap, hash_map};
use std::iter::Iterator;
use std::borrow::Cow;
use std::ops::Deref;

use enc::word::*;
use enc::ling::LingTag;
use enc::ling::grammeme::GrammCategory;

/// A word manager that stores information about the tags associated with words.
pub struct Encyclopedia {
	/// An association from word names into their tags.
	word_map: HashMap<String, HashSet<LingTag>>,
	/// All of the available grammatical categories.
	/// Map from category names into their values.
	/// The first value in the tuple is the default value if any.
	// TODO: optimize the default value into a raw pointer.
	gramm_cats: HashMap<String, (Option<String>, HashSet<String>)>,
}

impl Encyclopedia {
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia {
		Encyclopedia {
			word_map: HashMap::new(),
			gramm_cats: HashMap::new(),
		}
	}

	/// Adds a new word to the encyclopedia's word map.
	pub fn add_word(&mut self, word: Word) {
		let entry = word.into_map_entry();
		self.word_map.insert(entry.0, entry.1);
	}

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get_word<'a, T>(&'a self, name: T) -> Option<Word<'a>>
		where  T: 'a + AsRef<str> + Into<Cow<'a, str>>
	{
		match self.word_map.get(name.as_ref()) {
			Some(&ref set) => Some(Word::new_from_collection(name.into(), set)),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove_word<'a, U: 'a + AsRef<str>>(&'a mut self, name: U) {
		self.word_map.remove(name.as_ref());
	}

	/// Returns an iterator to the words
	pub fn iter_words<'a>(&'a self) -> WordIter<'a> {
		WordIter{ iter: self.word_map.iter() }
	}

	/// Returns the amount of words stored.
	pub fn word_amount(&self) -> usize {
		self.word_map.len()
	}

	pub fn add_gramm_cat(&mut self, categ: GrammCategory) {
		let entry = categ.into_map_entry();
		self.gramm_cats.insert(entry.0, entry.1);
	}

	pub fn get_gramm_cat<'a>(&'a self, name: &'a str) -> Option<GrammCategory> {
		let opt = self.gramm_cats.get(name);
		if let Some(&(ref def_opt, ref set)) = opt {
			Some(GrammCategory::new(name.clone(), def_opt.clone(), set.clone()))
		} else {
			None
		}
	}

	/// Tells if the encyclopedia has no words or grammatical categories.
	pub fn is_empty(&self) -> bool {
		self.word_map.is_empty() && self.gramm_cats.is_empty()
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
