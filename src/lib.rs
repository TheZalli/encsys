//! EncSys (**Enc**yclopedia **Sys**tem) is an entity-component-system framework built on top of [SPECS](https://github.com/slide-rs/specs).
//!
//! The EncSys project is based on an idea of an encyclopedia of words that can be used to create entities, implemented as the [`enc::Encyclopedia`](enc/struct.Encyclopedia.html) struct.
//! The words contain information tags about what kind of entity they represent that are interpreted by a user-given rule function.

pub mod util;
pub mod enc;
pub mod world;
