//! Contains the lexicographical features.
use super::WordName;
use super::word::{Word, TagIter};

use std::sync::Arc;
use std::collections::HashSet;
use std::fmt;
use std::error::Error;

/// A word that has a name and associated tags.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EncWord {
	name: Arc<WordName>,
	tags: HashSet<Arc<LingTag>>,
}

impl Word for EncWord {
	type Name = WordName;
	type Tag = LingTag;

	fn new<U: Into<Arc<WordName>> >(name: U) -> Self
	{
		EncWord { name: name.into(), tags: HashSet::new() }
	}

	fn new_from_collection<U, V, W>(name: U, coll: V) -> Self
		where	U: Into<Arc<WordName>>,
				V: IntoIterator<Item = W>,
				W: Into<Arc<LingTag>>,
	{
		EncWord {
			name: name.into(),
			tags: coll.into_iter().map(&Into::into).collect(),
		}
	}

	fn get_name(&self) -> Arc<WordName> {
		self.name.clone()
	}

	fn get_tags(&self) -> HashSet<Arc<LingTag>> {
		self.tags.clone()
	}

	fn add_tag<U>(&mut self, tag: U)
		where U: Into<Arc<LingTag>>
	{
		self.tags.insert(tag.into());
	}

	fn has_tag<U>(&self, tag: U) -> bool
		where U: Into<Arc<LingTag>>
	{
		self.tags.contains(&tag.into())
	}

	fn tag_amount(&self) -> usize {
		self.tags.len()
	}

	fn is_empty(&self) -> bool {
		self.tags.is_empty()
	}

	fn iter(&self) -> TagIter<LingTag> {
		TagIter{ set_iter: self.tags.iter() }
	}
}

/// An error created by an operation of an `EncLing` `Encyclopedia`.
#[derive(Debug)]
pub enum EncLingError {
	FmtError(fmt::Error),
	InvalidTags, // TODO
}

impl fmt::Display for EncLingError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!(f.write_str(self.description()));
		match self {
			&EncLingError::FmtError(ref e) => write!(f, ": {}", e),
			_ => Ok(())
		}
	}
}

impl Error for EncLingError {
	fn description(&self) -> &str {
		match self {
			&EncLingError::FmtError(_) => "format error",
			&EncLingError::InvalidTags => "invalid word tags",
		}
	}
}

/// An `EncLing` word tag.
/// As generic as possible and tries to take as little assumptions about the used language as possible.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum LingTag {
	Type(WordName),
	UseTagGroup(WordName),
	Parent(WordName),
	HasWords(Vec<WordName>),
	Synonym(WordName),
	//Grammemes(...),
	Custom(WordName),
	CustomVec(Vec<WordName>),
	//CustomSet(HashSet<WordName>),
}

/// TODO
pub trait EncLing {

}
