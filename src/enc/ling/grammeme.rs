//! Contains structs for storing information about lexicographical rules used in word formatting.
use std::collections::BTreeSet;
use std::iter::FromIterator;

/// A grammatical category, like case, person or verb tense.
/// The values of a grammatical category are called "grammemes" but in this struct's method
/// interface they are also called "values".
#[derive(Hash, PartialEq, Eq)]
pub struct GrammCategory {
	// could be optimized away
	default_value: Option<String>,
	grammeme_values: BTreeSet<String>,
}

impl GrammCategory {
	/// Creates a new grammatical category.
	/// The default value is optional. If given `None`, no default value is given.
	pub fn new<'a, U, V>(default_value: U, values: V) -> Self
		where /*T: 'a + AsRef<str> + Clone,*/
				U: Into<Option<&'a str>> + Clone,
				V: IntoIterator<Item=&'a str>,
	{
		GrammCategory {
			default_value: default_value.clone().into().map(&ToOwned::to_owned),
			grammeme_values: FromIterator::from_iter(
				default_value.into().into_iter().map(|x| x.to_owned())
				.chain(values.into_iter().map(|x| x.to_owned()) )
			),
		}
	}

	pub fn has_default_value(&self) -> bool {
		self.default_value.is_some()
	}

	/// Returns the default grammeme for this grammatical category.
	/// If there is no default value, `None` is returned.
	pub fn get_default_value(&self) -> &Option<String> {
		&self.default_value
	}

	/// Returns all of the grammemes in this grammatical category.
	pub fn get_values(&self) -> &BTreeSet<String> {
		&self.grammeme_values
	}

	/// Returns true if the given grammeme is a valid value for this grammatical category.
	pub fn is_value_valid<T: AsRef<str>>(&self, value: T) -> bool {
		self.grammeme_values.contains(value.as_ref())
	}

	/// Returns the reference to the given grammeme in this category or `None` if the value is not
	/// valid for this grammatical category.
	pub fn get_value<T: AsRef<str>>(&self, value: T) -> Option<&str> {
		self.grammeme_values.get(value.as_ref()).map(&AsRef::as_ref)
	}
}

/// A valid grammeme that also contains the information about the category it is in.
///
/// Does not contain a reference to the actual `GrammCategory` struct, just to it's name.
pub struct Grammeme<'a> {
	category: &'a str,
	value: &'a str,
}

impl<'a> Grammeme<'a> {
	/// Returns the name of the category.
	pub fn get_category_name(&self) -> &str {
		self.category
	}

	/// Returns the value. This is the textual presentation of the grammeme.
	pub fn get_value(&self) -> &str {
		self.value
	}
}
