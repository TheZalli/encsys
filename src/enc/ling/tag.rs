//! Contains the LingTag struct that is used to store a word's information.
use std::borrow::Cow;
use std::collections::BTreeSet;

/// A word tag with encyclopedic and lexicographical information.
///
/// Tries to take as little assumptions about the used language as possible.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum LingTag {
	Type(String),
	UseTagGroup(String),
	Parent(String),
	HasWords(Vec<String>),
	Synonym(String),
	//GrammemeRules(),
	Custom(String),
	CustomStr(String, Option<String>),
	CustomVec(String, Vec<String>),
	CustomSet(String, BTreeSet<String>),
}

impl<'a> From<&'a LingTag> for Cow<'a, LingTag> {
	fn from(t: &'a LingTag) -> Self {
		Cow::Borrowed(t)
	}
}

impl<'a> From<LingTag> for Cow<'a, LingTag> {
	fn from(t: LingTag) -> Self {
		Cow::Owned(t)
	}
}

impl LingTag {

}
