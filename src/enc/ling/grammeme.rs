//! Contains structs for storing information about lexicographical rules used in word formatting.
use std::sync::Arc;
use std::collections::HashSet;
use std::iter::FromIterator;

/// A grammatical category, like case, person or verb tense.
/// The values of a grammatical category are called "grammemes" but in this struct's method
/// interface they are also called "values".
pub struct GrammCategory {
	category_name: Arc<String>,
	default_value: Option<Arc<String>>,
	grammeme_values: HashSet<Arc<String>>,
}

impl GrammCategory {
	/// Creates a new grammatical category.
	/// The default value is optional. If given `None`, no default value is given.
	pub fn new<T, U>(name: T, default_value: Option<T>, values: U) -> Self
		where T: AsRef<str> + Clone, U: IntoIterator<Item=T>
	{
		let str2arc = |x: T| Arc::new(x.as_ref().to_owned());
		let defval_arc = default_value.map(&str2arc);

		GrammCategory {
			category_name: Arc::new(name.as_ref().to_owned()),

			default_value: defval_arc.clone(),

			grammeme_values: FromIterator::from_iter(
				defval_arc.into_iter().chain(values.into_iter().map(&str2arc))
			),
		}
	}

	/// Returns the name of this grammatical category.
	pub fn get_name(&self) -> Arc<String> {
		self.category_name.clone()
	}

	/// Returns the default grammeme for this grammatical category.
	/// If there is no default value, `None` is returned.
	pub fn get_default_value(&self) -> Option<Arc<String>> {
		self.default_value.clone()
	}

	/// Returns all of the grammemes in this grammatical category.
	pub fn get_values(&self) -> &HashSet<Arc<String>> {
		&self.grammeme_values
	}

	/// Returns true if the given grammeme is a valid value for this grammatical category.
	pub fn is_value_valid<T: AsRef<str>>(&self, value: T) -> bool {
		self.grammeme_values.contains(&value.as_ref().to_owned())
	}

	/// Returns the reference to the given grammeme in this category or `None` if the value is not
	/// valid for this grammatical category.
	pub fn get_value<T: AsRef<str>>(&self, value: T) -> Option<Arc<String>> {
		self.grammeme_values.get(&value.as_ref().to_owned()).cloned()
	}

	/// Constructs a grammeme having the given value if it is an allowed value of this grammatical
	/// category.
	pub fn get_grammeme<T: AsRef<str>>(&self, value: T) -> Option<Grammeme> {
		if let Some(g) = self.get_value(value) {
			Some(Grammeme{
				category: self.category_name.clone(),
				value: g.clone(),
			})
		}
		else {
			None
		}
	}

}

/// A valid grammeme that also contains the information about the category it is in.
///
/// Does not contain a reference to the actual `GrammCategory` struct, just to it's name.
pub struct Grammeme {
	category: Arc<String>,
	value: Arc<String>,
}

impl Grammeme {
	/// Returns the name of the category.
	pub fn get_category_name(&self) -> Arc<String> {
		self.category.clone()
	}

	/// Returns the value. This is the textual presentation of the grammeme.
	pub fn get_value(&self) -> Arc<String> {
		self.value.clone()
	}
}
