use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TagData<I>
	where I: Clone + PartialEq + Eq + Debug
{
	Info(I),
	Exists,
	Empty
}

impl<I> TagData<I>
	where I: Clone + PartialEq + Eq + Debug
{
	pub fn get_info(&self) -> Option<&I> {
		match self {
			&TagData::Info(ref i) => Some(i),
			_ => None
		}
	}

	pub fn is_empty(&self) -> bool {
		*self == TagData::Empty
	}
}

// A simple tag with name and information.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag<N, I>
	where	N: Clone + PartialEq + Eq + Debug,
			I: Clone + PartialEq + Eq + Debug
{
	pub name: N,
	pub data: TagData<I>
}

impl<N, I> Tag<N, I>
	where	N: Clone + PartialEq + Eq + Debug,
			I: Clone + PartialEq + Eq + Debug
{
	pub fn new_nullary(name: N) -> Tag<N, I> {
		Tag{ name: name, data: TagData::Exists }
	}

	pub fn new_w_data(name: N, data: I) -> Tag<N, I> {
		Tag{ name: name, data: TagData::Info(data) }
	}

	pub fn reconstruct(name: &N, data: &TagData<I>) -> Tag<N, I> {
		Tag{ name: name.clone(), data: data.clone() }
	}

	pub fn get_name<'a>(&'a self) -> &'a N {
		&self.name
	}

	pub fn get_data<'a>(&'a self) -> Option<&'a I> {
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
