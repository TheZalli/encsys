///! Contains shared utilities for the EncSys project.
use std::hash::Hash;

/// A marker type for any type that implements `Clone`, `PartialEq`, `Eq`, `Hash`.
pub trait EncSysType: Clone + PartialEq + Eq + Hash {}
impl<T: Clone + PartialEq + Eq + Hash> EncSysType for T {}
