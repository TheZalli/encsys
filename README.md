# EncSys

EncSys (**Enc**yclopedia **Sys**tem) is an entity-component-system framework built on top of [SPECS](https://github.com/slide-rs/specs).

The EncSys project is based on an idea of an encyclopedia of words that can be used to create entities.
The words contain information tags about what kind of entity they represent that are interpreted by a user-given rule function.

Feel free to fork the repository, ask questions or give suggestions!

### Current state

The project is still heavily a work-in-progress.
Encyclopedia and EncSys World parts have been fleshed out pretty well.

I am still not sure about the project layout, so expect some large scale restructuring of the code and namespaces.

### Future

I have no idea when the first real 0.1.0 release on crates.io will be, since this project is mainly for my own needs and it's development speed depends on my inspiration and motivation until if I get other contributors.

## Project layout

The crate is currently split into several modules described below.

### util

Utility module for all of the other EncSys modules.

### enc

Contains the encyclopedia features.

`Encyclopedia` struct contains `Word`s that consist of a name and a set of tags.

One of the main goals of enc is to work with many different languages to make translations of projects using EncSys into other languages easier.

Currently at early stage of development.

### world

Contains the the `EncSysWorld` struct that stores and changes the state of the world.

`EncSysWorld` owns an `Encyclopedia` for storing words and a `specs::World` for storing entities and their components.
