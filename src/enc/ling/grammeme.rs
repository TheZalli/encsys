//! Contains structs for storing information about lexicographical rules used in word formatting.
use std::collections::HashSet;
use std::iter::FromIterator;

/// A grammatical category, like case, person or verb tense.
/// The values of a grammatical category are called "grammemes" but in this struct's method
/// interface they are also called "values".
pub struct GrammCategory<'a> {
	category_name: &'a str,
	default_value: Option<&'a str>,
	grammeme_values: HashSet<&'a str>,
}

impl<'a> GrammCategory<'a> {
	/// Creates a new grammatical category.
	/// The default value is optional. If given `None`, no default value is given.
	pub fn new<U>(name: &'a str, default_value: Option<&'a str>, values: U) -> Self
		where /*T: 'a + AsRef<str> + Clone,*/ U: IntoIterator<Item=&'a str>
	{
		let def_str_opt = default_value.map(|x| x.clone());
		GrammCategory {
			category_name: name.clone(),
			default_value: def_str_opt,
			grammeme_values: FromIterator::from_iter(
				def_str_opt.into_iter().chain(values.into_iter().map(|x| x.clone()) )
			),
		}
	}

	/// Returns the name of this grammatical category.
	pub fn get_name(&self) -> &str {
		self.category_name
	}

	/// Returns the default grammeme for this grammatical category.
	/// If there is no default value, `None` is returned.
	pub fn get_default_value(&self) -> Option<&str> {
		self.default_value
	}

	/// Returns all of the grammemes in this grammatical category.
	pub fn get_values(&self) -> &HashSet<&str> {
		&self.grammeme_values
	}

	/// Returns true if the given grammeme is a valid value for this grammatical category.
	pub fn is_value_valid<T: AsRef<str>>(&self, value: T) -> bool {
		self.grammeme_values.contains(value.as_ref())
	}

	/// Returns the reference to the given grammeme in this category or `None` if the value is not
	/// valid for this grammatical category.
	pub fn get_value<T: AsRef<str>>(&self, value: T) -> Option<&str> {
		self.grammeme_values.get(value.as_ref()).map(|x| *x)
	}

	/// Constructs a grammeme having the given value if it is an allowed value of this grammatical
	/// category.
	pub fn get_grammeme<T: AsRef<str>>(&self, value: T) -> Option<Grammeme> {
		if let Some(g) = self.get_value(value) {
			Some(Grammeme{
				category: self.category_name,
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
