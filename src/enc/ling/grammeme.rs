//! Contains structs for storing information about lexicographical rules used in word formatting.
use std::collections::HashSet;
use std::iter::FromIterator;
use std::borrow::Cow;

/// A grammatical category, like case, person or verb tense.
/// The values of a grammatical category are called "grammemes" but in this struct's method
/// interface they are also called "values".
pub struct GrammCategory<'a> {
	name: Cow<'a, str>,
	default_value: Option<Cow<'a, str>>,
	values: HashSet<Cow<'a, str>>,
}

impl<'a> GrammCategory<'a> {
	/// Creates a new grammatical category.
	/// The default value is optional. If given `None`, no default value is given.
	pub fn new<T1, T2, T3, U>(name: T1, default_value: Option<T2>, values: U) -> Self
		where 	T1: 'a + Into<Cow<'a, str>>,
				T2: 'a + Into<Cow<'a, str>> + Clone,
				T3: 'a + Into<Cow<'a, str>>,
				U: IntoIterator<Item=T3>,
	{
		GrammCategory {
			name: name.into(),
			default_value: default_value.clone().map(&Into::into),
			values: FromIterator::from_iter(
				default_value.into_iter().map(&Into::into)
				.chain(values.into_iter().map(&Into::into))
			),
		}
	}

	pub fn get_name(&self) -> Cow<'a, str> {
		self.name.clone()
	}

	pub fn has_default_value(&self) -> bool {
		self.default_value.is_some()
	}

	/// Returns the default grammeme for this grammatical category.
	/// If there is no default value, `None` is returned.
	pub fn get_default_value(&self) -> Option<Cow<'a, str>> {
		self.default_value.clone()
	}

	/// Returns all of the grammemes in this grammatical category.
	pub fn get_values(&self) -> &HashSet<Cow<'a, str>> {
		&self.values
	}

	/// Returns true if the given grammeme is a valid value for this grammatical category.
	pub fn is_value_valid<T: AsRef<str>>(&self, value: T) -> bool {
		self.values.contains(value.as_ref())
	}

	/// Returns the reference to the given grammeme in this category or `None` if the value is not
	/// valid for this grammatical category.
	pub fn get_value<T: AsRef<str>>(&self, value: T) -> Option<Cow<'a, str>> {
		self.values.get(value.as_ref()).map(|x| x.clone())
	}

	/// Consumes self and returns a tuple that can be used to store this into a map structure.
	pub fn into_map_entry(self) -> (String, (Option<String>, HashSet<String>)) {
		(
			self.name.into_owned(),
			(
				self.default_value.map(&Cow::into_owned),
				self.values.into_iter().map(&Cow::into_owned).collect()
			)
		)
	}

}

/// A valid grammeme that also contains the information about the category it is in.
///
/// Does not contain a reference to the actual `GrammCategory` struct, just to it's name.
pub struct Grammeme<'a> {
	cat_name: &'a str,
	value: &'a str,
}

impl<'a> Grammeme<'a> {
	/// Returns the name of the category.
	pub fn get_category_name(&self) -> &str {
		self.cat_name
	}

	/// Returns the value. This is the textual presentation of the grammeme.
	pub fn get_value(&self) -> &str {
		self.value
	}
}
