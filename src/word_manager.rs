use std::collections::HashMap;

use tag::*;

/// The word manager stores information about the tags associated with words.
#[derive(Debug)]
pub struct WordManager {
	// a hash map from tags to the vectors containing all of the existing tags
	tags: HashMap<TagName, Vec<Option<TagInfo>>>,
	// the next word id
	next_id: usize
}

impl WordManager {
	pub fn new() -> WordManager {
		WordManager{ tags: HashMap::new(), next_id: 0 }
	}

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
	}

	pub fn get_word_by_id(&self, id: usize) -> Vec<Tag> {
		let mut word_tags = Vec::new();
		for tag_map_elem in self.tags.iter() {
			if let Some(&Some(ref tag_info)) = tag_map_elem.1.get(id) {
				word_tags.push(Tag::reconstruct(tag_map_elem.0, tag_info));
			}
		}
		return word_tags;
	}

}
