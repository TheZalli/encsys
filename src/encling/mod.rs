//! Lexicographical features for `Encyclopedia`.

use enc;

//use std::collections::HashSet;
use std::fmt;
use std::error::Error;

/// A type used as the word name.
pub type WordName = String;
pub type Word = enc::Word<WordName, LingTag>;

/// An error created by an operation of an EncLing `Encyclopedia`.
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

/// A rule for formatting words into strings
pub trait WordFmtRule {
	fn fmt_word(word: &Word, f: &mut fmt::Formatter) -> Result<(), EncLingError>;
}

/// An encling word tag.
/// As generic as possible and tries to take as little assumptions about the used language as possible.
#[derive(PartialEq, Eq, Hash, Clone)]
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
trait LingEnc {

}

impl LingEnc for enc::Encyclopedia<WordName, LingTag> {

}
