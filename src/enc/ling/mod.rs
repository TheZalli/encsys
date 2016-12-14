//! Contains the encyclopedias lexicographical features.

mod tag;
pub mod grammeme;
pub mod error;

#[cfg(test)]
mod test;

pub use self::tag::*;
pub use self::error::*;
