# EncSys

EncSys (**Enc**yclopedia **Sys**tem) is an entity-component-system framework built on top of [SPECS][specs].

The EncSys project is based on an idea of an encyclopedia of words that can be used to create entities.
The words contain information tags about what kind of entity they represent that are interpreted by a user-given rule function.

Feel free to fork the repository, ask questions or give suggestions!

## Project layout

The project is currently split into several crates described below.

### `encsys`

The core crate.
Contains imports from the other crates and is intended to be the interface to EncSys in normal use cases.

### `encsys_util`

Utility crate for all of the other EncSys crates.

### `encsys_world`

Contains the `Encyclopedia` and the `EncSysWorld` structs that are used to store and change the state of the world.

### `encling`

Contains features that specialize `Encyclopedia` struct to give lexicographical meaning to the words and formatting them.

One of the main goals of `encling` is to work with many different languages to make translation into other languages of projects using EncSys easier.

## Future

The project is still heavily a work-in-progress and I have no idea when the first real 0.1.0 release will be; and as this project is mainly for my own needs it depends on my inspiration and motivation.

[specs]: https://github.com/slide-rs/specs
