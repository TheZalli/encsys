pub mod enc;
pub mod ecs;
pub mod rules;

use std::hash::Hash;
use std::fmt::Debug;

use enc::*;
use ecs::*;

/// A container for values with associated id's and objects (tags, components, etc).
pub trait EncSysContainer<T> {
	/// Adds the given value to self. Implemented function can overwrite an existing value.
	/// Returns the id given to the value.
	fn add(&mut self, t: T) -> usize;

	/// Returns the value associated with the given id.
	fn get_by_id(&self, id: usize) -> Option<T>;

	/// Removes the value with the highest id and frees it's id-slot.
	fn remove_last_id(&mut self);

	/// Removes a value with the given id.
	/// Can leave the given id slot unused, unless the last entity is removed.
	fn remove_by_id(&mut self, id: usize);

	/// Returns the id after the last.
	/// No id should be higher than this value.
	fn get_end_id(&self) -> usize;

	/// Returns the amount of values stored.
	fn get_count(&self) -> usize;

	/// Returns true if there are no values stored.
	fn is_empty(&self) -> bool;
}

pub trait EncSysType: Clone + PartialEq + Eq {
}

impl<T: Clone + PartialEq + Eq> EncSysType for T {
}


pub struct EncSysMan<WordName, TagName, TagInfo, CompName, CompData>
	where	WordName: EncSysType + Hash + Debug,
			TagName: EncSysType + Hash + Debug,
			TagInfo: EncSysType + Debug,

			CompName: EncSysType + Hash + Debug,
			CompData: EncSysType + Debug,
			// now we're generic af
{
	enc: Encyclopedia<WordName, TagName, TagInfo>,
	ent_man: EntMan<CompName, CompData>,
}

impl<WordName, TagName, TagInfo, CompName, CompData>
	EncSysMan<WordName, TagName, TagInfo, CompName, CompData>
		where	WordName: EncSysType + Hash + Debug,
				TagName: EncSysType + Hash + Debug,
				TagInfo: EncSysType + Debug,

				CompName: EncSysType + Hash + Debug,
				CompData: EncSysType + Debug,
{
	pub fn new() -> Self {
		EncSysMan{ enc: Encyclopedia::new(), ent_man: EntMan::new() }
	}

	pub fn get_enc(&self) -> &Encyclopedia<WordName, TagName, TagInfo> {
		&self.enc
	}

	pub fn get_enc_mut(&mut self) -> &mut Encyclopedia<WordName, TagName, TagInfo> {
		&mut self.enc
	}

	pub fn get_ent_man(&self) -> &EntMan<CompName, CompData> {
		&self.ent_man
	}

	pub fn get_ent_man_mut(&mut self) -> &mut EntMan<CompName, CompData> {
		&mut self.ent_man
	}
}
