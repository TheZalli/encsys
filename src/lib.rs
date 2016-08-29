mod enc;
mod encsysman;
#[cfg(test)]
mod test;

use std::hash::Hash;

pub use encsysman::*;

/// A marker type for any type that implements `Clone`, `PartialEq`, `Eq`, `Hash` and `Debug`.
pub trait EncSysType: Clone + PartialEq + Eq + Send + Sync + Hash {}
impl<T: Clone + PartialEq + Eq + Send + Sync + Hash> EncSysType for T {}
