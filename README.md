# EncSys

EncSys (**Enc**yclopedia **Sys**tem) is an entity-component system framework built on top of [SPECS][specs].

The most important type in EncSys is the manager struct `EncSysWorld`.
In addition of entities and components `EncSysWorld` contains an encyclopedia of words that behave as entity archetypes and their names.

The words contain tags that can contain information about the entities they represent and how to use the word lexicographically.

[specs]: https://github.com/slide-rs/specs
