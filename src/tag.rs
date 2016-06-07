use std::rc::Rc;
use std::fmt::Debug;

/// An enum that contains the tags data.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TagData<I>
	where I: Clone + PartialEq + Eq + Debug
{
	Info(Rc<I>),
	Nullary,
	Empty
}

impl<I> TagData<I>
	where I: Clone + PartialEq + Eq + Debug
{
	/// Returns the information contained by the `TagData` or `None` if there is no info.
	pub fn get_info(&self) -> Option<Rc<I>> {
		match self {
			&TagData::Info(ref i) => Some(i.clone()),
			_ => None
		}
	}

	/// Returns true if the `TagData` is empty.
	pub fn is_empty(&self) -> bool {
		*self == TagData::Empty
	}

	/// Returns true if the `TagData` exists.
	/// This is equal to the negation of `is_empty`.
	pub fn exists(&self) -> bool {
		*self != TagData::Empty
	}
}

/// A tag with name and information.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag<N, I>
	where	N: Clone + PartialEq + Eq + Debug,
			I: Clone + PartialEq + Eq + Debug
{
	pub name: Rc<N>,
	pub data: TagData<I>
}

impl<N, I> Tag<N, I>
	where	N: Clone + PartialEq + Eq + Debug,
			I: Clone + PartialEq + Eq + Debug
{
	/// Creates a new tag with no information, meaning it is a nullary tag.
	pub fn new_nullary<T>(name: T) -> Tag<N, I>
		where T: Into<Rc<N>>,
	{
		Tag{ name: name.into(), data: TagData::Nullary }
	}

	/// Creates a new tag with the given information.
	pub fn new_with_info<T, U>(name: T, info: U) -> Tag<N, I>
		where	T: Into<Rc<N>>,
				U: Into<Rc<I>>,
	{
		Tag{ name: name.into(), data: TagData::Info(info.into()) }
	}

	/// Reconstructs a tag from the given name and the given `TagData` struct.
	pub fn reconstruct<T>(name: T, data: &TagData<I>) -> Tag<N, I>
		where	T: Into<Rc<N>>,
	{
		Tag{ name: name.into(), data: data.clone() }
	}

	/// Returns the name of the tag.
	pub fn get_name(&self) -> Rc<N> {
		self.name.clone()
	}

	/// Returns the data of the tag.
	pub fn get_data(&self) -> Option<Rc<I>> {
		self.data.get_info()
	}
	/*
	pub fn into_tuple(&self) -> (N, TagData<I>) {
		(self.name.clone(), self.data.clone())
	}

	pub fn as_tuple(self) -> (N, TagData<I>) {
		(self.name, self.data)
	}*/
}
