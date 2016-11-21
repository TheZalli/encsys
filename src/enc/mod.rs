//! Contains the encyclopedia features.
mod encyclopedia;
mod word;
pub mod ling;

#[cfg(test)]
mod test;

pub use self::encyclopedia::*;
pub use self::word::*;

/// A type used as the word name.
pub type WordName = String;
