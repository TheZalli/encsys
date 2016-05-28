use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::iter::Iterator;

use tag::*;
use word::*;

/// The word manager stores information about the tags associated with words.
#[derive(Debug)]
pub struct Encyclopedia<N, I>
	where	N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Hash + Debug
{
	// a hash map from tags to the vectors containing all of the existing tags
	// to improve memory consumption, optimize the vector values
	tags: HashMap<N, Vec<TagData<I>>>,
	// a group of tags using a single tag's name, used to avoid copying common tag combinations
	tag_groups: HashMap<N, Vec<Tag<N, I>>>,
	// the id of the next word that is added
	next_id: usize,
	// the amount of words in the encyclopedia
	// if no vacant slots have been created, this is next_id - 1
	word_count: usize,
}

impl<N, I> Encyclopedia<N, I>
	where	N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Hash + Debug
{
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia<N, I> {
		Encyclopedia{ tags: HashMap::new(), tag_groups: HashMap::new(), next_id: 0, word_count: 0}
	}

	/// Adds the word with the given tags
	pub fn add_word(&mut self, word: Word<N, I>) {
		let current_id = self.next_id;
		self.next_id += 1;

		for tag in word.to_tag_vec() {
			let (name, data) = tag.as_tuple();
			// get the tag's vector, if not found create it
			let vec = self.tags.entry(name).or_insert(Vec::new());
			// resize the vector to fit
			vec.resize(self.next_id, TagData::Empty);
			// add the tag's info
			vec[current_id] = data;
		}
		self.word_count += 1;
	}

	/// Declares a new tag group with the given name and containing the given tags.
	pub fn add_tag_group(&mut self, group_name: N, tags: Vec<Tag<N, I>>) {
		self.tag_groups.insert(group_name, tags);
	}

	/// Gets the word with given id or None if nothing was found or the id was out of bounds.
	pub fn get_word_by_id(&self, id: usize) -> Option<Word<N, I>> {
		// check if we are out of range
		if id >= self.next_id {
			return None;
		}

		let mut word = Word::new();

		// go through known tags
		for tag_map_elem in self.tags.iter() {
			// check if the tag is a group
			// currently this fails silently
			if let Some(ref tag_vec) = self.tag_groups.get(tag_map_elem.0) {
				// the tag_map_elem.1, which is the information of the tag is ignored
				word.extend(tag_vec.clone());
			}
			else {
				// check if we found data for the tag associated wit this word
				match tag_map_elem.1.get(id) {
					// nope didn't find
					Some(&TagData::Empty) | None => {},
					// yep we found
					Some(ref tag_data) => word.add_tag(Tag::reconstruct(tag_map_elem.0, tag_data))
				}
			}

		}

		if word.is_empty() {
			None
		}
		else {
			Some(word)
		}
	}

	/// Removes the word with the given id.
	/// Note that since removing words is seen as a rare operation, this function doesn't free id slots, except when removing the last element, in which case calling this function is the same as calling `remove_last_word`.
	pub fn remove_word_by_id(&mut self, id: usize) {
		if self.is_empty() {
			return;
		}

		// if we're removing the last id might as well call the specialized function for that
		// because it saves up one id
		if id == self.next_id - 1 {
			self.remove_last_word();
		}
		else {
			// go through tags
			for tag_map_elem in self.tags.iter_mut() {
				// clear the word's tag
				tag_map_elem.1.get_mut(id).map(|x| *x = TagData::Empty);
			}
			self.word_count -= 1;
		}
	}

	/// Removes the word with the last id and frees it's id slot.
	pub fn remove_last_word(&mut self) {
		if self.is_empty() {
			return;
		}

		// go through tags
		for tag_map_elem in self.tags.iter_mut() {
			// check if this vector is long enough to have a tag belonging to the last word
			if tag_map_elem.1.len() == self.next_id {
				// remove it
				tag_map_elem.1.pop();
			}
		}
		self.next_id -= 1;
		self.word_count -= 1
	}

	/// Returns the one after the last existing id.
	pub fn get_end_id(&self) -> usize {
		self.next_id
	}

	/// Returns the amount of words in the encyclopedia.
	pub fn get_word_count(&self) -> usize {
		self.word_count
	}

	/// Returns true if the encyclopedia is empty
	pub fn is_empty(&self) -> bool {
		self.word_count == 0
	}
}

impl<'a, N, I> IntoIterator for &'a Encyclopedia<N, I>
	where	N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Hash + Debug
{
	type Item = <WordIter<'a, N, I> as Iterator>::Item;
	type IntoIter = WordIter<'a, N, I>;

	fn into_iter(self) -> Self::IntoIter {
		WordIter{ enc_ref: self, index: 0 }
	}
}

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a, N, I>
	where	N: 'a + Clone + PartialEq + Eq + Hash + Debug,
			I: 'a + Clone + PartialEq + Eq + Hash + Debug
{
	enc_ref: &'a Encyclopedia<N, I>,
	index: usize,
}

impl<'a, N, I> Iterator for WordIter<'a, N, I>
	where	N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Hash + Debug
{
	type Item = Word<N, I>;

	fn next(&mut self) -> Option<Self::Item> {
		// reached the end
		if self.index >= self.enc_ref.get_end_id() {
			return None;
		}

		match self.enc_ref.get_word_by_id(self.index) {
			// found a word, advance self and return it
			Some(word) => {
				self.index += 1;
				Some(word)
			},
			// if we encountered a vacant id slot we have to ignore it and continue
			None => {
				self.index += 1;
				self.next()
			}
		}
	}

}
