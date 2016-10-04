//! Contains shared utilities for EncSys.

use std::hash::Hash;

/// A marker type for any type that implements `Clone`, `PartialEq`, `Eq` and `Hash`.
pub trait EncSysType: Clone + PartialEq + Eq + Hash {}
impl<T: Clone + PartialEq + Eq + Hash> EncSysType for T {}
