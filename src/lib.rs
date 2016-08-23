extern crate specs;

pub mod enc;

use std::hash::Hash;
use std::fmt::Debug;

use enc::*;

/// A marker type for any type that implements `Clone`, `PartialEq`, `Eq`, `Hash` and `Debug`.
pub trait EncSysType: Clone + PartialEq + Eq + Hash + Debug {}
impl<T: Clone + PartialEq + Eq + Hash + Debug> EncSysType for T {}

/// The master manager for the encyclopedia and entities.
pub struct EncSysMan<WordName, Tag, CompName>
	where	WordName: EncSysType,
			Tag: EncSysType,
			CompName: EncSysType,
{
	/// The encyclopedia that contains words with their associated tags.
	pub enc: Encyclopedia<WordName, Tag>,
	/// The `specs::World` that contains all of the entities and components.
	/// Notice that we are using the latest version from the git repo with the support for dynamic component types.
	pub ecs_world: specs::World<CompName>,
}

impl<WordName, Tag, CompName> EncSysMan<WordName, Tag, CompName>
		where	WordName: EncSysType,
				Tag: EncSysType,
				CompName: EncSysType,
{
	pub fn new() -> Self {
		EncSysMan{ enc: Encyclopedia::new(), ecs_world: specs::World::new_w_comp_id() }
	}
}
