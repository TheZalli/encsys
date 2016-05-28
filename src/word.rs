use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::{IntoIterator, Extend};

use tag::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Word<N, I>
	where	N: PartialEq + Eq + Clone + Debug + Hash,
			I: PartialEq + Eq + Clone + Debug,
{
	tags: HashMap<N, TagData<I>>
}

impl<N, I> Word<N, I>
	where	N: PartialEq + Eq + Clone + Debug + Hash,
			I: PartialEq + Eq + Clone + Debug,
{
	/// Creates a new empty word
	pub fn new() -> Word<N, I> {
		Word{ tags: HashMap::new() }
	}

	pub fn from_tag_vec(vec: Vec<Tag<N, I>>) -> Word<N, I> {
		Word{ tags: vec.into_iter().map(&Tag::as_tuple).collect() }
	}

	pub fn to_tag_vec(self) -> Vec<Tag<N, I>> {
		self.tags.into_iter().map(|(n, i)| Tag::reconstruct(&n, &i)).collect()
	}

	pub fn add_tag(&mut self, t: Tag<N, I>) {
		let (name, info) = t.as_tuple();
		self.tags.insert(name, info);
	}

	pub fn get_tag_data(&self, name: N) -> Option<&TagData<I>> {
		self.tags.get(&name)
	}

	pub fn get_tag_info(&self, name: N) -> Option<&I> {
		self.tags.get(&name).map(&TagData::get_info).unwrap_or(None)
	}

	pub fn has_tag(&self, name: N) -> bool {
		self.tags.contains_key(&name)
	}

	pub fn is_empty(&self) -> bool {
		self.tags.is_empty()
	}
}

impl<'a, N, I> Extend<&'a Tag<N, I>> for Word<N, I>
	where	N: PartialEq + Eq + Clone + Debug + Hash,
			I: PartialEq + Eq + Clone + Debug,
{
	fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=&'a Tag<N, I>> {
		self.tags.extend(iter.into_iter().map(|t: &Tag<N, I>| t.into_tuple() ))
	}
}
