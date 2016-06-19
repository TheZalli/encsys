use std::rc::Rc;
use std::fmt::Debug;

use EncSysType;

/// A tag with name and information.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag<N, I>
	where	N: EncSysType + Debug,
			I: EncSysType + Debug
{
	pub name: Rc<N>,
	pub data: Option<Rc<I>>
}

impl<N, I> Tag<N, I>
	where	N: EncSysType + Debug,
			I: EncSysType + Debug
{
	/// Creates a new tag with no information, meaning it is a nullary tag.
	pub fn new_nullary<T>(name: T) -> Tag<N, I>
		where T: Into<Rc<N>>,
	{
		Tag{ name: name.into(), data: None }
	}

	/// Creates a new tag with the given information.
	pub fn new_with_info<T, U>(name: T, info: U) -> Tag<N, I>
		where	T: Into<Rc<N>>,
				U: Into<Rc<I>>,
	{
		Tag{ name: name.into(), data: Some(info.into()) }
	}

	/// Reconstructs a tag from the given name and the given `TagData` struct.
	pub fn reconstruct<T, U>(name: T, data: Option<U>) -> Tag<N, I>
		where	T: Into<Rc<N>>,
				U: Into<Rc<I>>,
	{
		Tag{ name: name.into(), data: data.map(&Into::into) }
	}

	/// Returns the name of the tag.
	pub fn get_name(&self) -> Rc<N> {
		self.name.clone()
	}

	/// Returns the data of the tag.
	pub fn get_data(&self) -> Option<Rc<I>> {
		self.data.clone()
	}

	/// Returns true if this tag has information and is not a nullary tag.
	pub fn has_data(&self) -> bool {
		self.data != None
	}
}
