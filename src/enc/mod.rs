//! Contains `Word` and `Encyclopedia` structs.

mod word;
mod encyclopedia;

pub use self::word::*;
pub use self::encyclopedia::*;

#[cfg(test)]
mod test;
