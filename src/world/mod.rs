//! Contains `EncSysWorld` struct and the helper struct for building entities, `EncEntityBuilder`.
extern crate specs;

#[cfg(test)]
mod test;

use std::ptr;

use util::EncSysType;
use enc::Encyclopedia;
use enc::Word;

/// The master manager for the encyclopedia and entities.
pub struct EncSysWorld<C: EncSysType> {
    /// The encyclopedia that contains words with their associated tags.
    pub enc: Encyclopedia,

    /// The `specs::World` that contains all of the entities and components.
    ///
    /// Notice that we are using the latest version from the git repo with the support for dynamic
    /// component types.
    pub ecs: specs::World<C>,
}

impl<C: EncSysType> EncSysWorld<C> {
    /// Creates a new empty `EncSysWorld`.
    pub fn new() -> Self {
        EncSysWorld {
            enc: Encyclopedia::new(),
            ecs: specs::World::new_w_comp_id(),
        }
    }

    fn builder(&mut self) -> EncEntityBuilder<C> {
        EncEntityBuilder { builder: self.ecs.create_now() }
    }
}

/// Can create an entity from a word.
pub trait WordToEntity {
    type CompName;

    /// Creates and stores an entity based on a word by using the function `f` and returns the
    /// created `specs::Entity` value.
    fn entity_from_word<F>(&mut self, word: Word, f: &F) -> specs::Entity
        where F: Fn(Word, &mut EncEntityBuilder<Self::CompName>);
}

impl<C: EncSysType> WordToEntity for EncSysWorld<C> {
    type CompName = C;

    fn entity_from_word<F>(&mut self, word: Word, f: &F) -> specs::Entity
        where F: Fn(Word, &mut EncEntityBuilder<C>)
    {
        // here is the builder that will construct the entity
        let mut builder = self.builder();
        // magic happens here
        f(word, &mut builder);
        // return the created entity
        builder.finish()
    }
}

/// Constructs Entities by adding components one by one.
pub struct EncEntityBuilder<'a, CompName: 'a + EncSysType> {
    builder: specs::EntityBuilder<'a, CompName>,
}

impl<'a, CompName: EncSysType> EncEntityBuilder<'a, CompName> {
    /// Adds a component with the name CompName, type T and data value to the entity.
    ///
    /// The component identification pair, which means the comp_name and T in this case, have to be
    /// registered beforehand or this will panic.
    pub fn add_comp<T: specs::Component>(&mut self, comp_name: CompName, value: T) {
        // let's create an exact copy of the builder unsafely because it doesn't implement clone
        // this should be ok since specs::EntityBuilder has just a specs::Entity and &World
        let cloned_builder =
            unsafe { ptr::read(&self.builder as *const specs::EntityBuilder<CompName>) };

        // assign the cloned builder back after changing it
        self.builder = cloned_builder.with_w_comp_id(comp_name, value);
    }

    /// Finishes the entity building.
    /// Returns the created `specs::Entity`.
    fn finish(self) -> specs::Entity {
        self.builder.build() // return the entity
    }
}
