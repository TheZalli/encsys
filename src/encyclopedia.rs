use std::collections::HashMap;
use std::iter::IntoIterator;
use std::iter::Iterator;

use tag::*;

/// The word manager stores information about the tags associated with words.
#[derive(Debug)]
pub struct Encyclopedia {
	// a hash map from tags to the vectors containing all of the existing tags
	// to improve memory consumption, optimize the vector values
	tags: HashMap<TagName, Vec<Option<TagInfo>>>,
	// the next word id
	next_id: usize,
	// the amount of words in the encyclopedia
	// if no vacant slots have been generated, this is next_id - 1
	word_count: usize,
}

impl Encyclopedia {
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia {
		Encyclopedia{ tags: HashMap::new(), next_id: 0, word_count: 0}
	}

	/// Adds the word with the given tags
	pub fn add_word(&mut self, word_tags: Vec<Tag>) {
		let current_id = self.next_id;
		self.next_id += 1;

		for tag in word_tags {
			let (name, info) = tag.to_tuple();
			// get the tag's vector, if not found create it
			let vec = self.tags.entry(name).or_insert(Vec::new());
			vec.resize(self.next_id, None);
			vec[current_id] = Some(info);
		}
		self.word_count += 1;
	}

	/// Gets the word with given id or None if nothing was found or the id was out of bounds.
	pub fn get_word_by_id(&self, id: usize) -> Option<Vec<Tag>> {
		// check if we are out of range
		if id >= self.next_id {
			return None;
		}

		let mut word_tags = Vec::new();

		// go through known tags
		for tag_map_elem in self.tags.iter() {
			// check if we found a tag
			if let Some(&Some(ref tag_info)) = tag_map_elem.1.get(id) {
				word_tags.push(Tag::reconstruct(tag_map_elem.0, tag_info));
			}
		}

		if word_tags.is_empty() {
			None
		}
		else {
			Some(word_tags)
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
		if id == self.next_id-1 {
			self.remove_last_word();
		}
		else {
			// go through tags
			for tag_map_elem in self.tags.iter_mut() {
				// clear the word's tag
				tag_map_elem.1.get_mut(id).map(|x| *x = None);
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
	#[inline(always)]
	pub fn get_end_id(&self) -> usize {
		self.next_id
	}

	/// Returns the amount of words in the encyclopedia.
	#[inline(always)]
	pub fn get_word_count(&self) -> usize {
		self.word_count
	}

	/// Returns true if the encyclopedia is
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.word_count == 0
	}
}

impl<'a> IntoIterator for &'a Encyclopedia {
	type Item = <WordIter<'a> as Iterator>::Item;
	type IntoIter = WordIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		WordIter{enc_ref: self, index: 0}
	}
}

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a> {
	enc_ref: &'a Encyclopedia,
	index: usize,
}

impl<'a> Iterator for WordIter<'a> {
	type Item = Vec<Tag>;

	fn next(&mut self) -> Option<Self::Item> {
		// reached the end
		if self.index >= self.enc_ref.get_end_id() {
			return None;
		}

		match self.enc_ref.get_word_by_id(self.index) {
			// found a word, advance self and return it
			Some(vec) => {
				self.index += 1;
				Some(vec)
			},
			// if we encountered a vacant id slot we have to ignore it and continue
			None => {
				self.index += 1;
				self.next()
			}
		}
	}
}
