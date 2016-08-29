extern crate specs;

//use std::sync::Arc;
use std::any::Any;
//use std::fmt::Debug;
use std::ptr;

use enc::*;
use super::EncSysType;


// A wrapper struct for the word name so that we can implement `specs::Component` for it.
pub struct WordNameCompWrap<N>(N);

impl<N: Any + Send + Sync> specs::Component for WordNameCompWrap<N> {
	type Storage = specs::VecStorage<Self>;
}

/// The master manager for the encyclopedia and entities.
pub struct EncSysMan<WordName, Tag, CompName>
	where	WordName: EncSysType + Any,
			Tag: EncSysType,
			CompName: EncSysType,
{
	/// The encyclopedia  that contains words with their associated tags.
	pub enc: Encyclopedia<WordName, Tag>,
	/// The `specs::World` that contains all of the entities and components.
	///
	/// Notice that we are using the latest version from the git repo with the support for dynamic
	/// component types.
	pub ecs: specs::World<CompName>,
}

impl<WordName, Tag, CompName> EncSysMan<WordName, Tag, CompName>
	where	WordName: EncSysType + Any,
			Tag: EncSysType,
			CompName: EncSysType,
{
	/// Creates a new empty `EncSysMan`.
	pub fn new() -> Self {
		EncSysMan {
			enc: Encyclopedia::new(),
			ecs: specs::World::new_w_comp_id()
		}
	}

	/// Creates and stores an entity based on a word by using the function `f` and returns the
	/// created `specs::Entity` value.
	///
	/// This is the special feature that `EncSysMan` was built to do.
	pub fn entity_from_word<F>(&mut self, word: Word<WordName, Tag>, f: &F) -> specs::Entity
		where F: Fn(Word<WordName, Tag>, &mut MyEntityBuilder<CompName>)
	{
		// here is the builder that will construct the entity
		let mut builder = self.builder();
		// magic happens here
		f(word, &mut builder);
		// return the created entity
		builder.finish()
	}

	fn builder(&mut self) -> MyEntityBuilder<CompName>
	{
		MyEntityBuilder {
			builder: self.ecs.create_now(),
		}
	}
}

/// Works similarly to the struct `specs::EntityBuilder`.
pub struct MyEntityBuilder<'a, CompName: 'a + EncSysType> {
	builder: specs::EntityBuilder<'a, CompName>,
}

impl<'a, CompName> MyEntityBuilder<'a, CompName>
	where	CompName: EncSysType
{
	/// Adds a component with the name CompName, type T and data value to the entity.
	///
	/// The component identification pair, which means the comp_name and T in this case, have to be
	/// registered beforehand or this will panic.
	pub fn add_comp<T: specs::Component>(&mut self, comp_name: CompName, value: T) {
		// let's create an exact copy of the builder unsafely because it doesn't implement clone
		// this should be ok since specs::EntityBuilderBuilder has just a specs::Entity and &World
		let cloned_builder = unsafe {
			ptr::read(&self.builder as *const specs::EntityBuilder<CompName> )
		};

		self.builder = cloned_builder.with_w_comp_id(comp_name, value);
	}

	/// Finishes the entity building.
	/// Returns the created `specs::Entity`.
	fn finish(self) -> specs::Entity {
		self.builder.build() // return the entity
	}
}
