use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::Iterator;
use std::rc::Rc;

use tag::*;
use word::*;

/// The word manager stores information about the tags associated with words.
#[derive(Debug)]
pub struct Encyclopedia<W, N, I>
	where	W: Clone + PartialEq + Eq + Hash + Debug,
			N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Debug,
{
	// an association from word names into their ids/indexes
	word_map: HashMap<Rc<W>, usize>,
	// an association from id's into their word names
	word_vec: Vec<Option<Rc<W>>>,
	// a hash map from tags to the vectors containing all of the existing tags
	// to improve memory consumption, optimize the vector values
	tags: HashMap<Rc<N>, Vec<TagData<I>>>,
	// a group of tags using a single tag's name, used to avoid copying common tag combinations
	tag_groups: HashMap<N, Vec<Tag<N, I>>>,
	// the id of the next word that is added
	next_id: usize,
	// the amount of words in the encyclopedia
	// if no vacant slots have been created, this is next_id - 1
	word_count: usize,
}

impl<W, N, I> Encyclopedia<W, N, I>
	where	W: Clone + PartialEq + Eq + Hash + Debug,
			N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Debug,
{
	/// Creates a new empty encyclopedia.
	pub fn new() -> Encyclopedia<W, N, I> {
		Encyclopedia {
			word_map: HashMap::new(),
			word_vec: Vec::new(),
			tags: HashMap::new(),
			tag_groups: HashMap::new(),
			next_id: 0,
			word_count: 0
		}
	}

	/// Declares a new tag group with the given name and containing the given tags.
	pub fn add_tag_group(&mut self, group_name: N, tags: Vec<Tag<N, I>>) {
		self.tag_groups.insert(group_name, tags);
	}

	/// Adds the word with the given tags
	/// If a given word already exists it is replaced
	pub fn insert_word(&mut self, word: Word<W, N, I>) {
		let current_id;
		let name = word.get_name();

		// check if a word with the given name already exists
		match self.word_map.entry(name.clone()) {
			// replace existing word
			Entry::Occupied(x) => current_id = *x.get(),
			// add a new word
			Entry::Vacant(x) => {
				current_id = self.next_id;

				// add the name into both data structures
				x.insert(current_id);
				self.word_vec.push(Some(name));

				self.next_id += 1;
				self.word_count += 1;
			}
		}

		for tag in word.to_tag_vec() {
			// get the tag's vector, if not found create it
			let vec = self.tags.entry(tag.get_name()).or_insert(Vec::new());
			// resize the vector to fit
			vec.resize(self.next_id, TagData::Empty);
			// add the tag's info
			vec[current_id] = tag.data;
		}
	}

	/// Gets the word with given id or None if nothing was found or the id was out of bounds.
	pub fn get_word_by_id(&self, id: usize) -> Option<Word<W, N, I>> {
		// find the name of the word
		let word_name =
			match self.word_vec.get(id) {
				// found the name
				Some(&Some(ref name)) => name,
				// didn't found the name
				Some(&None) => return None,
				// we were out of range
				None => {
					assert!(id >= self.next_id);
					return None;
				}
			};

		// the returned variable
		let mut word = Word::new(word_name.clone());

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
					// yep we found it
					Some(ref tag_data) => word.add_tag(
						Tag::reconstruct(tag_map_elem.0.clone(), tag_data)
					)
				}
			}

		}

		// since we have the name field checked already we know there is a word
		// therefore we can return a word with just a name and without any tags
		Some(word)
	}

	/// Removes the word with the given id.
	/// Note that since removing words is seen as a rare operation, this function doesn't free id slots, except when removing the last element, in which case calling this function is the same as calling `remove_last_word`.
	pub fn remove_word_by_id(&mut self, id: usize) {
		// if we're removing the last id might as well call the specialized function for that
		// because it saves up one id slot
		if id == self.next_id - 1 {
			return self.remove_last_word();
		}
		else if self.is_empty() {
			return;
		}

		// check if this word exists by looking at it's name
		match self.word_vec.get(id) {
			Some(&Some(ref name)) => {
				// remove it from the map
				let rem_opt = self.word_map.remove(name);
				assert_eq!(rem_opt, Some(id));
			},
			// the word doesn't exist
			Some(&None) => return,
			// the index was out of range
			None => {
				assert!(id >= self.next_id);
				return;
			}
		}
		// remove the name from the vec
		self.word_vec[id] = None;

		// go through tags
		for tag_map_elem in self.tags.iter_mut() {
			// clear the word's tag
			tag_map_elem.1.get_mut(id).map(|x| *x = TagData::Empty);
		}
		self.word_count -= 1;
	}

	/// Removes the word with the last id and frees it's id slot.
	pub fn remove_last_word(&mut self) {
		if self.is_empty() {
			return;
		}

		// remove the name information
		// since we know self is not empty this is a safe unwrap.
		if let &Some(ref name) = &self.word_vec.pop().unwrap() {
			// remove it from the map also if the name was found
			self.word_map.remove(name);
			// we know we are removing a word's tags and not just popping empty in the next loop.
			self.word_count -= 1;
		}

		// even if the name doesn't exist and therefore neither should the tags,
		// we have to pop out the extra size.

		// go through tags
		for tag_map_elem in self.tags.iter_mut() {
			// check if this vector is long enough to have a tag belonging to the last word
			if tag_map_elem.1.len() == self.next_id {
				// remove it
				tag_map_elem.1.pop();
			}
		}

		// free the id slot
		self.next_id -= 1;
	}

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get_word_by_name<T>(&self, name: T) -> Option<Word<W, N, I>>
		where T: Into<Rc<W>>,
	{
		// OPTIMIZATION: use the information that the name exists and what it is
		match self.word_map.get(&name.into()) {
			Some(&id) => self.get_word_by_id(id),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove_word_by_name<T>(&mut self, name: T)
		where T: Into<Rc<W>>,
	{
		// OPTIMIZATION: use the information that the name exists and what it is
		if let Some(&id) = self.word_map.get(&name.into()) {
			self.remove_word_by_id(id)
		}
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

	pub fn iter(&self) -> WordIter<W, N, I> {
		WordIter{ enc_ref: self, index: 0 }
	}
}

/*impl<'a, N, I,W> IntoIterator for &'a Encyclopedia<W, N, I>
	where	W: Clone + PartialEq + Eq + Hash + Debug,
			N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Debug,
{
	type Item = <WordIter<'a, W, N, I> as Iterator>::Item;
	type IntoIter = WordIter<'a, W, N, I>;

	fn into_iter(self) -> Self::IntoIter {
		WordIter{ enc_ref: self, index: 0 }
	}
}*/

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a, W, N, I>
	where	W: 'a + Clone + PartialEq + Eq + Hash + Debug,
			N: 'a + Clone + PartialEq + Eq + Hash + Debug,
			I: 'a + Clone + PartialEq + Eq + Debug,
{
	enc_ref: &'a Encyclopedia<W, N, I>,
	index: usize,
}

impl<'a, W, N, I> Iterator for WordIter<'a, W, N, I>
	where	W: Clone + PartialEq + Eq + Hash + Debug,
			N: Clone + PartialEq + Eq + Hash + Debug,
			I: Clone + PartialEq + Eq + Debug,
{
	type Item = Word<W, N, I>;

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
