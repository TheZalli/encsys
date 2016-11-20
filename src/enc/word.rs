//! Contains the `Word` trait and an iterator for it's tags.
use std::collections::{HashSet, hash_set};
use std::iter::Iterator;
use std::sync::Arc;

/// A word that has a name and a set of tags.
pub trait Word {
	/// The words' name. Owned datatype, like `String`.
	type Name;

	/// The words' tag. Also an owned datatype like `Name`.
	type Tag;

	/// Creates a new empty word.
	fn new<U: Into<Arc<Self::Name>> >(name: U) -> Self;

	/// Creates a word from a name and a collection of words.
	fn new_from_collection<U, V, W>(name: U, coll: V) -> Self
		where	U: Into<Arc<Self::Name>>,
				V: IntoIterator<Item = W>,
				W: Into<Arc<Self::Tag>>;

	/// Returns the name of the word.
	fn get_name(&self) -> Arc<Self::Name>;

	// Returns the tags of the word.
	fn get_tags(&self) -> HashSet<Arc<Self::Tag>>;

	/// Adds the given tag to the word, replacing any previous tag with the same value.
	fn add_tag<U>(&mut self, tag: U)
		where U: Into<Arc<Self::Tag>>;

	/// Returns true if the word has the given tag.
	fn has_tag<U>(&self, tag: U) -> bool
		where U: Into<Arc<Self::Tag>>;

	/// Returns the amount of tags stored.
	fn tag_amount(&self) -> usize;

	/// Returns true if the word has no tags.
	fn is_empty(&self) -> bool;

	/// Returns an iterator over the tags of the word.
	fn iter(&self) -> TagIter<Self::Tag>;
}

/// An iterator over the tags of a word.
/// Implemented as a wrapper for the `hash_set`'s iterator.
pub struct TagIter<'a, T: 'a> {
	pub set_iter: hash_set::Iter<'a, Arc<T>>
}

impl<'a, T> Iterator for TagIter<'a, T> {
	type Item = Arc<T>;
	fn next(&mut self) -> Option<Self::Item> {
		self.set_iter.next().cloned()
	}
}
