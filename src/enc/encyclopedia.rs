use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::Iterator;
use std::rc::Rc;

use {EncSysContainer, EncSysType};
use enc::tag::*;
use enc::word::*;

/// The word manager stores information about the tags associated with words.
#[derive(Debug)]
pub struct Encyclopedia<W, N, I>
	where	W: EncSysType + Hash + Debug,
			N: EncSysType + Hash + Debug,
			I: EncSysType + Debug,
{
	// an association from word names into their ids/indexes
	word_map: HashMap<Rc<W>, usize>,
	// an association from id's into their word names
	word_vec: Vec<Option<Rc<W>>>,
	// a hash map from tags to the vectors containing all of the existing tags
	// to improve memory consumption, optimize the order of the values to minimize length.
	tags: HashMap<Rc<N>, Vec<Option<Option<Rc<I>>> >>,
	// a group of tags using a single tag's name, used to avoid copying common tag combinations
	tag_groups: HashMap<N, Vec<Tag<N, I>>>,
	// the id of the next word that is added
	next_id: usize,
	// the amount of words in the encyclopedia
	// if no vacant slots have been created, this is next_id - 1
	word_count: usize,
}

impl<W, N, I> Encyclopedia<W, N, I>
	where	W: EncSysType + Hash + Debug,
			N: EncSysType + Hash + Debug,
			I: EncSysType + Debug,
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

	/// Returns a word with the given name or `None` if no such word was found.
	pub fn get_by_name<T>(&self, name: T) -> Option<Word<W, N, I>>
		where T: Into<Rc<W>>,
	{
		match self.word_map.get(&name.into()) {
			// OPTIMIZATION: use the information that the name exists and what it is
			Some(&id) => self.get_by_id(id),
			None => None,
		}
	}

	/// Removes the word with the given name.
	pub fn remove_by_name<T>(&mut self, name: T)
		where T: Into<Rc<W>>,
	{
		// OPTIMIZATION: use the information that the name exists and what it is
		if let Some(&id) = self.word_map.get(&name.into()) {
			self.remove_by_id(id)
		}
	}

	pub fn iter(&self) -> WordIter<W, N, I> {
		WordIter{ enc_ref: self, index: 0 }
	}
}


impl<W, N, I> EncSysContainer<Word<W, N, I>> for Encyclopedia<W, N, I>
	where	W: EncSysType + Hash + Debug,
			N: EncSysType + Hash + Debug,
			I: EncSysType + Debug,
{
	/// If a given word already exists, it is replaced.
	fn add(&mut self, word: Word<W, N, I>) -> usize {
		let current_id;
		let name = word.get_name();

		// check if a word with the given name already exists
		match self.word_map.entry(name.clone()) {
			// replace existing word
			Entry::Occupied(x) => current_id = *x.get(),
			// add a new word
			Entry::Vacant(x) => {
				current_id = self.next_id;

				assert_eq!(current_id, self.word_vec.len());

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
			vec.resize(self.next_id, None);
			// add the tag's info
			vec[current_id] = Some(tag.data);
		}

		return current_id;
	}

	fn get_by_id(&self, id: usize) -> Option<Word<W, N, I>> {
		// find the name of the word
		let word_name =
			match self.word_vec.get(id) {
				// found the name
				Some(&Some(ref name)) => name,
				// didn't found the name
				_ => return None,
			};

		// the returned variable
		let mut word = Word::new(word_name.clone());

		// go through known tags
		for tag_map_elem in self.tags.iter() {
			// check if the tag is a group
			// currently this fails silently
			if let Some(ref tag_vec) = self.tag_groups.get(tag_map_elem.0) {
				// the tag_map_elem.1, which is the information of the tag is ignored
				for tag in tag_vec.iter() {
					// add the tag only if there is already none.
					// this is done because 'normally' added tags are given higher priority over
					// group tags so they overwrite them.
					word.add_new_tag(tag.clone());
				}
			}
			else {
				// check if we found data for the tag associated wit this word
				match tag_map_elem.1.get(id) {
					// yep we found it
					Some(&Some(ref tag_data)) => word.add_tag(
						Tag::reconstruct(tag_map_elem.0.clone(), (*tag_data).clone())
					),
					// nope didn't find
					_ => {},
				}
			}

		}

		// since we have the name field checked already we know there is a word
		// therefore we can return a word with just a name and without any tags
		Some(word)
	}

	fn remove_by_id(&mut self, id: usize) {
		// if we're removing the last id might as well call the specialized function for that
		// because it saves up one id slot
		if id == self.next_id - 1 {
			return self.remove_last_id();
		}
		else if self.is_empty() {
			return;
		}

		// check if this word exists by looking for it's name
		match self.word_vec.get_mut(id) {
			Some(ref mut val @ &mut Some(_)) => {
				// remove it from the map
				let rem_opt = self.word_map.remove(&val.clone().unwrap());
				// assert that the word_map mirrors the word_vec
				assert_eq!(rem_opt, Some(id));

				// remove the name from the vec
				**val = None;
			},
			// the word doesn't exist or the index was out of range
			_ => return,
		}

		// go through tags
		for tag_map_elem in self.tags.iter_mut() {
			// clear the word's tag
			tag_map_elem.1.get_mut(id).map(|x| *x = None);
		}
		self.word_count -= 1;
	}

	fn remove_last_id(&mut self) {
		// return if there is nothing to remove
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

	fn get_end_id(&self) -> usize {
		self.next_id
	}

	fn get_count(&self) -> usize {
		self.word_count
	}

	fn is_empty(&self) -> bool {
		self.word_count == 0
	}
}

/*impl<'a, N, I,W> IntoIterator for &'a Encyclopedia<W, N, I>
	where	W: EncSysType + Hash + Debug,
			N: EncSysType + Hash + Debug,
			I: EncSysType + Debug,
{
	type Item = <WordIter<'a, W, N, I> as Iterator>::Item;
	type IntoIter = WordIter<'a, W, N, I>;

	fn into_iter(self) -> Self::IntoIter {
		WordIter{ enc_ref: self, index: 0 }
	}
}*/

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a, W, N, I>
	where	W: 'a + EncSysType + Hash + Debug,
			N: 'a + EncSysType + Hash + Debug,
			I: 'a + EncSysType + Debug,
{
	enc_ref: &'a Encyclopedia<W, N, I>,
	index: usize,
}

impl<'a, W, N, I> Iterator for WordIter<'a, W, N, I>
	where	W: EncSysType + Hash + Debug,
			N: EncSysType + Hash + Debug,
			I: EncSysType + Debug,
{
	type Item = Word<W, N, I>;

	fn next(&mut self) -> Option<Self::Item> {
		// reached the end
		if self.index >= self.enc_ref.get_end_id() {
			return None;
		}

		match self.enc_ref.get_by_id(self.index) {
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
